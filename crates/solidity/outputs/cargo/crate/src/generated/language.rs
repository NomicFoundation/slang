// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "slang_napi_interfaces")]
use {napi::bindgen_prelude::*, napi_derive::napi};

use semver::Version;

use super::{
    cst,
    kinds::{ProductionKind, RuleKind, TokenKind},
    parse_output::ParseOutput,
    support::*,
};

#[cfg(feature = "slang_napi_interfaces")]
use super::napi::napi_parse_output::ParseOutput as NAPIParseOutput;

#[derive(Debug)]
#[cfg_attr(feature = "slang_napi_interfaces", napi(namespace = "language"))]
pub struct Language {
    pub(crate) version: Version,
    pub(crate) version_is_at_least_0_4_21: bool,
    pub(crate) version_is_at_least_0_4_22: bool,
    pub(crate) version_is_at_least_0_5_0: bool,
    pub(crate) version_is_at_least_0_5_3: bool,
    pub(crate) version_is_at_least_0_6_0: bool,
    pub(crate) version_is_at_least_0_6_2: bool,
    pub(crate) version_is_at_least_0_6_5: bool,
    pub(crate) version_is_at_least_0_6_11: bool,
    pub(crate) version_is_at_least_0_7_0: bool,
    pub(crate) version_is_at_least_0_7_1: bool,
    pub(crate) version_is_at_least_0_7_4: bool,
    pub(crate) version_is_at_least_0_8_0: bool,
    pub(crate) version_is_at_least_0_8_4: bool,
    pub(crate) version_is_at_least_0_8_8: bool,
    pub(crate) version_is_at_least_0_8_13: bool,
    pub(crate) version_is_at_least_0_8_18: bool,
    pub(crate) version_is_at_least_0_8_19: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported Solidity language version '{0}'.")]
    UnsupportedLanguageVersion(Version),

    #[cfg(feature = "slang_napi_interfaces")]
    #[error("Invalid semantic version '{0}'.")]
    InvalidSemanticVersion(String),
}

#[cfg(feature = "slang_napi_interfaces")]
impl From<Error> for napi::Error {
    fn from(value: Error) -> Self {
        napi::Error::from_reason(value.to_string())
    }
}

