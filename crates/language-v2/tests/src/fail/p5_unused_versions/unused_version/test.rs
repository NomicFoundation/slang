#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            lexical_context = Foo,
            items = [
                Struct(
                    name = Bar,
                    fields = (field_1 = Optional(reference = Baz, enabled = From("2.0.0")))
                ),
                Token(name = Baz, definitions = [TokenDefinition(Atom("baz"))])
            ]
        )]
    )],
    built_ins = []
));

fn main() {}
