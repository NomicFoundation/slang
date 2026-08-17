use std::sync::Arc;

use slang_solidity_v2_common::collections::{Map, OrderedSet, Set};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::NodeIdentity;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use super::units::{CodeUnit, visit_code_unit};
use crate::binder::{Binder, Definition, Resolution, Typing};
use crate::built_ins::InternalBuiltIn;
use crate::context::ContractReference;
use crate::types::{Type, TypeRegistry};

/// A reference to a callable or constant found in a code unit.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum CallableReference {
    /// Always leads to this definition.
    Static(NodeId),
    /// A virtual function or modifier referenced by bare name. Resolves to
    /// its most derived override.
    Virtual(NodeId),
    /// A `super.f()` call. Resolves against the most derived contract's
    /// linearisation, after the enclosing contract.
    Super {
        declaration: NodeId,
        enclosing_contract: NodeId,
    },
}

/// What a code unit contributes to the dependency analysis. Each unit is
/// walked once and its references are reused by every contract traversal.
pub(super) struct UnitReferences {
    /// Contracts whose bytecode this unit embeds, each paired with the
    /// expression that does it.
    pub(super) contracts: Vec<(NodeId, ContractReference)>,
    /// Callables this unit calls directly, in first-use order.
    pub(super) calls: OrderedSet<CallableReference>,
    /// Callables referenced without being called, ie. taken as a function
    /// value. They can still run through a stored pointer, so the ones found
    /// in creation code also seed the deployed walk.
    pub(super) indirect_calls: OrderedSet<CallableReference>,
}

/// Collects every unit's references.
pub(super) fn collect(
    binder: &Binder,
    types: &TypeRegistry,
    units: &[CodeUnit<'_>],
) -> Map<NodeId, UnitReferences> {
    let mut unit_references = Map::default();
    for unit in units {
        let mut collector = ReferenceCollector {
            binder,
            types,
            enclosing_definition: unit.enclosing_definition,
            contracts: Vec::new(),
            calls: OrderedSet::default(),
            indirect_calls: OrderedSet::default(),
            direct_callees: Set::default(),
        };
        visit_code_unit(unit, &mut collector);
        if !collector.contracts.is_empty()
            || !collector.calls.is_empty()
            || !collector.indirect_calls.is_empty()
        {
            unit_references.insert(
                unit.id,
                UnitReferences {
                    contracts: collector.contracts,
                    calls: collector.calls,
                    indirect_calls: collector.indirect_calls,
                },
            );
        }
    }
    unit_references
}

/// The contract created by a `new` expression, if it creates one.
fn new_expression_target(
    binder: &Binder,
    types: &TypeRegistry,
    node: &ir::NewExpression,
) -> Option<NodeId> {
    let Typing::NewExpression(type_id) = binder.node_typing(node.id()) else {
        return None;
    };
    // A library cannot be created, so only a contract is a target here.
    match types.get_type_by_id(type_id) {
        Type::Contract(contract) => Some(contract.definition_id),
        _ => None,
    }
}

/// The contract whose bytecode a `type(C).creationCode` or
/// `type(C).runtimeCode` access embeds, if it is one.
fn code_access_target(
    binder: &Binder,
    types: &TypeRegistry,
    node: &ir::MemberAccessExpression,
) -> Option<NodeId> {
    let reference = binder.find_reference_by_identifier_node_id(node.member.id())?;
    if !matches!(
        reference.resolution,
        Resolution::BuiltIn(InternalBuiltIn::TypeCreationCode | InternalBuiltIn::TypeRuntimeCode)
    ) {
        return None;
    }
    let operand_id = node.operand.node_id()?;
    let Typing::BuiltIn(InternalBuiltIn::Type(type_id)) = binder.node_typing(operand_id) else {
        return None;
    };
    // A library is deployed too, so its code can be read the same way.
    match types.get_type_by_id(type_id) {
        Type::Contract(contract) => Some(contract.definition_id),
        Type::Library(library) => Some(library.definition_id),
        _ => None,
    }
}

struct ReferenceCollector<'a> {
    binder: &'a Binder,
    types: &'a TypeRegistry,
    enclosing_definition: Option<NodeId>,
    contracts: Vec<(NodeId, ContractReference)>,
    // Both sets keep first-reference order. It decides which reference is
    // reported when several paths reach the same dependency.
    calls: OrderedSet<CallableReference>,
    indirect_calls: OrderedSet<CallableReference>,
    // Callee expression node ids. A function referenced outside this set
    // has its value taken instead of being called.
    direct_callees: Set<NodeId>,
}

