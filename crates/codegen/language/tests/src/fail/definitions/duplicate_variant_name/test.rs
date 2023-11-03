#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
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
                        EnumVariant(name = Variant1, reference = Baz),
                        EnumVariant(name = Variant2, reference = Baz),
                        EnumVariant(name = Variant1, reference = Baz)
                    ]
                ),
                Token(
                    name = Baz,
                    definitions = [TokenDefinition(scanner = Atom("baz"))]
                )
            ]
        )]
    )]
));

fn main() {}
