#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    versions = ["1.0.0"],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Enum(
                        name = Bar,
                        variants = [
                            EnumVariant(reference = Baz1),
                            EnumVariant(reference = Baz2),
                            EnumVariant(reference = Baz1)
                        ]
                    ),
                    Token(name = Baz1, scanner = Atom("baz1")),
                    Token(name = Baz2, scanner = Atom("baz2"))
                ]
            )]
        )]
    )]
));

fn main() {}
