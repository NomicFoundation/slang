#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Struct(
                    name = Bar,
                    fields = (
                        field_1 = Optional(
                            // should have been disabled in "3.0.0"
                            kind = Terminal([Baz]),
                            enabled_in = "2.0.0"
                        ),
                        field_2 = Optional(
                            // should have been enabled in "2.0.0"
                            kind = Terminal([Baz]),
                            disabled_in = "3.0.0"
                        ),
                        field_3 = Optional(
                            // should have been enabled in "2.0.0" and disabled in "3.0.0"
                            kind = Terminal([Baz])
                        ),
                        field_4 = Optional(
                            // correct
                            kind = Terminal([Baz]),
                            enabled_in = "2.0.0",
                            disabled_in = "3.0.0"
                        )
                    )
                ),
                Token(
                    name = Baz,
                    definitions = [TokenDefinition(
                        enabled_in = "2.0.0",
                        disabled_in = "3.0.0",
                        scanner = Atom("baz")
                    )]
                )
            ]
        )]
    )]
));

fn main() {}
