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
    pub no_default_map: bool,
    pub box_non_tokens: bool,
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
                "{}\n{}\n{}",
                vec![
                    "use chumsky::{{prelude::*, Parser}};",
                    "",
                    "pub type ErrorType = Simple<char>;"
                ]
                .join("\n"),
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
                    if context.box_non_tokens && !production.is_token {
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
            pub fn #function_name() -> impl Parser<char, #result_type_name, Error = ErrorType> {
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

    fn as_character_predicate(&self, negated: bool) -> Option<TokenStream>;

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
                // TODO: generalise to more than char sets
                if let Some(predicate) = expr.as_character_predicate(true) {
                    return quote!( filter(|&c: &char| #predicate) );
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
                let predicates = exprs
                    .iter()
                    .filter_map(|e| e.as_character_predicate(false))
                    .collect::<Vec<_>>();
                if predicates.len() == exprs.len() {
                    return quote!( filter(|&c: &char| #(#predicates)||*) );
                }

                // If the choice is between terminals, generate them all as strings
                // TODO: optimize using a prefix-tree search custom predicate
                let mut strings = exprs
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
                strings.sort_by_cached_key(|s| usize::MAX - s.chars().count());
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
                            if production.single_expression().unwrap().config.ignore {
                                suffixes.push(quote!( .ignored() ))
                            }
                            quote!( #expr #(#suffixes)* )
                        })
                        .collect()
                };
                // The choice combinator has a limit on the number of elements in the tuple
                if choices.len() > 16 {
                    choices = choices
                        .chunks(16)
                        .map(|chunk| quote!( choice::<_, ErrorType>((#(#chunk),*)) ))
                        .collect();
                }
                quote!( choice::<_, ErrorType>((#(#choices),*)) )
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
                minuend,
                subtrahend,
            }) => {
                // 1. char set - char set
                if let Some(minuend_predicate) = minuend.as_character_predicate(false) {
                    if let Some(subtrahend_predicate) = subtrahend.as_character_predicate(true) {
                        return quote!( filter(|&c: &char| #minuend_predicate && #subtrahend_predicate) );
                    }
                }

                // 2. x - y assuming x produces Vec<char>
                let minuend = minuend.generate(grammar, production, context);
                let subtrahend = subtrahend.generate(grammar, production, context);
                quote!( #minuend.excluding(#subtrahend) )
            }
            EBNF::Range(_) => {
                let predicate = self.as_character_predicate(false);
                quote!( filter(|&c: &char| #predicate ))
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
                    if !context.no_default_map {
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

    fn as_character_predicate(&self, negated: bool) -> Option<TokenStream> {
        match &self.ebnf {
            EBNF::Choice(exprs) => {
                let elements = exprs
                    .iter()
                    .map(|e| e.as_character_predicate(negated))
                    .collect::<Vec<_>>();
                if elements.iter().all(|e| e.is_some()) {
                    if negated {
                        Some(quote!( (#(#elements)&&*) ))
                    } else {
                        Some(quote!( (#(#elements)||*) ))
                    }
                } else {
                    None
                }
            }
            EBNF::Range(EBNFRange { from: 'a', to: 'z' }) => {
                if negated {
                    Some(quote!(!c.is_ascii_lowercase()))
                } else {
                    Some(quote!(c.is_ascii_lowercase()))
                }
            }
            EBNF::Range(EBNFRange { from: 'A', to: 'Z' }) => {
                if negated {
                    Some(quote!(!c.is_ascii_uppercase()))
                } else {
                    Some(quote!(c.is_ascii_uppercase()))
                }
            }
            EBNF::Range(EBNFRange { from: '0', to: '9' }) => {
                if negated {
                    Some(quote!(!c.is_ascii_digit()))
                } else {
                    Some(quote!(c.is_ascii_digit()))
                }
            }
            EBNF::Range(EBNFRange { from, to }) => {
                if negated {
                    Some(quote!( (c < #from || #to < c) ))
                } else {
                    Some(quote!( (#from <= c && c <= #to) ))
                }
            }
            EBNF::Terminal(string) => {
                if string.len() == 1 {
                    let c = string.chars().next().unwrap();
                    if negated {
                        Some(quote!( c != #c ))
                    } else {
                        Some(quote!( c == #c ))
                    }
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
