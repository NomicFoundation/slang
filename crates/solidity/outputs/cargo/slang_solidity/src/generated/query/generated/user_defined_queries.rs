// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

pub(crate) trait UserDefinedQueries {
    fn query_one(foo: &crate::cursor::Cursor);

    fn query_two(foo: &crate::cursor::Cursor, bar: &crate::cursor::Cursor);

    fn query_three(
        foo: &crate::cursor::Cursor,
        bar: &crate::cursor::Cursor,
        baz: &crate::cursor::Cursor,
    );
}
