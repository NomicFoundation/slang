#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = One,
    versions = ["1.0.0"],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Struct(
                        name = One,
                        fields = (
                            field_1 = Optional(reference = Two, enabled = From("3.0.0")),
                            field_2 = Optional(reference = Two)
                        )
                    ),
                    Token(name = Two, scanner = Atom("two"))
                ]
            )]
        )]
    )],
    built_ins = []
));

fn main() {}
