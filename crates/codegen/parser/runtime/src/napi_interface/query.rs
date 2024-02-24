// NAPI-exposed functions have to accept owned values
#![allow(clippy::needless_pass_by_value)]
// The functions are meant to be definitions for export, so they're not really used
#![allow(clippy::return_self_not_must_use)]

use napi::{Env, JsObject};
use napi_derive::napi;

use crate::napi_interface::cursor::Cursor;
use crate::napi_interface::{RustQuery, RustQueryResult, RustQueryResultIterator};

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
            |err| Err(napi::Error::from_reason(err)),
            |query| Ok(query.into()),
        )
    }
}

#[napi(namespace = "query")]
pub struct QueryResultIterator(RustQueryResultIterator);

#[napi(object, namespace = "query")]
pub struct QueryResult {
    pub query_number: u32,
    #[napi(ts_type = "{ [key: string]: cursor.Cursor[] }")]
    pub bindings: JsObject,
}

impl QueryResult {
    fn new(env: Env, result: RustQueryResult) -> napi::Result<Self> {
        #[allow(clippy::cast_possible_truncation)]
        let query_number = result.query_number as u32;
        let mut bindings = env.create_object()?;
        // transer all of the bindings eagerly on the assumption
        // that they've all been explicitly requested.
        for (key, value) in result.bindings {
            bindings.set_named_property(
                &key,
                value.into_iter().map(|x| x.into()).collect::<Vec<Cursor>>(),
            )?;
        }
        Ok(Self {
            query_number,
            bindings,
        })
    }
}

impl From<RustQueryResultIterator> for QueryResultIterator {
    fn from(value: RustQueryResultIterator) -> Self {
        Self(value)
    }
}

#[napi(namespace = "query")]
impl QueryResultIterator {
    #[napi(catch_unwind)]
    pub fn next(&mut self, env: Env) -> napi::Result<Option<QueryResult>> {
        match self.0.next() {
            | Some(result) => Ok(Some(QueryResult::new(env, result)?)),
            | None => Ok(None),
        }
    }
}

#[napi(namespace = "cursor")]
impl Cursor {
    #[napi(ts_return_type = "query.QueryResultIterator", catch_unwind)]
    pub fn query(
        &self,
        #[napi(ts_arg_type = "Array<query.Query>")] queries: Vec<&Query>,
    ) -> QueryResultIterator {
        self.0
            .clone()
            .query(queries.into_iter().map(|x| x.0.clone()).collect())
            .into()
    }
}
