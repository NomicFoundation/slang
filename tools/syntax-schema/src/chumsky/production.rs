use std::collections::{BTreeMap, BTreeSet};

use convert_case::{Case, Casing};
use quote::{format_ident, quote};

use super::grammar::Context;

use crate::schema::*;

impl Production {
    pub(super) fn expression_to_generate(&self) -> ExpressionRef {
        self.versions.iter().last().map(|(_, e)| e.clone()).unwrap()
    }

    pub(super) fn generate_chumsky(&self, grammar: &Grammar, context: &Context) -> String {
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
                let expr = production.expression_to_generate();
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
                let expr = production.expression_to_generate();
                for child in
                    expr.referenced_identifiers(grammar, production, &mut Default::default())
                {
                    if ordering[&child] >= *order {
                        backlinked.insert(child.clone());
                    }
                }
            }
        }
        for name in &backlinked {
            let id = format_ident!("{}_parser", name.to_case(Case::Snake));
            decls.push(quote!( let mut #id = Recursive::declare(); ).to_string());
        }

        // Define each production

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

                let type_expression = combinator_tree.to_type_expression();
                let type_comment = format!("// Type: {}", type_expression.to_string());

                let parser_expression = combinator_tree.to_parser_expression();

                let declaration = if backlinked.contains(&name) {
                    quote!( #id.define(#parser_expression.boxed()); )
                } else {
                    quote!( let #id = #parser_expression.boxed(); )
                };

                decls.push(vec![ebnf_comment, type_comment, declaration.to_string()].join("\n"))
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
