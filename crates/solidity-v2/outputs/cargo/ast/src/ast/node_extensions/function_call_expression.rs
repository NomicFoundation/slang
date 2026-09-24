use slang_solidity_v2_ir::ir::{self, NodeIdentity};
use slang_solidity_v2_semantic::built_ins::InternalBuiltIn;
use slang_solidity_v2_semantic::{binder, types};

use super::super::{FunctionCallExpressionStruct, Type};

impl FunctionCallExpressionStruct {
    /// Returns `true` if this call is a type conversion (e.g. `uint256(x)`,
    /// `address(y)`) rather than a function call: ie. its operand names a
    /// type — types as a meta-type — rather than a value.
    pub fn is_type_conversion(&self) -> bool {
        self.operand().get_type().is_some_and(|operand_type| {
            matches!(operand_type, Type::MetaType(_) | Type::UserMetaType(_))
        })
    }

    /// For an `abi.encodeCall`, the externalized function type the call encodes
    /// its arguments against, with `calldata` parameters transformed into
    /// `memory`. `None` for any other call, and for a callee that is not an
    /// externally callable function.
    pub fn encode_call_callee_type(&self) -> Option<Type> {
        let semantic = &self.semantic;
        let binder::Typing::BuiltIn(InternalBuiltIn::AbiEncodeCall) = semantic
            .binder()
            .node_typing(self.ir_node.operand.node_id()?)
        else {
            return None;
        };
        let ir::ArgumentsDeclaration::PositionalArguments(arguments) = &self.ir_node.arguments
        else {
            return None;
        };
        let callee_type_id = semantic
            .binder()
            .node_typing(arguments.first()?.node_id()?)
            .as_type_id()?;
        let type_id = match semantic.types().get_type_by_id(callee_type_id) {
            // A `Public` function value was named without a receiver, so it is internal.
            types::Type::Function(function_type)
                if function_type.visibility == types::FunctionTypeVisibility::External =>
            {
                semantic
                    .types()
                    .externalized_function_type_id(callee_type_id)
            }
            // A declaration reached through a type name carries its externalized type.
            types::Type::UserMetaType(types::UserMetaType { definition_id }) => {
                match semantic.binder().find_definition_by_id(*definition_id) {
                    Some(binder::Definition::Function(definition)) => {
                        definition.externalized_type_id
                    }
                    _ => None,
                }
            }
            _ => None,
        }?;
        Some(Type::create(type_id, semantic))
    }
}
