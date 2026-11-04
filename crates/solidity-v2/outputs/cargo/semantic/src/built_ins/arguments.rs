use slang_solidity_v2_common::versions::LanguageVersion;
use smallvec::{SmallVec, smallvec};

use super::{BuiltInsResolver, InternalBuiltIn};
use crate::binder::Definition;
use crate::types::{
    ArrayType, DataLocation, FixedSizeArrayType, StructType, Type, TypeId, UserDefinedValueType,
};

/// How the arguments of a call to a built-in match its overloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BuiltInOverloadMatch {
    /// The built-in is not overloaded, so its arguments are checked otherwise.
    NotOverloaded,
    /// One of the overloads accepts the arguments.
    Accepted,
    /// None of the overloads accepts the arguments.
    Rejected,
}

impl From<bool> for BuiltInOverloadMatch {
    fn from(accepted: bool) -> Self {
        if accepted {
            Self::Accepted
        } else {
            Self::Rejected
        }
    }
}

/// The parameters of the built-in functions, which their arguments are checked
/// against.
impl BuiltInsResolver<'_> {
    /// The parameter types of `built_in`, for the built-in functions with a
    /// single, fixed signature. The others are overloaded, take any number of
    /// arguments, or have rules of their own, so they have none here.
    pub(crate) fn parameter_types_of(
        &mut self,
        built_in: &InternalBuiltIn,
    ) -> Option<SmallVec<[TypeId; 4]>> {
        let parameter_types = match built_in {
            InternalBuiltIn::Addmod | InternalBuiltIn::Mulmod => {
                smallvec![self.types.uint256(); 3]
            }
            InternalBuiltIn::AddressCall
            | InternalBuiltIn::AddressDelegatecall
            | InternalBuiltIn::AddressStaticcall
            | InternalBuiltIn::Keccak256
            | InternalBuiltIn::Ripemd160
            | InternalBuiltIn::Sha256 => smallvec![self.types.bytes_memory()],
            InternalBuiltIn::AddressSend
            | InternalBuiltIn::AddressTransfer
            | InternalBuiltIn::Blobhash
            | InternalBuiltIn::Blockhash => smallvec![self.types.uint256()],
            InternalBuiltIn::ArrayPop | InternalBuiltIn::Gasleft => SmallVec::new(),
            InternalBuiltIn::Assert => smallvec![self.types.boolean()],
            InternalBuiltIn::Ecrecover => smallvec![
                self.types.bytes32(),
                self.types.uint8(),
                self.types.bytes32(),
                self.types.bytes32(),
            ],
            InternalBuiltIn::Erc7201 => smallvec![self.types.string_memory()],
            InternalBuiltIn::Selfdestruct => smallvec![self.types.address_payable()],
            InternalBuiltIn::Unwrap(definition_id) => {
                smallvec![
                    self.types
                        .register_type(Type::UserDefinedValue(UserDefinedValueType {
                            definition_id: *definition_id,
                        }))
                ]
            }
            InternalBuiltIn::Wrap(definition_id) => {
                let Some(Definition::UserDefinedValueType(udvt)) =
                    self.binder.find_definition_by_id(*definition_id)
                else {
                    unreachable!("definition bound to wrap built-in is not a UDVT");
                };
                // The UDVT's underlying type failed to resolve.
                smallvec![udvt.target_type_id?]
            }
            _ => return None,
        };
        Some(parameter_types)
    }

    /// Matches `argument_types` against the overloads of `built_in`.
    pub(crate) fn match_overload(
        &mut self,
        built_in: &InternalBuiltIn,
        argument_types: &[TypeId],
    ) -> BuiltInOverloadMatch {
        let accepted = match built_in {
            InternalBuiltIn::ArrayPush(element_type) => match argument_types {
                [] => true,
                // TODO(validation) SDR[1615]: an element that holds a mapping
                // cannot be pushed as a value. The argument is not checked.
                [_] if self.contains_mapping(*element_type) => true,
                // TODO: copying an array into storage also converts its
                // elements, eg. a `uint8[2]` into a `uint16[2]`, which no
                // conversion check covers yet. The argument is not checked.
                [_] if self.is_array(*element_type) => true,
                // The value is copied into storage, from any data location.
                [value] => {
                    let element_type = self.memory_type_id(*element_type);
                    self.types.implicitly_convertible_to(*value, element_type)
                }
                _ => false,
            },
            InternalBuiltIn::Require => match argument_types {
                [condition] => self
                    .types
                    .implicitly_convertible_to(*condition, self.types.boolean()),
                [condition, reason] => {
                    self.types
                        .implicitly_convertible_to(*condition, self.types.boolean())
                        && (self
                            .types
                            .implicitly_convertible_to(*reason, self.types.string_memory())
                            || self.is_error_since_custom_require(*reason))
                }
                _ => false,
            },
            InternalBuiltIn::Revert => match argument_types {
                [] => true,
                [reason] => self
                    .types
                    .implicitly_convertible_to(*reason, self.types.string_memory()),
                _ => false,
            },
            _ => return BuiltInOverloadMatch::NotOverloaded,
        };
        accepted.into()
    }

    /// Whether `type_id` is an error instance, which `require` takes as its
    /// reason from 0.8.26.
    fn is_error_since_custom_require(&self, type_id: TypeId) -> bool {
        self.types.language_version() >= LanguageVersion::V0_8_26
            && matches!(self.types.get_type_by_id(type_id), Type::Error(_))
    }

    /// Whether a value of `type_id` holds a mapping, which only lives in
    /// storage.
    fn contains_mapping(&self, type_id: TypeId) -> bool {
        match self.types.get_type_by_id(type_id) {
            Type::Mapping(_) => true,
            Type::Struct(StructType { definition_id, .. }) => matches!(
                self.binder.find_definition_by_id(*definition_id),
                Some(Definition::Struct(definition)) if definition.contains_mapping
            ),
            Type::Array(ArrayType { element_type, .. })
            | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
                self.contains_mapping(*element_type)
            }
            _ => false,
        }
    }

    /// Whether `type_id` is a dynamic or a fixed-size array.
    fn is_array(&self, type_id: TypeId) -> bool {
        matches!(
            self.types.get_type_by_id(type_id),
            Type::Array(_) | Type::FixedSizeArray(_)
        )
    }

    /// The memory form of `type_id`, if it is a reference type.
    fn memory_type_id(&mut self, type_id: TypeId) -> TypeId {
        let type_ = self.types.get_type_by_id(type_id);
        if type_.data_location().is_some() {
            self.types
                .register_type_with_data_location(type_.clone(), DataLocation::Memory)
        } else {
            type_id
        }
    }
}
