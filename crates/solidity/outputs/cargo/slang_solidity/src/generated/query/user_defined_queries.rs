// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cursor::Cursor;

pub(crate) trait UserDefinedQueries {
    fn query_one(foo: &Cursor);

    fn query_two(foo: &Cursor, bar: &Cursor);

    fn query_three(foo: &Cursor, bar: &Cursor, baz: &Cursor);
}
