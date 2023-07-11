#[macro_use]
extern crate napi_derive;

pub mod napi;
pub mod rust;

pub mod syntax {
    pub mod nodes {
        pub use crate::napi::kinds::{ProductionKind, RuleKind, TokenKind};
    }
}
