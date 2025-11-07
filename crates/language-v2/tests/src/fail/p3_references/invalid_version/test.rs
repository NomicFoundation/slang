#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    binding_rules_file = "bindings/rules.msgb",
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
                    fields = (
                        field_1 = Optional(
                            // should have been disabled in "3.0.0"
                            reference = Baz,
                            enabled = From("2.0.0")
                        ),
                        field_2 = Optional(
                            // should have been enabled in "2.0.0"
                            reference = Baz,
                            enabled = Till("3.0.0")
                        ),
                        field_3 = Optional(
                            // should have been enabled in "2.0.0" and disabled in "3.0.0"
                            reference = Baz
                        ),
                        field_4 = Optional(
                            // correct
                            reference = Baz,
                            enabled = Range(from = "2.0.0", till = "3.0.0")
                        )
                    )
                ),
                Token(
                    name = Baz,
                    definitions = [TokenDefinition(
                        enabled = Range(from = "2.0.0", till = "3.0.0"),
                        scanner = Atom("baz")
                    )]
                )
            ]
        )]
    )],
    built_ins = []
));

fn main() {}
