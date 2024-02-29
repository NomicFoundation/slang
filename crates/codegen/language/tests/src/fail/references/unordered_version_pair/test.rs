#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    documentation_dir = "foo/bar",
    root_item = One,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
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
                            enabled = Range(from = "3.0.0", till = "2.0.0")
                        ),
                        field_2 = Optional(reference = Two)
                    )
                ),
                Token(
                    name = Two,
                    definitions = [TokenDefinition(scanner = Atom("two"))]
                )
            ]
        )]
    )],
    queries = ()
));

fn main() {}
