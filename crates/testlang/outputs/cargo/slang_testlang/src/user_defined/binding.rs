use crate::generated::binding::Context;
use crate::generated::query::QueryResult;

pub struct ActionsImpl;

// TODO(#554): invoke these from the code that will build the scope graph:
impl crate::binding::Actions for ActionsImpl {
    fn binding_action_one(_context: &mut Context, _query_result: &QueryResult) {}
    fn binding_action_two(_context: &mut Context, _query_result: &QueryResult) {}
}