impl ReferenceCollector<'_> {
    /// Records a function reference as direct or indirect, depending on
    /// whether `node_id` is a callee expression.
    fn insert_function_reference(&mut self, node_id: NodeId, reference: CallableReference) {
        if self.direct_callees.contains(&node_id) {
            self.calls.insert(reference);
        } else {
            self.indirect_calls.insert(reference);
        }
    }

    fn resolved_reference(&self, identifier: &ir::Identifier) -> Option<Resolution> {
        let reference = self
            .binder
            .find_reference_by_identifier_node_id(identifier.id())?;
        Some(
            self.binder
                .follow_symbol_aliases(reference.resolution.clone()),
        )
    }

    /// Whether the unit's enclosing definition is `contract_id` or derives
    /// from it.
    fn derives_from(&self, contract_id: NodeId) -> bool {
        self.enclosing_definition.is_some_and(|definition| {
            self.binder
                .get_linearised_bases(definition)
                .is_some_and(|bases| bases.contains(&contract_id))
        })
    }

    /// Classifies a function reference reached through the member access
    /// with id `node_id`.
    fn collect_function_member(
        &mut self,
        node_id: NodeId,
        definition_id: NodeId,
        function: &ir::FunctionDefinition,
        operand_typing: &Typing,
    ) {
        match operand_typing {
            Typing::Super => {
                if let Some(enclosing_contract) = self.enclosing_definition {
                    self.insert_function_reference(
                        node_id,
                        CallableReference::Super {
                            declaration: definition_id,
                            enclosing_contract,
                        },
                    );
                }
            }
            Typing::Resolved(type_id) => match self.types.get_type_by_id(*type_id) {
                Type::UserMetaType(meta) => {
                    match self.binder.find_definition_by_id(meta.definition_id) {
                        // Only internal and private library functions run
                        // within the caller. Public ones are delegatecalls
                        // into the deployed library.
                        Some(Definition::Library(_)) => {
                            if matches!(
                                function.attributes.visibility,
                                ir::FunctionVisibility::Internal | ir::FunctionVisibility::Private
                            ) {
                                self.insert_function_reference(
                                    node_id,
                                    CallableReference::Static(definition_id),
                                );
                            }
                        }
                        // `Base.f()` is an internal call only in a contract
                        // deriving from `Base`. Elsewhere the member is just
                        // a declaration, eg. for `.selector`.
                        Some(Definition::Contract(_) | Definition::Interface(_))
                            if self.derives_from(meta.definition_id)
                                && !matches!(
                                    function.attributes.visibility,
                                    ir::FunctionVisibility::External
                                ) =>
                        {
                            self.insert_function_reference(
                                node_id,
                                CallableReference::Static(definition_id),
                            );
                        }
                        // A function member of an import alias is a free
                        // function of the imported file and runs within
                        // the caller.
                        Some(Definition::Import(_)) => {
                            self.insert_function_reference(
                                node_id,
                                CallableReference::Static(definition_id),
                            );
                        }
                        _ => {}
                    }
                }
                // Attached functions and members of contract values. An
                // attached free function or internal library function runs
                // within the caller. A member of a contract value is an
                // external call and embeds nothing.
                _ => match self.enclosing_definition_of(definition_id) {
                    None => {
                        self.insert_function_reference(
                            node_id,
                            CallableReference::Static(definition_id),
                        );
                    }
                    Some(Definition::Library(_)) => {
                        if matches!(
                            function.attributes.visibility,
                            ir::FunctionVisibility::Internal | ir::FunctionVisibility::Private
                        ) {
                            self.insert_function_reference(
                                node_id,
                                CallableReference::Static(definition_id),
                            );
                        }
                    }
                    Some(_) => {}
                },
            },
            _ => {}
        }
    }

    fn enclosing_definition_of(&self, definition_id: NodeId) -> Option<&Definition> {
        self.binder
            .enclosing_definition_node_id(definition_id)
            .and_then(|id| self.binder.find_definition_by_id(id))
    }

    fn collect_member_callables(&mut self, node: &ir::MemberAccessExpression) {
        let Some(resolution) = self.resolved_reference(&node.member) else {
            return;
        };
        let definition_ids = match &resolution {
            Resolution::Definition(id) => std::slice::from_ref(id),
            Resolution::Ambiguous(ids) => ids,
            _ => return,
        };
        let Some(operand_id) = node.operand.node_id() else {
            return;
        };
        let operand_typing = self.binder.node_typing(operand_id);
        for &definition_id in definition_ids {
            match self.binder.find_definition_by_id(definition_id) {
                Some(Definition::Function(function)) => {
                    self.collect_function_member(
                        node.id(),
                        definition_id,
                        &function.ir_node,
                        &operand_typing,
                    );
                }
                // A constant's value is compiled into the referencing unit,
                // the same as a bare name reference.
                Some(Definition::Constant(_)) => {
                    self.calls.insert(CallableReference::Static(definition_id));
                }
                // A public constant is compiled in only when read through
                // the type name. On a contract value the member is an
                // external getter call and embeds nothing.
                Some(Definition::StateVariable(state_variable)) => {
                    if let Typing::Resolved(type_id) = &operand_typing
                        && matches!(self.types.get_type_by_id(*type_id), Type::UserMetaType(_))
                        && matches!(
                            state_variable.ir_node.attributes.mutability,
                            ir::StateVariableMutability::Constant
                        )
                    {
                        self.calls.insert(CallableReference::Static(definition_id));
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_identifier_callables(&mut self, node: &ir::Identifier) {
        let Some(resolution) = self.resolved_reference(node) else {
            return;
        };
        let definition_ids = match &resolution {
            Resolution::Definition(id) => std::slice::from_ref(id),
            Resolution::Ambiguous(ids) => ids,
            _ => return,
        };
        for &definition_id in definition_ids {
            match self.binder.find_definition_by_id(definition_id) {
                Some(Definition::Function(function)) => {
                    let reference = match self.enclosing_definition_of(definition_id) {
                        // Free functions and same-library functions always
                        // run within the caller.
                        None | Some(Definition::Library(_)) => {
                            CallableReference::Static(definition_id)
                        }
                        // A contract function is resolved per contract only
                        // when it is virtual.
                        Some(Definition::Contract(_)) => {
                            if function.ir_node.attributes.is_virtual {
                                CallableReference::Virtual(definition_id)
                            } else {
                                CallableReference::Static(definition_id)
                            }
                        }
                        // Interface members are implicitly virtual.
                        Some(Definition::Interface(_)) => CallableReference::Virtual(definition_id),
                        Some(_) => continue,
                    };
                    self.insert_function_reference(node.id(), reference);
                }
                // A constant's value is compiled into the referencing unit,
                // so the walk continues into it. The constant is queued, so
                // the unit's own references are recorded first.
                Some(Definition::Constant(_)) => {
                    self.calls.insert(CallableReference::Static(definition_id));
                }
                Some(Definition::StateVariable(state_variable)) => {
                    if matches!(
                        state_variable.ir_node.attributes.mutability,
                        ir::StateVariableMutability::Constant
                    ) {
                        self.calls.insert(CallableReference::Static(definition_id));
                    }
                }
                _ => {}
            }
        }
    }

    /// Records the modifier a modifier invocation runs.
    fn collect_modifier_invocation(&mut self, node: &ir::ModifierInvocation) {
        let Some(name) = node.name.last() else {
            return;
        };
        let Some(Resolution::Definition(definition_id)) = self.resolved_reference(name) else {
            return;
        };
        let Some(Definition::Modifier(modifier)) = self.binder.find_definition_by_id(definition_id)
        else {
            return;
        };
        // A bare name like `m` resolves to the most derived override. A
        // qualified name like `A.m` opts out of virtual lookup and always
        // runs A's modifier. Both can resolve to the same declaration, so
        // only the number of path segments tells them apart.
        let reference = if node.name.len() == 1 && modifier.ir_node.attributes.is_virtual {
            CallableReference::Virtual(definition_id)
        } else {
            CallableReference::Static(definition_id)
        };
        self.calls.insert(reference);
    }

    /// Records the user-defined operator function an expression invokes, if
    /// any. Operator functions are free functions and are always called
    /// directly.
    fn collect_operator_function(&mut self, node_id: NodeId) {
        if let Some(target) = self.binder.resolved_operator_function(node_id) {
            self.calls.insert(CallableReference::Static(target));
        }
    }
}

/// Defines the visitor methods for the expressions that can invoke a
/// user-defined operator function. They all do the same thing, and the list
/// mirrors the operators `p5_resolve_references` resolves.
macro_rules! collect_operator_functions {
    ($($method:ident($node:ident)),+ $(,)?) => {
        $(
            fn $method(&mut self, node: &ir::$node) -> bool {
                self.collect_operator_function(node.id());
                true
            }
        )+
    };
}

impl Visitor for ReferenceCollector<'_> {
    fn enter_expression(&mut self, node: &ir::Expression) -> bool {
        // Only names used as expressions can reference callables and
        // constants. Identifiers elsewhere, eg. in type names or argument
        // labels, never reach the collector.
        if let ir::Expression::Identifier(identifier) = node {
            self.collect_identifier_callables(identifier);
        }
        true
    }

    fn enter_modifier_invocation(&mut self, node: &ir::ModifierInvocation) -> bool {
        self.collect_modifier_invocation(node);
        true
    }

    fn enter_function_call_expression(&mut self, node: &ir::FunctionCallExpression) -> bool {
        // Record the callee before it is visited, so its handler sees it
        // as a direct call.
        if let Some(callee_id) = node.operand.node_id() {
            self.direct_callees.insert(callee_id);
        }
        true
    }

    collect_operator_functions!(
        enter_additive_expression(AdditiveExpression),
        enter_bitwise_and_expression(BitwiseAndExpression),
        enter_bitwise_or_expression(BitwiseOrExpression),
        enter_bitwise_xor_expression(BitwiseXorExpression),
        enter_equality_expression(EqualityExpression),
        enter_inequality_expression(InequalityExpression),
        enter_multiplicative_expression(MultiplicativeExpression),
        enter_prefix_expression(PrefixExpression),
    );

    fn enter_new_expression(&mut self, node: &ir::NewExpression) -> bool {
        if let Some(target) = new_expression_target(self.binder, self.types, node) {
            self.contracts
                .push((target, ContractReference::New(Arc::clone(node))));
        }
        true
    }

    fn enter_member_access_expression(&mut self, node: &ir::MemberAccessExpression) -> bool {
        if let Some(target) = code_access_target(self.binder, self.types, node) {
            self.contracts
                .push((target, ContractReference::CodeAccess(Arc::clone(node))));
        } else {
            self.collect_member_callables(node);
        }
        true
    }
}
