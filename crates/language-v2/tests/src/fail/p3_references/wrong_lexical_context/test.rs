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
                        Struct(
                            name = One,
                            switch_lexical_context = ContextB,
                            fields = (entry = Required(Two), field_1 = Required(Three))
                        ),
                        Keyword(name = Two, value = Atom("two")),
                        Token(name = Three, definitions = [TokenDefinition(Atom("three"))])
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
                    items = [Token(
                        name = Four,
                        definitions = [TokenDefinition(Atom("four"))]
                    )]
                )]
            )]
        )
    ],
    built_ins = []
));

fn main() {}
