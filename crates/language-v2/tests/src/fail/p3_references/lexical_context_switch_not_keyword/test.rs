#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = One,
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
                    items = [Token(name = Three, scanner = Atom("three"))]
                )]
            )]
        )
    ],
    built_ins = []
));

fn main() {}
