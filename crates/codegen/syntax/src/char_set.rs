use std::{
    cmp::{max, min},
    ops::Range,
};

use codegen_schema::types::{Production, ScannerDefinition, ScannerRef};
use proc_macro2::TokenStream;
use quote::quote;

use crate::combinator_tree::CombinatorTree;

#[derive(Clone, Debug)]
pub struct CharSet(Vec<Range<u32>>);

impl CharSet {
    pub fn empty() -> Self {
        Self(vec![])
    }

    #[allow(dead_code)]
    pub fn less(char: char) -> Self {
        Self(vec![0..(char as u32)])
    }

    #[allow(dead_code)]
    pub fn greater_equal(char: char) -> Self {
        Self(vec![(char as u32)..u32::MAX])
    }

    pub fn single(char: char) -> Self {
        Self(vec![(char as u32)..(char as u32 + 1)])
    }

    pub fn range(start: char, end: char) -> Self {
        Self(vec![(start as u32)..(end as u32 + 1)])
    }

    pub fn not(self) -> Self {
        let ranges = self.0;
        let mut result = Vec::new();
        let mut last = 0;
        for range in ranges {
            if range.start > last {
                result.push(last..range.start);
            }
            last = range.end;
        }
        if last < u32::MAX {
            result.push(last..u32::MAX);
        }
        Self(result)
    }

    pub fn union(self, other: Self) -> Self {
        let mut ranges = self.0;
        let mut other_ranges = other.0;
        let mut result = Vec::new();
        while !ranges.is_empty() && !other_ranges.is_empty() {
            if ranges[0].end < other_ranges[0].start {
                result.push(ranges.remove(0));
            } else if other_ranges[0].end < ranges[0].start {
                result.push(other_ranges.remove(0));
            } else {
                let lower = min(ranges[0].start, other_ranges[0].start);
                let upper = max(ranges[0].end, other_ranges[0].end);
                ranges.remove(0);
                other_ranges.remove(0);
                result.push(lower..upper)
            }
        }
        result.extend(ranges.into_iter());
        result.extend(other_ranges.into_iter());
        Self(result)
    }

    pub fn intersection(self, other: Self) -> Self {
        self.not().union(other.not()).not()
    }

    pub fn single_char(&self) -> Option<char> {
        if self.0.len() == 1 {
            let range = &self.0[0];
            if range.start + 1 == range.end {
                return char::from_u32(range.start);
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_predicate(&self) -> TokenStream {
        if self.is_empty() {
            quote! { false }
        } else {
            if self.0[0].start == 0 && self.0[self.0.len() - 1].end == u32::MAX {
                let inverse = self.clone().not();
                let clauses = inverse.0.iter().map(|range| {
                    if range.start + 1 == range.end {
                        let start = unsafe { char::from_u32_unchecked(range.start) };
                        quote! { c != #start }
                    } else {
                        let start = unsafe { char::from_u32_unchecked(range.start) };
                        let end = unsafe { char::from_u32_unchecked(range.end - 1) };
                        quote! { (c < #start || #end < c) }
                    }
                });
                quote! { #(#clauses)&&* }
            } else {
                let clauses = self.0.iter().map(|range| {
                    if range.start == 0 {
                        let end = unsafe { char::from_u32_unchecked(range.end - 1) };
                        quote! { c <= #end }
                    } else if range.end == u32::MAX {
                        let start = unsafe { char::from_u32_unchecked(range.start) };
                        quote! { #start <= c }
                    } else if range.start + 1 == range.end {
                        let start = unsafe { char::from_u32_unchecked(range.start) };
                        quote! { c == #start }
                    } else {
                        let start = unsafe { char::from_u32_unchecked(range.start) };
                        let end = unsafe { char::from_u32_unchecked(range.end - 1) };
                        quote! { (#start <= c && c <= #end) }
                    }
                });
                quote! { #(#clauses)||* }
            }
        }
    }

    pub fn from_scanner(tree: &CombinatorTree<'_>, scanner: ScannerRef) -> Option<CharSet> {
        match &scanner.definition {
            ScannerDefinition::Not(child) => Self::from_scanner(tree, child.clone()).map(Self::not),

            ScannerDefinition::Choice(children) => {
                children.iter().fold(Some(Self::empty()), |acc, child| {
                    acc.and_then(|acc| {
                        Self::from_scanner(tree, child.clone()).map(|child| acc.union(child))
                    })
                })
            }

            ScannerDefinition::Terminal(string) => {
                if string.chars().count() == 1 {
                    Some(Self::single(string.chars().next().unwrap()))
                } else {
                    None
                }
            }

            ScannerDefinition::Range { from, to } => Some(Self::range(*from, *to)),

            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                if let (Some(minuend), Some(subtrahend)) = (
                    Self::from_scanner(tree, minuend.clone()),
                    Self::from_scanner(tree, subtrahend.clone()),
                ) {
                    Some(minuend.intersection(subtrahend.not()))
                } else {
                    None
                }
            }

            ScannerDefinition::Reference(name) => Self::from_scanner(
                tree,
                match tree.context.get_tree_by_name(name).production.as_ref() {
                    Production::Scanner { name, version_map } => {
                        version_map.get_for_version(&tree.context.version).expect(
                            &format!("Validation should have ensured: no version of {name} exists for version {version}", version = tree.context.version)
                        ).clone()
                    }
                    Production::TriviaParser { .. }
                    | Production::Parser { .. }
                    | Production::PrecedenceParser { .. } => {
                        unreachable!(
                            "Validation should have ensured: scanners can only reference scanners"
                        )
                    }
                },
            ),

            _ => None,
        }
    }
}

#[test]
fn test_negation() {
    let set = CharSet::range('a', 'z');
    let negated = set.not();
    assert!(negated.0 == vec![0..('a' as u32), ('z' as u32 + 1)..u32::MAX]);
}

#[test]
fn test_negation_with_lower_bound() {
    let set = CharSet::less('z');
    let negated = set.not();
    assert!(negated.0 == vec![('z' as u32)..u32::MAX]);
}

#[test]
fn test_negation_with_upper_bound() {
    let set = CharSet::greater_equal('a');
    let negated = set.not();
    assert!(negated.0 == vec![0..('a' as u32)]);
}

#[test]
fn test_double_negation() {
    let set = CharSet::range('a', 'z');
    let negated = set.not().not();
    assert!(negated.0 == vec![('a' as u32)..('z' as u32 + 1)]);
}

#[test]
fn test_union() {
    let set1 = CharSet::less('a');
    let set2 = CharSet::greater_equal('z');
    let set = set1.union(set2);
    assert!(set.0 == vec![0..('a' as u32), ('z' as u32)..u32::MAX]);
}

#[test]
fn test_empty_intersection() {
    let set1 = CharSet::less('a');
    let set2 = CharSet::greater_equal('z');
    let set = set1.intersection(set2);
    assert!(set.0 == vec![]);
}

#[test]
fn test_merging_union() {
    let set1 = CharSet::single('a');
    let set2 = CharSet::single('b');
    let set = set1.union(set2);
    assert!(set.0 == vec![('a' as u32)..('c' as u32)]);
}

#[test]
fn test_intersection() {
    let set1 = CharSet::less('a').not();
    let set2 = CharSet::greater_equal('z').not();
    let set = set1.intersection(set2);
    assert!(set.0 == vec![('a' as u32)..('z' as u32)]);
}
