#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [Token(
                    name = Bar,
                    definitions = [TokenDefinition(Atom("bar"))]
                )],
                unrecognized_field = true
            )]
        )]
    )],
    built_ins = []
));

fn main() {}
