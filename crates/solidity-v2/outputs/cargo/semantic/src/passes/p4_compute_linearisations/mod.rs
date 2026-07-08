use std::cmp::Ordering;
use std::sync::Arc;

use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::diagnostics::kinds::resolution::IdentifierRedeclaration;
use slang_solidity_v2_common::diagnostics::kinds::structure::ContractShouldBeAbstract;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::{ContractData, ContractLinearisations, FileNodeMapper};
use crate::types::{TypeId, TypeRegistry};

/// In this pass we walk every contract's and interface's linearised bases once,
/// accumulating the members visible across the hierarchy. From that single walk
/// we both:
///
/// - report `IdentifierRedeclaration` for a member that illegally redeclares a
///   same-named member inherited from a base (Solidity allows overloading
///   functions/events and overriding functions/modifiers; see
///   [`MemberKind`]);
/// - report `ContractShouldBeAbstract` for a non-`abstract` contract that
///   leaves a function or modifier (declared in it or inherited from a base
///   contract or interface) unimplemented; and
/// - pre-compute, for each contract, the collections of functions, state
///   variables, errors and events visible in its hierarchy, storing them in a
///   `ContractData`.
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
                let linearisations = compute_linearised_members(
                    binder,
                    types,
                    *definition_id,
                    file_node_mapper,
                    diagnostics,
                    &mut reported,
                );
                linearisations_by_id.insert(*definition_id, linearisations);
            }
            // Interfaces don't get a `ContractData` entry, but their members
            // still take part in redeclaration checks, so we walk them too and
            // discard the (mostly empty) collections they produce.
            Definition::Interface(_) => {
                compute_linearised_members(
                    binder,
                    types,
                    *definition_id,
                    file_node_mapper,
                    diagnostics,
                    &mut reported,
                );
            }
            _ => {}
        }
    }

    ContractData::new(contracts, linearisations_by_id)
}

