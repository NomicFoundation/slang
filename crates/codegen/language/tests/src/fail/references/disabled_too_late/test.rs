#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    documentation_dir = "foo/bar",
    root_item = One,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0", "4.0.0", "5.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Struct(
                    name = One,
                    fields = (
                        field_1 = Optional(
                            reference = Two,
                            enabled = Range(from = "2.0.0", till = "3.0.0")
                        ),
                        field_2 = Optional(reference = Three)
                    )
                ),
                Struct(
                    name = Two,
                    enabled = Range(from = "2.0.0", till = "3.0.0"),
                    fields = (field_1 = Optional(reference = Three, enabled = Till("4.0.0")))
                ),
                Token(
                    name = Three,
                    definitions = [TokenDefinition(scanner = Atom("three"))]
                )
            ]
        )]
    )]
));

fn main() {}
