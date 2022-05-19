use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::*;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum CharSetElement {
    Char(char),
    Range(char, char),
}

pub fn generate(grammar: &Grammar, _output_path: &PathBuf) {
    // TODO: Oh, obviously not.
    // We need some config so say what productions to generate
    // parse entry points for.
    let root = grammar
        .productions
        .iter()
        .flat_map(|(_, p)| p)
        .next()
        .unwrap()
        .name
        .clone();

    let productions_by_name = grammar
        .productions
        .iter()
        .flat_map(|(_, p)| p)
        .filter(|p| p.expr.is_some() || !p.versions.is_empty())
        .map(|p| {
            (
                p.name.clone(),
                // TODO: work out how we are going to generate
                // parsers over versions
                p.expr
                    .clone()
                    .unwrap_or_else(|| p.versions.iter().last().unwrap().1.clone()),
            )
        })
        .collect::<HashMap<_, _>>();

    // DFS search for a topological ordering, with backlinks ignored

    fn visit_production(
        productions_by_name: &HashMap<String, ExpressionRef>,
        name: &String,
        ordering: &mut BTreeMap<String, usize>,
    ) -> usize {
        let mut order = 0;
        ordering.insert(name.clone(), 0);
        if None == productions_by_name.get(name) {
            println!("Couldn't find production: {}", name);
        }
        for child in productions_by_name[name].referenced_identifiers(&mut Default::default()) {
            let child_order = if let Some(child_order) = ordering.get(&child) {
                *child_order
            } else {
                visit_production(productions_by_name, &child, ordering)
            };
            if child_order > order {
                order = child_order;
            }
        }
        order += 1;
        ordering.insert(name.clone(), order);
        order
    }

    let mut ordering = BTreeMap::new();
    visit_production(&productions_by_name, &root, &mut ordering);

    let mut decls = vec![];

    // Detect and declare each backlinked (recursively-referenced) production

    let mut backlinked = HashSet::new();
    for (name, order) in &ordering {
        for child in productions_by_name[name].referenced_identifiers(&mut Default::default()) {
            if ordering[&child] >= *order {
                backlinked.insert(child.clone());
            }
        }
    }
    for name in &backlinked {
        let id = format_ident!("{}_parser", name.to_case(Case::Snake));
        decls.push(quote!( let mut #id = Recursive::declare(); ));
    }

    // Define each production

    let mut ordered_productions = ordering.keys().cloned().collect::<Vec<String>>();
    ordered_productions.sort_by(|a, b| (&ordering[a]).cmp(&ordering[b]));
    for name in ordered_productions {
        let id = format_ident!("{}_parser", name.to_case(Case::Snake));
        let e = &productions_by_name[&name];
        let expr = e.generate();
        let suffixes = e.generate_expression_suffixes(Some(name.to_case(Case::Snake).to_owned()));
        if backlinked.contains(&name) {
            decls.push(quote!( #id.define(#expr #(#suffixes)*); ));
        } else {
            decls.push(quote!( let #id = #expr #(#suffixes)*; ));
        }
    }

    // Create the parser function

    let root_id = format_ident!("{}_parser", root.to_case(Case::Snake));
    let function_name = format_ident!("create_{}_parser", root.to_case(Case::Snake));
    let result_type_name = format_ident!("{}ParserResultType", root.to_case(Case::UpperCamel));
    let code = quote!(
        pub fn #function_name() -> impl Parser<char, #result_type_name, Error = Simple<char>> {
            #(#decls)*
            #root_id.recover_with(skip_then_retry_until([]))
        }
    );

    fs::write(_output_path, rustfmt(code.to_string())).expect("Unable to write to parser file");
}

impl Expression {
    fn generate_expression_suffixes(&self, default_map: Option<String>) -> Vec<TokenStream> {
        let mut suffixes = vec![];

        if let Some(expr) = &self.config.lookahead {
            let lookahead = expr.generate();
            suffixes.push(quote!( .then_ignore(#lookahead.rewind()) ))
        }

        if self.config.ignore {
            suffixes.push(quote!( .ignored() ))
        } else {
            if !self.config.nomap {
                if let Some(map) = &self.config.map {
                    let id = format_ident!("map_{}", map);
                    suffixes.push(quote!( .map(#id) ))
                } else if let Some(map) = default_map {
                    let id = format_ident!("map_{}", map);
                    suffixes.push(quote!( .map(#id) ))
                }
            }
            if self.config.unwrap {
                suffixes.push(quote!( .unwrapped() ))
            }
        }

        suffixes
    }

    fn is_ignorable_in_sequence(&self) -> bool {
        match &self.ebnf {
            EBNF::End { .. } | EBNF::Terminal { .. } => true,
            // Expression::Identifier { name, .. } => config.production(name).ignore(),
            _ => false,
        }
    }

    fn generate(&self) -> TokenStream {
        match &self.ebnf {
            EBNF::End => quote!(end()),
            EBNF::Not(expr) => {
                if let Some(char_set_elements) = expr.as_char_set_elements() {
                    return generate_char_set(&char_set_elements, true);
                }
                quote!(todo())
            }
            EBNF::Repeat(EBNFRepeat {
                min: 0,
                max: None,
                expr,
                separator,
            }) => {
                let expr = expr.generate();
                if let Some(separator) = separator {
                    let separator = separator.generate();
                    quote!( #expr.separated_by(#separator) )
                } else {
                    quote!( #expr.repeated() )
                }
            }
            EBNF::Repeat(EBNFRepeat {
                min,
                max: None,
                expr,
                separator,
            }) => {
                let expr = expr.generate();
                if let Some(separator) = separator {
                    let separator = separator.generate();
                    quote!( #expr.separated_by(#separator).at_least(#min) )
                } else {
                    quote!( #expr.repeated().at_least(#min) )
                }
            }
            EBNF::Repeat(EBNFRepeat {
                min: 0,
                max: Some(1),
                expr,
                ..
            }) => {
                let expr = expr.generate();
                quote!( #expr.or_not() )
            }
            EBNF::Repeat(EBNFRepeat {
                min: 0,
                max: Some(_max),
                ..
            }) => {
                // TODO: handle max
                todo!("Handle upper bound on repeat")
            }
            EBNF::Repeat(EBNFRepeat {
                min: _min,
                max: Some(_max),
                ..
            }) => {
                todo!("Handle upper bound on repeat")
            }
            EBNF::Choice(exprs) => {
                let choices = exprs
                    .iter()
                    .map(|e| {
                        let expr = e.generate();
                        let suffixes = e.generate_expression_suffixes(None);
                        quote!( #expr #(#suffixes)* )
                    })
                    .collect::<Vec<_>>();
                quote!( choice((#(#choices),*)) )
            }
            EBNF::Sequence(exprs) => {
                let mut seen_unignorable_content = false;
                let chain = self.config.chain;
                let exprs = exprs
                    .iter()
                    .enumerate()
                    .map(|(i, e)| {
                        let expr = e.generate();
                        let suffixes = e.generate_expression_suffixes(None);
                        if i > 0 {
                            if !seen_unignorable_content && exprs[i - 1].is_ignorable_in_sequence()
                            {
                                quote!( .ignore_then(#expr #(#suffixes)*) )
                            } else if e.is_ignorable_in_sequence() {
                                seen_unignorable_content = true;
                                quote!( .then_ignore(#expr #(#suffixes)*) )
                            } else {
                                seen_unignorable_content = true;
                                if chain {
                                    quote!( .chain(#expr #(#suffixes)*) )
                                } else {
                                    quote!( .then(#expr #(#suffixes)*) )
                                }
                            }
                        } else {
                            seen_unignorable_content = !e.is_ignorable_in_sequence();
                            quote!( #expr #(#suffixes)* )
                        }
                    })
                    .collect::<Vec<_>>();
                quote!( #(#exprs)* )
            }
            EBNF::Terminal(string) => {
                if string.len() == 1 {
                    let c = string.chars().next().unwrap();
                    quote!( just(#c) )
                } else {
                    quote!( just(#string) )
                }
            }
            EBNF::Reference(name) => {
                let id = format_ident!("{}_parser", name.to_case(Case::Snake));
                quote!( #id.clone() )
            }
            EBNF::Difference(EBNFDifference {
                minuend: _minuend,
                subtrahend: _subtrahend,
            }) => {
                // if minuend.is_any() {
                //     if let Some(char_set_elements) = subtrahend.as_char_set_elements() {
                //         return generate_char_set(&char_set_elements, true);
                //     }
                // }
                quote!(todo())
            }
            EBNF::Range(EBNFRange { from, to }) => {
                let fragment = generate_char_set(&vec![CharSetElement::Range(*from, *to)], false);
                quote!( #fragment )
            }
        }
    }

    fn referenced_identifiers(&self, accum: &mut HashSet<String>) -> HashSet<String> {
        match &self.ebnf {
            EBNF::Choice(exprs) | EBNF::Sequence(exprs) => {
                exprs.iter().for_each(|p| {
                    p.referenced_identifiers(accum);
                });
            }
            EBNF::Not(expr) => {
                expr.referenced_identifiers(accum);
            }
            EBNF::Repeat(EBNFRepeat {
                expr, separator, ..
            }) => {
                expr.referenced_identifiers(accum);
                if let Some(separator) = separator {
                    separator.referenced_identifiers(accum);
                }
            }
            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                minuend.referenced_identifiers(accum);
                subtrahend.referenced_identifiers(accum);
            }
            EBNF::Reference(name) => {
                accum.insert(name.clone());
            }
            EBNF::End | EBNF::Terminal(_) | EBNF::Range(_) => (),
        };

        accum.clone()
    }

    fn as_char_set_elements(&self) -> Option<Vec<CharSetElement>> {
        match &self.ebnf {
            EBNF::Choice(exprs) => {
                let elements = exprs
                    .iter()
                    .map(|e| e.as_char_set_elements())
                    .collect::<Vec<_>>();
                if elements.iter().all(|e| e.is_some()) {
                    Some(elements.into_iter().map(|e| e.unwrap()).flatten().collect())
                } else {
                    None
                }
            }
            EBNF::Terminal(string) => {
                if string.len() == 1 {
                    Some(vec![CharSetElement::Char(string.chars().next().unwrap())])
                } else {
                    None
                }
            }
            _ => None,
        }
    }
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
            quote!( filter(|&c: &char| c != #c) )
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
            quote!( filter(|&c: &char| !(#(#chars)||*)) )
        } else {
            quote!( filter(|&c: &char| #(#chars)||*) )
        }
    }
}

fn rustfmt(source: String) -> String {
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

    String::from_utf8(output.stdout).unwrap()
}
