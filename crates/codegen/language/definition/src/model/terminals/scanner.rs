use crate::model::{Identifier, Spanned};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use indexmap::IndexSet;
use serde::Serialize;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub enum Scanner {
    Sequence {
        scanners: Vec<Scanner>,
    },
    Choice {
        scanners: Vec<Scanner>,
    },
    Optional {
        scanner: Box<Scanner>,
    },
    ZeroOrMore {
        scanner: Box<Scanner>,
    },
    OneOrMore {
        scanner: Box<Scanner>,
    },
    Not {
        chars: IndexSet<Spanned<char>>,
    },
    Range {
        inclusive_start: Spanned<char>,
        inclusive_end: Spanned<char>,
    },
    Atom {
        atom: Spanned<String>,
    },
    TrailingContext {
        scanner: Box<Scanner>,
        not_followed_by: Box<Scanner>,
    },
    Fragment {
        reference: Spanned<Identifier>,
    },
}
