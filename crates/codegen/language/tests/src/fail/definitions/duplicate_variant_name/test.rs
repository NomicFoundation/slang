#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    documentation_dir = "foo/bar",
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
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
                Token(
                    name = Baz1,
                    definitions = [TokenDefinition(scanner = Atom("baz1"))]
                ),
                Token(
                    name = Baz2,
                    definitions = [TokenDefinition(scanner = Atom("baz2"))]
                )
            ]
        )]
    )],
    queries = ()
));

fn main() {}
