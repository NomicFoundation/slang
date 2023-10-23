use codegen_language_definition::{
    Field, FieldKind, Item, Language, Scanner, Section, StructItem, TokenDefinition, TokenItem,
    Topic, TriviaParser,
};
use semver::Version;

codegen_language_macros::compile!(Language(
    name = Tiny,
    root_item = Foo,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Struct(
                    name = Foo,
                    fields = (
                        bar = Required(Terminal([Bar])),
                        baz = Required(Terminal([Baz])),
                        baz_again = Required(Terminal([Baz]))
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
));

#[test]
fn definition() {
    assert_eq!(
        tiny::TinyDefinition::create(),
        Language {
            name: "Tiny".into(),
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
                    notes_file: None,
                    lexical_context: None,
                    items: [
                        Item::Struct {
                            item: StructItem {
                                name: "Foo".into(),
                                enabled_in: None,
                                disabled_in: None,
                                error_recovery: None,
                                fields: [
                                    (
                                        "bar".into(),
                                        Field::Required {
                                            kind: FieldKind::Terminal {
                                                items: ["Bar".into()].into()
                                            }
                                        }
                                    )
                                        .into(),
                                    (
                                        "baz".into(),
                                        Field::Required {
                                            kind: FieldKind::Terminal {
                                                items: ["Baz".into()].into()
                                            }
                                        }
                                    )
                                        .into(),
                                    (
                                        "baz_again".into(),
                                        Field::Required {
                                            kind: FieldKind::Terminal {
                                                items: ["Baz".into()].into()
                                            }
                                        }
                                    )
                                        .into()
                                ]
                                .into()
                            }
                        }
                        .into(),
                        Item::Token {
                            item: TokenItem {
                                name: "Bar".into(),
                                definitions: [TokenDefinition {
                                    enabled_in: None,
                                    disabled_in: None,
                                    scanner: Scanner::Atom { atom: "bar".into() }
                                }]
                                .into()
                            }
                        }
                        .into(),
                        Item::Token {
                            item: TokenItem {
                                name: "Baz".into(),
                                definitions: [TokenDefinition {
                                    enabled_in: None,
                                    disabled_in: None,
                                    scanner: Scanner::Atom { atom: "baz".into() }
                                }]
                                .into()
                            }
                        }
                        .into()
                    ]
                    .into()
                }],
            }],
        },
    );
}
