#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    binding_rules_file = "bindings/rules.msgb",
    root_item = One,
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
    built_ins = []
));

fn main() {}
