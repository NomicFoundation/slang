use language_v2_definition::model::{
    Field, Item, Language, Scanner, Section, StructItem, TokenDefinition, TokenItem, Topic,
    TriviaParser,
};
use semver::Version;

language_v2_macros::compile!(Language(
    name = Tiny,
    binding_rules_file = "tiny/bindings/rules.msgb",
    root_item = Foo,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            lexical_context = Tiny,
            items = [
                Struct(
                    name = Foo,
                    fields = (
                        bar = Required(Bar),
                        baz = Required(Baz),
                        baz_again = Required(Baz)
                    )
                ),
                Token(name = Bar, definitions = [TokenDefinition(Atom("bar"))]),
                Token(name = Baz, definitions = [TokenDefinition(Atom("baz"))])
            ]
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
            binding_rules_file: "tiny/bindings/rules.msgb".into(),
            root_item: "Foo".into(),
            leading_trivia: TriviaParser::Sequence { parsers: [].into() },
            trailing_trivia: TriviaParser::Sequence { parsers: [].into() },
            versions: [
                Version::parse("1.0.0").unwrap(),
                Version::parse("2.0.0").unwrap(),
                Version::parse("3.0.0").unwrap(),
            ]
            .into(),
            sections: vec![Section {
                title: "Section One".into(),
                topics: vec![Topic {
                    title: "Topic One".into(),
                    lexical_context: "Tiny".into(),
                    items: [
                        Item::Struct {
                            item: StructItem {
                                name: "Foo".into(),
                                enabled: None,
                                error_recovery: None,
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
                                .into()
                            }
                            .into()
                        },
                        Item::Token {
                            item: TokenItem {
                                name: "Bar".into(),
                                enabled: None,
                                definitions: [TokenDefinition {
                                    scanner: Scanner::Atom { atom: "bar".into() }
                                }]
                                .into()
                            }
                            .into()
                        },
                        Item::Token {
                            item: TokenItem {
                                name: "Baz".into(),
                                enabled: None,
                                definitions: [TokenDefinition {
                                    scanner: Scanner::Atom { atom: "baz".into() }
                                }]
                                .into()
                            }
                            .into()
                        }
                    ]
                    .into()
                }],
            }],
            built_ins: [].into(),
        }
        .into(),
    );
}
