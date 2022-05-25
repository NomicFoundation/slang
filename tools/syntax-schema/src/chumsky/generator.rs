use std::{
    collections::{BTreeMap, BTreeSet},
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

pub struct Context {
    pub no_builder: bool,
    pub boxed: bool,
}

impl Grammar {
    pub fn generate_chumsky(&self, context: &Context, output_path: &PathBuf) {
        let mut preludes = vec![];
        let mut parsers = vec![];
        self.productions.iter().flat_map(|(_, p)| p).for_each(|p| {
            if let Some(expr) = p.single_expression() {
                if let Some(prelude) = expr.config.prelude.clone() {
                    preludes.push(prelude);
                    parsers.push(p.generate(self, context).to_string())
                }
            }
        });

        fs::write(
            output_path,
            rustfmt(format!(
                "use chumsky::{{prelude::*, Parser}};\n\n{}\n\n{}",
                preludes.join("\n\n"),
                parsers.join("\n\n")
            )),
        )
        .expect("Unable to write to parser file");
    }
}

trait ChumskyProduction {
    fn generate(&self, grammar: &Grammar, context: &Context) -> TokenStream;
    fn single_expression(&self) -> Option<ExpressionRef>;
}

impl ChumskyProduction for Production {
    fn generate(&self, grammar: &Grammar, context: &Context) -> TokenStream {
        let root = self.name.clone();

        // DFS search for a topological ordering, with backlinks ignored

        fn visit_production(
            grammar: &Grammar,
            name: &String,
            ordering: &mut BTreeMap<String, usize>,
        ) -> usize {
            let mut order = 0;
            ordering.insert(name.clone(), 0);
            if let Some(production) = grammar.get_production(name) {
                if let Some(expr) = production.single_expression() {
                    for child in
                        expr.referenced_identifiers(grammar, production, &mut Default::default())
                    {
                        let child_order = if let Some(child_order) = ordering.get(&child) {
                            *child_order
                        } else {
                            visit_production(grammar, &child, ordering)
                        };
                        if child_order > order {
                            order = child_order;
                        }
                    }
                } else {
                    println!("Production {} has no expression(s)", name);
                }
            } else {
                println!("Couldn't find production: {}", name);
            }
            order += 1;
            ordering.insert(name.clone(), order);
            order
        }

        let mut ordering = BTreeMap::new();
        visit_production(grammar, &root, &mut ordering);

        let mut decls = vec![];

        // Detect and declare each backlinked (recursively-referenced) production

        let mut backlinked = BTreeSet::new();
        for (name, order) in &ordering {
            if let Some(production) = grammar.get_production(name) {
                if let Some(expr) = production.single_expression() {
                    for child in
                        expr.referenced_identifiers(grammar, production, &mut Default::default())
                    {
                        if ordering[&child] >= *order {
                            backlinked.insert(child.clone());
                        }
                    }
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
            if let Some(production) = grammar.get_production(&name) {
                if let Some(e) = production.single_expression() {
                    let expr = e.generate(grammar, production, context);
                    let mut suffixes = e.generate_suffixes(
                        grammar,
                        production,
                        Some(name.to_case(Case::Snake).to_owned()),
                        context,
                    );
                    if context.boxed && !production.is_token {
                        suffixes.push(quote!( .boxed() ))
                    }
                    if backlinked.contains(&name) {
                        decls.push(quote!( #id.define(#expr #(#suffixes)*); ));
                    } else {
                        decls.push(quote!( let #id = #expr #(#suffixes)*; ));
                    }
                }
            }
        }

        // Create the parser function

        let root_id = format_ident!("{}_parser", root.to_case(Case::Snake));
        let function_name = format_ident!("create_{}_parser", root.to_case(Case::Snake));
        let result_type_name = format_ident!("{}ParserResultType", root.to_case(Case::UpperCamel));
        quote!(
            pub fn #function_name() -> impl Parser<char, #result_type_name, Error = Simple<char>> {
                #(#decls)*
                #root_id.recover_with(skip_then_retry_until([]))
            }
        )
    }

    fn single_expression(&self) -> Option<ExpressionRef> {
        self.expr
            .clone()
            .or_else(|| self.versions.iter().last().map(|(_, e)| e.clone()))
    }
}

trait ChumskyExpression {
    fn generate(
        &self,
        grammar: &Grammar,
        production: &Production,
        context: &Context,
    ) -> TokenStream;

    fn generate_suffixes(
        &self,
        grammar: &Grammar,
        production: &Production,
        default_map: Option<String>,
        context: &Context,
    ) -> Vec<TokenStream>;

    fn as_char_set_elements(&self) -> Option<Vec<CharSetElement>>;

    fn is_ignorable_in_sequence(&self, grammar: &Grammar) -> bool;

    fn referenced_identifiers(
        &self,
        grammar: &Grammar,
        production: &Production,
        accum: &mut BTreeSet<String>,
    ) -> BTreeSet<String>;
}

impl ChumskyExpression for Expression {
    fn generate(
        &self,
        grammar: &Grammar,
        production: &Production,
        context: &Context,
    ) -> TokenStream {
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
                max: Some(1),
                expr,
                separator: None,
            }) => {
                let expr = expr.generate(grammar, production, context);
                quote!( #expr.or_not() )
            }
            EBNF::Repeat(EBNFRepeat {
                min,
                max,
                expr,
                separator,
            }) => {
                let expr = expr.generate(grammar, production, context);
                let separator = separator.clone().map_or_else(
                    || quote!( .repeated() ),
                    |s| {
                        let s = s.generate(grammar, production, context);
                        quote!( .separated_by(#s) )
                    },
                );
                let min = if *min == 0 {
                    quote!()
                } else {
                    quote!( .at_least(#min) )
                };
                let max = max.map_or_else(|| quote!(), |m| quote!( .at_most(#m) ));
                quote!( #expr #separator #min #max )
            }
            EBNF::Choice(exprs) => {
                // If the choice is between terminals, make sure they are all represented as strings
                let strings = exprs
                    .iter()
                    .filter_map(|e| {
                        if let EBNF::Terminal(s) = &e.ebnf {
                            if e.config.map.is_none() {
                                Some(s.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>();
                let mut choices: Vec<TokenStream> = if strings.len() == exprs.len()
                    && exprs.iter().all(|e| e.config.is_default())
                    && strings.iter().any(|s| s.chars().count() > 1)
                {
                    strings.iter().map(|s| quote!( just(#s) )).collect()
                } else {
                    exprs
                        .iter()
                        .map(|e| {
                            let expr = e.generate(grammar, production, context);
                            let mut suffixes =
                                e.generate_suffixes(grammar, production, None, context);
                            if context.no_builder {
                                suffixes.push(quote!( .ignored() ))
                            }
                            quote!( #expr #(#suffixes)* )
                        })
                        .collect()
                };
                if choices.len() > 16 {
                    choices = choices
                        .chunks(16)
                        .map(|chunk| quote!( choice::<_, Simple<char>>((#(#chunk),*)) ))
                        .collect();
                }
                quote!( choice::<_, Simple<char>>((#(#choices),*)) )
            }
            EBNF::Sequence(exprs) => {
                let mut seen_unignorable_content = false;
                let chain = self.config.chain;
                let exprs = exprs
                    .iter()
                    .enumerate()
                    .map(|(i, e)| {
                        let expr = e.generate(grammar, production, context);
                        let suffixes = e.generate_suffixes(grammar, production, None, context);
                        if i > 0 {
                            if !seen_unignorable_content
                                && exprs[i - 1].is_ignorable_in_sequence(grammar)
                            {
                                quote!( .ignore_then(#expr #(#suffixes)*) )
                            } else if e.is_ignorable_in_sequence(grammar) {
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
                            seen_unignorable_content = !e.is_ignorable_in_sequence(grammar);
                            quote!( #expr #(#suffixes)* )
                        }
                    })
                    .collect::<Vec<_>>();
                quote!( #(#exprs)* )
            }
            EBNF::Terminal(string) => {
                let ignore = if production.is_token {
                    quote!()
                } else {
                    quote!( .then_ignore(ignore_parser.clone()) )
                };
                if string.chars().count() == 1 {
                    let c = string.chars().next().unwrap();
                    quote!( just(#c) #ignore )
                } else {
                    quote!( just(#string) #ignore )
                }
            }
            EBNF::Reference(name) => {
                let ignore = if !production.is_token
                    && grammar
                        .get_production(name)
                        .map(|p| p.is_token)
                        .unwrap_or_default()
                {
                    quote!( .then_ignore(ignore_parser.clone()))
                } else {
                    quote!()
                };
                let id = format_ident!("{}_parser", name.to_case(Case::Snake));
                quote!( #id.clone() #ignore )
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
                quote!(just("todo()"))
            }
            EBNF::Range(EBNFRange { from, to }) => {
                let fragment = generate_char_set(&vec![CharSetElement::Range(*from, *to)], false);
                quote!( #fragment )
            }
        }
    }

    fn generate_suffixes(
        &self,
        grammar: &Grammar,
        production: &Production,
        default_map: Option<String>,
        context: &Context,
    ) -> Vec<TokenStream> {
        let mut suffixes = vec![];

        if let Some(expr) = &self.config.lookahead {
            let lookahead = expr.generate(grammar, production, context);
            suffixes.push(quote!( .then_ignore(#lookahead.rewind()) ))
        }

        if self.config.ignore {
            suffixes.push(quote!( .ignored() ))
        } else {
            if !self.config.nomap {
                if let Some(map) = &self.config.map {
                    let id = format_ident!("{}", map);
                    suffixes.push(quote!( .map(builder::#id) ))
                } else if let Some(map) = default_map {
                    if context.no_builder {
                        suffixes.push(quote!( .ignored() ))
                    } else {
                        let id = format_ident!("{}", map);
                        suffixes.push(quote!( .map(builder::#id) ))
                    }
                }
            }
            if self.config.unwrap {
                suffixes.push(quote!( .unwrapped() ))
            }
        }

        suffixes
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

    fn is_ignorable_in_sequence(&self, grammar: &Grammar) -> bool {
        match &self.ebnf {
            EBNF::End { .. } | EBNF::Terminal { .. } => true,
            EBNF::Reference(name) => grammar
                .get_production(name)
                .and_then(|p| p.single_expression())
                .map(|e| e.config.ignore)
                .unwrap_or_default(),
            _ => false,
        }
    }

    fn referenced_identifiers(
        &self,
        grammar: &Grammar,
        production: &Production,
        accum: &mut BTreeSet<String>,
    ) -> BTreeSet<String> {
        match &self.ebnf {
            EBNF::Choice(exprs) | EBNF::Sequence(exprs) => {
                exprs.iter().for_each(|p| {
                    p.referenced_identifiers(grammar, production, accum);
                });
            }
            EBNF::Not(expr) => {
                expr.referenced_identifiers(grammar, production, accum);
            }
            EBNF::Repeat(EBNFRepeat {
                expr, separator, ..
            }) => {
                expr.referenced_identifiers(grammar, production, accum);
                if let Some(separator) = separator {
                    separator.referenced_identifiers(grammar, production, accum);
                }
            }
            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                minuend.referenced_identifiers(grammar, production, accum);
                subtrahend.referenced_identifiers(grammar, production, accum);
            }
            EBNF::Terminal(_) => {
                if !production.is_token {
                    accum.insert("IGNORE".to_owned());
                }
            }
            EBNF::Reference(name) => {
                if !production.is_token
                    && grammar
                        .get_production(name)
                        .map(|p| p.is_token)
                        .unwrap_or_default()
                {
                    accum.insert("IGNORE".to_owned());
                }
                accum.insert(name.clone());
            }
            EBNF::End | EBNF::Range(_) => (),
        };

        accum.clone()
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
