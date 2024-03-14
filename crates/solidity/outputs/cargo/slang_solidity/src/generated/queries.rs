// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::sync::Once;

#[cfg(feature = "slang_napi_interfaces")]
use napi_derive::napi;

use crate::generated::query::Query;

mod __query_kind {
    #[allow(unreachable_code)]
    #[derive(
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde::Serialize,
        strum_macros::AsRefStr,
        strum_macros::Display,
        strum_macros::EnumString,
    )]
    #[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "queries"))]
    #[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
    pub enum QueryKind {}
}
pub use __query_kind::QueryKind;

static mut QUERIES: Vec<Query> = Vec::new();
static INIT: Once = Once::new();

pub struct Queries;

impl std::ops::Index<QueryKind> for Queries {
    type Output = Query;

    fn index(&self, kind: QueryKind) -> &Self::Output {
        unsafe {
            INIT.call_once(|| {});

            &QUERIES[kind as usize]
        }
    }
}

impl From<QueryKind> for Query {
    fn from(kind: QueryKind) -> Self {
        Queries[kind].clone()
    }
}
