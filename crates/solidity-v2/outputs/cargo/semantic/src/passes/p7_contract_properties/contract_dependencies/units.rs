use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use crate::binder::{Binder, Definition};

/// A unit of executable code that can hold contract references or callable
/// references.
pub(super) struct CodeUnit<'a> {
    /// Id of the defining node, ie. the function, state variable, constant
    /// or inheritance specifier. Callable references resolve to it.
    pub(super) id: NodeId,
    /// The contract or library the code is written in, if any.
    pub(super) enclosing_definition: Option<NodeId>,
    kind: CodeUnitKind<'a>,
}

enum CodeUnitKind<'a> {
    /// A function, modifier, constructor, fallback or receive definition.
    Callable(&'a ir::FunctionDefinition),
    /// A state variable or constant value. Non-constant values run in the
    /// creation code. Constant values are compiled into every referencing
    /// unit, and into the getter of a public one.
    Initializer(&'a ir::Expression),
    /// Base constructor arguments, eg. `f()` in `contract B is A(f())`.
    /// They run when the deriving contract is created.
    BaseArguments(&'a ir::ArgumentsDeclaration),
}

pub(super) fn collect(binder: &Binder) -> Vec<CodeUnit<'_>> {
    let mut units = Vec::new();
    for (definition_id, definition) in binder.definitions() {
        let enclosing_definition = binder.enclosing_definition_node_id(*definition_id);
        match definition {
            // Every named function, ie. free functions, contract methods and
            // library functions.
            Definition::Function(function) => units.push(CodeUnit {
                id: *definition_id,
                enclosing_definition,
                kind: CodeUnitKind::Callable(&function.ir_node),
            }),
            Definition::Modifier(modifier) => units.push(CodeUnit {
                id: *definition_id,
                enclosing_definition,
                kind: CodeUnitKind::Callable(&modifier.ir_node),
            }),
            // A constant without a getter is never an entry point. Its unit
            // is reached only through references.
            Definition::Constant(constant) => {
                if let Some(value) = &constant.ir_node.value {
                    units.push(CodeUnit {
                        id: *definition_id,
                        enclosing_definition,
                        kind: CodeUnitKind::Initializer(value),
                    });
                }
            }
            Definition::StateVariable(state_variable) => {
                if let Some(value) = &state_variable.ir_node.value {
                    units.push(CodeUnit {
                        id: *definition_id,
                        enclosing_definition,
                        kind: CodeUnitKind::Initializer(value),
                    });
                }
            }
            Definition::Contract(contract) => {
                // Constructors, fallback and receive functions are nameless
                // and have no `Definition`, so they are collected from the
                // contract members. Named members were collected above.
                for member in contract.ir_node.members.iter() {
                    if let ir::ContractMember::FunctionDefinition(function) = member
                        && matches!(
                            function.kind,
                            ir::FunctionKind::Constructor
                                | ir::FunctionKind::Fallback
                                | ir::FunctionKind::Receive
                        )
                    {
                        units.push(CodeUnit {
                            id: function.id(),
                            enclosing_definition: Some(*definition_id),
                            kind: CodeUnitKind::Callable(function),
                        });
                    }
                }
                for inheritance_type in contract.ir_node.inheritance_types.iter() {
                    if let Some(arguments) = &inheritance_type.arguments {
                        units.push(CodeUnit {
                            id: inheritance_type.id(),
                            enclosing_definition: Some(*definition_id),
                            kind: CodeUnitKind::BaseArguments(arguments),
                        });
                    }
                }
            }
            _ => {}
        }
    }
    units
}

pub(super) fn visit_code_unit(unit: &CodeUnit<'_>, visitor: &mut impl Visitor) {
    match unit.kind {
        CodeUnitKind::Callable(function) => {
            // Only the modifier invocations and the body can hold references.
            // Everything else is skipped.
            for modifier_invocation in function.attributes.modifier_invocations.iter() {
                ir::visitor::accept_modifier_invocation(modifier_invocation, visitor);
            }
            if let Some(body) = &function.body {
                ir::visitor::accept_block(body, visitor);
            }
        }
        CodeUnitKind::Initializer(value) => ir::visitor::accept_expression(value, visitor),
        CodeUnitKind::BaseArguments(arguments) => {
            ir::visitor::accept_arguments_declaration(arguments, visitor);
        }
    }
}
