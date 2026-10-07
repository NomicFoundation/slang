use std::fmt;
use std::sync::Arc;

use ruint::aliases::U256;
use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::TypeId;

use crate::ast::{Definition as AstDefinition, StructDefinition, Type as AstType};

/// A type that can appear in a Solidity ABI parameter, as defined in
/// <https://docs.soliditylang.org/en/latest/abi-spec.html#types>.
#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AbiType {
    Address,
    Boolean,
    Bytes,
    String,
    Function,
    Integer {
        is_signed: bool,
        bits: u32,
    },
    ByteArray {
        width: u32,
    },
    FixedPointNumber {
        is_signed: bool,
        bits: u32,
        decimal_places: u32,
    },
    Array {
        element: Box<AbiType>,
    },
    FixedSizeArray {
        element: Box<AbiType>,
        size: U256,
    },
    Tuple(Vec<TupleComponent>),
}

impl fmt::Display for AbiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbiType::Address => write!(f, "address"),
            AbiType::Boolean => write!(f, "bool"),
            AbiType::Bytes => write!(f, "bytes"),
            AbiType::String => write!(f, "string"),
            AbiType::Function => write!(f, "function"),
            AbiType::Integer {
                is_signed: signed,
                bits,
            } => {
                let prefix = if *signed { "int" } else { "uint" };
                write!(f, "{prefix}{bits}")
            }
            AbiType::ByteArray { width } => write!(f, "bytes{width}"),
            AbiType::FixedPointNumber {
                is_signed: signed,
                bits,
                decimal_places: precision_bits,
            } => {
                let prefix = if *signed { "fixed" } else { "ufixed" };
                write!(f, "{prefix}{bits}x{precision_bits}")
            }
            AbiType::Array { element } => write!(f, "{element}[]"),
            AbiType::FixedSizeArray { element, size } => write!(f, "{element}[{size}]"),
            AbiType::Tuple(components) => {
                // A struct is always rendered as the canonical-signature form
                // `(T1,T2,...)` used for selector/signature hashing. The JSON-ABI
                // `tuple`/`tuple[]` spelling is `ContractAbi`'s serialization.
                write!(f, "(")?;
                for (i, component) in components.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", component.ty)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct TupleComponent {
    pub(crate) name: String,
    pub(crate) ty: AbiType,
}

impl TupleComponent {
    pub fn new(name: impl Into<String>, ty: AbiType) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn abi_type(&self) -> &AbiType {
        &self.ty
    }
}

/// Converts a semantic [`TypeId`] to its [`AbiType`], or `None` if the type has
/// no ABI representation.
///
/// This is a thin adapter over the single conversion implementation
/// ([`abi_type_from_ast_type`]).
pub(crate) fn type_as_abi_type(
    semantic: &Arc<SemanticContext>,
    type_id: TypeId,
) -> Option<AbiType> {
    abi_type_from_ast_type(&AstType::create(type_id, semantic), &mut Set::default())
}

/// Whether the type has an ABI representation, decided by the same [`abi_shape`] as
/// [`abi_type_from_ast_type`] but without building it: nothing is allocated for a type that will
/// be rendered later.
pub(crate) fn is_abi_type(semantic: &Arc<SemanticContext>, type_id: TypeId) -> bool {
    has_abi_type(&AstType::create(type_id, semantic), &mut Set::default())
}

fn has_abi_type(value: &AstType, visited_structs: &mut Set<NodeId>) -> bool {
    match abi_shape(value) {
        None => false,
        Some(AbiShape::Scalar(_)) => true,
        Some(AbiShape::Array(element) | AbiShape::FixedSizeArray(element, _)) => {
            has_abi_type(&element, visited_structs)
        }
        Some(AbiShape::Struct(definition)) => {
            if !visited_structs.insert(definition.node_id()) {
                return false;
            }
            let representable = definition.members().iter().all(|member| {
                member
                    .get_type()
                    .is_some_and(|member_type| has_abi_type(&member_type, visited_structs))
            });
            visited_structs.remove(&definition.node_id());
            representable
        }
    }
}

/// Error returned by `TryFrom<&Type>` for [`AbiType`] when the given
/// [`Type`](crate::ast::Type) has no ABI representation — e.g. a mapping, an
/// internal tuple, a library, or a malformed recursive struct.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct NotAnAbiType;

impl fmt::Display for NotAnAbiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type has no ABI representation")
    }
}

