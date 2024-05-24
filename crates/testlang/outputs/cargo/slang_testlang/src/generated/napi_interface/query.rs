// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// NAPI-exposed functions have to accept owned values
#![allow(clippy::needless_pass_by_value)]
// The functions are meant to be definitions for export, so they're not really used
#![allow(clippy::return_self_not_must_use)]

use std::collections::HashMap;

use napi::bindgen_prelude::ClassInstance;
use napi::Env;
use napi_derive::napi;

use crate::napi_interface::cursor::Cursor;
use crate::napi_interface::{RustQuery, RustQueryMatch, RustQueryMatchIterator};

#[napi(namespace = "query")]
pub struct Query(RustQuery);

impl From<RustQuery> for Query {
    fn from(value: RustQuery) -> Self {
        Self(value)
    }
}

#[napi(namespace = "query")]
impl Query {
    #[napi(factory, catch_unwind)]
    pub fn parse(text: String) -> napi::Result<Query> {
        RustQuery::parse(text.as_str()).map_or_else(
            |err| Err(napi::Error::from_reason(err.message)),
            |query| Ok(query.into()),
        )
    }
}

#[napi(namespace = "query")]
pub struct QueryMatchIterator(RustQueryMatchIterator);

#[napi(object, namespace = "query")]
pub struct QueryMatch {
    pub query_number: u32,
    #[napi(ts_type = "{ [key: string]: cursor.Cursor[] }")]
    pub bindings: HashMap<String, Vec<ClassInstance<Cursor>>>,
}

impl QueryMatch {
    fn new(env: Env, result: RustQueryMatch) -> napi::Result<Self> {
        #[allow(clippy::cast_possible_truncation)]
        let query_number = result.query_number as u32;
        // transfer all of the bindings eagerly on the assumption
        // that they've all been explicitly requested.
        let bindings = result
            .captures
            .into_iter()
            .map(|(key, values)| {
                let instances = values
                    .into_iter()
                    .map(|cursor| Cursor(cursor).into_instance(env))
                    .collect::<napi::Result<_>>()?;

                Ok((key, instances))
            })
            .collect::<napi::Result<_>>()?;

        Ok(Self {
            query_number,
            bindings,
        })
    }
}

impl From<RustQueryMatchIterator> for QueryMatchIterator {
    fn from(value: RustQueryMatchIterator) -> Self {
        Self(value)
    }
}

#[napi(namespace = "query")]
impl QueryMatchIterator {
    #[napi(catch_unwind)]
    pub fn next(&mut self, env: Env) -> napi::Result<Option<QueryMatch>> {
        match self.0.next() {
            Some(result) => Ok(Some(QueryMatch::new(env, result)?)),
            None => Ok(None),
        }
    }
}

#[napi(namespace = "cursor")]
impl Cursor {
    #[napi(ts_return_type = "query.QueryMatchIterator", catch_unwind)]
    pub fn query(
        &self,
        #[napi(ts_arg_type = "Array<query.Query>")] queries: Vec<&Query>,
    ) -> QueryMatchIterator {
        self.0
            .clone()
            .query(queries.into_iter().map(|x| x.0.clone()).collect())
            .into()
    }
}
