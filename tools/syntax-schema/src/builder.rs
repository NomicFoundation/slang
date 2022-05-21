use std::rc::Rc;

use crate::schema::*;

fn ref_from_ebnf(ebnf: EBNF) -> ExpressionRef {
    Rc::new(Expression {
        config: Default::default(),
        ebnf,
    })
}

pub fn map_production(((name, is_token), ebnf): ((String, bool), EBNF)) -> Production {
    Production {
        name,
        is_token,
        title: None,
        expr: Some(ref_from_ebnf(ebnf)),
        versions: Default::default(),
    }
}

pub fn map_eof(_: char) -> EBNF {
    EBNF::End
}

pub fn map_string(chars: Vec<char>) -> EBNF {
    EBNF::Terminal(chars.iter().collect::<String>())
}

pub fn map_char_range((start, end): (char, char)) -> EBNF {
    EBNF::Range(EBNFRange {
        from: start,
        to: end,
    })
}

pub fn map_raw_identifier(chars: Vec<char>) -> String {
    chars.iter().collect()
}

pub fn map_sequence(mut ebnfs: Vec<EBNF>) -> EBNF {
    if ebnfs.len() == 1 {
        ebnfs.pop().unwrap()
    } else {
        EBNF::Sequence(ebnfs.into_iter().map(ref_from_ebnf).collect())
    }
}

pub fn map_expression(mut ebnfs: Vec<EBNF>) -> EBNF {
    if ebnfs.len() == 1 {
        ebnfs.pop().unwrap()
    } else {
        EBNF::Choice(ebnfs.into_iter().map(ref_from_ebnf).collect())
    }
}

pub fn map_difference((minuend, subtrahend): (EBNF, Option<EBNF>)) -> EBNF {
    if let Some(subtrahend) = subtrahend {
        EBNF::Difference(EBNFDifference {
            minuend: ref_from_ebnf(minuend),
            subtrahend: ref_from_ebnf(subtrahend),
        })
    } else {
        minuend
    }
}

pub fn map_negation((negation, ebnf): (Option<char>, EBNF)) -> EBNF {
    if negation.is_some() {
        EBNF::Not(ref_from_ebnf(ebnf))
    } else {
        ebnf
    }
}

pub fn map_optional(ebnf: EBNF) -> EBNF {
    EBNF::Repeat(EBNFRepeat {
        min: 0,
        max: Some(1),
        expr: ref_from_ebnf(ebnf),
        separator: None,
    })
}

pub fn map_repeated(
    ((range, ebnf), separator): ((Option<(usize, Option<usize>)>, EBNF), Option<EBNF>),
) -> EBNF {
    let (min, max) = range.unwrap_or((0, None));
    EBNF::Repeat(EBNFRepeat {
        min,
        max,
        expr: ref_from_ebnf(ebnf),
        separator: separator.map(|ebnf| ref_from_ebnf(ebnf)),
    })
}

pub fn map_production_reference((name, _): (String, bool)) -> EBNF {
    EBNF::Reference(name)
}

pub fn map_hex_digits_to_char(digits: Vec<char>) -> Result<char, ()> {
    let digits = digits.iter().collect::<String>();
    char::from_u32(u32::from_str_radix(digits.as_str(), 16).unwrap()).ok_or(())
}

pub fn map_repetition_prefix_1(
    (from, to): (usize, Option<Option<usize>>),
) -> (usize, Option<usize>) {
    (from, to.flatten())
}

pub fn map_repetition_prefix_2(to: usize) -> (usize, Option<usize>) {
    (0, Some(to))
}

pub fn map_number(digits: Vec<char>) -> usize {
    let digits = digits.iter().collect::<String>();
    usize::from_str_radix(digits.as_str(), 10).unwrap()
}

pub fn map_identifier_1(string: String) -> (String, bool) {
    (string, true)
}

pub fn map_identifier_2(string: String) -> (String, bool) {
    (string, false)
}
