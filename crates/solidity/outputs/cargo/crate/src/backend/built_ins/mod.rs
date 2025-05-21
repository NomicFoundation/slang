use super::types::{FunctionTypeKind, Type, TypeId, TypeRegistry};
use crate::extensions::built_ins::BuiltInTag;

pub fn built_in_type(tag: BuiltInTag, types: &mut TypeRegistry) -> TypeId {
    match tag {
        BuiltInTag::MsgSender => types.address(),
        BuiltInTag::RequireBool => types.register_type(Type::Function {
            parameter_types: vec![types.bool()],
            return_type: types.void(),
            external: false,
            kind: FunctionTypeKind::Pure,
        }),
        BuiltInTag::RequireBoolString => types.register_type(Type::Function {
            parameter_types: vec![types.bool(), types.string()],
            return_type: types.void(),
            external: false,
            kind: FunctionTypeKind::Pure,
        }),
        BuiltInTag::RequireBoolError => types.register_type(Type::Function {
            parameter_types: vec![types.bool(), types.error()],
            return_type: types.void(),
            external: false,
            kind: FunctionTypeKind::Pure,
        }),
    }
}
