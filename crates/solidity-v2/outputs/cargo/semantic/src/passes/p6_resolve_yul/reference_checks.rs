use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    YulAssignmentToConstant, YulAssignmentToNonVariable, YulAssignmentToOffset,
    YulAssignmentToStateVariable, YulCalldataArrayAccess, YulCalldataSuffix,
    YulExternalFunctionAccess, YulForwardReferencedConstant, YulFunctionPointerSuffix,
    YulImmutableAccess, YulInternalFunctionPointerSuffix, YulStorageSuffix,
    YulStorageVariableAccess, YulSuffixOnConstant, YulUnsupportedConstant, YulUnsupportedReference,
    YulUnsupportedSuffix,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::NodeIdentity;

use super::Pass;
use crate::binder::{Binder, Definition, Resolution, Typing};
use crate::types::{
    ArrayType, BytesType, DataLocation, FunctionTypeVisibility, LiteralKind, StringType, Type,
};

// What the end of the chain from a referenced constant holds.
enum RootSearchResult {
    // A constant whose value is not a reference.
    Constant(NodeId),
    // Something that is not an initialized constant.
    NonConstant,
    // The chain loops, or is longer than the depth limit.
    Circular,
}

impl Pass<'_> {
    // Validates an assembly reference to a Solidity declaration.
    pub(super) fn check_solidity_reference(
        &mut self,
        identifier: &ir::Identifier,
        resolution: &Resolution,
        suffix: Option<&ir::Identifier>,
    ) {
        let Some(definition_id) = self
            .binder
            .follow_symbol_aliases(resolution.clone())
            .as_definition_id()
        else {
            return;
        };
        let Some(definition) = self.binder.find_definition_by_id(definition_id) else {
            return;
        };

        match definition {
            // Yul locals are internal references, nothing to validate.
            Definition::YulFunction(_)
            | Definition::YulParameter(_)
            | Definition::YulVariable(_) => {}
            Definition::Constant(_) => {
                self.check_constant_reference(identifier, definition_id, suffix);
            }
            Definition::StateVariable(variable) => match variable.ir_node.attributes.mutability {
                ir::StateVariableMutability::Constant => {
                    self.check_constant_reference(identifier, definition_id, suffix);
                }
                ir::StateVariableMutability::Immutable => {
                    self.push_diagnostic(identifier, YulImmutableAccess);
                }
                ir::StateVariableMutability::Mutable | ir::StateVariableMutability::Transient => {
                    self.check_variable_reference(identifier, definition_id, suffix, true);
                }
            },
            Definition::Variable(_) | Definition::Parameter(_) => {
                self.check_variable_reference(identifier, definition_id, suffix, false);
            }
            _ => self.check_declaration_reference(identifier, definition_id, suffix),
        }
    }

    // Validates a reference to a variable. A storage variable is addressed
    // through `.slot` and `.offset`, a dynamic calldata array through
    // `.offset` and `.length`, and an external function pointer through
    // `.selector` and `.address`. Everything else is read directly and takes
    // no suffix.
    fn check_variable_reference(
        &mut self,
        identifier: &ir::Identifier,
        definition_id: NodeId,
        suffix: Option<&ir::Identifier>,
        is_state_variable: bool,
    ) {
        // A variable whose type did not resolve cannot be classified, so its
        // references are not checked.
        let Typing::Resolved(type_id) = self.binder.node_typing(definition_id) else {
            return;
        };

        let variable_type = self.types.get_type_by_id(type_id);
        if is_state_variable || variable_type.data_location() == Some(DataLocation::Storage) {
            match suffix {
                None => self.push_diagnostic(identifier, YulStorageVariableAccess),
                Some(suffix) => {
                    if !matches!(suffix.unparse(), "slot" | "offset") {
                        self.push_diagnostic(suffix, YulStorageSuffix);
                    } else if self.in_assignment_target {
                        if is_state_variable {
                            self.push_diagnostic(suffix, YulAssignmentToStateVariable);
                        } else if suffix.unparse() == "offset" {
                            self.push_diagnostic(suffix, YulAssignmentToOffset);
                        }
                    }
                }
            }
        } else if is_dynamic_calldata_array(variable_type) {
            match suffix {
                None => self.push_diagnostic(identifier, YulCalldataArrayAccess),
                Some(suffix) => {
                    if !matches!(suffix.unparse(), "offset" | "length") {
                        self.push_diagnostic(suffix, YulCalldataSuffix);
                    }
                }
            }
        } else if let Type::Function(function_type) = variable_type {
            let is_external = function_type.visibility == FunctionTypeVisibility::External;
            match suffix {
                None => {
                    if is_external {
                        self.push_diagnostic(identifier, YulExternalFunctionAccess);
                    }
                }
                Some(suffix) => {
                    if !matches!(suffix.unparse(), "selector" | "address") {
                        self.push_diagnostic(suffix, YulFunctionPointerSuffix);
                    } else if !is_external {
                        self.push_diagnostic(suffix, YulInternalFunctionPointerSuffix);
                    }
                }
            }
        } else if let Some(suffix) = suffix {
            self.push_diagnostic(suffix, YulUnsupportedSuffix);
        }
    }

    // Validates a reference to a declaration that is not a variable. It has
    // no addressable parts and cannot be assigned to. Only a library can be
    // read, which yields its address.
    fn check_declaration_reference(
        &mut self,
        identifier: &ir::Identifier,
        definition_id: NodeId,
        suffix: Option<&ir::Identifier>,
    ) {
        if let Some(suffix) = suffix {
            self.push_diagnostic(suffix, YulUnsupportedSuffix);
        } else if self.in_assignment_target {
            self.push_diagnostic(identifier, YulAssignmentToNonVariable);
        } else {
            let definition = self
                .binder
                .find_definition_by_id(definition_id)
                .expect("the dispatch resolved this definition");
            if !matches!(definition, Definition::Library(_)) {
                let kind = declaration_kind_name(definition);
                self.push_diagnostic(identifier, YulUnsupportedReference { kind });
            }
        }
    }

    // Validates an assembly reference that resolves to a constant.
    fn check_constant_reference(
        &mut self,
        identifier: &ir::Identifier,
        definition_id: NodeId,
        suffix: Option<&ir::Identifier>,
    ) {
        // An uninitialized constant is already reported during IR build.
        if self
            .binder
            .find_definition_by_id(definition_id)
            .and_then(|definition| definition.as_constant_value())
            .is_none()
        {
            return;
        }

        // Constants are read only.
        if self.in_assignment_target {
            self.push_diagnostic(identifier, YulAssignmentToConstant);
            return;
        }

        // A constant has no storage slot and no addressable parts, so no
        // suffix applies to it.
        if let Some(suffix) = suffix {
            self.push_diagnostic(suffix, YulSuffixOnConstant);
            return;
        }

        let root_id = match find_root_constant(self.binder, definition_id) {
            RootSearchResult::Constant(root_id) => root_id,
            // Code analysis pass will report this error, so no need to
            // repeat that here.
            RootSearchResult::Circular => return,
            RootSearchResult::NonConstant => {
                self.push_diagnostic(identifier, YulUnsupportedConstant);
                return;
            }
        };
        let root = self
            .binder
            .find_definition_by_id(root_id)
            .expect("the root definition exists");
        let value = root
            .as_constant_value()
            .expect("the root is an initialized constant");

        // Only a direct number constant is supported.
        if !self.is_direct_number_constant(value, constant_type_name(root)) {
            self.push_diagnostic(identifier, YulUnsupportedConstant);
            return;
        }

        // Hack to match solc behavior by rejecting non literal forward references
        // within the same file.
        if !is_literal(value)
            && self.is_forward_reference(identifier, root_id, root.identifier().range.start)
        {
            self.push_diagnostic(identifier, YulForwardReferencedConstant);
        }
    }

    // Whether the constant holds a number that assembly can push on the
    // stack.
    fn is_direct_number_constant(&self, value: &ir::Expression, type_name: &ir::TypeName) -> bool {
        // A `string` or dynamic `bytes` are not supported, as their value does not fit
        // in one stack slot.
        if is_string_or_dynamic_bytes(type_name) {
            return false;
        }
        // A literal is accepted as written, like `41`, `0x11`, `true` or
        // `1 ether`. Arithmetic over literals is accepted too, like
        // `1 + 2`, because the result is still an untyped number. Anything
        // that pins a type is rejected, like `uint(1) + 1`, `A + 1` or
        // `type(uint).max`.
        is_literal(value) || self.is_untyped_number_expression(value)
    }

    // Whether the constant is declared after the reference, in the same
    // file. Hack to match solc behaviour.
    fn is_forward_reference(
        &self,
        identifier: &ir::Identifier,
        definition_id: NodeId,
        declaration_start: usize,
    ) -> bool {
        declaration_start > identifier.range.start
            && self.file_node_mapper.file_id_from_node_id(definition_id)
                == self.file_node_mapper.file_id_from_node_id(identifier.id())
    }

    // Whether the expression is a number that carries no type of its own,
    // which is what arithmetic over literals evaluates to. Integers, hex
    // numbers, rationals and addresses count as numbers here. String
    // literals do not, and neither does a value with a concrete type.
    fn is_untyped_number_expression(&self, expression: &ir::Expression) -> bool {
        let node_id = expression.node_id().expect("expressions have node ids");
        let Typing::Resolved(type_id) = self.binder.node_typing(node_id) else {
            return false;
        };
        matches!(
            self.types.get_type_by_id(type_id),
            Type::Literal(
                LiteralKind::Integer { .. }
                    | LiteralKind::HexInteger { .. }
                    | LiteralKind::Rational { .. }
                    | LiteralKind::Address { .. }
            )
        )
    }
}

