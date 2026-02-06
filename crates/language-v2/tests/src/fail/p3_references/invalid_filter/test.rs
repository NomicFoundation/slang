#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Struct(name = Bar, fields = (field = Required(Baz))),
                    Fragment(name = Baz, scanner = Atom("baz"))
                ]
            )]
        )]
    )],
    built_ins = []
));

fn main() {}
