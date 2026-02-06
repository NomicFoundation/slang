#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0", "4.0.0", "5.0.0"],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            // title = "Section One"
            topics = []
        )]
    )],
    built_ins = []
));

fn main() {}