/// Walks `definition_id`'s linearised bases once, both reporting inherited
/// redeclarations and building the contract's linearised member collections.
///
/// The bases are visited most-base-first, so that a redeclaration is reported
/// on the more-derived member (matching solc) and state variables, errors and
/// events come out in base-to-derived source order.
#[allow(clippy::too_many_lines)]
fn compute_linearised_members(
    binder: &Binder,
    types: &TypeRegistry,
    definition_id: NodeId,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
    reported: &mut Set<NodeId>,
) -> ContractLinearisations {
    let Some(linearised_bases) = binder.get_linearised_bases(definition_id) else {
        return ContractLinearisations::default();
    };

    // The abstract-completeness check only applies to contracts: interfaces and
    // libraries are allowed to leave members unimplemented. When computing for a
    // contract, `abstract_slots` tracks every function/modifier visible in its
    // hierarchy together with whether it has an implementation.
    let contract = match binder.find_definition_by_id(definition_id) {
        Some(Definition::Contract(contract)) => Some(contract),
        _ => None,
    };
    let mut abstract_slots: Vec<AbstractSlot> = Vec::new();

    // The kind of member occupying each name so far; a base contributes only
    // the members that derived types inherit. A member redeclares an inherited
    // one when its kind can't coexist with the name's slot (see [`MemberKind`]).
    // Names are borrowed from the definitions (which live in the binder for the
    // whole walk), and the values are `Copy`, so tracking a member costs no
    // allocation.
    let mut visible_members_by_name: Map<&str, MemberKind> = Map::default();

    let mut state_variables = Vec::new();
    let mut errors = Vec::new();
    let mut events = Vec::new();
    // Functions are gathered per base so `linearise_functions` can later flatten
    // them most-derived-first.
    let mut functions_per_base: Vec<Vec<ir::FunctionDefinition>> = Vec::new();

    for base_id in linearised_bases.iter().rev() {
        let (members, from_interface) = match binder.find_definition_by_id(*base_id) {
            Some(Definition::Contract(contract)) => (&contract.ir_node.members, false),
            Some(Definition::Interface(interface)) => (&interface.ir_node.members, true),
            _ => continue,
        };
        // The head of the linearisation is the type we're computing for; every
        // other entry is one of its proper bases.
        let declared_here = *base_id == definition_id;

        let mut base_functions = Vec::new();
        // The definitions of this base's named members, gathered in a single
        // pass and reused below to both check for and record redeclarations.
        let mut member_definitions = Vec::new();

        for member in members {
            match member {
                // Interfaces don't contribute functions to the linearisation:
                // they must be implemented by inheriting contracts (enforced
                // elsewhere).
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
                    state_variables.push(Arc::clone(state_variable));
                }
                ir::ContractMember::ErrorDefinition(error) => errors.push(Arc::clone(error)),
                ir::ContractMember::EventDefinition(event) => events.push(Arc::clone(event)),
                _ => {}
            }

            if let Some(definition) = member_definition(binder, member) {
                member_definitions.push(definition);
            }
        }

        // Check this base's members against everything inherited from
        // less-derived bases. Only members in this type's namespace can clash:
        // its own declarations (even `private` ones, which still shadow a
        // visible inherited member), plus a proper base's members that are
        // visible to it. A proper base's own private member isn't inherited, so
        // it never clashes.
        for definition in &member_definitions {
            if !declared_here && !definition.is_internally_visible() {
                continue;
            }
            let name = definition.identifier().unparse();
            let Some(inherited) = visible_members_by_name.get(name) else {
                continue;
            };
            if inherited.clashes_with(definition) && reported.insert(definition.node_id()) {
                let file_id = file_node_mapper.file_id_from_node_id(definition.node_id());
                diagnostics.push(
                    file_id.to_owned(),
                    definition.identifier().range.clone(),
                    IdentifierRedeclaration,
                );
            }
        }

        for definition in member_definitions {
            // Fold this base's functions and modifiers into the abstract-slot set.
            // Since bases are visited most-base-first, a member declared here is
            // more-derived than anything already recorded, so it overrides (and
            // updates the implementation status of) the matching slot, mirroring
            // solc's base-to-derived overwrite of its unimplemented-declaration map.
            if contract.is_some() {
                if let Some(candidate) = abstract_slot(binder, definition) {
                    merge_abstract_slot(types, &mut abstract_slots, candidate);
                }
            }

            // Record this base's inheritable members so more-derived bases are
            // checked against them. Private members (and external functions) aren't
            // inherited, so reusing their name is allowed. Recording only after the
            // checks above keeps members declared together in the same base from
            // clashing with each other (a same-scope concern handled in `p1`).
            if definition.is_internally_visible() {
                let kind = MemberKind::of(definition);
                visible_members_by_name
                    .entry(definition.identifier().unparse())
                    .and_modify(|slot| *slot = slot.merge(kind))
                    .or_insert(kind);
            }
        }

        functions_per_base.push(base_functions);
    }

    // A non-`abstract` contract that still has an unimplemented function or
    // modifier cannot be deployed and must be marked `abstract`.
    if let Some(contract) = contract {
        if !contract.ir_node.is_abstract && abstract_slots.iter().any(|slot| !slot.implemented) {
            let file_id = file_node_mapper.file_id_from_node_id(definition_id);
            diagnostics.push(
                file_id.to_owned(),
                contract.ir_node.range.clone(),
                ContractShouldBeAbstract {
                    name: contract.ir_node.name.unparse().to_owned(),
                },
            );
        }
    }

    ContractLinearisations {
        functions: linearise_functions(binder, types, &functions_per_base),
        state_variables,
        errors,
        events,
    }
}

/// Flattens the per-base function lists (gathered most-base-first) into the
/// hierarchy's function list: most-derived-first, dropping a function once a
/// more-derived one overrides it, then sorted by name.
fn linearise_functions(
    binder: &Binder,
    types: &TypeRegistry,
    functions_per_base: &[Vec<ir::FunctionDefinition>],
) -> Vec<ir::FunctionDefinition> {
    let mut functions: Vec<ir::FunctionDefinition> = Vec::new();
    for base_functions in functions_per_base.iter().rev() {
        for function in base_functions {
            let already_overridden = functions
                .iter()
                .any(|kept| function_overrides(binder, types, kept, function));
            if !already_overridden {
                functions.push(Arc::clone(function));
            }
            // TODO(validation): if overriding, function must have the `override` specifier and the overriden functions must be marked `virtual`
            // TODO(validation): if overriding multiple ancestors, the function needs to specify the bases in a specifier
        }
    }
    functions.sort_by(|a, b| match (&a.name, &b.name) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a), Some(b)) => a.unparse().cmp(b.unparse()),
    });
    functions
}

