#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    binding_rules_file = "bindings/rules.msgb",
    root_item = Bar1,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [
            Topic(
                title = "Topic One",
                lexical_context = Foo,
                items = [
                    Struct(name = Bar1, fields = (field = Required(Bar2))),
                    Struct(name = Bar2, fields = (field = Required(Bar1)))
                ]
            ),
            Topic(
                title = "Topic Two",
                lexical_context = Foo,
                items = [Unrecognized(true)]
            )
        ]
    )],
    built_ins = []
));

fn main() {}
