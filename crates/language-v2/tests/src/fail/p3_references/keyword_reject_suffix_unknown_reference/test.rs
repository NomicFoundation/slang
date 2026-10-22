#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = Bar,
    versions = ["1.0.0"],
    evm_targets = [],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Struct(name = Bar, fields = (field_1 = Required(Baz))),
                    Keyword(name = Baz, scanner = Atom("baz"), reject_suffix = Fragment(Unknown))
                ]
            )]
        )]
    )],
    built_ins = []
));

fn main() {}
