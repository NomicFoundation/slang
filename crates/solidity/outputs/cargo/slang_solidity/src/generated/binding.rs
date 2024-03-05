// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use stack_graphs::graph::{File, StackGraph};

use super::cursor::Cursor;
use super::queries::QueryKind;
use super::query::QueryResult;
use crate::user_defined::binding::ActionsImpl;

pub(crate) struct Context {
    pub graph: StackGraph,
    pub file: File,
}

pub(crate) trait Actions {}

pub fn compute_bindings(context: &mut Context, cursor: Cursor) {
    type Action = dyn Fn(&mut Context, &QueryResult);
    let queries = vec![];
    let actions: Vec<&Action> = vec![];
    for result in cursor.query(queries) {
        actions[result.query_number](context, &result);
    }
}
