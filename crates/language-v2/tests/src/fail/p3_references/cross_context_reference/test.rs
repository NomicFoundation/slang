#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = One,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    contexts = [
        LexicalContext(
            name = ContextA,
            sections = [Section(
                title = "Section One",
                topics = [Topic(
                    title = "Topic One",
                    items = [
                        Struct(name = One, fields = (field_1 = Required(Two))),
                        Token(name = Two, scanner = Atom("two"))
                    ]
                )]
            )]
        ),
        LexicalContext(
            name = ContextB,
            sections = [Section(
                title = "Section Two",
                topics = [Topic(
                    title = "Topic Two",
                    items = [
                        Struct(name = Three, fields = (field_1 = Required(Two))),
                        Token(name = Four, scanner = Atom("four"))
                    ]
                )]
            )]
        )
    ]
));

fn main() {}
