mod abstractness;
mod linearisations;
mod redeclarations;

use std::sync::Arc;

use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::NodeIdentity;

use self::abstractness::AbstractSlots;
use self::linearisations::compute_linearisations;
use self::redeclarations::MemberKind;
use crate::binder::{Binder, Definition};
use crate::context::{ContractData, FileNodeMapper};
use crate::types::TypeRegistry;

/// In this pass we walk every contract's and interface's linearised bases and:
///
/// - check the members of the hierarchy, reporting `IdentifierRedeclaration`
///   for a member that illegally redeclares a same-named member inherited from
///   a base (see [`redeclarations`]), and `ContractShouldBeAbstract` for a
///   non-`abstract` contract that leaves a function or modifier unimplemented
///   (see [`abstractness`]); and
/// - pre-compute, per contract, the collections of functions, state variables,
///   errors and events visible in its hierarchy, stored in a `ContractData`
///   (see [`linearisations`]).
///
/// The two are independent traversals with different needs: the checks fold
/// the bases' members into per-name state, driven by a [`HierarchyChecker`],
/// the per-type analogue of the per-file `Pass` structs the other passes use,
/// while the collections just filter and gather IR members. Interfaces only
/// take part in the checks.
///
/// This pass depends on all definitions being fully typed, which is accomplished
/// in the previous pass. The next pass is not strictly dependant on the result
/// of this pass. Eventually we could use the linearisation information produced
/// here to aid in expressions/statements resolution and typing, but it's fully
/// independent for now.
pub fn run(
    binder: &Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) -> ContractData {
    let mut contracts = Vec::new();
    let mut linearisations_by_id = Map::default();
    // A redeclaration in a shared base is visited once per descendant, so we
    // dedup by the redeclaring member's node to report each clash only once.
    // The emitted diagnostic is fully determined by that node, so the result is
    // independent of the order in which we visit contracts.
    let mut reported = Set::default();

    for (definition_id, definition) in binder.definitions() {
        match definition {
            Definition::Contract(contract) => {
                contracts.push(Arc::clone(&contract.ir_node));
                HierarchyChecker::check(
                    binder,
                    types,
                    file_node_mapper,
                    diagnostics,
                    &mut reported,
                    *definition_id,
                );
                let linearisations = compute_linearisations(binder, types, *definition_id);
                linearisations_by_id.insert(*definition_id, linearisations);
            }
            // Interfaces don't get a `ContractData` entry, but their members
            // still take part in redeclaration checks.
            Definition::Interface(_) => {
                HierarchyChecker::check(
                    binder,
                    types,
                    file_node_mapper,
                    diagnostics,
                    &mut reported,
                    *definition_id,
                );
            }
            _ => {}
        }
    }

    ContractData::new(contracts, linearisations_by_id)
}

/// Per-contract/interface state and context for the hierarchy member checks.
/// The type's bases are folded in most-base-first, so that a redeclaration is
/// reported on the more-derived member (matching solc) and inherited members
/// are checked before the deriving type's own.
///
/// The per-check logic hangs off this type as methods defined in the sibling
/// submodules ([`redeclarations`], [`abstractness`]).
struct HierarchyChecker<'a> {
    binder: &'a Binder,
    types: &'a TypeRegistry,
    file_node_mapper: &'a FileNodeMapper,
    diagnostics: &'a mut DiagnosticCollection,
    reported: &'a mut Set<NodeId>,

    /// The type being checked (the head of the linearisation).
    definition_id: NodeId,
    /// The IR of the contract being checked, or `None` for an interface. The
    /// abstractness check only applies to contracts.
    contract: Option<&'a ir::ContractDefinition>,

    /// The kind of member occupying each name so far, for the redeclaration
    /// check. Names are borrowed from the definitions (which live in the binder
    /// for the whole walk), and the values are `Copy`, so tracking a member costs
    /// no allocation.
    members_by_name: Map<&'a str, MemberKind>,
    /// Every function/modifier visible in the hierarchy so far together with
    /// whether it is implemented, for the abstractness check. Grouped by name,
    /// so recording a member only ever compares it against same-named slots.
    abstract_slots: Map<&'a str, AbstractSlots<'a>>,
}

impl<'a> HierarchyChecker<'a> {
    /// Checks `definition_id`'s hierarchy, emitting its redeclaration and
    /// abstractness diagnostics.
    fn check(
        binder: &'a Binder,
        types: &'a TypeRegistry,
        file_node_mapper: &'a FileNodeMapper,
        diagnostics: &'a mut DiagnosticCollection,
        reported: &'a mut Set<NodeId>,
        definition_id: NodeId,
    ) {
        let Some(linearised_bases) = binder.get_linearised_bases(definition_id) else {
            return;
        };
        let contract = match binder.find_definition_by_id(definition_id) {
            Some(Definition::Contract(contract)) => Some(&contract.ir_node),
            _ => None,
        };

        let mut checker = HierarchyChecker {
            binder,
            types,
            file_node_mapper,
            diagnostics,
            reported,
            definition_id,
            contract,
            members_by_name: Map::default(),
            abstract_slots: Map::default(),
        };

        for base_id in linearised_bases.iter().rev() {
            checker.fold_base(*base_id);
        }
        checker.report_abstractness();
    }

    /// Folds one base into the running state: runs the redeclaration check of
    /// its members against everything inherited so far, then records them.
    /// Recording only after the check keeps members declared together in one
    /// base from clashing with each other (a same-scope concern handled in
    /// `p1`), only with inherited ones.
    fn fold_base(&mut self, base_id: NodeId) {
        let binder = self.binder;
        let members = match binder.find_definition_by_id(base_id) {
            Some(Definition::Contract(contract)) => &contract.ir_node.members[..],
            Some(Definition::Interface(interface)) => &interface.ir_node.members[..],
            _ => unreachable!("base should be a contract or interface"),
        };
        // The head of the linearisation is the type we're computing for; every
        // other entry is one of its proper bases.
        let declared_here = base_id == self.definition_id;

        // The definitions of this base's named members, gathered once and
        // shared by the checks below.
        let mut member_definitions = Vec::with_capacity(members.len());
        member_definitions.extend(
            members
                .iter()
                .filter_map(|member| member_definition(binder, member)),
        );

        self.check_redeclarations(&member_definitions, declared_here);
        // `report_abstractness` ignores the slots of `abstract` contracts, so
        // don't bother building them. This holds *only while* `record_abstract`
        // does nothing more than build `abstract_slots`: if it ever emits
        // diagnostics, this gate must be lifted.
        if self.contract.is_some_and(|contract| !contract.is_abstract) {
            self.record_abstract(&member_definitions);
        }
        // The head is folded last, so nothing is ever checked against its
        // members: recording them would be dead work.
        if !declared_here {
            self.record_members(&member_definitions);
        }
    }
}

/// Returns the `Definition` naming `member`, or `None` for members that don't
/// introduce a name into the contract namespace (using directives, and unnamed
/// functions such as constructors, fallback and receive).
fn member_definition<'a>(
    binder: &'a Binder,
    member: &ir::ContractMember,
) -> Option<&'a Definition> {
    member
        .node_id()
        .and_then(|node_id| binder.find_definition_by_id(node_id))
}
