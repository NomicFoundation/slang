#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    versions = ["1.0.0"],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "S",
            topics = [Topic(
                title = "T",
                items = [
                    Struct(
                        name = Bar,
                        fields = (baz = Required(Baz)),
                        parser_options = ParserOptions(inline = false, verbatim = NotCode(body))
                    ),
                    Token(name = Baz, scanner = Atom("baz"))
                ]
            )]
        )]
    )]
));

fn main() {}
