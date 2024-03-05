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

pub(crate) trait Actions {
    fn binding_action_one(context: &mut Context, query_result: &QueryResult);
    fn binding_action_two(context: &mut Context, query_result: &QueryResult);
}

pub fn compute_bindings(context: &mut Context, cursor: Cursor) {
    type Action = dyn Fn(&mut Context, &QueryResult);
    let queries = vec![
        QueryKind::PairsOfIdentifiers.into(),
        QueryKind::AllIdentifiers.into(),
    ];
    let actions: Vec<&Action> = vec![
        &ActionsImpl::binding_action_one,
        &ActionsImpl::binding_action_two,
    ];
    for result in cursor.query(queries) {
        actions[result.query_number](context, &result);
    }
}
