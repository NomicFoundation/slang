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

        let generated_src = vec![
            quote!(
                use chumsky::prelude::*;
                use chumsky::Parser;

                pub type ErrorType = Simple<char>;
                pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;
            )
            .to_string(),
            preludes.join("\n\n"),
            parsers.join("\n\n"),
        ]
        .join("\n\n");

        // Make it possible to debug the code even when rustfmt dies
        fs::write(output_path, generated_src.as_str()).expect("Unable to write to parser file");

        let formatted_src = rustfmt(generated_src);
        if formatted_src != "" {
            fs::write(output_path, formatted_src).expect("Unable to write to parser file");
        }
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

        let mut module_definitions = vec![];
        let mut parser_definitions = vec![];
        let mut parser_fields = vec![];
        let mut parser_field_assignments = vec![];

        for name in &backlinked {
            let id = format_ident!("{}_parser", name.to_case(Case::Snake));
            parser_definitions.push(quote!( let mut #id = Recursive::declare(); ).to_string());
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

                let combinator_tree = production.to_combinator_tree(grammar);

                let type_tree = combinator_tree.to_type_tree();
                let module_name = format_ident!("{}", production.name.to_case(Case::Snake));
                let type_definitions = type_tree.to_type_definition_code();
                module_definitions.push(format!(
                    "mod {} {{\n{}\n{}\n}}",
                    module_name.to_string(),
                    ebnf_comment,
                    type_definitions.to_string(),
                ));

                let parser_expression = combinator_tree.to_parser_combinator_code();
                let type_tuple_size = type_tree.tuple_size();
                let type_mapping = if type_tuple_size > 0 {
                    let members: Vec<_> = (0..type_tuple_size)
                        .map(|i| {
                            let ident = format_ident!("_{}", i);
                            quote!( #ident )
                        })
                        .collect();
                    quote!( .map(|(#(#members),*)| #module_name::N(#(#members),*)) )
                } else {
                    quote!()
                };
                let parser_definition = if backlinked.contains(&name) {
                    quote!( #id.define(#parser_expression #type_mapping); )
                } else {
                    quote!( let #id = #parser_expression #type_mapping; )
                };
                parser_definitions.push(format!(
                    "{}\n{}",
                    ebnf_comment,
                    parser_definition.to_string()
                ));

                let field_name = format_ident!("{}", name.to_case(Case::Snake));
                parser_fields
                    .push(quote!( pub #field_name: ParserType<#module_name::N>, ).to_string());
                parser_field_assignments.push(quote!( #field_name: #id.boxed(), ).to_string())
            }
        }

        // Create the parser function

        format!("{}\n\n#[allow(dead_code)] pub struct Parsers {{ {} }}\n\nimpl Parsers {{\npub fn new() -> Self {{\n{}\n\nSelf {{ {} }}\n}}\n}}",
            module_definitions.join("\n\n"),
            parser_fields.join("\n"),
            parser_definitions.join("\n\n"),
            parser_field_assignments.join("\n")
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