impl std::error::Error for NotAnAbiType {}

impl TryFrom<&AstType> for AbiType {
    type Error = NotAnAbiType;

    fn try_from(value: &AstType) -> Result<Self, Self::Error> {
        abi_type_from_ast_type(value, &mut Set::default()).ok_or(NotAnAbiType)
    }
}

/// The single source of truth for converting a Solidity type to its [`AbiType`].
/// Both the public `TryFrom<&AstType>` and the semantic-`TypeId` entry point
/// ([`type_as_abi_type`]) funnel through here.
fn abi_type_from_ast_type(value: &AstType, visited_structs: &mut Set<NodeId>) -> Option<AbiType> {
    match abi_shape(value)? {
        AbiShape::Scalar(scalar) => Some(scalar),
        AbiShape::Array(element) => Some(AbiType::Array {
            element: Box::new(abi_type_from_ast_type(&element, visited_structs)?),
        }),
        AbiShape::FixedSizeArray(element, size) => Some(AbiType::FixedSizeArray {
            element: Box::new(abi_type_from_ast_type(&element, visited_structs)?),
            size,
        }),
        AbiShape::Struct(definition) => {
            // Recursive structs are not valid Solidity, but guard against cycles
            // to avoid unbounded recursion if malformed types reach this point.
            if !visited_structs.insert(definition.node_id()) {
                return None;
            }
            let mut components = Vec::new();
            for member in definition.members().iter() {
                let name = member.name().name().to_owned();
                let ty = abi_type_from_ast_type(&member.get_type()?, visited_structs)?;
                components.push(TupleComponent::new(name, ty));
            }
            visited_structs.remove(&definition.node_id());
            Some(AbiType::Tuple(components))
        }
    }
}

/// One level of a type's ABI representation, seen through slices and user-defined value types.
pub(crate) enum AbiShape {
    Scalar(AbiType),
    Array(AstType),
    FixedSizeArray(AstType, U256),
    Struct(StructDefinition),
}

/// `None` when the type has no ABI representation at this level.
pub(crate) fn abi_shape(value: &AstType) -> Option<AbiShape> {
    let scalar = match value {
        AstType::Address(_) | AstType::Contract(_) | AstType::Interface(_) => AbiType::Address,
        AstType::Boolean(_) => AbiType::Boolean,
        AstType::Bytes(_) => AbiType::Bytes,
        AstType::String(_) => AbiType::String,
        // Every function type maps to the ABI `function` type regardless of its
        // visibility. Strictly, only *external* function types are ABI-encodable
        // (as a `bytes24` of address + selector); internal function types are
        // not. We render them permissively rather than rejecting non-external
        // ones, because a valid external signature can never contain an internal
        // function type anyway, and callers that care can inspect visibility on
        // the source type themselves.
        AstType::Function(_) => AbiType::Function,
        AstType::Integer(integer) => AbiType::Integer {
            is_signed: integer.is_signed(),
            bits: integer.bits(),
        },
        AstType::Enum(_) => AbiType::Integer {
            is_signed: false,
            bits: 8,
        },
        AstType::ByteArray(byte_array) => AbiType::ByteArray {
            width: byte_array.width(),
        },
        AstType::FixedPointNumber(fixed) => AbiType::FixedPointNumber {
            is_signed: fixed.is_signed(),
            bits: fixed.bits(),
            decimal_places: fixed.decimal_places(),
        },
        AstType::Array(array) => return Some(AbiShape::Array(array.element_type())),
        // A slice ABI-encodes exactly like the array it slices.
        AstType::ArraySlice(slice) => return abi_shape(&slice.array_type()),
        AstType::FixedSizeArray(array) => {
            return Some(AbiShape::FixedSizeArray(array.element_type(), array.size()));
        }
        AstType::Struct(struct_type) => {
            let AstDefinition::Struct(definition) = struct_type.definition() else {
                unreachable!("a struct type resolves to a struct definition");
            };
            return Some(AbiShape::Struct(definition));
        }
        AstType::UserDefinedValue(udvt) => return abi_shape(&udvt.target_type()?),
        AstType::Error(_)
        | AstType::Event(_)
        | AstType::Library(_)
        | AstType::Literal(_)
        | AstType::Mapping(_)
        | AstType::MetaType(_)
        | AstType::Tuple(_)
        | AstType::UserMetaType(_) => return None,
    };
    Some(AbiShape::Scalar(scalar))
}