/// Returns the `Definition` naming `member`, or `None` for members that don't
/// introduce a name into the contract namespace (using directives, and unnamed
/// functions such as constructors, fallback and receive).
fn member_definition<'a>(
    binder: &'a Binder,
    member: &ir::ContractMember,
) -> Option<&'a Definition> {
    let node_id = match member {
        ir::ContractMember::UsingDirective(_) => return None,
        ir::ContractMember::FunctionDefinition(function) => function.id(),
        ir::ContractMember::StructDefinition(struct_) => struct_.id(),
        ir::ContractMember::EnumDefinition(enum_) => enum_.id(),
        ir::ContractMember::EventDefinition(event) => event.id(),
        ir::ContractMember::ErrorDefinition(error) => error.id(),
        ir::ContractMember::UserDefinedValueTypeDefinition(udvt) => udvt.id(),
        ir::ContractMember::StateVariableDefinition(variable) => variable.id(),
        ir::ContractMember::ConstantDefinition(constant) => constant.id(),
    };
    binder.find_definition_by_id(node_id)
}

/// The kind of a named member, for redeclaration purposes. Used both to
/// classify a member and to summarise the members occupying a name across a
/// hierarchy.
///
/// Same-named members coexist only when they're all the same overloadable kind
/// (all functions, all modifiers, or all events); Solidity allows overloading
/// functions/events and overriding functions/modifiers. Once members of
/// differing kinds share a name the slot collapses to [`Self::Other`], under
/// which any further same-named member is a redeclaration — so accumulating
/// members only ever moves a name's slot towards `Other`, never back.
#[derive(Clone, Copy, PartialEq, Eq)]
enum MemberKind {
    Function,
    Modifier,
    Event,
    /// A `public` state variable. As a *derived* member its getter may override
    /// an inherited function; as an inherited member (or name slot) it never
    /// lets a same-named member coexist, behaving like [`Self::Other`].
    StateVarPublic,
    /// Any other named member (non-public state variables, structs, enums,
    /// errors, ...), or a name slot occupied by members of differing kinds:
    /// never coexists with a same-named member.
    Other,
}

impl MemberKind {
    /// Classifies `definition`.
    fn of(definition: &Definition) -> Self {
        match definition {
            Definition::Function(_) => Self::Function,
            Definition::Modifier(_) => Self::Modifier,
            Definition::Event(_) => Self::Event,
            Definition::StateVariable(state_variable)
                if matches!(
                    state_variable.ir_node.attributes.visibility,
                    ir::StateVariableVisibility::Public
                ) =>
            {
                Self::StateVarPublic
            }
            _ => Self::Other,
        }
    }

    /// Folds another same-named member into a name's slot; differing kinds
    /// collapse to [`Self::Other`].
    fn merge(self, other: Self) -> Self {
        if self == other {
            self
        } else {
            Self::Other
        }
    }

    /// Whether a `derived` member redeclares the members occupying this slot. A
    /// function — or a public state variable's getter — may share its name only
    /// with functions; a modifier only with modifiers; an event only with
    /// events; anything else with nothing.
    fn clashes_with(self, derived: &Definition) -> bool {
        let derived = Self::of(derived);
        match self {
            Self::Function => !matches!(derived, Self::Function | Self::StateVarPublic),
            Self::Modifier => derived != Self::Modifier,
            Self::Event => derived != Self::Event,
            Self::StateVarPublic | Self::Other => true,
        }
    }
}

