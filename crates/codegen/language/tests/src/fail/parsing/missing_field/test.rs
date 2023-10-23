#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(
    name = Foo,
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0", "4.0.0", "5.0.0"],
    // sections = []
);

fn main() {}
