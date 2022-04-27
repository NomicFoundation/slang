use std::{
    collections::{HashMap, HashSet},
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use yaml_rust::Yaml;

use super::tree_builder::*;

pub fn generate(productions: TreeType, annotations: &Yaml) {
    let productions = productions
        .into_iter()
        .map(|p| Production {
            name: p.name,
            expr: p.expr.rewrite_tree(),
        })
        .collect::<Vec<_>>();

    let productions_annotations = &annotations["productions"];

    let rules = productions
        .iter()
        .rev()
        .map(|p| {
            let id = Expression::ident_for_name(&p.name);
            let expression = p
                .expr
                .generate_expression(p.name.clone(), productions_annotations);
            let mut suffixes = vec![];
            let ignored = productions_annotations[p.name.as_str()]["ignore"]
                .as_bool()
                .unwrap_or(false);

            if ignored {
                suffixes.push(quote!(.ignored()));
            } else {
                let map = &productions_annotations[p.name.as_str()]["map"];
                if map.is_badvalue() {
                    let id = format_ident!("map_{}", p.name.to_case(Case::Snake));
                    suffixes.push(quote!(.map(#id)));
                    // TODO: allow map key to be a string with a custom map name
                }
                let unwrap = &productions_annotations[p.name.as_str()]["unwrap"];
                if !unwrap.is_badvalue() {
                    suffixes.push(quote!(.unwrapped()));
                }
            }

            quote!(
                let #id = #expression #(#suffixes)*;
            )
        })
        .collect::<Vec<_>>();

    let mut reference_graph_edges: HashMap<String, HashSet<String>> = HashMap::new();
    productions.iter().for_each(|p| {
        let mut identifiers = HashSet::new();
        p.expr.collect_identifiers_into(&mut identifiers);
        reference_graph_edges.insert(p.name.clone(), identifiers);
    });

    let root = Expression::ident_for_name(&productions[0].name);
    // let mapping_use = format_ident!("{}", annotations["mapping"]["use"].as_str().unwrap());
    let src = quote!(
    use chumsky::prelude::*;
    use super::tree_builder::*;

    pub fn create_parser() -> impl Parser<char, TreeType, Error = Simple<char>> {
        #(#rules)*
        #root.then_ignore(end().recover_with(skip_then_retry_until([])))
    });

    println!("{}", rustfmt(src.to_string()).unwrap());

    // println!("{:#?}", productions);
}

fn rustfmt(source: String) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(source.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");

    Ok(String::from_utf8(output.stdout)?)
}

// TODO: let this be specified by the grammar annotation
static DELIMITERS: [(char, char); 5] =
    [('[', ']'), ('(', ')'), ('{', '}'), ('\'', '\''), ('"', '"')];

impl Expression {
    fn generate_expression(&self, config_path: String, config: &Yaml) -> TokenStream {
        let mut suffixes = vec![];
        if let Some(name) = config[config_path.as_str()]["map"].as_str() {
            let id = format_ident!("{}", name);
            suffixes.push(quote!( .map(#id) ))
        }
        match self {
            Expression::End => quote!(todo()),
            Expression::Any => quote!(todo()),
            Expression::Repeat(expr, RepeatCount::ZeroOrOne) => {
                let expr = expr.generate_expression(config_path, config);
                quote!( #expr.or_not() #(#suffixes)* )
            }
            Expression::Repeat(expr, RepeatCount::ZeroOrMore) => {
                let expr = expr.generate_expression(config_path, config);
                quote!( #expr.repeated() #(#suffixes)* )
            }
            Expression::Repeat(expr, RepeatCount::OneOrMore) => {
                let expr = expr.generate_expression(config_path, config);
                quote!( #expr.repeated().at_least(1) #(#suffixes)* )
            }
            Expression::Choice(choices) => {
                let choices = choices
                    .iter()
                    .enumerate()
                    .map(|(i, e)| e.generate_expression(format!("{}/{}", config_path, i), config))
                    .collect::<Vec<_>>();
                quote!( choice((#(#choices),*)) #(#suffixes)* )
            }
            Expression::Sequence(exprs) => {
                let mut exprs = exprs
                    .iter()
                    .enumerate()
                    .map(|(i, e)| e.generate_expression(format!("{}/{}", config_path, i), config))
                    .collect::<Vec<_>>();
                let first = exprs.remove(0);
                quote!( #first #(.then(#exprs))* #(#suffixes)* )
            }
            Expression::DelimitedBy(expr, open, close) => {
                let expr = expr.generate_expression(config_path, config);
                quote!( #expr.delimited_by(just(#open), just(#close)) #(#suffixes)* )
            }
            Expression::PaddedBy(expr, padding) => {
                let expr = expr.generate_expression(config_path.clone(), config);
                let padding = padding.generate_expression(config_path, config);
                quote!( #expr.padded_by(#padding) #(#suffixes)* )
            }
            Expression::SeparatedBy(expr, separator) => {
                let expr = expr.generate_expression(config_path.clone(), config);
                let padding = separator.generate_expression(config_path, config);
                quote!( #expr.separated_by(#padding) #(#suffixes)* )
            }
            Expression::Difference(_, _) => quote!(todo()),
            Expression::Chars(chars) => {
                if chars.len() == 1 {
                    let c = chars.chars().next().unwrap();
                    quote!( just(#c) #(#suffixes)* )
                } else {
                    quote!( just(#chars) #(#suffixes)* )
                }
            }
            Expression::Identifier(name) => {
                let id = Self::ident_for_name(name);
                quote!( #id #(#suffixes)* )
            }
            Expression::CharSet(elements, negated) => {
                let fragment = Self::generate_char_set(elements, *negated);
                quote!( #fragment #(#suffixes)* )
            }
        }
    }

    fn collect_identifiers_into(&self, identifiers: &mut HashSet<String>) {
        match self {
            Expression::Choice(exprs) | Expression::Sequence(exprs) => {
                exprs
                    .iter()
                    .for_each(|p| p.collect_identifiers_into(identifiers));
            }
            Expression::Repeat(expr, _)
            | Expression::DelimitedBy(expr, _, _)
            | Expression::PaddedBy(expr, _)
            | Expression::SeparatedBy(expr, _) => {
                expr.collect_identifiers_into(identifiers);
            }
            Expression::Difference(expr1, expr2) => {
                expr1.collect_identifiers_into(identifiers);
                expr2.collect_identifiers_into(identifiers);
            }
            Expression::Identifier(name) => {
                identifiers.insert(name.clone());
            }
            _ => (),
        };
    }

    fn rewrite_tree(self) -> Expression {
        fn convert_to_padded_by(exprs: &mut Vec<Expression>) {
            let mut i = 0;
            while i + 2 < exprs.len() {
                if exprs[i] == exprs[i + 2] {
                    let _second = exprs.remove(i + 2);
                    let inner = exprs.remove(i + 1);
                    let first = exprs.remove(i);
                    exprs.insert(i, Expression::PaddedBy(Box::new(inner), Box::new(first)));
                }
                i += 1;
            }
        }

        fn convert_to_delimited_by(exprs: &mut Vec<Expression>) {
            // TODO: convert to a from-each-end narrowing search?
            fn match_delimiters(exprs: &[Expression]) -> Option<(char, char)> {
                match (&exprs[0], &exprs[exprs.len() - 1]) {
                    (Expression::Chars(open), Expression::Chars(close))
                        if open.len() == 1 && close.len() == 1 =>
                    {
                        Some((open.chars().next().unwrap(), close.chars().next().unwrap()))
                    }
                    _ => None,
                }
            }

            if exprs.len() > 2 {
                if let Some(pair) = match_delimiters(exprs) {
                    if DELIMITERS.contains(&pair) {
                        let new_exprs = exprs[1..exprs.len() - 1].to_vec();
                        exprs.clear();
                        exprs.push(Expression::DelimitedBy(
                            Box::new(Expression::Sequence(new_exprs).rewrite_tree()),
                            pair.0,
                            pair.1,
                        ));
                    }
                }
            }
        }

        fn convert_to_separated_by(exprs: &mut Vec<Expression>) {
            // TODO: find a better way to pattern match on the tree
            fn matches_pattern(exprs: &[Expression], i: usize) -> bool {
                let first = &exprs[i];
                let second = &exprs[i + 1];
                if let Expression::Repeat(inner, RepeatCount::ZeroOrMore) = second {
                    if let Expression::Sequence(inner_exprs) = inner.as_ref() {
                        return inner_exprs.len() == 2 && &inner_exprs[1] == first;
                    }
                }
                false
            }

            fn transform_pattern(first: Expression, second: Expression) -> Expression {
                if let Expression::Repeat(x, RepeatCount::ZeroOrMore) = second {
                    if let Expression::Sequence(mut inner) = *x {
                        let separator = inner.remove(0);
                        return Expression::SeparatedBy(Box::new(first), Box::new(separator));
                    }
                }
                unreachable!("Pattern already detected")
            }

            let mut i = 0;
            while i + 1 < exprs.len() {
                if matches_pattern(exprs, i) {
                    let second = exprs.remove(i + 1);
                    let first = exprs.remove(i);
                    exprs.insert(i, transform_pattern(first, second));
                }
                i += 1
            }
        }

        match self {
            Expression::Sequence(exprs) => {
                let mut exprs = exprs
                    .into_iter()
                    .map(|e| e.rewrite_tree())
                    .collect::<Vec<_>>();

                convert_to_delimited_by(&mut exprs);
                convert_to_padded_by(&mut exprs);
                convert_to_separated_by(&mut exprs);

                if exprs.len() == 1 {
                    exprs.remove(0)
                } else {
                    Expression::Sequence(exprs)
                }
            }
            Expression::Repeat(expr, count) => {
                Expression::Repeat(Box::new(expr.rewrite_tree()), count)
            }
            Expression::Choice(exprs) => {
                Expression::Choice(exprs.into_iter().map(|e| e.rewrite_tree()).collect())
            }
            Expression::DelimitedBy(expr, open, close) => {
                Expression::DelimitedBy(Box::new(expr.rewrite_tree()), open, close)
            }
            Expression::PaddedBy(expr, padding) => Expression::PaddedBy(
                Box::new(expr.rewrite_tree()),
                Box::new(padding.rewrite_tree()),
            ),
            Expression::SeparatedBy(expr, separator) => Expression::SeparatedBy(
                Box::new(expr.rewrite_tree()),
                Box::new(separator.rewrite_tree()),
            ),
            Expression::Difference(expr1, expr2) => Expression::Difference(
                Box::new(expr1.rewrite_tree()),
                Box::new(expr2.rewrite_tree()),
            ),
            _ => self,
        }
    }

    fn ident_for_name(name: &String) -> Ident {
        format_ident!("{}", name.to_case(Case::Snake))
    }

    fn generate_char_set(elements: &[CharSetElement], negated: bool) -> TokenStream {
        let chars = elements
            .iter()
            .filter_map(|element| match element {
                CharSetElement::Char(c) => Some(*c),
                _ => None,
            })
            .collect::<Vec<char>>();
        if chars.len() == elements.len() {
            if negated && chars.len() == 1 {
                let c = chars[0];
                quote!( filter(|c| c != #c) )
            } else {
                let string = chars.into_iter().collect::<String>();
                if negated {
                    quote!( none_of(#string) )
                } else {
                    quote!( one_of(#string) )
                }
            }
        } else {
            fn filter_range(
                from: char,
                to: char,
                elements: &[CharSetElement],
            ) -> (Vec<CharSetElement>, bool) {
                let old_len = elements.len();
                let elements = elements
                .iter()
                .filter(|element| ! matches!(element, CharSetElement::Range(f, t) if *f == from && *t == to))
                .cloned()
                .collect::<Vec<_>>();
                let found = elements.len() < old_len;
                (elements, found)
            }

            let (elements, mut has_ascii_lowercase) = filter_range('a', 'z', elements);
            let (elements, mut has_ascii_uppercase) = filter_range('A', 'Z', &elements);
            let (elements, mut has_ascii_digit) = filter_range('0', '9', &elements);
            let (elements, mut has_ascii_lowercase_hex) = filter_range('a', 'f', &elements);
            let (elements, mut has_ascii_uppercase_hex) = filter_range('A', 'F', &elements);

            let mut chars = elements
                .iter()
                .map(|element| match element {
                    CharSetElement::Char(c) => quote!( c == #c ),
                    CharSetElement::Range('a', 'z') => quote!(c.is_ascii_lowercase()),
                    CharSetElement::Range('A', 'Z') => quote!(c.is_ascii_uppercase()),
                    CharSetElement::Range('0', '9') => quote!(c.is_ascii_digit()),
                    CharSetElement::Range(from, to) => {
                        quote!( (#from <= c && c <= #to) )
                    }
                })
                .collect::<Vec<_>>();

            if has_ascii_uppercase && has_ascii_lowercase && has_ascii_digit {
                chars.push(quote!(c.is_ascii_alphanumeric()));
                has_ascii_uppercase = false;
                has_ascii_lowercase = false;
                has_ascii_digit = false;
            }

            if has_ascii_uppercase && has_ascii_lowercase {
                chars.push(quote!(c.is_ascii_alphabetic()));
                has_ascii_uppercase = false;
                has_ascii_lowercase = false;
            }
            if has_ascii_lowercase_hex && has_ascii_uppercase_hex && has_ascii_digit {
                chars.push(quote!(c.is_ascii_hexdigit()));
                has_ascii_lowercase_hex = false;
                has_ascii_uppercase_hex = false;
                has_ascii_digit = false;
            }
            if has_ascii_uppercase {
                chars.push(quote!(c.is_ascii_uppercase()));
            }
            if has_ascii_lowercase {
                chars.push(quote!(c.is_ascii_lowercase()));
            }
            if has_ascii_digit {
                chars.push(quote!(c.is_ascii_digit()));
            }
            if has_ascii_uppercase_hex {
                chars.push(quote!(('A' <= c && c <= 'F')));
            }
            if has_ascii_lowercase_hex {
                chars.push(quote!(('a' <= c && c <= 'f')));
            }

            if negated {
                quote!( filter(|c| !(#(#chars)||*)) )
            } else {
                quote!( filter(|c| #(#chars)||*) )
            }
        }
    }
}
