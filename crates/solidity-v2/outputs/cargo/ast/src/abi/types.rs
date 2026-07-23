use std::fmt;
use std::sync::Arc;

use ruint::aliases::U256;
use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::TypeId;

use crate::ast::{Definition as AstDefinition, Type as AstType};

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
                // `tuple`/`tuple[]` spelling is intentionally not produced here; it
                // is left to a (future) `Serialize` implementation.
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
    match value {
        AstType::Address(_) | AstType::Contract(_) | AstType::Interface(_) => {
            Some(AbiType::Address)
        }
        AstType::Boolean(_) => Some(AbiType::Boolean),
        AstType::Bytes(_) => Some(AbiType::Bytes),
        AstType::String(_) => Some(AbiType::String),
        // Every function type maps to the ABI `function` type regardless of its
        // visibility. Strictly, only *external* function types are ABI-encodable
        // (as a `bytes24` of address + selector); internal function types are
        // not. We render them permissively rather than rejecting non-external
        // ones, because a valid external signature can never contain an internal
        // function type anyway, and callers that care can inspect visibility on
        // the source type themselves.
        AstType::Function(_) => Some(AbiType::Function),
        AstType::Integer(integer) => Some(AbiType::Integer {
            is_signed: integer.is_signed(),
            bits: integer.bits(),
        }),
        AstType::Enum(_) => Some(AbiType::Integer {
            is_signed: false,
            bits: 8,
        }),
        AstType::ByteArray(byte_array) => Some(AbiType::ByteArray {
            width: byte_array.width(),
        }),
        AstType::FixedPointNumber(fixed) => Some(AbiType::FixedPointNumber {
            is_signed: fixed.is_signed(),
            bits: fixed.bits(),
            decimal_places: fixed.decimal_places(),
        }),
        AstType::Array(array) => {
            let element = abi_type_from_ast_type(&array.element_type(), visited_structs)?;
            Some(AbiType::Array {
                element: Box::new(element),
            })
        }
        AstType::FixedSizeArray(array) => {
            let element = abi_type_from_ast_type(&array.element_type(), visited_structs)?;
            Some(AbiType::FixedSizeArray {
                element: Box::new(element),
                size: array.size(),
            })
        }
        AstType::Struct(struct_type) => {
            let AstDefinition::Struct(definition) = struct_type.definition() else {
                unreachable!("a struct type resolves to a struct definition");
            };
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
        AstType::UserDefinedValue(udvt) => {
            abi_type_from_ast_type(&udvt.target_type()?, visited_structs)
        }
        AstType::Library(_)
        | AstType::Literal(_)
        | AstType::Mapping(_)
        | AstType::MetaType(_)
        | AstType::Tuple(_)
        | AstType::UserMetaType(_)
        | AstType::Void(_) => None,
    }
}