fn is_dynamic_calldata_array(variable_type: &Type) -> bool {
    matches!(
        variable_type,
        Type::Array(ArrayType {
            location: DataLocation::Calldata,
            ..
        }) | Type::Bytes(BytesType {
            location: DataLocation::Calldata
        }) | Type::String(StringType {
            location: DataLocation::Calldata
        })
    )
}

// The kind of a declaration that assembly cannot reference, worded like `a function`.
fn declaration_kind_name(definition: &Definition) -> &'static str {
    match definition {
        Definition::Contract(_) => "a contract type",
        Definition::Enum(_) => "an enum type",
        Definition::EnumMember(_) => "an enum member",
        Definition::Error(_) => "an error",
        Definition::Event(_) => "an event",
        Definition::Function(_) => "a function",
        Definition::Import(_) | Definition::ImportedSymbol(_) => "an import",
        Definition::Interface(_) => "an interface type",
        Definition::Modifier(_) => "a modifier",
        Definition::Struct(_) => "a struct type",
        Definition::StructMember(_) => "a struct member",
        Definition::TypeParameter(_) => "a type parameter",
        Definition::UserDefinedValueType(_) => "a user defined value type",
        Definition::Library(_)
        | Definition::Constant(_)
        | Definition::StateVariable(_)
        | Definition::Variable(_)
        | Definition::Parameter(_)
        | Definition::YulFunction(_)
        | Definition::YulParameter(_)
        | Definition::YulVariable(_) => {
            unreachable!("assembly can reference these declarations")
        }
    }
}

