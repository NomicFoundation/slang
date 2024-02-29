pub struct UserDefinedQueriesImpl;

// TODO(#554): invoke these from the code that will build the scope graph:
impl crate::query::UserDefinedQueries for UserDefinedQueriesImpl {
    fn query_one(_foo: &crate::cursor::Cursor) {}

    fn query_two(_foo: &crate::cursor::Cursor, _bar: &crate::cursor::Cursor) {}

    fn query_three(
        _foo: &crate::cursor::Cursor,
        _bar: &crate::cursor::Cursor,
        _baz: &crate::cursor::Cursor,
    ) {
    }
}
