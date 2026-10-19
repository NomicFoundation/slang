use indexmap::IndexSet;
use language_v2_definition::model::{
    Field, Item, Language, LexicalContext, Section, StructItem, TokenDefinition, TokenItem,
    TokenScanner, Topic,
};
use semver::Version;

language_v2_macros::compile!(Language(
    name = Tiny,
    root_item = Foo,
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    evm_targets = [],
    contexts = [LexicalContext(
        name = Tiny,
        sections = [Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                items = [
                    Struct(
                        name = Foo,
                        fields = (
                            bar = Required(Bar),
                            baz = Required(Baz),
                            baz_again = Required(Baz)
                        )
                    ),
                    Token(
                        name = Bar,
                        definitions = [TokenDefinition(scanner = Atom("bar"))]
                    ),
                    Token(
                        name = Baz,
                        definitions = [TokenDefinition(scanner = Atom("baz"))]
                    )
                ]
            )]
        )]
    )],
    built_ins = []
));

#[test]
fn definition() {
    assert_eq!(
        tiny::TinyDefinition::create(),
        Language {
            name: "Tiny".into(),
            root_item: "Foo".into(),
            versions: [
                Version::parse("1.0.0").unwrap(),
                Version::parse("2.0.0").unwrap(),
                Version::parse("3.0.0").unwrap(),
            ]
            .into(),
            evm_targets: IndexSet::default(),
            contexts: vec![LexicalContext {
                name: "Tiny".into(),
                identifier_token: None,
                sections: vec![Section {
                    title: "Section One".into(),
                    topics: vec![Topic {
                        title: "Topic One".into(),
                        items: [
                            Item::Struct {
                                item: StructItem {
                                    name: "Foo".into(),
                                    documentable: None,
                                    enabled: None,
                                    switch_lexical_context: None,
                                    fields: [
                                        (
                                            "bar".into(),
                                            Field::Required {
                                                reference: "Bar".into()
                                            }
                                        ),
                                        (
                                            "baz".into(),
                                            Field::Required {
                                                reference: "Baz".into()
                                            }
                                        ),
                                        (
                                            "baz_again".into(),
                                            Field::Required {
                                                reference: "Baz".into()
                                            }
                                        )
                                    ]
                                    .into(),
                                    parser_options: None
                                }
                                .into()
                            },
                            Item::Token {
                                item: TokenItem {
                                    name: "Bar".into(),
                                    enabled: None,
                                    definitions: vec![TokenDefinition {
                                        scanner: TokenScanner::Atom { atom: "bar".into() },
                                        reject_suffix: None,
                                    }],
                                }
                                .into()
                            },
                            Item::Token {
                                item: TokenItem {
                                    name: "Baz".into(),
                                    enabled: None,
                                    definitions: vec![TokenDefinition {
                                        scanner: TokenScanner::Atom { atom: "baz".into() },
                                        reject_suffix: None,
                                    }],
                                }
                                .into()
                            }
                        ]
                        .into()
                    }],
                }],
            }],
            built_ins: vec![],
        }
        .into(),
    );
}
