use smallvec::{SmallVec, smallvec};

use super::{BuiltInsResolver, InternalBuiltIn};
use crate::binder::Definition;
use crate::types::{Type, TypeId, UserDefinedValueType};

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
}
