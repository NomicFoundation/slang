use std::collections::{BTreeSet, HashSet};

use convert_case::{Case, Casing};
use patricia_tree::{node::Node, PatriciaSet};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::grammar::Context;

use crate::schema::*;

impl Expression {
    // pub(super) fn generate_chumsky(
    //     &self,
    //     grammar: &Grammar,
    //     production: &Production,
    //     context: &Context,
    // ) -> TokenStream {
    //     match &self.ebnf {
    //         EBNF::End => quote!(end()),
    //         EBNF::Not(expr) => {
    //             // TODO: generalise to more than char sets
    //             if let Some((predicate, _)) = expr.as_character_predicate(true) {
    //                 return quote!( filter(|&c: &char| #predicate) );
    //             }
    //             quote!(todo())
    //         }
    //         EBNF::Repeat(EBNFRepeat {
    //             min: 0,
    //             max: Some(1),
    //             expr,
    //             separator: None,
    //         }) => {
    //             let expr = expr.generate_chumsky(grammar, production, context);
    //             quote!( #expr.or_not() )
    //         }
    //         EBNF::Repeat(EBNFRepeat {
    //             min,
    //             max,
    //             expr,
    //             separator,
    //         }) => {
    //             let expr = expr.generate_chumsky(grammar, production, context);
    //             let separator = separator.clone().map_or_else(
    //                 || quote!( .repeated() ),
    //                 |s| {
    //                     let s = s.generate_chumsky(grammar, production, context);
    //                     quote!( .separated_by(#s) )
    //                 },
    //             );
    //             let minmax = match (min, max) {
    //                 (0, None) => quote!(),
    //                 (x, None) => quote!( .at_least(#x) ),
    //                 (0, Some(y)) => quote!( .at_most(#y) ),
    //                 (x, Some(y)) if x == y => quote!( .exactly(#x) ),
    //                 (x, Some(y)) => quote!( .at_least(#x).at_most(#y) ),
    //             };
    //             quote!( #expr #separator #minmax )
    //         }
    //         EBNF::Choice(exprs) => {
    //             if let Some((predicate, _)) = self.as_character_predicate(false) {
    //                 return quote!( filter(|&c: &char| #predicate) );
    //             }

    //             let mut choices: Vec<TokenStream> =
    //                 if let Some(terminals) = self.as_terminal_set(grammar) {
    //                     fn generate_from_trie(
    //                         node: Option<&Node<()>>,
    //                         depth: usize,
    //                     ) -> Vec<TokenStream> {
    //                         let mut result = vec![];
    //                         let mut n = node;
    //                         while let Some(node) = n {
    //                             let label = String::from_utf8_lossy(node.label());
    //                             let mut children = generate_from_trie(node.child(), depth + 1);
    //                             if node.child().is_some() && node.value().is_some() {
    //                                 children.push(quote!(empty()));
    //                             }
    //                             if children.is_empty() {
    //                                 result.push(quote!( just(#label).ignored() ))
    //                             } else if children.len() == 1 {
    //                                 let child = &children[0];
    //                                 result.push(quote!( just(#label).then(#child).ignored() ))
    //                             } else {
    //                                 result.push(quote!( just(#label).then(choice((
    //                                     #(#children),*
    //                                 ))).ignored() ))
    //                             }
    //                             n = node.sibling()
    //                         }
    //                         result
    //                     }

    //                     let trie: PatriciaSet = terminals.into_iter().collect();
    //                     generate_from_trie(trie.as_ref().child(), 0)
    //                 } else {
    //                     exprs
    //                         .iter()
    //                         .map(|e| {
    //                             let expr = e.generate_chumsky(grammar, production, context);
    //                             let mut suffixes =
    //                                 e.generate_suffixes(grammar, production, None, context);
    //                             if production.expression_to_generate().config.ignore {
    //                                 suffixes.push(quote!( .ignored() ))
    //                             }
    //                             quote!( #expr #(#suffixes)* )
    //                         })
    //                         .collect()
    //                 };

