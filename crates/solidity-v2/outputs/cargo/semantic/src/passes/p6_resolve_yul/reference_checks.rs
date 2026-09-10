use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    YulAssignmentToConstant, YulForwardReferencedConstant, YulSuffixOnConstant,
    YulUnsupportedConstant,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::NodeIdentity;

use super::Pass;
use crate::binder::{Binder, Definition, Resolution, Typing};
use crate::types::{LiteralKind, Type};

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
        is_lvalue: bool,
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
            Definition::Constant(_) => {
                self.check_constant_reference(identifier, definition_id, suffix, is_lvalue);
            }
            Definition::StateVariable(variable) => {
                if matches!(
                    variable.ir_node.attributes.mutability,
                    ir::StateVariableMutability::Constant
                ) {
                    self.check_constant_reference(identifier, definition_id, suffix, is_lvalue);
                }
            }
            // TODO(validation): Add other diagnostics.
            _ => {}
        }
    }

    // Validates an assembly reference that resolves to a constant.
    fn check_constant_reference(
        &mut self,
        identifier: &ir::Identifier,
        definition_id: NodeId,
        suffix: Option<&ir::Identifier>,
        is_lvalue: bool,
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
        if is_lvalue {
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
