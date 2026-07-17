mod abstractness;
mod functions;
mod redeclarations;

use std::sync::Arc;

use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::NodeIdentity;

use self::abstractness::AbstractSlot;
use self::redeclarations::MemberKind;
use crate::binder::{Binder, Definition};
use crate::context::{ContractData, ContractLinearisations, FileNodeMapper};
use crate::types::TypeRegistry;

/// In this pass we walk every contract's and interface's linearised bases once,
/// accumulating the members visible across the hierarchy. From that single walk
/// we, for each type:
///
/// - report `IdentifierRedeclaration` for a member that illegally redeclares a
///   same-named member inherited from a base (see [`redeclarations`]);
/// - report `ContractShouldBeAbstract` for a non-`abstract` contract that leaves
///   a function or modifier (declared in it or inherited from a base contract or
///   interface) unimplemented (see [`abstractness`]); and
/// - pre-compute the collections of functions, state variables, errors and
///   events visible in its hierarchy, stored in a `ContractData` (see
///   [`functions`] for the function list).
///
/// The work is driven by a [`Lineariser`], one per contract/interface — the
/// per-type analogue of the per-file `Pass` structs the other passes use. Its
/// state lives here, while each check's logic and auxiliary types live in the
/// submodule named above.
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
                let linearisations = Lineariser::compute(
                    binder,
                    types,
                    file_node_mapper,
                    diagnostics,
                    &mut reported,
                    *definition_id,
                );
                linearisations_by_id.insert(*definition_id, linearisations);
            }
            // Interfaces don't get a `ContractData` entry, but their members
            // still take part in redeclaration checks, so we walk them too and
            // discard the (mostly empty) collections they produce.
            Definition::Interface(_) => {
                Lineariser::compute(
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

/// Per-contract/interface state and context for the pass. Its bases are folded
/// in most-base-first, so that a redeclaration is reported on the more-derived
/// member (matching solc), inherited members are checked before the deriving
/// type's own, and state variables, errors and events come out in
/// base-to-derived source order.
///
/// The per-check logic hangs off this type as methods defined in the sibling
/// submodules ([`redeclarations`], [`abstractness`], [`functions`]).
struct Lineariser<'a> {
    binder: &'a Binder,
    types: &'a TypeRegistry,
    file_node_mapper: &'a FileNodeMapper,
    diagnostics: &'a mut DiagnosticCollection,
    reported: &'a mut Set<NodeId>,

    /// The type being linearised (the head of the linearisation).
    definition_id: NodeId,
    /// The IR of the contract being linearised, or `None` for an interface. The
    /// abstractness check only applies to contracts.
    contract: Option<&'a ir::ContractDefinition>,

    /// The kind of member occupying each name so far, for the redeclaration
    /// check. Names are borrowed from the definitions (which live in the binder
    /// for the whole walk), and the values are `Copy`, so tracking a member costs
    /// no allocation.
    members_by_name: Map<&'a str, MemberKind>,
    /// Every function/modifier visible in the hierarchy so far together with
    /// whether it is implemented, for the abstractness check.
    abstract_slots: Vec<AbstractSlot<'a>>,

    /// Functions gathered per base so they can be flattened most-derived-first.
    functions_per_base: Vec<Vec<ir::FunctionDefinition>>,
    state_variables: Vec<ir::StateVariableDefinition>,
    errors: Vec<ir::ErrorDefinition>,
    events: Vec<ir::EventDefinition>,
}

impl<'a> Lineariser<'a> {
    /// Linearises `definition_id`, emitting its redeclaration and abstractness
    /// diagnostics and returning its member collections.
    fn compute(
        binder: &'a Binder,
        types: &'a TypeRegistry,
        file_node_mapper: &'a FileNodeMapper,
        diagnostics: &'a mut DiagnosticCollection,
        reported: &'a mut Set<NodeId>,
        definition_id: NodeId,
    ) -> ContractLinearisations {
        let Some(linearised_bases) = binder.get_linearised_bases(definition_id) else {
            return ContractLinearisations::default();
        };
        let contract = match binder.find_definition_by_id(definition_id) {
            Some(Definition::Contract(contract)) => Some(&contract.ir_node),
            _ => None,
        };

        let mut lineariser = Lineariser {
            binder,
            types,
            file_node_mapper,
            diagnostics,
            reported,
            definition_id,
            contract,
            members_by_name: Map::default(),
            abstract_slots: Vec::new(),
            functions_per_base: Vec::new(),
            state_variables: Vec::new(),
            errors: Vec::new(),
            events: Vec::new(),
        };

        for base_id in linearised_bases.iter().rev() {
            lineariser.fold_base(*base_id);
        }
        lineariser.report_abstractness();

        ContractLinearisations {
            functions: lineariser.linearise_functions(),
            state_variables: lineariser.state_variables,
            errors: lineariser.errors,
            events: lineariser.events,
        }
    }

    /// Folds one base into the running state: collects its members, runs the
    /// redeclaration check against everything inherited so far, then records the
    /// base's members. Recording only after the check keeps members declared
    /// together in one base from clashing with each other (a same-scope concern
    /// handled in `p1`), only with inherited ones.
    fn fold_base(&mut self, base_id: NodeId) {
        let (members, from_interface) = match self.binder.find_definition_by_id(base_id) {
            Some(Definition::Contract(contract)) => (&contract.ir_node.members, false),
            Some(Definition::Interface(interface)) => (&interface.ir_node.members, true),
            _ => unreachable!("base should be a contract or interface"),
        };
        // The head of the linearisation is the type we're computing for; every
        // other entry is one of its proper bases.
        let declared_here = base_id == self.definition_id;

        let mut base_functions = Vec::new();
        // The definitions of this base's named members, gathered once and reused
        // by both checks below.
        let mut member_definitions = Vec::new();

        for member in members {
            match member {
                // Interfaces don't contribute functions to the linearisation:
                // they must be implemented by inheriting contracts (enforced by
                // the abstractness check).
                ir::ContractMember::FunctionDefinition(function)
                    if !from_interface
                        && matches!(
                            function.kind,
                            ir::FunctionKind::Regular
                                | ir::FunctionKind::Fallback
                                | ir::FunctionKind::Receive
                        ) =>
                {
                    base_functions.push(Arc::clone(function));
                }
                // Interfaces don't have state variables in Solidity.
                ir::ContractMember::StateVariableDefinition(state_variable) if !from_interface => {
                    self.state_variables.push(Arc::clone(state_variable));
                }
                ir::ContractMember::ErrorDefinition(error) => self.errors.push(Arc::clone(error)),
                ir::ContractMember::EventDefinition(event) => self.events.push(Arc::clone(event)),
                _ => {}
            }

            if let Some(definition) = member_definition(self.binder, member) {
                member_definitions.push(definition);
            }
        }

        self.check_redeclarations(&member_definitions, declared_here);
        if self.contract.is_some() {
            // NOTE: we could also skip `record_abstract` if
            // `self.contract.is_abstract`, and *only if* `record_abstract` does
            // nothing more than building `abstract_slots`. Adding diagnostics
            // checks there would break this premise.
            self.record_abstract(&member_definitions);
        }
        self.record_members(&member_definitions);
        self.functions_per_base.push(base_functions);
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
