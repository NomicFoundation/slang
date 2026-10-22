#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = One,
    versions = ["1.0.0"],
    evm_targets = [],
    contexts = [LexicalContext(
        name = ContextA,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Struct(name = One, fields = (field_1 = Required(Two), field_2 = Required(Three))),
                    Keyword(name = Two, scanner = Atom("two")),
                    Keyword(name = Three, scanner = Atom("three"), reject_suffix = Atom("_"))
                ]
            )]
        )]
    )],
    built_ins = []
));

fn main() {}
