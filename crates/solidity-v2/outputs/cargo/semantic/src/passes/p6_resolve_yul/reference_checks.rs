use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    UnsupportedReferenceKind, YulAssignmentToConstant, YulAssignmentToNonVariable,
    YulAssignmentToOffset, YulAssignmentToStateVariable, YulCalldataArrayAccess, YulCalldataSuffix,
    YulExternalFunctionAccess, YulForwardReferencedConstant, YulFunctionPointerSuffix,
    YulImmutableAccess, YulInternalFunctionPointerSuffix, YulStorageSuffix,
    YulStorageVariableAccess, YulSuffixOnConstant, YulUnsupportedConstant, YulUnsupportedReference,
    YulUnsupportedSuffix,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::NodeIdentity;

use super::Pass;
use crate::binder::{Binder, Definition, Reference, Resolution, Typing};
use crate::built_ins::InternalBuiltIn;
use crate::types::{
    ArrayType, BytesType, DataLocation, FunctionTypeVisibility, LiteralKind, StringType, Type,
};

// Whether a path is read or assigned to.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum AccessKind {
    Read,
    Write,
}

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
        suffix: Option<&Reference>,
        access: AccessKind,
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
                self.check_constant_reference(identifier, definition_id, suffix, access);
            }
            Definition::StateVariable(variable) => match variable.ir_node.attributes.mutability {
                ir::StateVariableMutability::Constant => {
                    self.check_constant_reference(identifier, definition_id, suffix, access);
                }
                ir::StateVariableMutability::Immutable => {
                    self.push_diagnostic(identifier, YulImmutableAccess);
                }
                ir::StateVariableMutability::Mutable | ir::StateVariableMutability::Transient => {
                    self.check_variable_reference(identifier, definition_id, suffix, access);
                }
            },
            Definition::Variable(_) | Definition::Parameter(_) => {
                self.check_variable_reference(identifier, definition_id, suffix, access);
            }
            _ => self.check_declaration_reference(identifier, definition_id, suffix, access),
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
        suffix: Option<&Reference>,
        access: AccessKind,
    ) {
        // A variable whose type did not resolve cannot be classified, so its
        // references are not checked.
        let Typing::Resolved(type_id) = self.binder.node_typing(definition_id) else {
            return;
        };

        // A state variable has a slot of its own, where a local or a parameter
        // only ever points at one.
        let is_state_variable = matches!(
            self.binder.find_definition_by_id(definition_id),
            Some(Definition::StateVariable(_))
        );

        let variable_type = self.types.get_type_by_id(*type_id);
        if is_state_variable || variable_type.data_location() == Some(DataLocation::Storage) {
            match suffix {
                None => self.push_diagnostic(identifier, YulStorageVariableAccess),
                Some(suffix) => {
                    if !matches!(
                        suffix.resolution,
                        Resolution::BuiltIn(InternalBuiltIn::YulSlot | InternalBuiltIn::YulOffset)
                    ) {
                        self.push_diagnostic(&suffix.identifier, YulStorageSuffix);
                    } else if access == AccessKind::Write {
                        if is_state_variable {
                            self.push_diagnostic(&suffix.identifier, YulAssignmentToStateVariable);
                        } else if matches!(
                            suffix.resolution,
                            Resolution::BuiltIn(InternalBuiltIn::YulOffset)
                        ) {
                            self.push_diagnostic(&suffix.identifier, YulAssignmentToOffset);
                        }
                    }
                }
            }
        } else if is_dynamic_calldata_array(variable_type) {
            match suffix {
                None => self.push_diagnostic(identifier, YulCalldataArrayAccess),
                Some(suffix) => {
                    if !matches!(
                        suffix.resolution,
                        Resolution::BuiltIn(
                            InternalBuiltIn::YulOffset | InternalBuiltIn::YulLengthField
                        )
                    ) {
                        self.push_diagnostic(&suffix.identifier, YulCalldataSuffix);
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
                    if !matches!(
                        suffix.resolution,
                        Resolution::BuiltIn(
                            InternalBuiltIn::YulSelector | InternalBuiltIn::YulAddressField
                        )
                    ) {
                        self.push_diagnostic(&suffix.identifier, YulFunctionPointerSuffix);
                    } else if !is_external {
                        self.push_diagnostic(&suffix.identifier, YulInternalFunctionPointerSuffix);
                    }
                }
            }
        } else if let Some(suffix) = suffix {
            self.push_diagnostic(&suffix.identifier, YulUnsupportedSuffix);
        }
    }

    // Validates a reference to a declaration that is not a variable. It has
    // no addressable parts and cannot be assigned to. Only a library can be
    // read, which yields its address.
    fn check_declaration_reference(
        &mut self,
        identifier: &ir::Identifier,
        definition_id: NodeId,
        suffix: Option<&Reference>,
        access: AccessKind,
    ) {
        if let Some(suffix) = suffix {
            self.push_diagnostic(&suffix.identifier, YulUnsupportedSuffix);
        } else if access == AccessKind::Write {
            self.push_diagnostic(identifier, YulAssignmentToNonVariable);
        } else {
            let definition = self
                .binder
                .find_definition_by_id(definition_id)
                .expect("the dispatch resolved this definition");
            if !matches!(definition, Definition::Library(_)) {
                let kind = declaration_kind(definition);
                self.push_diagnostic(identifier, YulUnsupportedReference { kind });
            }
        }
    }

    // Validates an assembly reference that resolves to a constant.
    fn check_constant_reference(
        &mut self,
        identifier: &ir::Identifier,
        definition_id: NodeId,
        suffix: Option<&Reference>,
        access: AccessKind,
    ) {
        // An uninitialized constant is already reported during IR build.
        if self.binder.constant_value(definition_id).is_none() {
            return;
        }

        // Constants are read only.
        if access == AccessKind::Write {
            self.push_diagnostic(identifier, YulAssignmentToConstant);
            return;
        }

        // A constant has no storage slot and no addressable parts, so no
        // suffix applies to it.
        if let Some(suffix) = suffix {
            self.push_diagnostic(&suffix.identifier, YulSuffixOnConstant);
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
        if !self.is_direct_number_constant(root_id, value) {
            self.push_diagnostic(identifier, YulUnsupportedConstant);
            return;
        }

        // Hack to match solc behavior by rejecting non literal forward references
        // within the same file.
        if !value.is_literal()
            && self.is_forward_reference(identifier, root_id, root.identifier().range.start)
        {
            self.push_diagnostic(identifier, YulForwardReferencedConstant);
        }
    }

    // Whether the constant holds a number that assembly can push on the
    // stack.
    fn is_direct_number_constant(&self, definition_id: NodeId, value: &ir::Expression) -> bool {
        // A `string` or dynamic `bytes` are not supported, as their value does not fit
        // in one stack slot.
        if self.is_string_or_dynamic_bytes(definition_id) {
            return false;
        }
        // A literal is accepted as written, like `41`, `0x11`, `true` or
        // `1 ether`. Arithmetic over literals is accepted too, like
        // `1 + 2`, because the result is still an untyped number. Anything
        // that pins a type is rejected, like `uint(1) + 1`, `A + 1` or
        // `type(uint).max`.
        value.is_literal() || self.is_untyped_number_expression(value)
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

    fn is_string_or_dynamic_bytes(&self, definition_id: NodeId) -> bool {
        let Typing::Resolved(type_id) = self.binder.node_typing(definition_id) else {
            return false;
        };
        matches!(
            self.types.get_type_by_id(*type_id),
            Type::String(_) | Type::Bytes(_)
        )
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
            self.types.get_type_by_id(*type_id),
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

// The kind of a declaration that assembly cannot reference.
fn declaration_kind(definition: &Definition) -> UnsupportedReferenceKind {
    match definition {
        Definition::Contract(_) => UnsupportedReferenceKind::Contract,
        Definition::Enum(_) => UnsupportedReferenceKind::Enum,
        Definition::Error(_) => UnsupportedReferenceKind::Error,
        Definition::Event(_) => UnsupportedReferenceKind::Event,
        Definition::Function(_) => UnsupportedReferenceKind::Function,
        Definition::Import(_) | Definition::ImportedSymbol(_) => UnsupportedReferenceKind::Import,
        Definition::Interface(_) => UnsupportedReferenceKind::Interface,
        Definition::Modifier(_) => UnsupportedReferenceKind::Modifier,
        Definition::Struct(_) => UnsupportedReferenceKind::Struct,
        Definition::UserDefinedValueType(_) => UnsupportedReferenceKind::UserDefinedValueType,
        // Name lookup walks block, contract and file scopes only. Members are
        // registered in the scope of their enum or struct, and a type
        // parameter is registered in no scope at all.
        Definition::EnumMember(_) | Definition::StructMember(_) | Definition::TypeParameter(_) => {
            unreachable!("a name lookup cannot find these declarations")
        }
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

// Follows the chain from `start` to the constant that holds the value. A
// step follows a value that is nothing but a reference to another
// initialized constant. Each constant has at most one such reference, so the
// chain ends, loops, or runs past the depth guard.
fn find_root_constant(binder: &Binder, start: NodeId) -> RootSearchResult {
    // A longer chain is left to the cycle detection in p8, which rejects it at
    // the declarations.
    // __SLANG_CONSTANT_CYCLE_MAX_DEPTH__ keep in sync with `cycle_detection::MAX_DEPTH`
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
            .constant_value(current)
            .expect("the walk only visits initialized constants")
        else {
            // This is a value that is not a reference, so return the current constant.
            return RootSearchResult::Constant(current);
        };

        let Some(next) = binder.find_constant_definition_by_identifier_node_id(identifier.id())
        else {
            // The reference is not an initialized constant.
            return RootSearchResult::NonConstant;
        };
        current = next;
    }
}
