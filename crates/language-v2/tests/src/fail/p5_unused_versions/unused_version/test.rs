#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Struct(
                        name = Bar,
                        fields = (field_1 = Optional(reference = Baz, enabled = From("2.0.0")))
                    ),
                    Token(name = Baz, scanner = Atom("baz"))
                ]
            )]
        )]
    )]
));

fn main() {}
