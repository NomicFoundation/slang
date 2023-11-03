#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    root_item = One,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Struct(
                    name = One,
                    fields = (
                        field_1 = Optional(kind = Terminal([Two]), enabled = From("3.0.0")),
                        field_2 = Optional(kind = Terminal([Two]))
                    )
                ),
                Token(
                    name = Two,
                    definitions = [TokenDefinition(scanner = Atom("two"))]
                )
            ]
        )]
    )]
));

fn main() {}