impl Language {
    const VERSIONS: &'static [&'static str] = &[
        "0.4.11", "0.4.12", "0.4.13", "0.4.14", "0.4.15", "0.4.16", "0.4.17", "0.4.18", "0.4.19",
        "0.4.20", "0.4.21", "0.4.22", "0.4.23", "0.4.24", "0.4.25", "0.4.26", "0.5.0", "0.5.1",
        "0.5.2", "0.5.3", "0.5.4", "0.5.5", "0.5.6", "0.5.7", "0.5.8", "0.5.9", "0.5.10", "0.5.11",
        "0.5.12", "0.5.13", "0.5.14", "0.5.15", "0.5.16", "0.5.17", "0.6.0", "0.6.1", "0.6.2",
        "0.6.3", "0.6.4", "0.6.5", "0.6.6", "0.6.7", "0.6.8", "0.6.9", "0.6.10", "0.6.11",
        "0.6.12", "0.7.0", "0.7.1", "0.7.2", "0.7.3", "0.7.4", "0.7.5", "0.7.6", "0.8.0", "0.8.1",
        "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8", "0.8.9", "0.8.10", "0.8.11",
        "0.8.12", "0.8.13", "0.8.14", "0.8.15", "0.8.16", "0.8.17", "0.8.18", "0.8.19",
    ];

    pub fn new(version: Version) -> std::result::Result<Self, Error> {
        if Self::VERSIONS.contains(&version.to_string().as_str()) {
            Ok(Self {
                version_is_at_least_0_4_21: Version::parse("0.4.21").unwrap() <= version,
                version_is_at_least_0_4_22: Version::parse("0.4.22").unwrap() <= version,
                version_is_at_least_0_5_0: Version::parse("0.5.0").unwrap() <= version,
                version_is_at_least_0_5_3: Version::parse("0.5.3").unwrap() <= version,
                version_is_at_least_0_6_0: Version::parse("0.6.0").unwrap() <= version,
                version_is_at_least_0_6_2: Version::parse("0.6.2").unwrap() <= version,
                version_is_at_least_0_6_5: Version::parse("0.6.5").unwrap() <= version,
                version_is_at_least_0_6_11: Version::parse("0.6.11").unwrap() <= version,
                version_is_at_least_0_7_0: Version::parse("0.7.0").unwrap() <= version,
                version_is_at_least_0_7_1: Version::parse("0.7.1").unwrap() <= version,
                version_is_at_least_0_7_4: Version::parse("0.7.4").unwrap() <= version,
                version_is_at_least_0_8_0: Version::parse("0.8.0").unwrap() <= version,
                version_is_at_least_0_8_4: Version::parse("0.8.4").unwrap() <= version,
                version_is_at_least_0_8_8: Version::parse("0.8.8").unwrap() <= version,
                version_is_at_least_0_8_13: Version::parse("0.8.13").unwrap() <= version,
                version_is_at_least_0_8_18: Version::parse("0.8.18").unwrap() <= version,
                version_is_at_least_0_8_19: Version::parse("0.8.19").unwrap() <= version,
                version,
            })
        } else {
            Err(Error::UnsupportedLanguageVersion(version))
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn supported_versions() -> Vec<Version> {
        return Self::VERSIONS
            .iter()
            .map(|v| Version::parse(v).unwrap())
            .collect();
    }

    #[allow(dead_code)]
    fn default_parse_token_with_trivia(
        &self,
        stream: &mut Stream,
        kind: TokenKind,
    ) -> ParserResult {
        let mut children = vec![];

        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.leading_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }

        let start = stream.position();
        if self.default_next_token(stream) != Some(kind) {
            stream.set_position(restore);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();
        children.push(cst::Node::token(kind, stream.content(start.utf8..end.utf8)));

        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.trailing_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }

        return ParserResult::r#match(children, vec![]);
    }

    #[allow(dead_code)]
    fn default_parse_token(&self, stream: &mut Stream, kind: TokenKind) -> ParserResult {
        let start = stream.position();
        if self.default_next_token(stream) != Some(kind) {
            stream.set_position(start);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();
        return ParserResult::r#match(
            vec![cst::Node::token(kind, stream.content(start.utf8..end.utf8))],
            vec![],
        );
    }

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn default_next_token(&self, stream: &mut Stream) -> Option<TokenKind> {
        let save = stream.position();
        let mut furthest_position = stream.position();
        let mut longest_token = TokenKind::SKIPPED; // just a marker

        if let Some(kind) = match stream.next() {
            Some('a') => match stream.next() {
                Some('b') => match stream.next() {
                    Some('i') => {
                        if scan_chars!(stream, 'c', 'o', 'd', 'e', 'r') {
                            Some(TokenKind::ABICoderKeyword)
                        } else {
                            None
                        }
                    }
                    Some('s') => {
                        if scan_chars!(stream, 't', 'r', 'a', 'c', 't') {
                            Some(TokenKind::AbstractKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('d') => {
                    if scan_chars!(stream, 'd', 'r', 'e', 's', 's') {
                        Some(TokenKind::AddressKeyword)
                    } else {
                        None
                    }
                }
                Some('f') => {
                    if scan_chars!(stream, 't', 'e', 'r') {
                        Some(TokenKind::AfterKeyword)
                    } else {
                        None
                    }
                }
                Some('l') => {
                    if self.version_is_at_least_0_5_0 {
                        if scan_chars!(stream, 'i', 'a', 's') {
                            Some(TokenKind::AliasKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some('n') => {
                    if scan_chars!(stream, 'o', 'n', 'y', 'm', 'o', 'u', 's') {
                        Some(TokenKind::AnonymousKeyword)
                    } else {
                        None
                    }
                }
                Some('p') => {
                    if self.version_is_at_least_0_5_0 {
                        if scan_chars!(stream, 'p', 'l', 'y') {
                            Some(TokenKind::ApplyKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some('s') => match stream.next() {
                    Some('s') => {
                        if scan_chars!(stream, 'e', 'm', 'b', 'l', 'y') {
                            Some(TokenKind::AssemblyKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        Some(TokenKind::AsKeyword)
                    }
                    None => Some(TokenKind::AsKeyword),
                },
                Some('u') => {
                    if self.version_is_at_least_0_5_0 {
                        if scan_chars!(stream, 't', 'o') {
                            Some(TokenKind::AutoKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('b') => match stream.next() {
                Some('o') => {
                    if scan_chars!(stream, 'o', 'l') {
                        Some(TokenKind::BoolKeyword)
                    } else {
                        None
                    }
                }
                Some('r') => {
                    if scan_chars!(stream, 'e', 'a', 'k') {
                        Some(TokenKind::BreakKeyword)
                    } else {
                        None
                    }
                }
                Some('y') => {
                    if scan_chars!(stream, 't', 'e') {
                        Some(TokenKind::ByteKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('c') => match stream.next() {
                Some('a') => match stream.next() {
                    Some('l') => {
                        if self.version_is_at_least_0_5_0 {
                            if scan_chars!(stream, 'l', 'd', 'a', 't', 'a') {
                                Some(TokenKind::CalldataKeyword)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Some('s') => {
                        if scan_chars!(stream, 'e') {
                            Some(TokenKind::CaseKeyword)
                        } else {
                            None
                        }
                    }
                    Some('t') => {
                        if scan_chars!(stream, 'c', 'h') {
                            Some(TokenKind::CatchKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('o') => match stream.next() {
                    Some('n') => match stream.next() {
                        Some('s') => {
                            if scan_chars!(stream, 't') {
                                match stream.next() {
                                    Some('a') => {
                                        if scan_chars!(stream, 'n', 't') {
                                            Some(TokenKind::ConstantKeyword)
                                        } else {
                                            None
                                        }
                                    }
                                    Some('r') => {
                                        if self.version_is_at_least_0_4_22 {
                                            if scan_chars!(stream, 'u', 'c', 't', 'o', 'r') {
                                                Some(TokenKind::ConstructorKeyword)
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Some(_) => {
                                        stream.undo();
                                        None
                                    }
                                    None => None,
                                }
                            } else {
                                None
                            }
                        }
                        Some('t') => match stream.next() {
                            Some('i') => {
                                if scan_chars!(stream, 'n', 'u', 'e') {
                                    Some(TokenKind::ContinueKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('r') => {
                                if scan_chars!(stream, 'a', 'c', 't') {
                                    Some(TokenKind::ContractKeyword)
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        },
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('p') => {
                        if self.version_is_at_least_0_5_0 {
                            if scan_chars!(stream, 'y', 'o', 'f') {
                                Some(TokenKind::CopyofKeyword)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('d') => match stream.next() {
                Some('a') => {
                    if scan_chars!(stream, 'y', 's') {
                        Some(TokenKind::DaysKeyword)
                    } else {
                        None
                    }
                }
                Some('e') => match stream.next() {
                    Some('f') => match stream.next() {
                        Some('a') => {
                            if scan_chars!(stream, 'u', 'l', 't') {
                                Some(TokenKind::DefaultKeyword)
                            } else {
                                None
                            }
                        }
                        Some('i') => {
                            if self.version_is_at_least_0_5_0 {
                                if scan_chars!(stream, 'n', 'e') {
                                    Some(TokenKind::DefineKeyword)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some('l') => {
                        if scan_chars!(stream, 'e', 't', 'e') {
                            Some(TokenKind::DeleteKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('o') => Some(TokenKind::DoKeyword),
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('e') => match stream.next() {
                Some('l') => {
                    if scan_chars!(stream, 's', 'e') {
                        Some(TokenKind::ElseKeyword)
                    } else {
                        None
                    }
                }
                Some('m') => {
                    if self.version_is_at_least_0_4_21 {
                        if scan_chars!(stream, 'i', 't') {
                            Some(TokenKind::EmitKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some('n') => {
                    if scan_chars!(stream, 'u', 'm') {
                        Some(TokenKind::EnumKeyword)
                    } else {
                        None
                    }
                }
                Some('r') => {
                    if self.version_is_at_least_0_8_4 {
                        if scan_chars!(stream, 'r', 'o', 'r') {
                            Some(TokenKind::ErrorKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some('t') => {
                    if scan_chars!(stream, 'h', 'e', 'r') {
                        Some(TokenKind::EtherKeyword)
                    } else {
                        None
                    }
                }
                Some('v') => {
                    if scan_chars!(stream, 'e', 'n', 't') {
                        Some(TokenKind::EventKeyword)
                    } else {
                        None
                    }
                }
                Some('x') => match stream.next() {
                    Some('p') => {
                        if scan_chars!(stream, 'e', 'r', 'i', 'm', 'e', 'n', 't', 'a', 'l') {
                            Some(TokenKind::ExperimentalKeyword)
                        } else {
                            None
                        }
                    }
                    Some('t') => {
                        if scan_chars!(stream, 'e', 'r', 'n', 'a', 'l') {
                            Some(TokenKind::ExternalKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('f') => match stream.next() {
                Some('a') => {
                    if scan_chars!(stream, 'l') {
                        match stream.next() {
                            Some('l') => {
                                if scan_chars!(stream, 'b', 'a', 'c', 'k') {
                                    Some(TokenKind::FallbackKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('s') => {
                                if scan_chars!(stream, 'e') {
                                    Some(TokenKind::FalseKeyword)
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        }
                    } else {
                        None
                    }
                }
                Some('i') => {
                    if scan_chars!(stream, 'n') {
                        match stream.next() {
                            Some('a') => {
                                if scan_chars!(stream, 'l') {
                                    Some(TokenKind::FinalKeyword)
                                } else {
                                    None
                                }
                            }
                            Some('n') => {
                                if scan_chars!(stream, 'e', 'y') {
                                    Some(TokenKind::FinneyKeyword)
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                stream.undo();
                                None
                            }
                            None => None,
                        }
                    } else {
                        None
                    }
                }
                Some('o') => {
                    if scan_chars!(stream, 'r') {
                        Some(TokenKind::ForKeyword)
                    } else {
                        None
                    }
                }
                Some('r') => {
                    if scan_chars!(stream, 'o', 'm') {
                        Some(TokenKind::FromKeyword)
                    } else {
                        None
                    }
                }
                Some('u') => {
                    if scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n') {
                        Some(TokenKind::FunctionKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('g') => match stream.next() {
                Some('l') => {
                    if scan_chars!(stream, 'o', 'b', 'a', 'l') {
                        Some(TokenKind::GlobalKeyword)
                    } else {
                        None
                    }
                }
                Some('w') => {
                    if self.version_is_at_least_0_6_11 {
                        if scan_chars!(stream, 'e', 'i') {
                            Some(TokenKind::GweiKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('h') => match stream.next() {
                Some('e') => {
                    if scan_chars!(stream, 'x') {
                        Some(TokenKind::HexKeyword)
                    } else {
                        None
                    }
                }
                Some('o') => {
                    if scan_chars!(stream, 'u', 'r', 's') {
                        Some(TokenKind::HoursKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('i') => match stream.next() {
                Some('f') => Some(TokenKind::IfKeyword),
                Some('m') => match stream.next() {
                    Some('m') => {
                        if self.version_is_at_least_0_6_5 {
                            if scan_chars!(stream, 'u', 't', 'a', 'b', 'l', 'e') {
                                Some(TokenKind::ImmutableKeyword)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Some('p') => match stream.next() {
                        Some('l') => {
                            if self.version_is_at_least_0_5_0 {
                                if scan_chars!(stream, 'e', 'm', 'e', 'n', 't', 's') {
                                    Some(TokenKind::ImplementsKeyword)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some('o') => {
                            if scan_chars!(stream, 'r', 't') {
                                Some(TokenKind::ImportKeyword)
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('n') => match stream.next() {
                    Some('d') => {
                        if scan_chars!(stream, 'e', 'x', 'e', 'd') {
                            Some(TokenKind::IndexedKeyword)
                        } else {
                            None
                        }
                    }
                    Some('l') => {
                        if scan_chars!(stream, 'i', 'n', 'e') {
                            Some(TokenKind::InlineKeyword)
                        } else {
                            None
                        }
                    }
                    Some('t') => {
                        if scan_chars!(stream, 'e', 'r') {
                            match stream.next() {
                                Some('f') => {
                                    if scan_chars!(stream, 'a', 'c', 'e') {
                                        Some(TokenKind::InterfaceKeyword)
                                    } else {
                                        None
                                    }
                                }
                                Some('n') => {
                                    if scan_chars!(stream, 'a', 'l') {
                                        Some(TokenKind::InternalKeyword)
                                    } else {
                                        None
                                    }
                                }
                                Some(_) => {
                                    stream.undo();
                                    None
                                }
                                None => None,
                            }
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        Some(TokenKind::InKeyword)
                    }
                    None => Some(TokenKind::InKeyword),
                },
                Some('s') => Some(TokenKind::IsKeyword),
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('l') => match stream.next() {
                Some('e') => match stream.next() {
                    Some('a') => {
                        if self.version_is_at_least_0_6_0 {
                            if scan_chars!(stream, 'v', 'e') {
                                Some(TokenKind::LeaveKeyword)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Some('t') => Some(TokenKind::LetKeyword),
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('i') => {
                    if scan_chars!(stream, 'b', 'r', 'a', 'r', 'y') {
                        Some(TokenKind::LibraryKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('m') => match stream.next() {
                Some('a') => match stream.next() {
                    Some('c') => {
                        if self.version_is_at_least_0_5_0 {
                            if scan_chars!(stream, 'r', 'o') {
                                Some(TokenKind::MacroKeyword)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Some('p') => {
                        if scan_chars!(stream, 'p', 'i', 'n', 'g') {
                            Some(TokenKind::MappingKeyword)
                        } else {
                            None
                        }
                    }
                    Some('t') => {
                        if scan_chars!(stream, 'c', 'h') {
                            Some(TokenKind::MatchKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('e') => {
                    if scan_chars!(stream, 'm', 'o', 'r', 'y') {
                        Some(TokenKind::MemoryKeyword)
                    } else {
                        None
                    }
                }
                Some('i') => {
                    if scan_chars!(stream, 'n', 'u', 't', 'e', 's') {
                        Some(TokenKind::MinutesKeyword)
                    } else {
                        None
                    }
                }
                Some('o') => {
                    if scan_chars!(stream, 'd', 'i', 'f', 'i', 'e', 'r') {
                        Some(TokenKind::ModifierKeyword)
                    } else {
                        None
                    }
                }
                Some('u') => {
                    if self.version_is_at_least_0_5_0 {
                        if scan_chars!(stream, 't', 'a', 'b', 'l', 'e') {
                            Some(TokenKind::MutableKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('n') => match stream.next() {
                Some('e') => {
                    if scan_chars!(stream, 'w') {
                        Some(TokenKind::NewKeyword)
                    } else {
                        None
                    }
                }
                Some('u') => {
                    if scan_chars!(stream, 'l', 'l') {
                        Some(TokenKind::NullKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('o') => match stream.next() {
                Some('f') => Some(TokenKind::OfKeyword),
                Some('v') => {
                    if scan_chars!(stream, 'e', 'r', 'r', 'i', 'd', 'e') {
                        Some(TokenKind::OverrideKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('p') => match stream.next() {
                Some('a') => match stream.next() {
                    Some('r') => {
                        if self.version_is_at_least_0_5_0 {
                            if scan_chars!(stream, 't', 'i', 'a', 'l') {
                                Some(TokenKind::PartialKeyword)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Some('y') => {
                        if scan_chars!(stream, 'a', 'b', 'l', 'e') {
                            Some(TokenKind::PayableKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('r') => match stream.next() {
                    Some('a') => {
                        if scan_chars!(stream, 'g', 'm', 'a') {
                            Some(TokenKind::PragmaKeyword)
                        } else {
                            None
                        }
                    }
                    Some('i') => {
                        if scan_chars!(stream, 'v', 'a', 't', 'e') {
                            Some(TokenKind::PrivateKeyword)
                        } else {
                            None
                        }
                    }
                    Some('o') => {
                        if self.version_is_at_least_0_5_0 {
                            if scan_chars!(stream, 'm', 'i', 's', 'e') {
                                Some(TokenKind::PromiseKeyword)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('u') => match stream.next() {
                    Some('b') => {
                        if scan_chars!(stream, 'l', 'i', 'c') {
                            Some(TokenKind::PublicKeyword)
                        } else {
                            None
                        }
                    }
                    Some('r') => {
                        if scan_chars!(stream, 'e') {
                            Some(TokenKind::PureKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('r') => {
                if scan_chars!(stream, 'e') {
                    match stream.next() {
                        Some('c') => {
                            if scan_chars!(stream, 'e', 'i', 'v', 'e') {
                                Some(TokenKind::ReceiveKeyword)
                            } else {
                                None
                            }
                        }
                        Some('f') => {
                            if self.version_is_at_least_0_5_0 {
                                if scan_chars!(stream, 'e', 'r', 'e', 'n', 'c', 'e') {
                                    Some(TokenKind::ReferenceKeyword)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some('l') => {
                            if scan_chars!(stream, 'o', 'c', 'a', 't', 'a', 'b', 'l', 'e') {
                                Some(TokenKind::RelocatableKeyword)
                            } else {
                                None
                            }
                        }
                        Some('t') => {
                            if scan_chars!(stream, 'u', 'r', 'n') {
                                match stream.next() {
                                    Some('s') => Some(TokenKind::ReturnsKeyword),
                                    Some(_) => {
                                        stream.undo();
                                        Some(TokenKind::ReturnKeyword)
                                    }
                                    None => Some(TokenKind::ReturnKeyword),
                                }
                            } else {
                                None
                            }
                        }
                        Some('v') => {
                            if scan_chars!(stream, 'e', 'r', 't') {
                                Some(TokenKind::RevertKeyword)
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    }
                } else {
                    None
                }
            }
            Some('s') => match stream.next() {
                Some('e') => match stream.next() {
                    Some('a') => {
                        if self.version_is_at_least_0_5_0 {
                            if scan_chars!(stream, 'l', 'e', 'd') {
                                Some(TokenKind::SealedKeyword)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Some('c') => {
                        if scan_chars!(stream, 'o', 'n', 'd', 's') {
                            Some(TokenKind::SecondsKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('i') => {
                    if self.version_is_at_least_0_5_0 {
                        if scan_chars!(stream, 'z', 'e', 'o', 'f') {
                            Some(TokenKind::SizeofKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some('o') => {
                    if scan_chars!(stream, 'l', 'i', 'd', 'i', 't', 'y') {
                        Some(TokenKind::SolidityKeyword)
                    } else {
                        None
                    }
                }
                Some('t') => match stream.next() {
                    Some('a') => {
                        if scan_chars!(stream, 't', 'i', 'c') {
                            Some(TokenKind::StaticKeyword)
                        } else {
                            None
                        }
                    }
                    Some('o') => {
                        if scan_chars!(stream, 'r', 'a', 'g', 'e') {
                            Some(TokenKind::StorageKeyword)
                        } else {
                            None
                        }
                    }
                    Some('r') => match stream.next() {
                        Some('i') => {
                            if scan_chars!(stream, 'n', 'g') {
                                Some(TokenKind::StringKeyword)
                            } else {
                                None
                            }
                        }
                        Some('u') => {
                            if scan_chars!(stream, 'c', 't') {
                                Some(TokenKind::StructKeyword)
                            } else {
                                None
                            }
                        }
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    },
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('u') => {
                    if self.version_is_at_least_0_5_0 {
                        if scan_chars!(stream, 'p', 'p', 'o', 'r', 't', 's') {
                            Some(TokenKind::SupportsKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some('w') => {
                    if scan_chars!(stream, 'i', 't', 'c', 'h') {
                        Some(TokenKind::SwitchKeyword)
                    } else {
                        None
                    }
                }
                Some('z') => {
                    if scan_chars!(stream, 'a', 'b', 'o') {
                        Some(TokenKind::SzaboKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('t') => match stream.next() {
                Some('h') => {
                    if scan_chars!(stream, 'r', 'o', 'w') {
                        Some(TokenKind::ThrowKeyword)
                    } else {
                        None
                    }
                }
                Some('r') => match stream.next() {
                    Some('u') => {
                        if scan_chars!(stream, 'e') {
                            Some(TokenKind::TrueKeyword)
                        } else {
                            None
                        }
                    }
                    Some('y') => {
                        if self.version_is_at_least_0_6_0 {
                            Some(TokenKind::TryKeyword)
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('y') => {
                    if scan_chars!(stream, 'p', 'e') {
                        match stream.next() {
                            Some('d') => {
                                if self.version_is_at_least_0_5_0 {
                                    if scan_chars!(stream, 'e', 'f') {
                                        Some(TokenKind::TypedefKeyword)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some('o') => {
                                if scan_chars!(stream, 'f') {
                                    Some(TokenKind::TypeofKeyword)
                                } else {
                                    None
                                }
                            }
                            Some(_) => {
                                stream.undo();
                                Some(TokenKind::TypeKeyword)
                            }
                            None => Some(TokenKind::TypeKeyword),
                        }
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('u') => match stream.next() {
                Some('n') => {
                    if self.version_is_at_least_0_8_0 {
                        if scan_chars!(stream, 'c', 'h', 'e', 'c', 'k', 'e', 'd') {
                            Some(TokenKind::UncheckedKeyword)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Some('s') => {
                    if scan_chars!(stream, 'i', 'n', 'g') {
                        Some(TokenKind::UsingKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('v') => match stream.next() {
                Some('a') => {
                    if scan_chars!(stream, 'r') {
                        Some(TokenKind::VarKeyword)
                    } else {
                        None
                    }
                }
                Some('i') => match stream.next() {
                    Some('e') => {
                        if scan_chars!(stream, 'w') {
                            Some(TokenKind::ViewKeyword)
                        } else {
                            None
                        }
                    }
                    Some('r') => {
                        if self.version_is_at_least_0_6_0 {
                            if scan_chars!(stream, 't', 'u', 'a', 'l') {
                                Some(TokenKind::VirtualKeyword)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('w') => match stream.next() {
                Some('e') => match stream.next() {
                    Some('e') => {
                        if scan_chars!(stream, 'k', 's') {
                            Some(TokenKind::WeeksKeyword)
                        } else {
                            None
                        }
                    }
                    Some('i') => Some(TokenKind::WeiKeyword),
                    Some(_) => {
                        stream.undo();
                        None
                    }
                    None => None,
                },
                Some('h') => {
                    if scan_chars!(stream, 'i', 'l', 'e') {
                        Some(TokenKind::WhileKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('y') => {
                if scan_chars!(stream, 'e', 'a', 'r', 's') {
                    Some(TokenKind::YearsKeyword)
                } else {
                    None
                }
            }
            Some(_) => {
                stream.undo();
                None
            }
            None => None,
        } {
            // Make sure that this is not the start of an identifier
            if !self.identifier_part(stream) {
                furthest_position = stream.position();
                longest_token = kind;
            }
        }
        stream.set_position(save);

        if let Some(kind) = match stream.next() {
            Some('!') => match stream.next() {
                Some('=') => Some(TokenKind::BangEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Bang)
                }
                None => Some(TokenKind::Bang),
            },
            Some('%') => match stream.next() {
                Some('=') => Some(TokenKind::PercentEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Percent)
                }
                None => Some(TokenKind::Percent),
            },
            Some('&') => match stream.next() {
                Some('&') => Some(TokenKind::AmpersandAmpersand),
                Some('=') => Some(TokenKind::AmpersandEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Ampersand)
                }
                None => Some(TokenKind::Ampersand),
            },
            Some('(') => Some(TokenKind::OpenParen),
            Some(')') => Some(TokenKind::CloseParen),
            Some('*') => match stream.next() {
                Some('*') => Some(TokenKind::AsteriskAsterisk),
                Some('=') => Some(TokenKind::AsteriskEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Asterisk)
                }
                None => Some(TokenKind::Asterisk),
            },
            Some('+') => match stream.next() {
                Some('+') => Some(TokenKind::PlusPlus),
                Some('=') => Some(TokenKind::PlusEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Plus)
                }
                None => Some(TokenKind::Plus),
            },
            Some(',') => Some(TokenKind::Comma),
            Some('-') => match stream.next() {
                Some('-') => Some(TokenKind::MinusMinus),
                Some('=') => Some(TokenKind::MinusEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Minus)
                }
                None => Some(TokenKind::Minus),
            },
            Some('.') => Some(TokenKind::Period),
            Some('/') => match stream.next() {
                Some('=') => Some(TokenKind::SlashEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Slash)
                }
                None => Some(TokenKind::Slash),
            },
            Some(':') => Some(TokenKind::Colon),
            Some(';') => Some(TokenKind::Semicolon),
            Some('<') => match stream.next() {
                Some('<') => match stream.next() {
                    Some('=') => Some(TokenKind::LessThanLessThanEqual),
                    Some(_) => {
                        stream.undo();
                        Some(TokenKind::LessThanLessThan)
                    }
                    None => Some(TokenKind::LessThanLessThan),
                },
                Some('=') => Some(TokenKind::LessThanEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::LessThan)
                }
                None => Some(TokenKind::LessThan),
            },
            Some('=') => match stream.next() {
                Some('=') => Some(TokenKind::EqualEqual),
                Some('>') => Some(TokenKind::EqualGreaterThan),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Equal)
                }
                None => Some(TokenKind::Equal),
            },
            Some('>') => match stream.next() {
                Some('=') => Some(TokenKind::GreaterThanEqual),
                Some('>') => match stream.next() {
                    Some('=') => Some(TokenKind::GreaterThanGreaterThanEqual),
                    Some('>') => match stream.next() {
                        Some('=') => Some(TokenKind::GreaterThanGreaterThanGreaterThanEqual),
                        Some(_) => {
                            stream.undo();
                            Some(TokenKind::GreaterThanGreaterThanGreaterThan)
                        }
                        None => Some(TokenKind::GreaterThanGreaterThanGreaterThan),
                    },
                    Some(_) => {
                        stream.undo();
                        Some(TokenKind::GreaterThanGreaterThan)
                    }
                    None => Some(TokenKind::GreaterThanGreaterThan),
                },
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::GreaterThan)
                }
                None => Some(TokenKind::GreaterThan),
            },
            Some('?') => Some(TokenKind::QuestionMark),
            Some('[') => Some(TokenKind::OpenBracket),
            Some(']') => Some(TokenKind::CloseBracket),
            Some('^') => match stream.next() {
                Some('=') => Some(TokenKind::CaretEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Caret)
                }
                None => Some(TokenKind::Caret),
            },
            Some('{') => Some(TokenKind::OpenBrace),
            Some('|') => match stream.next() {
                Some('=') => Some(TokenKind::BarEqual),
                Some('|') => Some(TokenKind::BarBar),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::Bar)
                }
                None => Some(TokenKind::Bar),
            },
            Some('}') => Some(TokenKind::CloseBrace),
            Some('~') => Some(TokenKind::Tilde),
            Some(_) => {
                stream.undo();
                None
            }
            None => None,
        } {
            furthest_position = stream.position();
            longest_token = kind;
        }
        stream.set_position(save);

        macro_rules! longest_match {
                ($( { $kind:ident = $function:ident } )*) => {
                    $(
                        if self.$function(stream) && stream.position() > furthest_position {
                            furthest_position = stream.position();
                            longest_token = TokenKind::$kind;
                        }
                        stream.set_position(save);
                    )*
                };
            }

        longest_match! {
            { AsciiStringLiteral = ascii_string_literal }
            { DecimalLiteral = decimal_literal }
            { EndOfLine = end_of_line }
            { FixedBytesType = fixed_bytes_type }
            { HexLiteral = hex_literal }
            { HexStringLiteral = hex_string_literal }
            { MultilineComment = multiline_comment }
            { SignedFixedType = signed_fixed_type }
            { SignedIntegerType = signed_integer_type }
            { SingleLineComment = single_line_comment }
            { UnicodeStringLiteral = unicode_string_literal }
            { UnsignedFixedType = unsigned_fixed_type }
            { UnsignedIntegerType = unsigned_integer_type }
            { Whitespace = whitespace }
            { Identifier = identifier }
        }

        if longest_token != TokenKind::SKIPPED {
            stream.set_position(furthest_position);
            Some(longest_token)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn version_pragma_parse_token_with_trivia(
        &self,
        stream: &mut Stream,
        kind: TokenKind,
    ) -> ParserResult {
        let mut children = vec![];

        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.leading_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }

        let start = stream.position();
        if self.version_pragma_next_token(stream) != Some(kind) {
            stream.set_position(restore);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();
        children.push(cst::Node::token(kind, stream.content(start.utf8..end.utf8)));

        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.trailing_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }

        return ParserResult::r#match(children, vec![]);
    }

    #[allow(dead_code)]
    fn version_pragma_parse_token(&self, stream: &mut Stream, kind: TokenKind) -> ParserResult {
        let start = stream.position();
        if self.version_pragma_next_token(stream) != Some(kind) {
            stream.set_position(start);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();
        return ParserResult::r#match(
            vec![cst::Node::token(kind, stream.content(start.utf8..end.utf8))],
            vec![],
        );
    }

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma_next_token(&self, stream: &mut Stream) -> Option<TokenKind> {
        let save = stream.position();
        let mut furthest_position = stream.position();
        let mut longest_token = TokenKind::SKIPPED; // just a marker

        if let Some(kind) = if scan_chars!(stream, 's', 'o', 'l', 'i', 'd', 'i', 't', 'y') {
            Some(TokenKind::SolidityKeyword)
        } else {
            None
        } {
            // Make sure that this is not the start of an identifier
            if !self.identifier_part(stream) {
                furthest_position = stream.position();
                longest_token = kind;
            }
        }
        stream.set_position(save);

        if let Some(kind) = match stream.next() {
            Some('-') => Some(TokenKind::Minus),
            Some('.') => Some(TokenKind::Period),
            Some('<') => match stream.next() {
                Some('=') => Some(TokenKind::LessThanEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::LessThan)
                }
                None => Some(TokenKind::LessThan),
            },
            Some('=') => Some(TokenKind::Equal),
            Some('>') => match stream.next() {
                Some('=') => Some(TokenKind::GreaterThanEqual),
                Some(_) => {
                    stream.undo();
                    Some(TokenKind::GreaterThan)
                }
                None => Some(TokenKind::GreaterThan),
            },
            Some('^') => Some(TokenKind::Caret),
            Some('|') => {
                if scan_chars!(stream, '|') {
                    Some(TokenKind::BarBar)
                } else {
                    None
                }
            }
            Some('~') => Some(TokenKind::Tilde),
            Some(_) => {
                stream.undo();
                None
            }
            None => None,
        } {
            furthest_position = stream.position();
            longest_token = kind;
        }
        stream.set_position(save);

        macro_rules! longest_match {
                ($( { $kind:ident = $function:ident } )*) => {
                    $(
                        if self.$function(stream) && stream.position() > furthest_position {
                            furthest_position = stream.position();
                            longest_token = TokenKind::$kind;
                        }
                        stream.set_position(save);
                    )*
                };
            }

        longest_match! {
            { VersionPragmaValue = version_pragma_value }
        }

        if longest_token != TokenKind::SKIPPED {
            stream.set_position(furthest_position);
            Some(longest_token)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn yul_block_parse_token_with_trivia(
        &self,
        stream: &mut Stream,
        kind: TokenKind,
    ) -> ParserResult {
        let mut children = vec![];

        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.leading_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }

        let start = stream.position();
        if self.yul_block_next_token(stream) != Some(kind) {
            stream.set_position(restore);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();
        children.push(cst::Node::token(kind, stream.content(start.utf8..end.utf8)));

        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.trailing_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }

        return ParserResult::r#match(children, vec![]);
    }

    #[allow(dead_code)]
    fn yul_block_parse_token(&self, stream: &mut Stream, kind: TokenKind) -> ParserResult {
        let start = stream.position();
        if self.yul_block_next_token(stream) != Some(kind) {
            stream.set_position(start);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();
        return ParserResult::r#match(
            vec![cst::Node::token(kind, stream.content(start.utf8..end.utf8))],
            vec![],
        );
    }

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_block_next_token(&self, stream: &mut Stream) -> Option<TokenKind> {
        let save = stream.position();
        let mut furthest_position = stream.position();
        let mut longest_token = TokenKind::SKIPPED; // just a marker

        if let Some(kind) = match stream.next() {
            Some('b') => {
                if scan_chars!(stream, 'r', 'e', 'a', 'k') {
                    Some(TokenKind::BreakKeyword)
                } else {
                    None
                }
            }
            Some('c') => match stream.next() {
                Some('a') => {
                    if scan_chars!(stream, 's', 'e') {
                        Some(TokenKind::CaseKeyword)
                    } else {
                        None
                    }
                }
                Some('o') => {
                    if scan_chars!(stream, 'n', 't', 'i', 'n', 'u', 'e') {
                        Some(TokenKind::ContinueKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('d') => {
                if scan_chars!(stream, 'e', 'f', 'a', 'u', 'l', 't') {
                    Some(TokenKind::DefaultKeyword)
                } else {
                    None
                }
            }
            Some('f') => match stream.next() {
                Some('a') => {
                    if scan_chars!(stream, 'l', 's', 'e') {
                        Some(TokenKind::FalseKeyword)
                    } else {
                        None
                    }
                }
                Some('o') => {
                    if scan_chars!(stream, 'r') {
                        Some(TokenKind::ForKeyword)
                    } else {
                        None
                    }
                }
                Some('u') => {
                    if scan_chars!(stream, 'n', 'c', 't', 'i', 'o', 'n') {
                        Some(TokenKind::FunctionKeyword)
                    } else {
                        None
                    }
                }
                Some(_) => {
                    stream.undo();
                    None
                }
                None => None,
            },
            Some('h') => {
                if scan_chars!(stream, 'e', 'x') {
                    Some(TokenKind::HexKeyword)
                } else {
                    None
                }
            }
            Some('i') => {
                if scan_chars!(stream, 'f') {
                    Some(TokenKind::IfKeyword)
                } else {
                    None
                }
            }
            Some('l') => {
                if scan_chars!(stream, 'e') {
                    match stream.next() {
                        Some('a') => {
                            if self.version_is_at_least_0_6_0 {
                                if scan_chars!(stream, 'v', 'e') {
                                    Some(TokenKind::LeaveKeyword)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Some('t') => Some(TokenKind::LetKeyword),
                        Some(_) => {
                            stream.undo();
                            None
                        }
                        None => None,
                    }
                } else {
                    None
                }
            }
            Some('s') => {
                if scan_chars!(stream, 'w', 'i', 't', 'c', 'h') {
                    Some(TokenKind::SwitchKeyword)
                } else {
                    None
                }
            }
            Some('t') => {
                if scan_chars!(stream, 'r', 'u', 'e') {
                    Some(TokenKind::TrueKeyword)
                } else {
                    None
                }
            }
            Some(_) => {
                stream.undo();
                None
            }
            None => None,
        } {
            // Make sure that this is not the start of an identifier
            if !self.identifier_part(stream) {
                furthest_position = stream.position();
                longest_token = kind;
            }
        }
        stream.set_position(save);

        if let Some(kind) = match stream.next() {
            Some('(') => Some(TokenKind::OpenParen),
            Some(')') => Some(TokenKind::CloseParen),
            Some(',') => Some(TokenKind::Comma),
            Some('-') => {
                if scan_chars!(stream, '>') {
                    Some(TokenKind::MinusGreaterThan)
                } else {
                    None
                }
            }
            Some('.') => Some(TokenKind::Period),
            Some(':') => {
                if scan_chars!(stream, '=') {
                    Some(TokenKind::ColonEqual)
                } else {
                    None
                }
            }
            Some('{') => Some(TokenKind::OpenBrace),
            Some('}') => Some(TokenKind::CloseBrace),
            Some(_) => {
                stream.undo();
                None
            }
            None => None,
        } {
            furthest_position = stream.position();
            longest_token = kind;
        }
        stream.set_position(save);

        macro_rules! longest_match {
                ($( { $kind:ident = $function:ident } )*) => {
                    $(
                        if self.$function(stream) && stream.position() > furthest_position {
                            furthest_position = stream.position();
                            longest_token = TokenKind::$kind;
                        }
                        stream.set_position(save);
                    )*
                };
            }

        longest_match! {
            { AsciiStringLiteral = ascii_string_literal }
            { HexStringLiteral = hex_string_literal }
            { YulDecimalLiteral = yul_decimal_literal }
            { YulHexLiteral = yul_hex_literal }
            { YulIdentifier = yul_identifier }
        }

        if longest_token != TokenKind::SKIPPED {
            stream.set_position(furthest_position);
            Some(longest_token)
        } else {
            None
        }
    }

    /********************************************
     *         Parser Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn abi_coder_pragma(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::ABICoderKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ABICoderPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn address_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::AddressKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.default_parse_token_with_trivia(stream, TokenKind::PayableKeyword),
                        )) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        }
        .with_kind(RuleKind::AddressType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn arguments_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result = self.positional_arguments_list(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.named_arguments_declaration(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                })) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::OpenBracket),
                ) {
                    break;
                }
                if helper.handle_next_result(self.array_values_list(stream)) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::CloseBracket),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ArrayExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn array_values_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.expression(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.expression(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ArrayValuesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_string_literals_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral)
        })
        .with_kind(RuleKind::AsciiStringLiteralsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_flags_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                ) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::AsciiStringLiteral,
                            ),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::AssemblyFlagsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn assembly_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::AssemblyKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                )) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.assembly_flags_list(stream)) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                if helper.handle_next_result(self.yul_block(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::AssemblyStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn block(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                ) {
                    break;
                }
                if helper
                    .handle_next_result(OptionalHelper::transform(self.statements_list(stream)))
                {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::Block)
    }

    #[allow(unused_assignments, unused_parens)]
    fn break_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::BreakKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::BreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::CatchKeyword),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.catch_clause_error(stream),
                    )) {
                        break;
                    }
                    if helper.handle_next_result(self.block(stream)) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::CatchClause)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clause_error(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                    )) {
                        break;
                    }
                    if helper.handle_next_result(self.parameters_declaration(stream)) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::CatchClauseError)
    }

    #[allow(unused_assignments, unused_parens)]
    fn catch_clauses_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(stream, |stream| self.catch_clause(stream))
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::CatchClausesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constant_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_7_4 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result({
                        let mut helper = SequenceHelper::new();
                        loop {
                            if helper.handle_next_result(self.type_name(stream)) {
                                break;
                            }
                            if helper.handle_next_result(self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::ConstantKeyword,
                            )) {
                                break;
                            }
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                            ) {
                                break;
                            }
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(stream, TokenKind::Equal),
                            ) {
                                break;
                            }
                            if helper.handle_next_result(self.expression(stream)) {
                                break;
                            }
                            break;
                        }
                        helper.result()
                    }) {
                        break;
                    }
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                    ) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::ConstantDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            OneOrMoreHelper::run(stream, |stream| {
                if self.version_is_at_least_0_4_22 {
                    {
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result = self.modifier_invocation(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::InternalKeyword,
                            );
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PublicKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    }
                } else {
                    ParserResult::no_match(vec![])
                }
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::ConstructorAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn constructor_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_4_22 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::ConstructorKeyword),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(self.parameters_declaration(stream)) {
                        break;
                    }
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.constructor_attributes_list(stream),
                    )) {
                        break;
                    }
                    if helper.handle_next_result(self.block(stream)) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::ConstructorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn continue_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::ContinueKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if self.version_is_at_least_0_6_0 {
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::AbstractKeyword),
                    )) {
                        break;
                    }
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::ContractKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.inheritance_specifier(stream),
                )) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.contract_members_list(stream),
                        )) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ContractDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn contract_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.using_directive(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.function_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.modifier_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.struct_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.enum_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.event_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.state_variable_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if self.version_is_at_least_0_4_22 {
                    let result = self.constructor_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_6_0 {
                    let result = {
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result = self.fallback_function_definition(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.receive_function_definition(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    };
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = self.unnamed_function_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_4 {
                    let result = self.error_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_8 {
                    let result = self.user_defined_value_type_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::ContractMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn deconstruction_import(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        ) {
                            break;
                        }
                        if helper
                            .handle_next_result(self.deconstruction_import_symbols_list(stream))
                        {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::FromKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::DeconstructionImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn deconstruction_import_symbol(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::AsKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::DeconstructionImportSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn deconstruction_import_symbols_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.deconstruction_import_symbol(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.deconstruction_import_symbol(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::DeconstructionImportSymbolsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn delete_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::DeleteKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.expression(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::DeleteStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn do_while_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::DoKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.statement(stream)) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::WhileKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result({
                            let mut helper = SequenceHelper::new();
                            loop {
                                if helper.handle_next_result(
                                    self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::OpenParen,
                                    ),
                                ) {
                                    break;
                                }
                                if helper.handle_next_result(self.expression(stream)) {
                                    break;
                                }
                                if helper.handle_next_result(
                                    self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::CloseParen,
                                    ),
                                ) {
                                    break;
                                }
                                break;
                            }
                            helper.result()
                        }) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::DoWhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn emit_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_4_21 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result({
                        let mut helper = SequenceHelper::new();
                        loop {
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::EmitKeyword,
                                ),
                            ) {
                                break;
                            }
                            if helper.handle_next_result(self.identifier_path(stream)) {
                                break;
                            }
                            if helper.handle_next_result(self.arguments_declaration(stream)) {
                                break;
                            }
                            break;
                        }
                        helper.result()
                    }) {
                        break;
                    }
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                    ) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::EmitStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn end_of_file_trivia(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.default_parse_token(stream, TokenKind::Whitespace);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token(stream, TokenKind::EndOfLine);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token(stream, TokenKind::MultilineComment);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token(stream, TokenKind::SingleLineComment);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::EndOfFileTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn enum_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::EnumKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.identifiers_list(stream),
                        )) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::EnumDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result({
                        let mut helper = SequenceHelper::new();
                        loop {
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::ErrorKeyword,
                                ),
                            ) {
                                break;
                            }
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                            ) {
                                break;
                            }
                            if helper.handle_next_result({
                                let mut helper = SequenceHelper::new();
                                loop {
                                    if helper.handle_next_result(
                                        self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::OpenParen,
                                        ),
                                    ) {
                                        break;
                                    }
                                    if helper.handle_next_result(OptionalHelper::transform(
                                        self.error_parameters_list(stream),
                                    )) {
                                        break;
                                    }
                                    if helper.handle_next_result(
                                        self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::CloseParen,
                                        ),
                                    ) {
                                        break;
                                    }
                                    break;
                                }
                                helper.result()
                            }) {
                                break;
                            }
                            break;
                        }
                        helper.result()
                    }) {
                        break;
                    }
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                    ) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::ErrorDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameter(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(self.type_name(stream)) {
                        break;
                    }
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                    )) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::ErrorParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn error_parameters_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_4 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(self.error_parameter(stream)) {
                        break;
                    }
                    if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                        let mut helper = SequenceHelper::new();
                        loop {
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                            ) {
                                break;
                            }
                            if helper.handle_next_result(self.error_parameter(stream)) {
                                break;
                            }
                            break;
                        }
                        helper.result()
                    })) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::ErrorParametersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::EventKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        ) {
                            break;
                        }
                        if helper.handle_next_result({
                            let mut helper = SequenceHelper::new();
                            loop {
                                if helper.handle_next_result(
                                    self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::OpenParen,
                                    ),
                                ) {
                                    break;
                                }
                                if helper.handle_next_result(OptionalHelper::transform(
                                    self.event_parameters_list(stream),
                                )) {
                                    break;
                                }
                                if helper.handle_next_result(
                                    self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::CloseParen,
                                    ),
                                ) {
                                    break;
                                }
                                break;
                            }
                            helper.result()
                        }) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::AnonymousKeyword,
                            ),
                        )) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::EventDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameter(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.type_name(stream)) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(stream, TokenKind::IndexedKeyword),
                )) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                )) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::EventParameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn event_parameters_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.event_parameter(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.event_parameter(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::EventParametersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn experimental_pragma(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::ExperimentalKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                }) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ExperimentalPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression(&self, stream: &mut Stream) -> ParserResult {
        let parse_assignment_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::BinaryExpression, 1u8, 2u8, {
                let mut helper = ChoiceHelper::new(stream);
                loop {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Equal);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::BarEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::PlusEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::MinusEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::CaretEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::SlashEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::PercentEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::AsteriskEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::AmpersandEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self
                        .default_parse_token_with_trivia(stream, TokenKind::LessThanLessThanEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(
                        stream,
                        TokenKind::GreaterThanGreaterThanEqual,
                    );
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(
                        stream,
                        TokenKind::GreaterThanGreaterThanGreaterThanEqual,
                    );
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    break;
                }
                helper.result(stream)
            })
        };
        let parse_conditional_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::ConditionalExpression, 3u8, 255u8, {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::QuestionMark),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(self.expression(stream)) {
                        break;
                    }
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::Colon),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(self.expression(stream)) {
                        break;
                    }
                    break;
                }
                helper.result()
            })
        };
        let parse_or_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::BinaryExpression,
                5u8,
                6u8,
                self.default_parse_token_with_trivia(stream, TokenKind::BarBar),
            )
        };
        let parse_and_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::BinaryExpression,
                7u8,
                8u8,
                self.default_parse_token_with_trivia(stream, TokenKind::AmpersandAmpersand),
            )
        };
        let parse_equality_comparison_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::BinaryExpression, 9u8, 10u8, {
                let mut helper = ChoiceHelper::new(stream);
                loop {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::EqualEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::BangEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    break;
                }
                helper.result(stream)
            })
        };
        let parse_order_comparison_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::BinaryExpression, 11u8, 12u8, {
                let mut helper = ChoiceHelper::new(stream);
                loop {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::LessThan);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::GreaterThan);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::LessThanEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::GreaterThanEqual);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    break;
                }
                helper.result(stream)
            })
        };
        let parse_bitwise_or_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::BinaryExpression,
                13u8,
                14u8,
                self.default_parse_token_with_trivia(stream, TokenKind::Bar),
            )
        };
        let parse_bitwise_x_or_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::BinaryExpression,
                15u8,
                16u8,
                self.default_parse_token_with_trivia(stream, TokenKind::Caret),
            )
        };
        let parse_bitwise_and_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::BinaryExpression,
                17u8,
                18u8,
                self.default_parse_token_with_trivia(stream, TokenKind::Ampersand),
            )
        };
        let parse_shift_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::BinaryExpression, 19u8, 20u8, {
                let mut helper = ChoiceHelper::new(stream);
                loop {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::LessThanLessThan);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self
                        .default_parse_token_with_trivia(stream, TokenKind::GreaterThanGreaterThan);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(
                        stream,
                        TokenKind::GreaterThanGreaterThanGreaterThan,
                    );
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    break;
                }
                helper.result(stream)
            })
        };
        let parse_add_sub_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::BinaryExpression, 21u8, 22u8, {
                let mut helper = ChoiceHelper::new(stream);
                loop {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Plus);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Minus);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    break;
                }
                helper.result(stream)
            })
        };
        let parse_mul_div_mod_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::BinaryExpression, 23u8, 24u8, {
                let mut helper = ChoiceHelper::new(stream);
                loop {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Asterisk);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Slash);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Percent);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    break;
                }
                helper.result(stream)
            })
        };
        let parse_exponentiation_operator_removed_from_0_6_0 = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::BinaryExpression,
                25u8,
                26u8,
                self.default_parse_token_with_trivia(stream, TokenKind::AsteriskAsterisk),
            )
        };
        let parse_exponentiation_operator_introduced_from_0_6_0 = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::BinaryExpression,
                28u8,
                27u8,
                self.default_parse_token_with_trivia(stream, TokenKind::AsteriskAsterisk),
            )
        };
        let parse_unary_postfix_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::UnaryPostfixExpression, 29u8, 255u8, {
                let mut helper = ChoiceHelper::new(stream);
                loop {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::PlusPlus);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::MinusMinus);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    break;
                }
                helper.result(stream)
            })
        };
        let parse_unary_prefix_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::UnaryPrefixExpression, 255u8, 31u8, {
                let mut helper = ChoiceHelper::new(stream);
                loop {
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::PlusPlus);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::MinusMinus);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Tilde);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Bang);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    let result = self.default_parse_token_with_trivia(stream, TokenKind::Minus);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                    if !self.version_is_at_least_0_5_0 {
                        let result = self.default_parse_token_with_trivia(stream, TokenKind::Plus);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                    }
                    break;
                }
                helper.result(stream)
            })
        };
        let parse_function_call_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::FunctionCallExpression, 33u8, 255u8, {
                let mut helper = SequenceHelper::new();
                loop {
                    if self.version_is_at_least_0_6_2 {
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.function_call_options(stream),
                        )) {
                            break;
                        }
                    }
                    if helper.handle_next_result(self.arguments_declaration(stream)) {
                        break;
                    }
                    break;
                }
                helper.result()
            })
        };
        let parse_member_access_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::MemberAccessExpression, 35u8, 255u8, {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::Period),
                    ) {
                        break;
                    }
                    if helper.handle_next_result({
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::Identifier);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::AddressKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    }) {
                        break;
                    }
                    break;
                }
                helper.result()
            })
        };
        let parse_index_access_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::IndexAccessExpression, 37u8, 255u8, {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::OpenBracket),
                    ) {
                        break;
                    }
                    if helper.handle_next_result({
                        let mut helper = SequenceHelper::new();
                        loop {
                            if helper.handle_next_result(OptionalHelper::transform(
                                self.expression(stream),
                            )) {
                                break;
                            }
                            if helper.handle_next_result(OptionalHelper::transform({
                                let mut helper = SequenceHelper::new();
                                loop {
                                    if helper.handle_next_result(
                                        self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Colon,
                                        ),
                                    ) {
                                        break;
                                    }
                                    if helper.handle_next_result(OptionalHelper::transform(
                                        self.expression(stream),
                                    )) {
                                        break;
                                    }
                                    break;
                                }
                                helper.result()
                            })) {
                                break;
                            }
                            break;
                        }
                        helper.result()
                    }) {
                        break;
                    }
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::CloseBracket),
                    ) {
                        break;
                    }
                    break;
                }
                helper.result()
            })
        };
        let prefix_operator_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = parse_unary_prefix_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        };
        let primary_expression_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.new_expression(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.tuple_expression(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.array_expression(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = {
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::TrueKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::FalseKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.numeric_expression(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = {
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result = self.hex_string_literals_list(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.ascii_string_literals_list(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        if self.version_is_at_least_0_7_0 {
                            let result = self.unicode_string_literals_list(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        }
                        break;
                    }
                    helper.result(stream)
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = {
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::BoolKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::StringKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.address_type(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::FixedBytesType);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::SignedIntegerType);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.default_parse_token_with_trivia(
                            stream,
                            TokenKind::UnsignedIntegerType,
                        );
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::SignedFixedType);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::UnsignedFixedType);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        if !self.version_is_at_least_0_8_0 {
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::ByteKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        }
                        break;
                    }
                    helper.result(stream)
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token_with_trivia(stream, TokenKind::Identifier);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if self.version_is_at_least_0_5_3 {
                    let result = self.type_expression(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        };
        let postfix_operator_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = parse_conditional_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_unary_postfix_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_function_call_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_member_access_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_index_access_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        };
        let binary_operand_parser = |stream: &mut Stream| {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    prefix_operator_parser(stream)
                })) {
                    break;
                }
                if helper.handle_next_result(primary_expression_parser(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    postfix_operator_parser(stream)
                })) {
                    break;
                }
                break;
            }
            helper.result()
        };
        let binary_operator_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = parse_assignment_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_or_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_and_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_equality_comparison_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_order_comparison_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_bitwise_or_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_bitwise_x_or_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_bitwise_and_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_shift_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_add_sub_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_mul_div_mod_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = parse_exponentiation_operator_removed_from_0_6_0(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_6_0 {
                    let result = parse_exponentiation_operator_introduced_from_0_6_0(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        };
        let linear_expression_parser = |stream: &mut Stream| {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(binary_operand_parser(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(binary_operator_parser(stream)) {
                            break;
                        }
                        if helper.handle_next_result(binary_operand_parser(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::Expression),
            linear_expression_parser(stream),
        )
        .with_kind(RuleKind::Expression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn expression_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.expression(stream)) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ExpressionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(stream, |stream| {
                if self.version_is_at_least_0_6_0 {
                    {
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result = self.modifier_invocation(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.override_specifier(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::ExternalKeyword,
                            );
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PureKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::ViewKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::VirtualKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    }
                } else {
                    ParserResult::no_match(vec![])
                }
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::FallbackFunctionAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn fallback_function_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::FallbackKeyword),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(self.parameters_declaration(stream)) {
                        break;
                    }
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.fallback_function_attributes_list(stream),
                    )) {
                        break;
                    }
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.returns_declaration(stream),
                    )) {
                        break;
                    }
                    if helper.handle_next_result({
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.block(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    }) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::FallbackFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn for_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::ForKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        ) {
                            break;
                        }
                        if helper.handle_next_result({
                            let mut helper = SequenceHelper::new();
                            loop {
                                if helper.handle_next_result({
                                    let mut helper = ChoiceHelper::new(stream);
                                    loop {
                                        let result = {
                                            let mut helper = ChoiceHelper::new(stream);
                                            loop {
                                                let result = self.expression_statement(stream);
                                                if helper.handle_next_result(stream, result) {
                                                    break;
                                                }
                                                let result =
                                                    self.variable_declaration_statement(stream);
                                                if helper.handle_next_result(stream, result) {
                                                    break;
                                                }
                                                let result =
                                                    self.tuple_deconstruction_statement(stream);
                                                if helper.handle_next_result(stream, result) {
                                                    break;
                                                }
                                                break;
                                            }
                                            helper.result(stream)
                                        };
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Semicolon,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        break;
                                    }
                                    helper.result(stream)
                                }) {
                                    break;
                                }
                                if helper.handle_next_result({
                                    let mut helper = ChoiceHelper::new(stream);
                                    loop {
                                        let result = self.expression_statement(stream);
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Semicolon,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        break;
                                    }
                                    helper.result(stream)
                                }) {
                                    break;
                                }
                                if helper.handle_next_result(OptionalHelper::transform(
                                    self.expression(stream),
                                )) {
                                    break;
                                }
                                break;
                            }
                            helper.result()
                        }) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(self.statement(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.modifier_invocation(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.override_specifier(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::ExternalKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::InternalKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PrivateKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PublicKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PureKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token_with_trivia(stream, TokenKind::ViewKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if !self.version_is_at_least_0_5_0 {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::ConstantKeyword);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_6_0 {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::VirtualKeyword);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::FunctionAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_call_options(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                if self.version_is_at_least_0_6_2 && !self.version_is_at_least_0_8_0 {
                    let result = OneOrMoreHelper::run(stream, |stream| {
                        self.named_arguments_declaration(stream)
                    });
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_0 {
                    let result = self.named_arguments_declaration(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        }
        .with_kind(RuleKind::FunctionCallOptions)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::FunctionKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::FallbackKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::ReceiveKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                }) {
                    break;
                }
                if helper.handle_next_result(self.parameters_declaration(stream)) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.function_attributes_list(stream),
                )) {
                    break;
                }
                if helper
                    .handle_next_result(OptionalHelper::transform(self.returns_declaration(stream)))
                {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.block(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                }) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::FunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::FunctionKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(self.parameters_declaration(stream)) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.function_type_attributes_list(stream),
                )) {
                    break;
                }
                if helper
                    .handle_next_result(OptionalHelper::transform(self.returns_declaration(stream)))
                {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::FunctionType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn function_type_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::InternalKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::ExternalKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PrivateKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PublicKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PureKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token_with_trivia(stream, TokenKind::ViewKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::FunctionTypeAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literals_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            self.default_parse_token_with_trivia(stream, TokenKind::HexStringLiteral)
        })
        .with_kind(RuleKind::HexStringLiteralsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_path(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Period),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::IdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_paths_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.identifier_path(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.identifier_path(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::IdentifierPathsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifiers_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::IdentifiersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn if_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::IfKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.expression(stream)) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(self.statement(stream)) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::ElseKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.statement(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::IfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn import_directive(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::ImportKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result({
                            let mut helper = ChoiceHelper::new(stream);
                            loop {
                                let result = self.path_import(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.named_import(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.deconstruction_import(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                break;
                            }
                            helper.result(stream)
                        }) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ImportDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::IsKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(self.inheritance_types_list(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::InheritanceSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.identifier_path(stream)) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.arguments_declaration(stream),
                )) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::InheritanceType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn inheritance_types_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.inheritance_type(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.inheritance_type(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::InheritanceTypesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::InterfaceKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.inheritance_specifier(stream),
                )) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.interface_members_list(stream),
                        )) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::InterfaceDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn interface_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.using_directive(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.function_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.modifier_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.struct_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.enum_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.event_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.state_variable_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if self.version_is_at_least_0_4_22 {
                    let result = self.constructor_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_6_0 {
                    let result = {
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result = self.fallback_function_definition(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.receive_function_definition(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    };
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = self.unnamed_function_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_4 {
                    let result = self.error_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_8 {
                    let result = self.user_defined_value_type_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::InterfaceMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn leading_trivia(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.default_parse_token(stream, TokenKind::Whitespace);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token(stream, TokenKind::EndOfLine);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token(stream, TokenKind::MultilineComment);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token(stream, TokenKind::SingleLineComment);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::LeadingTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::LibraryKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.library_members_list(stream),
                        )) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::LibraryDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn library_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.using_directive(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.function_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.modifier_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.struct_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.enum_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.event_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.state_variable_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if self.version_is_at_least_0_4_22 {
                    let result = self.constructor_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_6_0 {
                    let result = {
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result = self.fallback_function_definition(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.receive_function_definition(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    };
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if !self.version_is_at_least_0_6_0 {
                    let result = self.unnamed_function_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_4 {
                    let result = self.error_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_8 {
                    let result = self.user_defined_value_type_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::LibraryMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_key_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result = {
                            let mut helper = ChoiceHelper::new(stream);
                            loop {
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::BoolKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::StringKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.address_type(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::FixedBytesType,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::SignedIntegerType,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::UnsignedIntegerType,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::SignedFixedType,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::UnsignedFixedType,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                if !self.version_is_at_least_0_8_0 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::ByteKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                }
                                break;
                            }
                            helper.result(stream)
                        };
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.identifier_path(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                }) {
                    break;
                }
                if self.version_is_at_least_0_8_18 {
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                    )) {
                        break;
                    }
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::MappingKeyType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::MappingKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        ) {
                            break;
                        }
                        if helper.handle_next_result({
                            let mut helper = SequenceHelper::new();
                            loop {
                                if helper.handle_next_result(self.mapping_key_type(stream)) {
                                    break;
                                }
                                if helper.handle_next_result(self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::EqualGreaterThan,
                                )) {
                                    break;
                                }
                                if helper.handle_next_result(self.mapping_value_type(stream)) {
                                    break;
                                }
                                break;
                            }
                            helper.result()
                        }) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::MappingType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn mapping_value_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.type_name(stream)) {
                    break;
                }
                if self.version_is_at_least_0_8_18 {
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                    )) {
                        break;
                    }
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::MappingValueType)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.override_specifier(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if self.version_is_at_least_0_6_0 {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::VirtualKeyword);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::ModifierAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::ModifierKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.parameters_declaration(stream),
                )) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.modifier_attributes_list(stream),
                )) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.block(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                }) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ModifierDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn modifier_invocation(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.identifier_path(stream)) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.arguments_declaration(stream),
                )) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ModifierInvocation)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_argument(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Colon),
                ) {
                    break;
                }
                if helper.handle_next_result(self.expression(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::NamedArgument)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.named_arguments_list(stream),
                )) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::NamedArgumentsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_arguments_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.named_argument(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.named_argument(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::NamedArgumentsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn named_import(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Asterisk),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::FromKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::NamedImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn new_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::NewKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(self.type_name(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::NewExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn numeric_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::HexLiteral),
                        ) {
                            break;
                        }
                        if !self.version_is_at_least_0_5_0 {
                            if helper.handle_next_result(OptionalHelper::transform({
                                let mut helper = ChoiceHelper::new(stream);
                                loop {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::DaysKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::EtherKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::HoursKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::MinutesKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::SecondsKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::WeeksKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::WeiKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    if !self.version_is_at_least_0_5_0 {
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::YearsKeyword,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                    }
                                    if self.version_is_at_least_0_6_11 {
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::GweiKeyword,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                    }
                                    if !self.version_is_at_least_0_7_0 {
                                        let result = {
                                            let mut helper = ChoiceHelper::new(stream);
                                            loop {
                                                let result = self.default_parse_token_with_trivia(
                                                    stream,
                                                    TokenKind::FinneyKeyword,
                                                );
                                                if helper.handle_next_result(stream, result) {
                                                    break;
                                                }
                                                let result = self.default_parse_token_with_trivia(
                                                    stream,
                                                    TokenKind::SzaboKeyword,
                                                );
                                                if helper.handle_next_result(stream, result) {
                                                    break;
                                                }
                                                break;
                                            }
                                            helper.result(stream)
                                        };
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                    }
                                    break;
                                }
                                helper.result(stream)
                            })) {
                                break;
                            }
                        }
                        break;
                    }
                    helper.result()
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::DecimalLiteral),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform({
                            let mut helper = ChoiceHelper::new(stream);
                            loop {
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::DaysKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::EtherKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::HoursKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::MinutesKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::SecondsKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::WeeksKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self
                                    .default_parse_token_with_trivia(stream, TokenKind::WeiKeyword);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                if !self.version_is_at_least_0_5_0 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::YearsKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                }
                                if self.version_is_at_least_0_6_11 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::GweiKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                }
                                if !self.version_is_at_least_0_7_0 {
                                    let result = {
                                        let mut helper = ChoiceHelper::new(stream);
                                        loop {
                                            let result = self.default_parse_token_with_trivia(
                                                stream,
                                                TokenKind::FinneyKeyword,
                                            );
                                            if helper.handle_next_result(stream, result) {
                                                break;
                                            }
                                            let result = self.default_parse_token_with_trivia(
                                                stream,
                                                TokenKind::SzaboKeyword,
                                            );
                                            if helper.handle_next_result(stream, result) {
                                                break;
                                            }
                                            break;
                                        }
                                        helper.result(stream)
                                    };
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                }
                                break;
                            }
                            helper.result(stream)
                        })) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        }
        .with_kind(RuleKind::NumericExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn override_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::OverrideKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.identifier_paths_list(stream),
                        )) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::OverrideSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameter(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.type_name(stream)) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::MemoryKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::StorageKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        if self.version_is_at_least_0_5_0 {
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::CalldataKeyword,
                            );
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        }
                        break;
                    }
                    helper.result(stream)
                })) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                )) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::Parameter)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                ) {
                    break;
                }
                if helper
                    .handle_next_result(OptionalHelper::transform(self.parameters_list(stream)))
                {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn parameters_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.parameter(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.parameter(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ParametersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn path_import(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::AsciiStringLiteral),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::AsKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::PathImport)
    }

    #[allow(unused_assignments, unused_parens)]
    fn positional_arguments_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.expression(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.expression(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::PositionalArgumentsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn pragma_directive(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::PragmaKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result({
                            let mut helper = ChoiceHelper::new(stream);
                            loop {
                                let result = self.abi_coder_pragma(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.experimental_pragma(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.version_pragma(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                break;
                            }
                            helper.result(stream)
                        }) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::PragmaDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(stream, |stream| {
                if self.version_is_at_least_0_6_0 {
                    {
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result = self.modifier_invocation(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.override_specifier(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::ExternalKeyword,
                            );
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::VirtualKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    }
                } else {
                    ParserResult::no_match(vec![])
                }
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::ReceiveFunctionAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn receive_function_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::ReceiveKeyword),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(self.parameters_declaration(stream)) {
                        break;
                    }
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.receive_function_attributes_list(stream),
                    )) {
                        break;
                    }
                    if helper.handle_next_result({
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.block(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    }) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::ReceiveFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn return_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::ReturnKeyword),
                        ) {
                            break;
                        }
                        if helper
                            .handle_next_result(OptionalHelper::transform(self.expression(stream)))
                        {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ReturnStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn returns_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::ReturnsKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(self.parameters_declaration(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::ReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn revert_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::RevertKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.identifier_path(stream),
                        )) {
                            break;
                        }
                        if helper.handle_next_result(self.arguments_declaration(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::RevertStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(OptionalHelper::transform(
                    self.source_unit_members_list(stream),
                )) {
                    break;
                }
                if helper
                    .handle_next_result(OptionalHelper::transform(self.end_of_file_trivia(stream)))
                {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::SourceUnit)
    }

    #[allow(unused_assignments, unused_parens)]
    fn source_unit_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.pragma_directive(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.import_directive(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.contract_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.interface_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.library_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = {
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result = self.struct_definition(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.enum_definition(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    };
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_7_1 {
                    let result = self.function_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_7_4 {
                    let result = self.constant_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_4 {
                    let result = self.error_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_8 {
                    let result = self.user_defined_value_type_definition(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                if self.version_is_at_least_0_8_13 {
                    let result = self.using_directive(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::SourceUnitMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.override_specifier(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::ConstantKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::InternalKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result =
                    self.default_parse_token_with_trivia(stream, TokenKind::PrivateKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.default_parse_token_with_trivia(stream, TokenKind::PublicKeyword);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if self.version_is_at_least_0_6_5 {
                    let result =
                        self.default_parse_token_with_trivia(stream, TokenKind::ImmutableKeyword);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::StateVariableAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn state_variable_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(self.type_name(stream)) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.state_variable_attributes_list(stream),
                        )) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform({
                            let mut helper = SequenceHelper::new();
                            loop {
                                if helper.handle_next_result(
                                    self.default_parse_token_with_trivia(stream, TokenKind::Equal),
                                ) {
                                    break;
                                }
                                if helper.handle_next_result(self.expression(stream)) {
                                    break;
                                }
                                break;
                            }
                            helper.result()
                        })) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::StateVariableDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = {
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result = self.expression_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.variable_declaration_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.tuple_deconstruction_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = {
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result = self.if_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.for_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.while_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.do_while_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.continue_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.break_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.delete_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.return_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.revert_statement(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        if self.version_is_at_least_0_4_21 {
                            let result = self.emit_statement(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        }
                        if !self.version_is_at_least_0_5_0 {
                            let result = self.throw_statement(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        }
                        if self.version_is_at_least_0_6_0 {
                            let result = self.try_statement(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        }
                        break;
                    }
                    helper.result(stream)
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.assembly_statement(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.block(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if self.version_is_at_least_0_8_0 {
                    let result = self.unchecked_block(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        }
        .with_kind(RuleKind::Statement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn statements_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.statement(stream))
            .with_kind(RuleKind::StatementsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::StructKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.struct_members_list(stream),
                        )) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::StructDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_member(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(self.type_name(stream)) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::StructMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn struct_members_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.struct_member(stream))
            .with_kind(RuleKind::StructMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn throw_statement(&self, stream: &mut Stream) -> ParserResult {
        if !self.version_is_at_least_0_5_0 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::ThrowKeyword),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                    ) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::ThrowStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn trailing_trivia(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(OptionalHelper::transform(
                    self.default_parse_token(stream, TokenKind::Whitespace),
                )) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.default_parse_token(stream, TokenKind::SingleLineComment),
                )) {
                    break;
                }
                if helper.handle_next_result(self.default_parse_token(stream, TokenKind::EndOfLine))
                {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::TrailingTrivia)
    }

    #[allow(unused_assignments, unused_parens)]
    fn try_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::TryKeyword),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(self.expression(stream)) {
                        break;
                    }
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.returns_declaration(stream),
                    )) {
                        break;
                    }
                    if helper.handle_next_result(self.block(stream)) {
                        break;
                    }
                    if helper.handle_next_result(self.catch_clauses_list(stream)) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::TryStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_deconstruction_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result({
                            let mut helper = SequenceHelper::new();
                            loop {
                                if helper.handle_next_result(
                                    self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::OpenParen,
                                    ),
                                ) {
                                    break;
                                }
                                if helper.handle_next_result(OptionalHelper::transform(
                                    self.tuple_members_list(stream),
                                )) {
                                    break;
                                }
                                if helper.handle_next_result(
                                    self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::CloseParen,
                                    ),
                                ) {
                                    break;
                                }
                                break;
                            }
                            helper.result()
                        }) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Equal),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.expression(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::TupleDeconstructionStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                ) {
                    break;
                }
                if helper.handle_next_result(self.tuple_values_list(stream)) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::TupleExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_member(&self, stream: &mut Stream) -> ParserResult {
        OptionalHelper::transform({
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(self.type_name(stream)) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform({
                            let mut helper = ChoiceHelper::new(stream);
                            loop {
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::MemoryKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::StorageKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                if self.version_is_at_least_0_5_0 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::CalldataKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                }
                                break;
                            }
                            helper.result(stream)
                        })) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(OptionalHelper::transform({
                            let mut helper = ChoiceHelper::new(stream);
                            loop {
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::MemoryKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::StorageKeyword,
                                );
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                if self.version_is_at_least_0_5_0 {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::CalldataKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                }
                                break;
                            }
                            helper.result(stream)
                        })) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        })
        .with_kind(RuleKind::TupleMember)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_members_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.tuple_member(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.tuple_member(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::TupleMembersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn tuple_values_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(OptionalHelper::transform(self.expression(stream))) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper
                            .handle_next_result(OptionalHelper::transform(self.expression(stream)))
                        {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::TupleValuesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_5_3 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::TypeKeyword),
                    ) {
                        break;
                    }
                    if helper.handle_next_result({
                        let mut helper = SequenceHelper::new();
                        loop {
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                            ) {
                                break;
                            }
                            if helper.handle_next_result(self.type_name(stream)) {
                                break;
                            }
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                            ) {
                                break;
                            }
                            break;
                        }
                        helper.result()
                    }) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::TypeExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn type_name(&self, stream: &mut Stream) -> ParserResult {
        let parse_array_type_name_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(RuleKind::ArrayTypeName, 1u8, 255u8, {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::OpenBracket),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(OptionalHelper::transform(self.expression(stream)))
                    {
                        break;
                    }
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::CloseBracket),
                    ) {
                        break;
                    }
                    break;
                }
                helper.result()
            })
        };
        let primary_expression_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.function_type(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.mapping_type(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = {
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::BoolKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::StringKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.address_type(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::FixedBytesType);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::SignedIntegerType);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.default_parse_token_with_trivia(
                            stream,
                            TokenKind::UnsignedIntegerType,
                        );
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::SignedFixedType);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .default_parse_token_with_trivia(stream, TokenKind::UnsignedFixedType);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        if !self.version_is_at_least_0_8_0 {
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::ByteKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        }
                        break;
                    }
                    helper.result(stream)
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.identifier_path(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        };
        let postfix_operator_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = parse_array_type_name_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        };
        let linear_expression_parser = |stream: &mut Stream| {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(primary_expression_parser(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    postfix_operator_parser(stream)
                })) {
                    break;
                }
                break;
            }
            helper.result()
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::TypeName),
            linear_expression_parser(stream),
        )
        .with_kind(RuleKind::TypeName)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unchecked_block(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_0 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::UncheckedKeyword),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(self.block(stream)) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::UncheckedBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_string_literals_list(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_7_0 {
            OneOrMoreHelper::run(stream, |stream| {
                self.default_parse_token_with_trivia(stream, TokenKind::UnicodeStringLiteral)
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::UnicodeStringLiteralsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            OneOrMoreHelper::run(stream, |stream| {
                if !self.version_is_at_least_0_6_0 {
                    {
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result = self.modifier_invocation(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.override_specifier(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::ExternalKeyword,
                            );
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PayableKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::PureKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self
                                .default_parse_token_with_trivia(stream, TokenKind::ViewKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    }
                } else {
                    ParserResult::no_match(vec![])
                }
            })
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::UnnamedFunctionAttributesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn unnamed_function_definition(&self, stream: &mut Stream) -> ParserResult {
        if !self.version_is_at_least_0_6_0 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::FunctionKeyword),
                    ) {
                        break;
                    }
                    if helper.handle_next_result(self.parameters_declaration(stream)) {
                        break;
                    }
                    if helper.handle_next_result(OptionalHelper::transform(
                        self.unnamed_function_attributes_list(stream),
                    )) {
                        break;
                    }
                    if helper.handle_next_result({
                        let mut helper = ChoiceHelper::new(stream);
                        loop {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::Semicolon);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            let result = self.block(stream);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                            break;
                        }
                        helper.result(stream)
                    }) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::UnnamedFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn user_defined_value_type_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_8_8 {
            {
                let mut helper = SequenceHelper::new();
                loop {
                    if helper.handle_next_result({
                        let mut helper = SequenceHelper::new();
                        loop {
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(
                                    stream,
                                    TokenKind::TypeKeyword,
                                ),
                            ) {
                                break;
                            }
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                            ) {
                                break;
                            }
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(stream, TokenKind::IsKeyword),
                            ) {
                                break;
                            }
                            if helper.handle_next_result({
                                let mut helper = ChoiceHelper::new(stream);
                                loop {
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::BoolKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::StringKeyword,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.address_type(stream);
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::FixedBytesType,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::SignedIntegerType,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::UnsignedIntegerType,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::SignedFixedType,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    let result = self.default_parse_token_with_trivia(
                                        stream,
                                        TokenKind::UnsignedFixedType,
                                    );
                                    if helper.handle_next_result(stream, result) {
                                        break;
                                    }
                                    if !self.version_is_at_least_0_8_0 {
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::ByteKeyword,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                    }
                                    break;
                                }
                                helper.result(stream)
                            }) {
                                break;
                            }
                            break;
                        }
                        helper.result()
                    }) {
                        break;
                    }
                    if helper.handle_next_result(
                        self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                    ) {
                        break;
                    }
                    break;
                }
                helper.result()
            }
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::UserDefinedValueTypeDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::UsingKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result({
                            let mut helper = ChoiceHelper::new(stream);
                            loop {
                                let result = self.using_directive_path(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.using_directive_deconstruction(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                break;
                            }
                            helper.result(stream)
                        }) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::ForKeyword),
                        ) {
                            break;
                        }
                        if helper.handle_next_result({
                            let mut helper = ChoiceHelper::new(stream);
                            loop {
                                let result = self
                                    .default_parse_token_with_trivia(stream, TokenKind::Asterisk);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                let result = self.type_name(stream);
                                if helper.handle_next_result(stream, result) {
                                    break;
                                }
                                break;
                            }
                            helper.result(stream)
                        }) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.default_parse_token_with_trivia(stream, TokenKind::GlobalKeyword),
                        )) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::UsingDirective)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_deconstruction(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                ) {
                    break;
                }
                if helper.handle_next_result(self.using_directive_symbols_list(stream)) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::UsingDirectiveDeconstruction)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_path(&self, stream: &mut Stream) -> ParserResult {
        self.identifier_path(stream)
            .with_kind(RuleKind::UsingDirectivePath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_symbol(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.identifier_path(stream)) {
                    break;
                }
                if self.version_is_at_least_0_8_19 {
                    if helper.handle_next_result(OptionalHelper::transform({
                        let mut helper = SequenceHelper::new();
                        loop {
                            if helper.handle_next_result(
                                self.default_parse_token_with_trivia(stream, TokenKind::AsKeyword),
                            ) {
                                break;
                            }
                            if helper.handle_next_result(if self.version_is_at_least_0_8_19 {
                                {
                                    let mut helper = ChoiceHelper::new(stream);
                                    loop {
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Ampersand,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Asterisk,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::BangEqual,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Bar,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Caret,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::EqualEqual,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::GreaterThan,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::GreaterThanEqual,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::LessThan,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::LessThanEqual,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Minus,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Percent,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Plus,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Slash,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.default_parse_token_with_trivia(
                                            stream,
                                            TokenKind::Tilde,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        break;
                                    }
                                    helper.result(stream)
                                }
                            } else {
                                ParserResult::no_match(vec![])
                            }) {
                                break;
                            }
                            break;
                        }
                        helper.result()
                    })) {
                        break;
                    }
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::UsingDirectiveSymbol)
    }

    #[allow(unused_assignments, unused_parens)]
    fn using_directive_symbols_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.using_directive_symbol(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.using_directive_symbol(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::UsingDirectiveSymbolsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        if !self.version_is_at_least_0_5_0 {
                            let result =
                                self.default_parse_token_with_trivia(stream, TokenKind::VarKeyword);
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        }
                        let result = self.type_name(stream);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                }) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::MemoryKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.default_parse_token_with_trivia(stream, TokenKind::StorageKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        if self.version_is_at_least_0_5_0 {
                            let result = self.default_parse_token_with_trivia(
                                stream,
                                TokenKind::CalldataKeyword,
                            );
                            if helper.handle_next_result(stream, result) {
                                break;
                            }
                        }
                        break;
                    }
                    helper.result(stream)
                })) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Identifier),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::VariableDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn variable_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(self.variable_declaration(stream)) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform({
                            let mut helper = SequenceHelper::new();
                            loop {
                                if helper.handle_next_result(
                                    self.default_parse_token_with_trivia(stream, TokenKind::Equal),
                                ) {
                                    break;
                                }
                                if helper.handle_next_result(self.expression(stream)) {
                                    break;
                                }
                                break;
                            }
                            helper.result()
                        })) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::Semicolon),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::VariableDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.version_pragma_parse_token_with_trivia(stream, TokenKind::SolidityKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(self.version_pragma_expressions_list(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::VersionPragma)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_expression(&self, stream: &mut Stream) -> ParserResult {
        let parse_version_pragma_or_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::VersionPragmaBinaryExpression,
                1u8,
                2u8,
                self.version_pragma_parse_token_with_trivia(stream, TokenKind::BarBar),
            )
        };
        let parse_version_pragma_range_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::VersionPragmaBinaryExpression,
                3u8,
                4u8,
                self.version_pragma_parse_token_with_trivia(stream, TokenKind::Minus),
            )
        };
        let parse_version_pragma_unary_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::VersionPragmaUnaryExpression,
                255u8,
                5u8,
                {
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.version_pragma_parse_token_with_trivia(stream, TokenKind::Caret);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.version_pragma_parse_token_with_trivia(stream, TokenKind::Tilde);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.version_pragma_parse_token_with_trivia(stream, TokenKind::Equal);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .version_pragma_parse_token_with_trivia(stream, TokenKind::LessThan);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .version_pragma_parse_token_with_trivia(stream, TokenKind::GreaterThan);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.version_pragma_parse_token_with_trivia(
                            stream,
                            TokenKind::LessThanEqual,
                        );
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.version_pragma_parse_token_with_trivia(
                            stream,
                            TokenKind::GreaterThanEqual,
                        );
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                },
            )
        };
        let prefix_operator_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = parse_version_pragma_unary_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        };
        let primary_expression_parser = |stream: &mut Stream| self.version_pragma_specifier(stream);
        let binary_operand_parser = |stream: &mut Stream| {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    prefix_operator_parser(stream)
                })) {
                    break;
                }
                if helper.handle_next_result(primary_expression_parser(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        };
        let binary_operator_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = parse_version_pragma_or_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = parse_version_pragma_range_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        };
        let linear_expression_parser = |stream: &mut Stream| {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(binary_operand_parser(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(binary_operator_parser(stream)) {
                            break;
                        }
                        if helper.handle_next_result(binary_operand_parser(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::VersionPragmaExpression),
            linear_expression_parser(stream),
        )
        .with_kind(RuleKind::VersionPragmaExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_expressions_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.version_pragma_expression(stream))
            .with_kind(RuleKind::VersionPragmaExpressionsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.version_pragma_parse_token_with_trivia(
                        stream,
                        TokenKind::VersionPragmaValue,
                    ),
                ) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.version_pragma_parse_token_with_trivia(stream, TokenKind::Period),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.version_pragma_parse_token_with_trivia(
                            stream,
                            TokenKind::VersionPragmaValue,
                        )) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::VersionPragmaSpecifier)
    }

    #[allow(unused_assignments, unused_parens)]
    fn while_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.default_parse_token_with_trivia(stream, TokenKind::WhileKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.expression(stream)) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.default_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                }) {
                    break;
                }
                if helper.handle_next_result(self.statement(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::WhileStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_assignment_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.yul_identifier_paths_list(stream)) {
                    break;
                }
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::ColonEqual),
                ) {
                    break;
                }
                if helper.handle_next_result(self.yul_expression(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulAssignmentStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_block(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::OpenBrace),
                ) {
                    break;
                }
                if helper
                    .handle_next_result(OptionalHelper::transform(self.yul_statements_list(stream)))
                {
                    break;
                }
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::CloseBrace),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulBlock)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_break_statement(&self, stream: &mut Stream) -> ParserResult {
        self.yul_block_parse_token_with_trivia(stream, TokenKind::BreakKeyword)
            .with_kind(RuleKind::YulBreakStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_continue_statement(&self, stream: &mut Stream) -> ParserResult {
        self.yul_block_parse_token_with_trivia(stream, TokenKind::ContinueKeyword)
            .with_kind(RuleKind::YulContinueStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::LetKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(self.yul_identifier_paths_list(stream)) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform({
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::ColonEqual),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.yul_expression(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulDeclarationStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_expression(&self, stream: &mut Stream) -> ParserResult {
        let parse_yul_function_call_operator = |stream: &mut Stream| {
            PrecedenceHelper::to_precedence_result(
                RuleKind::YulFunctionCallExpression,
                1u8,
                255u8,
                {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::OpenParen),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(OptionalHelper::transform(
                            self.yul_expressions_list(stream),
                        )) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::CloseParen),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                },
            )
        };
        let primary_expression_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = {
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result =
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::TrueKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result =
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::FalseKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .yul_block_parse_token_with_trivia(stream, TokenKind::YulHexLiteral);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.yul_block_parse_token_with_trivia(
                            stream,
                            TokenKind::YulDecimalLiteral,
                        );
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self
                            .yul_block_parse_token_with_trivia(stream, TokenKind::HexStringLiteral);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = self.yul_block_parse_token_with_trivia(
                            stream,
                            TokenKind::AsciiStringLiteral,
                        );
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                };
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_identifier_path(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        };
        let postfix_operator_parser = |stream: &mut Stream| {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = parse_yul_function_call_operator(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                break;
            }
            helper.result(stream)
        };
        let linear_expression_parser = |stream: &mut Stream| {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(primary_expression_parser(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    postfix_operator_parser(stream)
                })) {
                    break;
                }
                break;
            }
            helper.result()
        };
        PrecedenceHelper::reduce_precedence_result(
            Some(RuleKind::YulExpression),
            linear_expression_parser(stream),
        )
        .with_kind(RuleKind::YulExpression)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_expressions_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.yul_expression(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.yul_expression(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulExpressionsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_for_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::ForKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(self.yul_block(stream)) {
                    break;
                }
                if helper.handle_next_result(self.yul_expression(stream)) {
                    break;
                }
                if helper.handle_next_result(self.yul_block(stream)) {
                    break;
                }
                if helper.handle_next_result(self.yul_block(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulForStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_function_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::FunctionKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::YulIdentifier),
                ) {
                    break;
                }
                if helper.handle_next_result(self.yul_parameters_declaration(stream)) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.yul_returns_declaration(stream),
                )) {
                    break;
                }
                if helper.handle_next_result(self.yul_block(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulFunctionDefinition)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier_path(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::YulIdentifier),
                ) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::Period),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.yul_block_parse_token_with_trivia(
                                stream,
                                TokenKind::YulIdentifier,
                            ),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulIdentifierPath)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier_paths_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(self.yul_identifier_path(stream)) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(self.yul_identifier_path(stream)) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulIdentifierPathsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifiers_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::YulIdentifier),
                ) {
                    break;
                }
                if helper.handle_next_result(ZeroOrMoreHelper::run(stream, |stream| {
                    let mut helper = SequenceHelper::new();
                    loop {
                        if helper.handle_next_result(
                            self.yul_block_parse_token_with_trivia(stream, TokenKind::Comma),
                        ) {
                            break;
                        }
                        if helper.handle_next_result(
                            self.yul_block_parse_token_with_trivia(
                                stream,
                                TokenKind::YulIdentifier,
                            ),
                        ) {
                            break;
                        }
                        break;
                    }
                    helper.result()
                })) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulIdentifiersList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_if_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::IfKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(self.yul_expression(stream)) {
                    break;
                }
                if helper.handle_next_result(self.yul_block(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulIfStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_leave_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_at_least_0_6_0 {
            self.yul_block_parse_token_with_trivia(stream, TokenKind::LeaveKeyword)
        } else {
            ParserResult::no_match(vec![])
        }
        .with_kind(RuleKind::YulLeaveStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_parameters_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::OpenParen),
                ) {
                    break;
                }
                if helper.handle_next_result(OptionalHelper::transform(
                    self.yul_identifiers_list(stream),
                )) {
                    break;
                }
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::CloseParen),
                ) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulParametersDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_returns_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::MinusGreaterThan),
                ) {
                    break;
                }
                if helper.handle_next_result(self.yul_identifiers_list(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulReturnsDeclaration)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = ChoiceHelper::new(stream);
            loop {
                let result = self.yul_block(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_function_definition(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_declaration_statement(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_assignment_statement(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_if_statement(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_for_statement(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_switch_statement(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_break_statement(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_continue_statement(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                let result = self.yul_expression(stream);
                if helper.handle_next_result(stream, result) {
                    break;
                }
                if self.version_is_at_least_0_6_0 {
                    let result = self.yul_leave_statement(stream);
                    if helper.handle_next_result(stream, result) {
                        break;
                    }
                }
                break;
            }
            helper.result(stream)
        }
        .with_kind(RuleKind::YulStatement)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_statements_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.yul_statement(stream))
            .with_kind(RuleKind::YulStatementsList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_case(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result({
                    let mut helper = ChoiceHelper::new(stream);
                    loop {
                        let result = self
                            .yul_block_parse_token_with_trivia(stream, TokenKind::DefaultKeyword);
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        let result = {
                            let mut helper = SequenceHelper::new();
                            loop {
                                if helper.handle_next_result(
                                    self.yul_block_parse_token_with_trivia(
                                        stream,
                                        TokenKind::CaseKeyword,
                                    ),
                                ) {
                                    break;
                                }
                                if helper.handle_next_result({
                                    let mut helper = ChoiceHelper::new(stream);
                                    loop {
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::TrueKeyword,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::FalseKeyword,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::YulHexLiteral,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::YulDecimalLiteral,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::HexStringLiteral,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        let result = self.yul_block_parse_token_with_trivia(
                                            stream,
                                            TokenKind::AsciiStringLiteral,
                                        );
                                        if helper.handle_next_result(stream, result) {
                                            break;
                                        }
                                        break;
                                    }
                                    helper.result(stream)
                                }) {
                                    break;
                                }
                                break;
                            }
                            helper.result()
                        };
                        if helper.handle_next_result(stream, result) {
                            break;
                        }
                        break;
                    }
                    helper.result(stream)
                }) {
                    break;
                }
                if helper.handle_next_result(self.yul_block(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulSwitchCase)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_cases_list(&self, stream: &mut Stream) -> ParserResult {
        OneOrMoreHelper::run(stream, |stream| self.yul_switch_case(stream))
            .with_kind(RuleKind::YulSwitchCasesList)
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_switch_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut helper = SequenceHelper::new();
            loop {
                if helper.handle_next_result(
                    self.yul_block_parse_token_with_trivia(stream, TokenKind::SwitchKeyword),
                ) {
                    break;
                }
                if helper.handle_next_result(self.yul_expression(stream)) {
                    break;
                }
                if helper.handle_next_result(self.yul_switch_cases_list(stream)) {
                    break;
                }
                break;
            }
            helper.result()
        }
        .with_kind(RuleKind::YulSwitchStatement)
    }

    /********************************************
     *         Scanner Functions
     ********************************************/

    #[allow(unused_assignments, unused_parens)]
    fn ascii_character_without_double_quote_or_backslash(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_char_range!(stream, ' ', '!'),
            scan_char_range!(stream, '#', '['),
            scan_char_range!(stream, ']', '~')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_character_without_single_quote_or_backslash(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_char_range!(stream, ' ', '&'),
            scan_char_range!(stream, '(', '['),
            scan_char_range!(stream, ']', '~')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_escape(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, 't'),
            scan_chars!(stream, 'r'),
            scan_chars!(stream, 'n'),
            scan_chars!(stream, '\\'),
            scan_chars!(stream, '\''),
            scan_chars!(stream, '"'),
            scan_chars!(stream, '\r'),
            scan_chars!(stream, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.single_quoted_ascii_string_literal(stream),
            self.double_quoted_ascii_string_literal(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_digit(&self, stream: &mut Stream) -> bool {
        scan_char_range!(stream, '0', '9')
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_digits(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_one_or_more!(stream, self.decimal_digit(stream)),
            scan_zero_or_more!(
                stream,
                scan_sequence!(
                    scan_chars!(stream, '_'),
                    scan_one_or_more!(stream, self.decimal_digit(stream))
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_exponent(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(stream, scan_chars!(stream, 'e'), scan_chars!(stream, 'E')),
            scan_optional!(stream, scan_chars!(stream, '-')),
            self.decimal_digits(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn decimal_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(
                stream,
                if !self.version_is_at_least_0_5_0 {
                    scan_sequence!(
                        self.decimal_digits(stream),
                        scan_optional!(
                            stream,
                            scan_sequence!(
                                scan_chars!(stream, '.'),
                                scan_optional!(stream, self.decimal_digits(stream))
                            )
                        )
                    )
                } else {
                    false
                },
                if self.version_is_at_least_0_5_0 {
                    scan_sequence!(
                        self.decimal_digits(stream),
                        scan_optional!(
                            stream,
                            scan_sequence!(scan_chars!(stream, '.'), self.decimal_digits(stream))
                        )
                    )
                } else {
                    false
                },
                scan_sequence!(scan_chars!(stream, '.'), self.decimal_digits(stream))
            ),
            scan_optional!(stream, self.decimal_exponent(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '"'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.escape_sequence(stream),
                    self.ascii_character_without_double_quote_or_backslash(stream)
                )
            ),
            scan_chars!(stream, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'h', 'e', 'x', '"'),
            scan_optional!(stream, self.hex_string_contents(stream)),
            scan_chars!(stream, '"')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn double_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '"'),
                scan_zero_or_more!(
                    stream,
                    scan_choice!(
                        stream,
                        self.escape_sequence(stream),
                        scan_none_of!(stream, '\n', '\r', '"', '\\')
                    )
                ),
                scan_chars!(stream, '"')
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn end_of_line(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_optional!(stream, scan_chars!(stream, '\r')),
            scan_chars!(stream, '\n')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn escape_sequence(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '\\'),
            scan_choice!(
                stream,
                self.ascii_escape(stream),
                self.hex_byte_escape(stream),
                self.unicode_escape(stream)
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn fixed_bytes_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'b', 'y', 't', 'e', 's'),
            self.fixed_bytes_type_size(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn fixed_bytes_type_size(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '9'),
            scan_chars!(stream, '8'),
            scan_chars!(stream, '7'),
            scan_chars!(stream, '6'),
            scan_chars!(stream, '5'),
            scan_chars!(stream, '4'),
            scan_chars!(stream, '3', '2'),
            scan_chars!(stream, '3', '1'),
            scan_chars!(stream, '3', '0'),
            scan_chars!(stream, '3'),
            scan_chars!(stream, '2', '9'),
            scan_chars!(stream, '2', '8'),
            scan_chars!(stream, '2', '7'),
            scan_chars!(stream, '2', '6'),
            scan_chars!(stream, '2', '5'),
            scan_chars!(stream, '2', '4'),
            scan_chars!(stream, '2', '3'),
            scan_chars!(stream, '2', '2'),
            scan_chars!(stream, '2', '1'),
            scan_chars!(stream, '2', '0'),
            scan_chars!(stream, '2'),
            scan_chars!(stream, '1', '9'),
            scan_chars!(stream, '1', '8'),
            scan_chars!(stream, '1', '7'),
            scan_chars!(stream, '1', '6'),
            scan_chars!(stream, '1', '5'),
            scan_chars!(stream, '1', '4'),
            scan_chars!(stream, '1', '3'),
            scan_chars!(stream, '1', '2'),
            scan_chars!(stream, '1', '1'),
            scan_chars!(stream, '1', '0'),
            scan_chars!(stream, '1')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn fixed_type_size(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_one_or_more!(stream, scan_char_range!(stream, '0', '9')),
            scan_chars!(stream, 'x'),
            scan_one_or_more!(stream, scan_char_range!(stream, '0', '9'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_byte_escape(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'x'),
            self.hex_character(stream),
            self.hex_character(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_character(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.decimal_digit(stream),
            scan_char_range!(stream, 'A', 'F'),
            scan_char_range!(stream, 'a', 'f')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_choice!(
                stream,
                scan_chars!(stream, '0', 'x'),
                if !self.version_is_at_least_0_5_0 {
                    scan_chars!(stream, '0', 'X')
                } else {
                    false
                }
            ),
            scan_one_or_more!(stream, self.hex_character(stream)),
            scan_zero_or_more!(
                stream,
                scan_sequence!(
                    scan_chars!(stream, '_'),
                    scan_one_or_more!(stream, self.hex_character(stream))
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_contents(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            self.hex_character(stream),
            self.hex_character(stream),
            scan_zero_or_more!(
                stream,
                scan_sequence!(
                    scan_optional!(stream, scan_chars!(stream, '_')),
                    self.hex_character(stream),
                    self.hex_character(stream)
                )
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.single_quoted_hex_string_literal(stream),
            self.double_quoted_hex_string_literal(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier(&self, stream: &mut Stream) -> bool {
        self.raw_identifier(stream)
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_part(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            self.identifier_start(stream),
            scan_char_range!(stream, '0', '9')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn identifier_start(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '_'),
            scan_chars!(stream, '$'),
            scan_char_range!(stream, 'A', 'Z'),
            scan_char_range!(stream, 'a', 'z')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn integer_type_size(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '9', '6'),
            scan_chars!(stream, '8', '8'),
            scan_chars!(stream, '8', '0'),
            scan_chars!(stream, '8'),
            scan_chars!(stream, '7', '2'),
            scan_chars!(stream, '6', '4'),
            scan_chars!(stream, '5', '6'),
            scan_chars!(stream, '4', '8'),
            scan_chars!(stream, '4', '0'),
            scan_chars!(stream, '3', '2'),
            scan_chars!(stream, '2', '5', '6'),
            scan_chars!(stream, '2', '4', '8'),
            scan_chars!(stream, '2', '4', '0'),
            scan_chars!(stream, '2', '4'),
            scan_chars!(stream, '2', '3', '2'),
            scan_chars!(stream, '2', '2', '4'),
            scan_chars!(stream, '2', '1', '6'),
            scan_chars!(stream, '2', '0', '8'),
            scan_chars!(stream, '2', '0', '0'),
            scan_chars!(stream, '1', '9', '2'),
            scan_chars!(stream, '1', '8', '4'),
            scan_chars!(stream, '1', '7', '6'),
            scan_chars!(stream, '1', '6', '8'),
            scan_chars!(stream, '1', '6', '0'),
            scan_chars!(stream, '1', '6'),
            scan_chars!(stream, '1', '5', '2'),
            scan_chars!(stream, '1', '4', '4'),
            scan_chars!(stream, '1', '3', '6'),
            scan_chars!(stream, '1', '2', '8'),
            scan_chars!(stream, '1', '2', '0'),
            scan_chars!(stream, '1', '1', '2'),
            scan_chars!(stream, '1', '0', '4')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn multiline_comment(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '/'),
            scan_chars!(stream, '*'),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    scan_none_of!(stream, '*'),
                    scan_sequence!(scan_chars!(stream, '*'), scan_none_of!(stream, '/'))
                )
            ),
            scan_chars!(stream, '*'),
            scan_chars!(stream, '/')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn raw_identifier(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            self.identifier_start(stream),
            scan_zero_or_more!(stream, self.identifier_part(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn signed_fixed_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'f', 'i', 'x', 'e', 'd'),
            scan_optional!(stream, self.fixed_type_size(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn signed_integer_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'i', 'n', 't'),
            scan_optional!(stream, self.integer_type_size(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_line_comment(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '/', '/'),
            scan_zero_or_more!(stream, scan_none_of!(stream, '\n', '\r'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_ascii_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '\''),
            scan_zero_or_more!(
                stream,
                scan_choice!(
                    stream,
                    self.escape_sequence(stream),
                    self.ascii_character_without_single_quote_or_backslash(stream)
                )
            ),
            scan_chars!(stream, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_hex_string_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'h', 'e', 'x', '\''),
            scan_optional!(stream, self.hex_string_contents(stream)),
            scan_chars!(stream, '\'')
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn single_quoted_unicode_string_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_sequence!(
                scan_chars!(stream, 'u', 'n', 'i', 'c', 'o', 'd', 'e', '\''),
                scan_zero_or_more!(
                    stream,
                    scan_choice!(
                        stream,
                        self.escape_sequence(stream),
                        scan_none_of!(stream, '\n', '\r', '\'', '\\')
                    )
                ),
                scan_chars!(stream, '\'')
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_escape(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u'),
            self.hex_character(stream),
            self.hex_character(stream),
            self.hex_character(stream),
            self.hex_character(stream)
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn unicode_string_literal(&self, stream: &mut Stream) -> bool {
        if self.version_is_at_least_0_7_0 {
            scan_choice!(
                stream,
                self.single_quoted_unicode_string_literal(stream),
                self.double_quoted_unicode_string_literal(stream)
            )
        } else {
            false
        }
    }

    #[allow(unused_assignments, unused_parens)]
    fn unsigned_fixed_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u', 'f', 'i', 'x', 'e', 'd'),
            scan_optional!(stream, self.fixed_type_size(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn unsigned_integer_type(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, 'u', 'i', 'n', 't'),
            scan_optional!(stream, self.integer_type_size(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn version_pragma_value(&self, stream: &mut Stream) -> bool {
        scan_one_or_more!(
            stream,
            scan_choice!(
                stream,
                scan_chars!(stream, 'x'),
                scan_chars!(stream, 'X'),
                scan_chars!(stream, '*'),
                scan_char_range!(stream, '0', '9')
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn whitespace(&self, stream: &mut Stream) -> bool {
        scan_one_or_more!(
            stream,
            scan_choice!(stream, scan_chars!(stream, ' '), scan_chars!(stream, '\t'))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_decimal_literal(&self, stream: &mut Stream) -> bool {
        scan_choice!(
            stream,
            scan_chars!(stream, '0'),
            scan_sequence!(
                scan_char_range!(stream, '1', '9'),
                scan_zero_or_more!(stream, self.decimal_digit(stream))
            )
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_hex_literal(&self, stream: &mut Stream) -> bool {
        scan_sequence!(
            scan_chars!(stream, '0', 'x'),
            scan_one_or_more!(stream, self.hex_character(stream))
        )
    }

    #[allow(unused_assignments, unused_parens)]
    fn yul_identifier(&self, stream: &mut Stream) -> bool {
        self.raw_identifier(stream)
    }

    pub fn scan(&self, lexical_context: LexicalContext, input: &str) -> Option<TokenKind> {
        let mut stream = Stream::new(input);
        match lexical_context {
            LexicalContext::Default => self.default_next_token(&mut stream),
            LexicalContext::VersionPragma => self.version_pragma_next_token(&mut stream),
            LexicalContext::YulBlock => self.yul_block_next_token(&mut stream),
        }
    }

    pub fn parse(&self, production_kind: ProductionKind, input: &str) -> ParseOutput {
        match production_kind {
            ProductionKind::ABICoderPragma => Self::abi_coder_pragma.parse(self, input),
            ProductionKind::AddressType => Self::address_type.parse(self, input),
            ProductionKind::ArgumentsDeclaration => Self::arguments_declaration.parse(self, input),
            ProductionKind::ArrayExpression => Self::array_expression.parse(self, input),
            ProductionKind::ArrayValuesList => Self::array_values_list.parse(self, input),
            ProductionKind::AsciiStringLiteralsList => {
                Self::ascii_string_literals_list.parse(self, input)
            }
            ProductionKind::AssemblyFlagsList => Self::assembly_flags_list.parse(self, input),
            ProductionKind::AssemblyStatement => Self::assembly_statement.parse(self, input),
            ProductionKind::Block => Self::block.parse(self, input),
            ProductionKind::BreakStatement => Self::break_statement.parse(self, input),
            ProductionKind::CatchClause => Self::catch_clause.parse(self, input),
            ProductionKind::CatchClauseError => Self::catch_clause_error.parse(self, input),
            ProductionKind::CatchClausesList => Self::catch_clauses_list.parse(self, input),
            ProductionKind::ConstantDefinition => Self::constant_definition.parse(self, input),
            ProductionKind::ConstructorAttributesList => {
                Self::constructor_attributes_list.parse(self, input)
            }
            ProductionKind::ConstructorDefinition => {
                Self::constructor_definition.parse(self, input)
            }
            ProductionKind::ContinueStatement => Self::continue_statement.parse(self, input),
            ProductionKind::ContractDefinition => Self::contract_definition.parse(self, input),
            ProductionKind::ContractMembersList => Self::contract_members_list.parse(self, input),
            ProductionKind::DeconstructionImport => Self::deconstruction_import.parse(self, input),
            ProductionKind::DeconstructionImportSymbol => {
                Self::deconstruction_import_symbol.parse(self, input)
            }
            ProductionKind::DeconstructionImportSymbolsList => {
                Self::deconstruction_import_symbols_list.parse(self, input)
            }
            ProductionKind::DeleteStatement => Self::delete_statement.parse(self, input),
            ProductionKind::DoWhileStatement => Self::do_while_statement.parse(self, input),
            ProductionKind::EmitStatement => Self::emit_statement.parse(self, input),
            ProductionKind::EndOfFileTrivia => Self::end_of_file_trivia.parse(self, input),
            ProductionKind::EnumDefinition => Self::enum_definition.parse(self, input),
            ProductionKind::ErrorDefinition => Self::error_definition.parse(self, input),
            ProductionKind::ErrorParameter => Self::error_parameter.parse(self, input),
            ProductionKind::ErrorParametersList => Self::error_parameters_list.parse(self, input),
            ProductionKind::EventDefinition => Self::event_definition.parse(self, input),
            ProductionKind::EventParameter => Self::event_parameter.parse(self, input),
            ProductionKind::EventParametersList => Self::event_parameters_list.parse(self, input),
            ProductionKind::ExperimentalPragma => Self::experimental_pragma.parse(self, input),
            ProductionKind::Expression => Self::expression.parse(self, input),
            ProductionKind::ExpressionStatement => Self::expression_statement.parse(self, input),
            ProductionKind::FallbackFunctionAttributesList => {
                Self::fallback_function_attributes_list.parse(self, input)
            }
            ProductionKind::FallbackFunctionDefinition => {
                Self::fallback_function_definition.parse(self, input)
            }
            ProductionKind::ForStatement => Self::for_statement.parse(self, input),
            ProductionKind::FunctionAttributesList => {
                Self::function_attributes_list.parse(self, input)
            }
            ProductionKind::FunctionCallOptions => Self::function_call_options.parse(self, input),
            ProductionKind::FunctionDefinition => Self::function_definition.parse(self, input),
            ProductionKind::FunctionType => Self::function_type.parse(self, input),
            ProductionKind::FunctionTypeAttributesList => {
                Self::function_type_attributes_list.parse(self, input)
            }
            ProductionKind::HexStringLiteralsList => {
                Self::hex_string_literals_list.parse(self, input)
            }
            ProductionKind::IdentifierPath => Self::identifier_path.parse(self, input),
            ProductionKind::IdentifierPathsList => Self::identifier_paths_list.parse(self, input),
            ProductionKind::IdentifiersList => Self::identifiers_list.parse(self, input),
            ProductionKind::IfStatement => Self::if_statement.parse(self, input),
            ProductionKind::ImportDirective => Self::import_directive.parse(self, input),
            ProductionKind::InheritanceSpecifier => Self::inheritance_specifier.parse(self, input),
            ProductionKind::InheritanceType => Self::inheritance_type.parse(self, input),
            ProductionKind::InheritanceTypesList => Self::inheritance_types_list.parse(self, input),
            ProductionKind::InterfaceDefinition => Self::interface_definition.parse(self, input),
            ProductionKind::InterfaceMembersList => Self::interface_members_list.parse(self, input),
            ProductionKind::LeadingTrivia => Self::leading_trivia.parse(self, input),
            ProductionKind::LibraryDefinition => Self::library_definition.parse(self, input),
            ProductionKind::LibraryMembersList => Self::library_members_list.parse(self, input),
            ProductionKind::MappingKeyType => Self::mapping_key_type.parse(self, input),
            ProductionKind::MappingType => Self::mapping_type.parse(self, input),
            ProductionKind::MappingValueType => Self::mapping_value_type.parse(self, input),
            ProductionKind::ModifierAttributesList => {
                Self::modifier_attributes_list.parse(self, input)
            }
            ProductionKind::ModifierDefinition => Self::modifier_definition.parse(self, input),
            ProductionKind::ModifierInvocation => Self::modifier_invocation.parse(self, input),
            ProductionKind::NamedArgument => Self::named_argument.parse(self, input),
            ProductionKind::NamedArgumentsDeclaration => {
                Self::named_arguments_declaration.parse(self, input)
            }
            ProductionKind::NamedArgumentsList => Self::named_arguments_list.parse(self, input),
            ProductionKind::NamedImport => Self::named_import.parse(self, input),
            ProductionKind::NewExpression => Self::new_expression.parse(self, input),
            ProductionKind::NumericExpression => Self::numeric_expression.parse(self, input),
            ProductionKind::OverrideSpecifier => Self::override_specifier.parse(self, input),
            ProductionKind::Parameter => Self::parameter.parse(self, input),
            ProductionKind::ParametersDeclaration => {
                Self::parameters_declaration.parse(self, input)
            }
            ProductionKind::ParametersList => Self::parameters_list.parse(self, input),
            ProductionKind::PathImport => Self::path_import.parse(self, input),
            ProductionKind::PositionalArgumentsList => {
                Self::positional_arguments_list.parse(self, input)
            }
            ProductionKind::PragmaDirective => Self::pragma_directive.parse(self, input),
            ProductionKind::ReceiveFunctionAttributesList => {
                Self::receive_function_attributes_list.parse(self, input)
            }
            ProductionKind::ReceiveFunctionDefinition => {
                Self::receive_function_definition.parse(self, input)
            }
            ProductionKind::ReturnStatement => Self::return_statement.parse(self, input),
            ProductionKind::ReturnsDeclaration => Self::returns_declaration.parse(self, input),
            ProductionKind::RevertStatement => Self::revert_statement.parse(self, input),
            ProductionKind::SourceUnit => Self::source_unit.parse(self, input),
            ProductionKind::SourceUnitMembersList => {
                Self::source_unit_members_list.parse(self, input)
            }
            ProductionKind::StateVariableAttributesList => {
                Self::state_variable_attributes_list.parse(self, input)
            }
            ProductionKind::StateVariableDefinition => {
                Self::state_variable_definition.parse(self, input)
            }
            ProductionKind::Statement => Self::statement.parse(self, input),
            ProductionKind::StatementsList => Self::statements_list.parse(self, input),
            ProductionKind::StructDefinition => Self::struct_definition.parse(self, input),
            ProductionKind::StructMember => Self::struct_member.parse(self, input),
            ProductionKind::StructMembersList => Self::struct_members_list.parse(self, input),
            ProductionKind::ThrowStatement => Self::throw_statement.parse(self, input),
            ProductionKind::TrailingTrivia => Self::trailing_trivia.parse(self, input),
            ProductionKind::TryStatement => Self::try_statement.parse(self, input),
            ProductionKind::TupleDeconstructionStatement => {
                Self::tuple_deconstruction_statement.parse(self, input)
            }
            ProductionKind::TupleExpression => Self::tuple_expression.parse(self, input),
            ProductionKind::TupleMember => Self::tuple_member.parse(self, input),
            ProductionKind::TupleMembersList => Self::tuple_members_list.parse(self, input),
            ProductionKind::TupleValuesList => Self::tuple_values_list.parse(self, input),
            ProductionKind::TypeExpression => Self::type_expression.parse(self, input),
            ProductionKind::TypeName => Self::type_name.parse(self, input),
            ProductionKind::UncheckedBlock => Self::unchecked_block.parse(self, input),
            ProductionKind::UnicodeStringLiteralsList => {
                Self::unicode_string_literals_list.parse(self, input)
            }
            ProductionKind::UnnamedFunctionAttributesList => {
                Self::unnamed_function_attributes_list.parse(self, input)
            }
            ProductionKind::UnnamedFunctionDefinition => {
                Self::unnamed_function_definition.parse(self, input)
            }
            ProductionKind::UserDefinedValueTypeDefinition => {
                Self::user_defined_value_type_definition.parse(self, input)
            }
            ProductionKind::UsingDirective => Self::using_directive.parse(self, input),
            ProductionKind::UsingDirectiveDeconstruction => {
                Self::using_directive_deconstruction.parse(self, input)
            }
            ProductionKind::UsingDirectivePath => Self::using_directive_path.parse(self, input),
            ProductionKind::UsingDirectiveSymbol => Self::using_directive_symbol.parse(self, input),
            ProductionKind::UsingDirectiveSymbolsList => {
                Self::using_directive_symbols_list.parse(self, input)
            }
            ProductionKind::VariableDeclaration => Self::variable_declaration.parse(self, input),
            ProductionKind::VariableDeclarationStatement => {
                Self::variable_declaration_statement.parse(self, input)
            }
            ProductionKind::VersionPragma => Self::version_pragma.parse(self, input),
            ProductionKind::VersionPragmaExpression => {
                Self::version_pragma_expression.parse(self, input)
            }
            ProductionKind::VersionPragmaExpressionsList => {
                Self::version_pragma_expressions_list.parse(self, input)
            }
            ProductionKind::VersionPragmaSpecifier => {
                Self::version_pragma_specifier.parse(self, input)
            }
            ProductionKind::WhileStatement => Self::while_statement.parse(self, input),
            ProductionKind::YulAssignmentStatement => {
                Self::yul_assignment_statement.parse(self, input)
            }
            ProductionKind::YulBlock => Self::yul_block.parse(self, input),
            ProductionKind::YulBreakStatement => Self::yul_break_statement.parse(self, input),
            ProductionKind::YulContinueStatement => Self::yul_continue_statement.parse(self, input),
            ProductionKind::YulDeclarationStatement => {
                Self::yul_declaration_statement.parse(self, input)
            }
            ProductionKind::YulExpression => Self::yul_expression.parse(self, input),
            ProductionKind::YulExpressionsList => Self::yul_expressions_list.parse(self, input),
            ProductionKind::YulForStatement => Self::yul_for_statement.parse(self, input),
            ProductionKind::YulFunctionDefinition => {
                Self::yul_function_definition.parse(self, input)
            }
            ProductionKind::YulIdentifierPath => Self::yul_identifier_path.parse(self, input),
            ProductionKind::YulIdentifierPathsList => {
                Self::yul_identifier_paths_list.parse(self, input)
            }
            ProductionKind::YulIdentifiersList => Self::yul_identifiers_list.parse(self, input),
            ProductionKind::YulIfStatement => Self::yul_if_statement.parse(self, input),
            ProductionKind::YulLeaveStatement => Self::yul_leave_statement.parse(self, input),
            ProductionKind::YulParametersDeclaration => {
                Self::yul_parameters_declaration.parse(self, input)
            }
            ProductionKind::YulReturnsDeclaration => {
                Self::yul_returns_declaration.parse(self, input)
            }
            ProductionKind::YulStatement => Self::yul_statement.parse(self, input),
            ProductionKind::YulStatementsList => Self::yul_statements_list.parse(self, input),
            ProductionKind::YulSwitchCase => Self::yul_switch_case.parse(self, input),
            ProductionKind::YulSwitchCasesList => Self::yul_switch_cases_list.parse(self, input),
            ProductionKind::YulSwitchStatement => Self::yul_switch_statement.parse(self, input),
        }
    }
}

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr( feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "language") )]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum LexicalContext {
    Default,
    VersionPragma,
    YulBlock,
}

#[cfg(feature = "slang_napi_interfaces")]
#[napi(namespace = "language")]
impl Language {
    #[napi(constructor)]
    pub fn new_napi(version: String) -> std::result::Result<Self, napi::Error> {
        let version =
            Version::parse(&version).map_err(|_| Error::InvalidSemanticVersion(version))?;
        Self::new(version).map_err(|e| e.into())
    }

    #[napi(getter, js_name = "version")]
    pub fn version_napi(&self) -> String {
        self.version.to_string()
    }

    #[napi(js_name = "supportedVersions")]
    pub fn supported_versions_napi() -> Vec<String> {
        return Self::VERSIONS.iter().map(|v| v.to_string()).collect();
    }

    #[napi(js_name = "scan")]
    pub fn scan_napi(&self, lexical_context: LexicalContext, input: String) -> Option<TokenKind> {
        self.scan(lexical_context, input.as_str())
    }

    #[napi(js_name = "parse", ts_return_type = "ParseOutput")]
    pub fn parse_napi(&self, production_kind: ProductionKind, input: String) -> NAPIParseOutput {
        self.parse(production_kind, input.as_str()).into()
    }
}