/// A member of a contract's hierarchy that requires an implementation for the
/// contract to be concrete: a function or a modifier, together with whether it
/// is currently implemented.
struct AbstractSlot {
    kind: AbstractSlotKind,
    /// The member's name, used to match declarations across bases.
    name: String,
    /// The member's (function) type, used to distinguish overloads and detect
    /// overrides. `None` for modifiers, which cannot be overloaded and so match
    /// on name alone.
    type_id: Option<TypeId>,
    implemented: bool,
}

#[derive(PartialEq, Eq)]
enum AbstractSlotKind {
    Function,
    Modifier,
}

/// Builds the [`AbstractSlot`] for a member that requires an implementation, or
/// `None` for members that don't participate in the check.
///
/// A `public` state variable contributes its (always-implemented) getter, which
/// can satisfy a function declared in a base contract or interface.
fn abstract_slot(binder: &Binder, definition: &Definition) -> Option<AbstractSlot> {
    let (kind, type_id, implemented) = match definition {
        Definition::Function(function) => (
            AbstractSlotKind::Function,
            binder.node_typing(function.ir_node.id()).as_type_id(),
            function.ir_node.body.is_some(),
        ),
        Definition::Modifier(modifier) => (
            AbstractSlotKind::Modifier,
            None,
            modifier.ir_node.body.is_some(),
        ),
        Definition::StateVariable(state_variable)
            if matches!(
                state_variable.ir_node.attributes.visibility,
                ir::StateVariableVisibility::Public
            ) =>
        {
            (
                AbstractSlotKind::Function,
                state_variable.getter_type_id,
                true,
            )
        }
        _ => return None,
    };
    Some(AbstractSlot {
        kind,
        name: definition.identifier().unparse().to_owned(),
        type_id,
        implemented,
    })
}

/// Merges `candidate` into `slots`. `candidate` is always more-derived than the
/// existing slots, so when it overrides one it takes over that slot's identity
/// and implementation status; otherwise it introduces a new slot.
fn merge_abstract_slot(
    types: &TypeRegistry,
    slots: &mut Vec<AbstractSlot>,
    candidate: AbstractSlot,
) {
    for slot in slots.iter_mut() {
        if slot_overridden_by(types, slot, &candidate) {
            slot.type_id = candidate.type_id;
            slot.implemented = candidate.implemented;
            return;
        }
    }
    slots.push(candidate);
}

/// Whether the more-derived `candidate` overrides the already-recorded `slot`:
/// they are the same kind of member with the same name, and (for functions)
/// their signatures are in an override relationship. Modifiers match on name
/// alone since they cannot be overloaded.
fn slot_overridden_by(types: &TypeRegistry, slot: &AbstractSlot, candidate: &AbstractSlot) -> bool {
    if slot.kind != candidate.kind || slot.name != candidate.name {
        return false;
    }
    match candidate.kind {
        AbstractSlotKind::Modifier => true,
        AbstractSlotKind::Function => match (candidate.type_id, slot.type_id) {
            (Some(candidate_type_id), Some(slot_type_id)) => {
                types.type_id_is_function_and_overrides(candidate_type_id, slot_type_id)
            }
            _ => false,
        },
    }
}

/// Whether `overriding` overrides `overridden`: they share a name (or are the
/// same kind of unnamed function) and their signatures are in an override
/// relationship.
fn function_overrides(
    binder: &Binder,
    types: &TypeRegistry,
    overriding: &ir::FunctionDefinition,
    overridden: &ir::FunctionDefinition,
) -> bool {
    let name_matches = match (&overriding.name, &overridden.name) {
        (None, None) => overriding.kind == overridden.kind,
        (Some(name), Some(other_name)) => name.unparse() == other_name.unparse(),
        _ => false,
    };
    if !name_matches {
        return false;
    }
    let overriding_type_id = binder.node_typing(overriding.id()).as_type_id();
    let overridden_type_id = binder.node_typing(overridden.id()).as_type_id();
    match (overriding_type_id, overridden_type_id) {
        (Some(overriding_type_id), Some(overridden_type_id)) => {
            types.type_id_is_function_and_overrides(overriding_type_id, overridden_type_id)
        }
        _ => false,
    }
    // TODO(validation) SDR[6]: check also that the function mutability is stricter than the overridden one's
}
