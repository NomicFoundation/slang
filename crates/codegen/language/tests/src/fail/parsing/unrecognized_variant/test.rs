#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    root_item = Bar1,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [
            Topic(
                title = "Topic One",
                items = [
                    Struct(name = Bar1, fields = (field = Required(Bar2))),
                    Struct(name = Bar2, fields = (field = Required(Bar1)))
                ]
            ),
            Topic(title = "Topic Two", items = [Unrecognized(true)])
        ]
    )]
));

fn main() {}