    //             // The choice combinator has a limit on the number of elements in the tuple
    //             if choices.len() > 16 {
    //                 choices = choices
    //                     .chunks(16)
    //                     .map(|chunk| {
    //                         if chunk.len() == 1 {
    //                             chunk[0].clone()
    //                         } else {
    //                             quote!( choice::<_, ErrorType>((#(#chunk),*)) )
    //                         }
    //                     })
    //                     .collect();
    //             }
    //             quote!( choice::<_, ErrorType>((#(#choices),*)) )
    //         }
    //         EBNF::Sequence(exprs) => {
    //             let mut seen_unignorable_content = false;
    //             let chain = self.config.chain;
    //             let exprs = exprs
    //                 .iter()
    //                 .enumerate()
    //                 .map(|(i, e)| {
    //                     let expr = e.generate_chumsky(grammar, production, context);
    //                     let suffixes = e.generate_suffixes(grammar, production, None, context);
    //                     if i > 0 {
    //                         if !seen_unignorable_content
    //                             && exprs[i - 1].is_ignorable_in_sequence(grammar)
    //                         {
    //                             quote!( .ignore_then(#expr #(#suffixes)*) )
    //                         } else if e.is_ignorable_in_sequence(grammar) {
    //                             seen_unignorable_content = true;
    //                             quote!( .then_ignore(#expr #(#suffixes)*) )
    //                         } else {
    //                             seen_unignorable_content = true;
    //                             if chain {
    //                                 quote!( .chain(#expr #(#suffixes)*) )
    //                             } else {
    //                                 quote!( .then(#expr #(#suffixes)*) )
    //                             }
    //                         }
    //                     } else {
    //                         seen_unignorable_content = !e.is_ignorable_in_sequence(grammar);
    //                         quote!( #expr #(#suffixes)* )
    //                     }
    //                 })
    //                 .collect::<Vec<_>>();
    //             quote!( #(#exprs)* )
    //         }
    //         EBNF::Terminal(string) => {
    //             let ignore = if production.is_token {
    //                 quote!()
    //             } else {
    //                 quote!( .then_ignore(ignore_parser.clone()) )
    //             };
    //             if string.chars().count() == 1 {
    //                 let c = string.chars().next().unwrap();
    //                 quote!( just(#c) #ignore )
    //             } else {
    //                 quote!( just(#string) #ignore )
    //             }
    //         }
    //         EBNF::Reference(name) => {
    //             let ignore = if !production.is_token
    //                 && grammar
    //                     .get_production(name)
    //                     .map(|p| p.is_token)
    //                     .unwrap_or_default()
    //             {
    //                 quote!( .then_ignore(ignore_parser.clone()))
    //             } else {
    //                 quote!()
    //             };
    //             let id = format_ident!("{}_parser", name.to_case(Case::Snake));
    //             quote!( #id.clone() #ignore )
    //         }
    //         EBNF::Difference(EBNFDifference {
    //             minuend,
    //             subtrahend,
    //         }) => {
    //             // 1. char set - char set
    //             if let Some((mut minuend_predicate, minuend_conjunction)) =
    //                 minuend.as_character_predicate(false)
    //             {
    //                 if minuend_conjunction == Some(false) {
    //                     minuend_predicate = quote!( (#minuend_predicate) )
    //                 }
    //                 if let Some((mut subtrahend_predicate, subtrahend_conjunction)) =
    //                     subtrahend.as_character_predicate(true)
    //                 {
    //                     if subtrahend_conjunction == Some(false) {
    //                         subtrahend_predicate = quote!( (#subtrahend_predicate) )
    //                     }
    //                     return quote!( filter(|&c: &char| #minuend_predicate && #subtrahend_predicate) );
    //                 }
    //             }

    //             // 2. x - y assuming x produces Vec<char>
    //             let minuend = minuend.generate_chumsky(grammar, production, context);
    //             let subtrahend = subtrahend.generate_chumsky(grammar, production, context);
    //             quote!( #minuend.excluding(#subtrahend) )
    //         }
    //         EBNF::Range(_) => {
    //             if let Some((predicate, _)) = self.as_character_predicate(false) {
    //                 quote!( filter(|&c: &char| #predicate ))
    //             } else {
    //                 unreachable!("Ranges produce a character predicate")
    //             }
    //         }
    //     }
    // }

    // pub(super) fn generate_suffixes(
    //     &self,
    //     grammar: &Grammar,
    //     production: &Production,
    //     default_map: Option<String>,
    //     context: &Context,
    // ) -> Vec<TokenStream> {
    //     let mut suffixes = vec![];

    //     if let Some(expr) = &self.config.lookahead {
    //         let lookahead = expr.generate_chumsky(grammar, production, context);
    //         suffixes.push(quote!( .then_ignore(#lookahead.rewind()) ))
    //     }

    //     if self.config.ignore {
    //         suffixes.push(quote!( .ignored() ))
    //     } else {
    //         if !self.config.nomap {
    //             if let Some(map) = &self.config.map {
    //                 let id = format_ident!("{}", map);
    //                 suffixes.push(quote!( .map(builder::#id) ))
    //             } else if let Some(map) = default_map {
    //                 if !context.no_default_map {
    //                     let id = format_ident!("{}", map);
    //                     suffixes.push(quote!( .map(builder::#id) ))
    //                 }
    //             }
    //         }
    //         if self.config.unwrap {
    //             suffixes.push(quote!( .unwrapped() ))
    //         }
    //     }

