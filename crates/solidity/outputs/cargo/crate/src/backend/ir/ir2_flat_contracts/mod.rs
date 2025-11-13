#[path = "nodes.generated.rs"]
mod nodes;
pub use nodes::*;

#[path = "index.generated.rs"]
pub mod index;
#[path = "transformer.generated.rs"]
pub mod transformer;
#[path = "visitor.generated.rs"]
pub mod visitor;

pub use super::ir1_structured_ast as input;
use crate::backend::{binder, types};

impl From<&FunctionVisibility> for binder::FunctionVisibility {
    fn from(value: &FunctionVisibility) -> Self {
        match value {
            FunctionVisibility::External => Self::External,
            FunctionVisibility::Internal => Self::Internal,
            FunctionVisibility::Public => Self::Public,
            FunctionVisibility::Private => Self::Private,
        }
    }
}

impl From<&StorageLocation> for types::DataLocation {
    fn from(value: &StorageLocation) -> Self {
        match value {
            StorageLocation::MemoryKeyword => Self::Memory,
            StorageLocation::StorageKeyword => Self::Storage,
            StorageLocation::CallDataKeyword => Self::Calldata,
        }
    }
}

impl From<&FunctionMutability> for types::FunctionTypeKind {
    fn from(value: &FunctionMutability) -> Self {
        match value {
            FunctionMutability::Pure => Self::Pure,
            FunctionMutability::View => Self::View,
            FunctionMutability::NonPayable => Self::NonPayable,
            FunctionMutability::Payable => Self::Payable,
        }
    }
}

impl From<&StateVariableVisibility> for binder::StateVariableVisibility {
    fn from(value: &StateVariableVisibility) -> Self {
        match value {
            StateVariableVisibility::Public => Self::Public,
            StateVariableVisibility::Private => Self::Private,
            StateVariableVisibility::Internal => Self::Internal,
        }
    }
}
