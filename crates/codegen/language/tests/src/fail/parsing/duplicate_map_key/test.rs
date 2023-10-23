#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(
    name = Foo,
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Struct(
                    name = Bar,
                    fields = (
                        field_1 = Required(Terminal([Baz])),
                        field_1 = Required(Terminal([Baz])),
                        field_3 = Required(Terminal([Baz]))
                    )
                ),
                Token(
                    name = Baz,
                    definitions = [TokenDefinition(scanner = Atom("baz"))]
                )
            ]
        )]
    )]
);

fn main() {}
