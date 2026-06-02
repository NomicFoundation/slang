#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = One,
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    evm_hard_forks = [Frontier, Homestead, Cancun],
    contexts = [LexicalContext(
        name = Foo,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Struct(name = One, fields = (field_1 = Required(Two))),
                    Token(name = Two, scanner = Atom("two"))
                ]
            )]
        )]
    )],
    built_ins = [BuiltInContext(
        name = Globals,
        scopes = [BuiltInScope(
            name = Global,
            definitions = [BuiltInDefinition(name = MyBuiltIn, evm_enabled = Always)]
        )]
    )]
));

fn main() {}
