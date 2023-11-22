use codegen_language_definition::model::{
    Field, FieldKind, Item, Language, Scanner, Section, Spanned, StructItem, TokenDefinition,
    TokenItem, Topic, TriviaParser,
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
            name: Spanned::without_span("Tiny".into()),
            root_item: Spanned::without_span("Foo".into()),
            leading_trivia: TriviaParser::Sequence { parsers: [].into() },
            trailing_trivia: TriviaParser::Sequence { parsers: [].into() },
            versions: [
                Spanned::without_span(Version::parse("1.0.0").unwrap()),
                Spanned::without_span(Version::parse("2.0.0").unwrap()),
                Spanned::without_span(Version::parse("3.0.0").unwrap()),
            ]
            .into(),
            sections: vec![Section {
                title: Spanned::without_span("Section One".into()),
                topics: vec![Topic {
                    title: Spanned::without_span("Topic One".into()),
                    notes_file: None,
                    lexical_context: None,
                    items: [
                        Item::Struct {
                            item: StructItem {
                                name: Spanned::without_span("Foo".into()),
                                enabled: None,
                                error_recovery: None,
                                fields: [
                                    (
                                        Spanned::without_span("bar".into()),
                                        Field::Required {
                                            kind: FieldKind::Terminal {
                                                items: [Spanned::without_span("Bar".into())].into()
                                            }
                                        }
                                    )
                                        .into(),
                                    (
                                        Spanned::without_span("baz".into()),
                                        Field::Required {
                                            kind: FieldKind::Terminal {
                                                items: [Spanned::without_span("Baz".into())].into()
                                            }
                                        }
                                    )
                                        .into(),
                                    (
                                        Spanned::without_span("baz_again".into()),
                                        Field::Required {
                                            kind: FieldKind::Terminal {
                                                items: [Spanned::without_span("Baz".into())].into()
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
                                name: Spanned::without_span("Bar".into()),
                                definitions: [TokenDefinition {
                                    enabled: None,
                                    scanner: Scanner::Atom {
                                        atom: Spanned::without_span("bar".into())
                                    }
                                }]
                                .into()
                            }
                        }
                        .into(),
                        Item::Token {
                            item: TokenItem {
                                name: Spanned::without_span("Baz".into()),
                                definitions: [TokenDefinition {
                                    enabled: None,
                                    scanner: Scanner::Atom {
                                        atom: Spanned::without_span("baz".into())
                                    }
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
