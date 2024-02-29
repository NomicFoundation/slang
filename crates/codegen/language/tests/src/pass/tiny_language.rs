use std::path::Path;

use codegen_language_definition::model::{
    Field, Item, Language, Scanner, Section, StructItem, TokenDefinition, TokenItem, Topic,
    TriviaParser,
};
use infra_utils::paths::PathExtensions;
use semver::Version;

codegen_language_macros::compile!(Language(
    name = Tiny,
    documentation_dir = "tiny/docs",
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
    )],
    queries = ()
));

#[test]
fn definition() {
    assert_eq!(
        tiny::TinyDefinition::create(),
        Language {
            name: "Tiny".into(),
            documentation_dir: Path::repo_path("tiny/docs"),
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
                                definitions: [TokenDefinition {
                                    enabled: None,
                                    scanner: Scanner::Atom { atom: "bar".into() }
                                }]
                                .into()
                            }
                            .into()
                        },
                        Item::Token {
                            item: TokenItem {
                                name: "Baz".into(),
                                definitions: [TokenDefinition {
                                    enabled: None,
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
            queries: [].into()
        }
        .into(),
    );
}