    //     suffixes
    // }

    // // Returned Option<bool> indicates if the expression is:
    // //   Some(true)  => conjunction
    // //   Some(false) => disjunction
    // //   None        => simple expression
    // // which allows callers to add parens iff required
    // fn as_character_predicate(&self, negated: bool) -> Option<(TokenStream, Option<bool>)> {
    //     match &self.ebnf {
    //         EBNF::Choice(exprs) => {
    //             let elements = exprs
    //                 .iter()
    //                 .map(|e| e.as_character_predicate(negated))
    //                 .collect::<Vec<_>>();
    //             if elements.iter().all(|e| e.is_some()) {
    //                 let elements = elements.into_iter().map(|c| c.unwrap()).map(|(p, c)| {
    //                     if c == Some(!negated) {
    //                         quote!( (#p) )
    //                     } else {
    //                         p
    //                     }
    //                 });
    //                 if negated {
    //                     Some((quote!( #(#elements)&&* ), Some(true)))
    //                 } else {
    //                     Some((quote!( #(#elements)||* ), Some(false)))
    //                 }
    //             } else {
    //                 None
    //             }
    //         }
    //         EBNF::Range(EBNFRange { from: 'a', to: 'z' }) => {
    //             if negated {
    //                 Some((quote!(!c.is_ascii_lowercase()), None))
    //             } else {
    //                 Some((quote!(c.is_ascii_lowercase()), None))
    //             }
    //         }
    //         EBNF::Range(EBNFRange { from: 'A', to: 'Z' }) => {
    //             if negated {
    //                 Some((quote!(!c.is_ascii_uppercase()), None))
    //             } else {
    //                 Some((quote!(c.is_ascii_uppercase()), None))
    //             }
    //         }
    //         EBNF::Range(EBNFRange { from: '0', to: '9' }) => {
    //             if negated {
    //                 Some((quote!(!c.is_ascii_digit()), None))
    //             } else {
    //                 Some((quote!(c.is_ascii_digit()), None))
    //             }
    //         }
    //         EBNF::Range(EBNFRange { from, to }) => {
    //             if negated {
    //                 Some((quote!( c < #from || #to < c ), Some(false)))
    //             } else {
    //                 Some((quote!( #from <= c && c <= #to ), Some(true)))
    //             }
    //         }
    //         EBNF::Terminal(string) => {
    //             if string.len() == 1 {
    //                 let c = string.chars().next().unwrap();
    //                 if negated {
    //                     Some((quote!( c != #c ), None))
    //                 } else {
    //                     Some((quote!( c == #c ), None))
    //                 }
    //             } else {
    //                 None
    //             }
    //         }
    //         _ => None,
    //     }
    // }

    // fn as_terminal_set(&self, grammar: &Grammar) -> Option<HashSet<String>> {
    //     fn collect_terminals(
    //         expr: &Expression,
    //         grammar: &Grammar,
    //         accum: &mut HashSet<String>,
    //     ) -> bool {
    //         match &expr.ebnf {
    //             EBNF::End => false,
    //             EBNF::Repeat(_) => false,
    //             EBNF::Not(_) => false,
    //             EBNF::Choice(exprs) => exprs.iter().all(|e| collect_terminals(e, grammar, accum)),
    //             EBNF::Sequence(_) => false,
    //             EBNF::Terminal(string) => {
    //                 if expr.config.map.is_none() {
    //                     accum.insert(string.clone());
    //                     true
    //                 } else {
    //                     false
    //                 }
    //             }
    //             EBNF::Reference(name) => grammar
    //                 .get_production(name)
    //                 .map(|p| collect_terminals(&p.expression_to_generate(), grammar, accum))
    //                 .unwrap_or(false),
    //             EBNF::Difference(_) => false,
    //             EBNF::Range(_) => false,
    //         }
    //     }

    //     let mut accum = HashSet::new();
    //     if collect_terminals(self, grammar, &mut accum) {
    //         Some(accum)
    //     } else {
    //         None
    //     }
    // }

    // fn is_ignorable_in_sequence(&self, grammar: &Grammar) -> bool {
    //     match &self.ebnf {
    //         EBNF::End { .. } | EBNF::Terminal { .. } => true,
    //         EBNF::Reference(name) => grammar
    //             .get_production(name)
    //             .map(|p| p.expression_to_generate().config.ignore)
    //             .unwrap_or_default(),
    //         _ => false,
    //     }
    // }

    pub(super) fn referenced_identifiers(
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
