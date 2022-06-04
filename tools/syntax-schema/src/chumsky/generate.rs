use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
};

use convert_case::{Case, Casing};
use quote::{format_ident, quote};

use super::rustfmt::rustfmt;
use crate::schema::*;

pub struct Context {
    pub no_default_map: bool,
    pub box_non_tokens: bool,
}

impl Grammar {
    pub fn generate_chumsky(&self, context: &Context, output_path: &PathBuf) {
        let mut preludes: Vec<String> = vec![];
        let mut parsers: Vec<String> = vec![];
        self.productions.iter().flat_map(|(_, p)| p).for_each(|p| {
            let expr = p.expression_to_generate();
            if let Some(prelude) = expr.config.prelude.clone() {
                preludes.push(prelude);
                parsers.push(p.generate_chumsky(self, context))
            }
        });

        let generated_source = vec![
            quote!(
                use chumsky::prelude::*;
                use chumsky::Parser;

                pub type ErrorType = Simple<char>;
            )
            .to_string(),
            preludes.join("\n\n"),
            parsers.join("\n\n"),
        ]
        .join("\n\n");

        let formatted_src = rustfmt(generated_source);

        fs::write(output_path, formatted_src).expect("Unable to write to parser file");
    }

    // Compute a topological ordering, with backlinks ignored

    fn production_ordering(&self, root: &String) -> BTreeMap<String, usize> {
        fn visit_production(
            grammar: &Grammar,
            name: &String,
            ordering: &mut BTreeMap<String, usize>,
        ) -> usize {
            let mut order = 0;
            ordering.insert(name.clone(), 0);
            if let Some(production) = grammar.get_production(name) {
                let expr = production.expression_to_generate();
                let mut identifiers = expr.referenced_identifiers();
                if production.is_token {
                    identifiers.insert("IGNORE".to_owned());
                }
                for name in identifiers {
                    let child_order = if let Some(child_order) = ordering.get(&name) {
                        *child_order
                    } else {
                        visit_production(grammar, &name, ordering)
                    };
                    if child_order > order {
                        order = child_order;
                    }
                }
            } else {
                println!("Couldn't find production: {}", name);
            }
            order += 1;
            ordering.insert(name.clone(), order);
            order
        }

        let mut ordering = BTreeMap::new();
        visit_production(self, root, &mut ordering);

        ordering
    }

    // Detect backlinked (recursively-referenced) productions

    fn recursive_productions(&self, ordering: &BTreeMap<String, usize>) -> BTreeSet<String> {
        let mut backlinked = BTreeSet::new();

        for (name, order) in ordering {
            if let Some(production) = self.get_production(name) {
                let expr = production.expression_to_generate();
                let mut identifiers = expr.referenced_identifiers();
                if production.is_token {
                    identifiers.insert("IGNORE".to_owned());
                }
                for name in identifiers {
                    if ordering[&name] >= *order {
                        backlinked.insert(name.clone());
                    }
                }
            }
        }

        backlinked
    }
}

impl Production {
    pub(super) fn expression_to_generate(&self) -> ExpressionRef {
        self.versions.iter().last().map(|(_, e)| e.clone()).unwrap()
    }

    fn generate_chumsky(&self, grammar: &Grammar, _context: &Context) -> String {
        let root = self.name.clone();
        let ordering = grammar.production_ordering(&root);
        let backlinked = grammar.recursive_productions(&ordering);

        // Define each production

        let mut decls = vec![];

        for name in &backlinked {
            let id = format_ident!("{}_parser", name.to_case(Case::Snake));
            decls.push(quote!( let mut #id = Recursive::declare(); ).to_string());
        }

        let mut ordered_productions = ordering.keys().cloned().collect::<Vec<String>>();
        ordered_productions.sort_by(|a, b| (&ordering[a]).cmp(&ordering[b]));
        for name in ordered_productions {
            let id = format_ident!("{}_parser", name.to_case(Case::Snake));
            if let Some(production) = grammar.get_production(&name) {
                let ebnf_comment = production
                    .generate_ebnf(grammar)
                    .iter()
                    .map(|s| format!("// {}", s))
                    .collect::<Vec<_>>()
                    .join("\n");

                let combinator_tree = production
                    .expression_to_generate()
                    .to_combinator_tree(production, grammar);

                // let type_expression = combinator_tree.to_type_expression();
                // let type_comment = format!("// Type: {}", type_expression.to_string());

                let parser_expression = combinator_tree.to_parser_expression();

                let declaration = if backlinked.contains(&name) {
                    quote!( #id.define(#parser_expression.boxed()); )
                } else {
                    quote!( let #id = #parser_expression.boxed(); )
                };

                decls.push(vec![ebnf_comment, declaration.to_string()].join("\n"))
            }
        }

        // Create the parser function

        let root_id = format_ident!("{}_parser", root.to_case(Case::Snake));
        let function_name = format_ident!("create_{}_parser", root.to_case(Case::Snake));
        let result_type_name = format_ident!("{}ParserResultType", root.to_case(Case::UpperCamel));

        format!("{} {{\n{}\n\n{}\n}}",
            quote!(pub fn #function_name() -> impl Parser<char, #result_type_name, Error = ErrorType>).to_string(),
            decls.join("\n\n"),
            quote!(#root_id.recover_with(skip_then_retry_until([]))).to_string()
        )
    }
}

impl Expression {
    fn referenced_identifiers(&self) -> BTreeSet<String> {
        let mut accum = BTreeSet::new();
        self.collect_identifiers(&mut accum);
        accum
    }

    fn collect_identifiers(&self, accum: &mut BTreeSet<String>) {
        match &self.ebnf {
            EBNF::Choice(exprs) | EBNF::Sequence(exprs) => {
                exprs.iter().for_each(|p| {
                    p.collect_identifiers(accum);
                });
            }
            EBNF::Not(expr) => {
                expr.collect_identifiers(accum);
            }
            EBNF::Repeat(EBNFRepeat {
                expr, separator, ..
            }) => {
                expr.collect_identifiers(accum);
                if let Some(separator) = separator {
                    separator.collect_identifiers(accum);
                }
            }
            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                minuend.collect_identifiers(accum);
                subtrahend.collect_identifiers(accum);
            }
            EBNF::Reference(name) => {
                accum.insert(name.clone());
            }
            EBNF::Terminal(_) | EBNF::End | EBNF::Range(_) => (),
        };
    }
}
