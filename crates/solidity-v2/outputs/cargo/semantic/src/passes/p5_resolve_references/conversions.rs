use num_bigint::BigInt;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::Definition;
use crate::types::{
    AddressType, ArraySliceType, ByteArrayType, ContractType, ConversionError, EnumType,
    FixedPointNumberType, IntegerType, InterfaceType, LiteralKind, Number, Type, TypeId,
    UserMetaType, literals,
};

impl Pass<'_> {
    /// Checks whether a value of type `from_type_id` can be explicitly
    /// converted to `to_type_id`, eg. in `uint8(x)`, and if not, why. Every
    /// implicit conversion is also an explicit one. A reference target is
    /// expected to be already relocated to the data location of the argument.
    pub(super) fn check_explicit_conversion(
        &self,
        from_type_id: TypeId,
        to_type_id: TypeId,
    ) -> Result<(), ConversionError> {
        let implicit = self
            .types
            .check_implicit_conversion(from_type_id, to_type_id);
        if implicit.is_ok() {
            return implicit;
        }
        let to_type = self.types.get_type_by_id(to_type_id);

        let allowed = match self.types.get_type_by_id(from_type_id) {
            Type::ArraySlice(ArraySliceType { array_type_id }) => {
                return self.check_explicit_conversion(*array_type_id, to_type_id);
            }

            // A string literal only converts implicitly, so why it does not is
            // why its explicit conversion fails too.
            Type::Literal(LiteralKind::String { .. } | LiteralKind::HexString { .. }) => {
                return implicit;
            }

            Type::Address(AddressType { is_payable }) => {
                self.address_explicitly_convertible_to(*is_payable, to_type)
            }
            Type::Literal(LiteralKind::Address { .. }) => {
                self.address_explicitly_convertible_to(false, to_type)
            }

            Type::Integer(IntegerType { is_signed, bits }) => {
                integer_explicitly_convertible_to(*is_signed, *bits, to_type)
            }

            Type::Literal(
                kind @ (LiteralKind::Integer { .. }
                | LiteralKind::HexInteger { .. }
                | LiteralKind::Rational { .. }),
            ) => self.number_literal_explicitly_convertible_to(kind, to_type),

            Type::FixedPointNumber(_) => fixed_point_explicitly_convertible_to(to_type),

            Type::ByteArray(ByteArrayType { width }) => match to_type {
                Type::ByteArray(_) => true,
                Type::Integer(IntegerType { is_signed, bits }) => !is_signed && *bits == width * 8,
                Type::Address(AddressType { is_payable }) => !is_payable && *width == 20,
                Type::FixedPointNumber(FixedPointNumberType { bits, .. }) => *bits == width * 8,
                _ => false,
            },

            // `this` in a library is typed as the library.
            Type::Library(_) => matches!(to_type, Type::Address(AddressType { is_payable: false })),

            Type::Contract(ContractType { definition_id })
            | Type::Interface(InterfaceType { definition_id }) => match to_type {
                Type::Address(AddressType { is_payable }) => {
                    !is_payable || self.is_payable_contract(*definition_id)
                }
                _ => false,
            },

            // `bytes` and `string` convert into each other within the same data
            // location, and `bytes` into `bytesN`.
            Type::Bytes(_) => match to_type {
                Type::String(_) => true,
                Type::ByteArray(_) => self.types.language_version() >= LanguageVersion::V0_8_5,
                _ => false,
            },
            Type::String(_) => matches!(to_type, Type::Bytes(_)),

            Type::Enum(_) => matches!(
                to_type,
                Type::Integer(IntegerType {
                    is_signed: false,
                    ..
                })
            ),

            // A library name converts to its address, as in `address(L)`.
            Type::UserMetaType(UserMetaType { definition_id }) => {
                matches!(to_type, Type::Address(AddressType { is_payable: false }))
                    && matches!(
                        self.binder.find_definition_by_id(*definition_id),
                        Some(Definition::Library(_))
                    )
            }

            // Function values have no expressible conversion target, and the rest
            // only convert implicitly.
            _ => false,
        };
        if allowed {
            Ok(())
        } else {
            Err(ConversionError::NotAllowed)
        }
    }

    fn address_explicitly_convertible_to(&self, is_payable: bool, to_type: &Type) -> bool {
        match to_type {
            Type::Address(_) => true,
            Type::Contract(ContractType { definition_id })
            | Type::Interface(InterfaceType { definition_id }) => {
                is_payable || !self.is_payable_contract(*definition_id)
            }
            Type::Library(_) => true,
            Type::Integer(IntegerType { is_signed, bits }) => {
                !is_payable && !is_signed && *bits == 160
            }
            Type::ByteArray(ByteArrayType { width }) => !is_payable && *width == 20,
            _ => false,
        }
    }

    fn number_literal_explicitly_convertible_to(&self, kind: &LiteralKind, to_type: &Type) -> bool {
        let Some(value) = Number::from_literal_kind(kind) else {
            return false;
        };
        match to_type {
            Type::ByteArray(_) | Type::Integer(_) => false,
            Type::Address(AddressType { is_payable }) => {
                value.is_zero()
                    || (!is_payable
                        && value.as_integer().is_some_and(|value| {
                            literals::numbers::integer_literal_fits(value, false, 160)
                        }))
            }
            Type::Enum(EnumType { definition_id }) => {
                !value.is_negative()
                    && value.as_integer().is_some_and(|value| {
                        *value < BigInt::from(self.enum_member_count(*definition_id))
                    })
            }
            // Otherwise the literal converts as its mobile type does.
            _ => match kind.mobile_type() {
                Some(Type::Integer(IntegerType { is_signed, bits })) => {
                    integer_explicitly_convertible_to(is_signed, bits, to_type)
                }
                Some(Type::FixedPointNumber(_)) => fixed_point_explicitly_convertible_to(to_type),
                _ => false,
            },
        }
    }

    fn enum_member_count(&self, definition_id: NodeId) -> usize {
        match self.binder.find_definition_by_id(definition_id) {
            Some(Definition::Enum(definition)) => definition.ir_node.members.len(),
            _ => unreachable!("enum type does not link an enum definition"),
        }
    }

    /// Whether the contract or interface accepts plain ether transfers: it, or
    /// a base in its linearisation, declares a `receive` function or a
    /// `payable` `fallback`.
    fn is_payable_contract(&self, definition_id: NodeId) -> bool {
        let bases = self
            .binder
            .get_linearised_bases(definition_id)
            .map_or(std::slice::from_ref(&definition_id), Vec::as_slice);
        bases.iter().any(|base_id| {
            let members = match self.binder.find_definition_by_id(*base_id) {
                Some(Definition::Contract(contract)) => &contract.ir_node.members,
                Some(Definition::Interface(interface)) => &interface.ir_node.members,
                _ => return false,
            };
            members.iter().any(|member| {
                let ir::ContractMember::FunctionDefinition(function) = member else {
                    return false;
                };
                match function.kind {
                    ir::FunctionKind::Receive => true,
                    ir::FunctionKind::Fallback => matches!(
                        function.attributes.mutability,
                        ir::FunctionMutability::Payable
                    ),
                    _ => false,
                }
            })
        })
    }
}

fn integer_explicitly_convertible_to(is_signed: bool, bits: u32, to_type: &Type) -> bool {
    match to_type {
        Type::Integer(IntegerType {
            is_signed: to_signed,
            bits: to_bits,
        }) => bits == *to_bits || is_signed == *to_signed,
        Type::Address(AddressType { is_payable }) => !is_payable && !is_signed && bits == 160,
        Type::ByteArray(ByteArrayType { width }) => !is_signed && bits == width * 8,
        Type::Enum(_) => true,
        Type::FixedPointNumber(FixedPointNumberType {
            is_signed: to_signed,
            bits: to_bits,
            ..
        }) => is_signed == *to_signed && bits == *to_bits,
        _ => false,
    }
}

fn fixed_point_explicitly_convertible_to(to_type: &Type) -> bool {
    matches!(to_type, Type::FixedPointNumber(_) | Type::Integer(_))
}
