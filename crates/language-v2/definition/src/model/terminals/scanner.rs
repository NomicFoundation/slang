use indexmap::IndexSet;
use language_v2_internal_macros::{ParseInputTokens, WriteOutputTokens, derive_spanned_type};
use serde::{Deserialize, Serialize};

use crate::model::Identifier;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
#[serde(tag = "type")]
pub enum TokenScanner {
    Sequence {
        scanners: Vec<TokenScanner>,
    },
    Choice {
        scanners: Vec<TokenScanner>,
    },
    Optional {
        scanner: Box<TokenScanner>,
    },
    ZeroOrMore {
        scanner: Box<TokenScanner>,
    },
    OneOrMore {
        scanner: Box<TokenScanner>,
    },
    Not {
        chars: IndexSet<char>,
    },
    Range {
        inclusive_start: char,
        inclusive_end: char,
    },
    Atom {
        atom: String,
    },
    Fragment {
        reference: Identifier,
    },
}

impl TokenScanner {
    /// Calculate the unique symbol for this token scanner if it exists. Returns `None` if it couldn't calculate it.
    ///
    /// This is a best effort approach, in particular:
    /// - We don't consider empty scanners, this means that a `Choice` without any scanners is
    ///   not considered to have a unique symbol (this is ok), but a `Choice` containing that empty
    ///   `Choice` and an atom is also not considered to have a unique symbol, even though it could
    ///   be considered to have the same unique symbol as the atom.
    /// - `Fragments` are not considered to be unique symbols, even though they could.
    pub fn unique_symbol(&self) -> Option<String> {
        match self {
            Self::Sequence { scanners } if !scanners.is_empty() => scanners
                .iter()
                .map(|s| s.unique_symbol())
                .collect::<Option<Vec<String>>>()
                .map(|s| s.join("")),
            Self::Atom { atom } => Some(atom.clone()),
            Self::Choice { scanners } if scanners.len() == 1 => scanners[0].unique_symbol(),
            Self::Range {
                inclusive_start,
                inclusive_end,
            } if *inclusive_start == *inclusive_end => Some(inclusive_start.to_string()),
            Self::Sequence { .. }
            | Self::Choice { .. }
            | Self::Optional { .. }
            | Self::ZeroOrMore { .. }
            | Self::OneOrMore { .. }
            | Self::Not { .. }
            | Self::Range { .. }
            | Self::Fragment { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_is_unique_symbol() {
        let scanner = TokenScanner::Atom {
            atom: "&a".to_string(),
        };
        assert_eq!(scanner.unique_symbol(), Some("&a".to_string()));
    }

    #[test]
    fn test_empty_sequence_is_not_unique() {
        let scanner = TokenScanner::Sequence { scanners: vec![] };
        assert_eq!(scanner.unique_symbol(), None);
    }

    #[test]
    fn test_sequence_of_unique_symbols_is_unique() {
        let scanner = TokenScanner::Sequence {
            scanners: vec![
                TokenScanner::Atom {
                    atom: "a".to_string(),
                },
                TokenScanner::Choice {
                    scanners: vec![TokenScanner::Atom {
                        atom: "b".to_string(),
                    }],
                },
                TokenScanner::Range {
                    inclusive_start: 'c',
                    inclusive_end: 'c',
                },
            ],
        };
        assert_eq!(scanner.unique_symbol(), Some("abc".to_string()));
    }

    #[test]
    fn test_sequence_of_non_unique_is_not_unique() {
        let scanner = TokenScanner::Sequence {
            scanners: vec![
                TokenScanner::Atom {
                    atom: "a".to_string(),
                },
                TokenScanner::Choice {
                    scanners: vec![
                        TokenScanner::Atom {
                            atom: "b".to_string(),
                        },
                        TokenScanner::Atom {
                            atom: "c".to_string(),
                        },
                    ],
                },
            ],
        };
        assert_eq!(scanner.unique_symbol(), None);
    }

    #[test]
    fn test_choice_of_one_unique_is_unique() {
        let scanner = TokenScanner::Choice {
            scanners: vec![TokenScanner::Atom {
                atom: "a".to_string(),
            }],
        };
        assert_eq!(scanner.unique_symbol(), Some("a".to_string()));
    }

    #[test]
    fn test_choice_of_multiple_is_not_unique() {
        let scanner = TokenScanner::Choice {
            scanners: vec![
                TokenScanner::Atom {
                    atom: "a".to_string(),
                },
                TokenScanner::Atom {
                    atom: "b".to_string(),
                },
            ],
        };
        assert_eq!(scanner.unique_symbol(), None);
    }

    #[test]
    fn test_choice_of_non_unique_is_not_unique() {
        let scanner = TokenScanner::Choice {
            scanners: vec![TokenScanner::Range {
                inclusive_start: 'a',
                inclusive_end: 'b',
            }],
        };
        assert_eq!(scanner.unique_symbol(), None);
    }

    #[test]
    fn test_range_of_single_char_is_unique() {
        let scanner = TokenScanner::Range {
            inclusive_start: 'a',
            inclusive_end: 'a',
        };
        assert_eq!(scanner.unique_symbol(), Some("a".to_string()));
    }

    #[test]
    fn test_range_of_different_chars_is_not_unique() {
        let scanner = TokenScanner::Range {
            inclusive_start: 'a',
            inclusive_end: 'b',
        };
        assert_eq!(scanner.unique_symbol(), None);
    }

    #[test]
    fn test_not_is_not_unique() {
        let scanner = TokenScanner::Not {
            chars: IndexSet::from_iter(vec!['a', 'b', 'c']),
        };
        assert_eq!(scanner.unique_symbol(), None);
    }

    #[test]
    fn test_optional_is_not_unique() {
        let scanner = TokenScanner::Optional {
            scanner: Box::new(TokenScanner::Atom {
                atom: "a".to_string(),
            }),
        };
        assert_eq!(scanner.unique_symbol(), None);
    }

    #[test]
    fn test_zero_or_more_is_not_unique() {
        let scanner = TokenScanner::ZeroOrMore {
            scanner: Box::new(TokenScanner::Atom {
                atom: "a".to_string(),
            }),
        };
        assert_eq!(scanner.unique_symbol(), None);
    }

    #[test]
    fn test_one_or_more_is_not_unique() {
        let scanner = TokenScanner::OneOrMore {
            scanner: Box::new(TokenScanner::Atom {
                atom: "a".to_string(),
            }),
        };
        assert_eq!(scanner.unique_symbol(), None);
    }

    #[test]
    fn test_fragment_is_not_unique() {
        let scanner = TokenScanner::Fragment {
            reference: "fragment".into(),
        };
        assert_eq!(scanner.unique_symbol(), None);
    }
}