fn constant_type_name(definition: &Definition) -> &ir::TypeName {
    match definition {
        Definition::Constant(constant) => &constant.ir_node.type_name,
        Definition::StateVariable(variable) => &variable.ir_node.type_name,
        _ => unreachable!("only constant shapes have a constant value"),
    }
}

// The initialized constant a resolution points at, if any.
fn constant_definition_id(binder: &Binder, resolution: &Resolution) -> Option<NodeId> {
    binder
        .follow_symbol_aliases(resolution.clone())
        .as_definition_id()
        .filter(|definition_id| {
            binder
                .find_definition_by_id(*definition_id)
                .is_some_and(|definition| definition.as_constant_value().is_some())
        })
}

// Follows the chain from `start` to the constant that holds the value. A
// step follows a value that is nothing but a reference to another
// initialized constant. Each constant has at most one such reference, so the
// chain ends, loops, or runs past the depth guard.
fn find_root_constant(binder: &Binder, start: NodeId) -> RootSearchResult {
    const MAX_DEPTH: usize = 256;
    let mut seen = Set::default();
    let mut current = start;
    loop {
        // Reaching a constant twice means the chain loops. A chain past
        // the limit is treated the same way.
        if !seen.insert(current) || seen.len() >= MAX_DEPTH {
            return RootSearchResult::Circular;
        }

        let ir::Expression::Identifier(identifier) = binder
            .find_definition_by_id(current)
            .and_then(|definition| definition.as_constant_value())
            .expect("the walk only visits initialized constants")
        else {
            // This is a value that is not a reference, so return the current constant.
            return RootSearchResult::Constant(current);
        };

        let Some(next) = binder
            .find_reference_by_identifier_node_id(identifier.id())
            .and_then(|reference| constant_definition_id(binder, &reference.resolution))
        else {
            // The reference is not an initialized constant.
            return RootSearchResult::NonConstant;
        };
        current = next;
    }
}

// A literal value expression. A number unit is part of the literal.
fn is_literal(expression: &ir::Expression) -> bool {
    matches!(
        expression,
        ir::Expression::DecimalNumberExpression(_)
            | ir::Expression::HexNumberExpression(_)
            | ir::Expression::StringExpression(_)
            | ir::Expression::TrueKeyword(_)
            | ir::Expression::FalseKeyword(_)
    )
}

fn is_string_or_dynamic_bytes(type_name: &ir::TypeName) -> bool {
    match type_name {
        ir::TypeName::ElementaryType(ir::ElementaryType::StringKeyword(_)) => true,
        ir::TypeName::ElementaryType(ir::ElementaryType::BytesKeyword(keyword)) => {
            // The keyword covers both dynamic `bytes` and sized `bytesN`.
            // Parsing tells them apart, a sized keyword gives a byte array
            // type. The data location does not matter here.
            matches!(
                Type::from_bytes_keyword(keyword.unparse(), Some(DataLocation::Memory)),
                Some(Type::Bytes(_))
            )
        }
        _ => false,
    }
}
