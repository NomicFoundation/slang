use std::{collections::BTreeMap, rc::Rc};

use semver::Version;

use crate::schema::*;

fn ref_from_ebnf(ebnf: EBNF) -> ExpressionRef {
    Rc::new(Expression {
        config: Default::default(),
        ebnf,
    })
}

pub fn production(((name, _is_token), ebnf): ((String, bool), EBNF)) -> Production {
    Production {
        name,
        kind: None,
        title: None,
        versions: BTreeMap::from([(Version::parse("0.0.0").unwrap(), ref_from_ebnf(ebnf))]),
        combinator_tree: Default::default(),
    }
}

pub fn eof(_: char) -> EBNF {
    EBNF::End
}

pub fn string(chars: Vec<char>) -> EBNF {
    EBNF::Terminal(chars.iter().collect::<String>())
}

pub fn char_range((start, end): (char, char)) -> EBNF {
    EBNF::Range(EBNFRange {
        from: start,
        to: end,
    })
}

pub fn raw_identifier(chars: Vec<char>) -> String {
    chars.iter().collect()
}

pub fn sequence(mut ebnfs: Vec<EBNF>) -> EBNF {
    if ebnfs.len() == 1 {
        ebnfs.pop().unwrap()
    } else {
        EBNF::Sequence(ebnfs.into_iter().map(ref_from_ebnf).collect())
    }
}

pub fn expression(mut ebnfs: Vec<EBNF>) -> EBNF {
    if ebnfs.len() == 1 {
        ebnfs.pop().unwrap()
    } else {
        EBNF::Choice(ebnfs.into_iter().map(ref_from_ebnf).collect())
    }
}

pub fn difference((minuend, subtrahend): (EBNF, Option<EBNF>)) -> EBNF {
    if let Some(subtrahend) = subtrahend {
        EBNF::Difference(EBNFDifference {
            minuend: ref_from_ebnf(minuend),
            subtrahend: ref_from_ebnf(subtrahend),
        })
    } else {
        minuend
    }
}

pub fn negation((negation, ebnf): (Option<char>, EBNF)) -> EBNF {
    if negation.is_some() {
        EBNF::Not(ref_from_ebnf(ebnf))
    } else {
        ebnf
    }
}

pub fn optional(ebnf: EBNF) -> EBNF {
    EBNF::Repeat(EBNFRepeat {
        min: 0,
        max: Some(1),
        expr: ref_from_ebnf(ebnf),
        separator: None,
    })
}

pub fn repeated(
    ((range, ebnf), separator): ((Option<(usize, Option<usize>)>, EBNF), Option<String>),
) -> EBNF {
    let (min, max) = range.unwrap_or((0, None));
    EBNF::Repeat(EBNFRepeat {
        min,
        max,
        expr: ref_from_ebnf(ebnf),
        separator,
    })
}

pub fn production_reference((name, _): (String, bool)) -> EBNF {
    EBNF::Reference(name)
}

pub fn hex_digits_to_char(digits: Vec<char>) -> Result<char, ()> {
    let digits = digits.iter().collect::<String>();
    char::from_u32(u32::from_str_radix(digits.as_str(), 16).unwrap()).ok_or(())
}

pub fn repetition_prefix_1((from, to): (usize, Option<Option<usize>>)) -> (usize, Option<usize>) {
    (from, to.unwrap_or_else(|| Some(from)))
}

pub fn repetition_prefix_2(to: usize) -> (usize, Option<usize>) {
    (0, Some(to))
}

pub fn number(digits: Vec<char>) -> usize {
    let digits = digits.iter().collect::<String>();
    usize::from_str_radix(digits.as_str(), 10).unwrap()
}

pub fn identifier_1(string: String) -> (String, bool) {
    (string, true)
}

pub fn identifier_2(string: String) -> (String, bool) {
    (string, false)
}
