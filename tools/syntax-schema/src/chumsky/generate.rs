use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
};

use quote::quote;

use super::{rustfmt::rustfmt, slang_name::SlangName};
use crate::schema::*;

pub struct GenerationContext {
    pub output_dir: PathBuf,
}

impl Grammar {
    pub fn generate_chumsky(&self, context: &GenerationContext) {
        let root = self
            .productions
            .iter()
            .flat_map(|(_, p)| p)
            .find(|p| p.expression_to_generate().config.prelude.is_some())
            .expect("Didn't find any expression with a prelude");
        let generated_output = root.generate_chumsky(self, context);

        Self::format_and_write_source(
            &context.output_dir.join("parser_interface.rs"),
            generated_output.parser_interface,
        );
        Self::format_and_write_source(
            &context.output_dir.join("parser_implementation.rs"),
            generated_output.parser_implementation,
        );
        Self::format_and_write_source(
            &context.output_dir.join("tree_interface.rs"),
            generated_output.tree_interface,
        );
        Self::format_and_write_source(
            &context.output_dir.join("tree_implementation.rs"),
            generated_output.tree_implementation,
        );
    }

    fn format_and_write_source(path: &PathBuf, source: String) {
        // Make it possible to debug the code even when rustfmt dies
        // by writing the unformatted code to the file.
        fs::write(path, source.as_str()).expect(format!("Unable to write to {:?}", path).as_str());
        let formatted_src = rustfmt(source);
        if formatted_src != "" {
            fs::write(path, formatted_src)
                .expect(format!("Unable to write to {:?}", path).as_str());
        };
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

pub struct GeneratedOutput {
    pub parser_interface: String,
    pub tree_interface: String,
    pub parser_implementation: String,
    pub tree_implementation: String,
}

impl Production {
    pub fn expression_to_generate(&self) -> ExpressionRef {
        self.versions.iter().last().map(|(_, e)| e.clone()).unwrap()
    }

    fn generate_chumsky(&self, grammar: &Grammar, _context: &GenerationContext) -> GeneratedOutput {
        let root = self.name.clone();
        let ordering = grammar.production_ordering(&root);
        let backlinked = grammar.recursive_productions(&ordering);

        // Define each production

        let mut tree_interfaces = vec![];
        let mut tree_implementations = vec![];

        let mut module_names = vec![];
        let mut parsers_field_names = vec![];
        let mut parser_names = vec![];
        let mut parser_implementations = vec![];

        for name in &backlinked {
            let parser_name = SlangName::from_string(name).to_parser_name_ident();
            // parser_implementations.push("// Forward Reference");
            parser_implementations
                .push(quote!( let mut #parser_name = Recursive::declare(); ).to_string());
        }

        let mut ordered_productions = ordering.keys().cloned().collect::<Vec<String>>();
        ordered_productions.sort_by(|a, b| (&ordering[a]).cmp(&ordering[b]));
        for name in ordered_productions {
            if let Some(production) = grammar.get_production(&name) {
                let slang_name = SlangName::from_string(&name);
                let ebnf_comment = production
                    .generate_ebnf(grammar)
                    .iter()
                    .map(|s| format!("// {}", s))
                    .collect::<Vec<_>>()
                    .join("\n");
                let ebnf_doc_comment = production
                    .generate_ebnf(grammar)
                    .iter()
                    .map(|s| format!("/// {}", s))
                    .collect::<Vec<_>>()
                    .join("\n");

                let module_name = slang_name.to_module_name_ident();
                module_names.push(module_name.clone());

                let combinator_tree = production.to_combinator_tree(grammar);
                let type_tree = combinator_tree.to_type_tree();
                let (tree_interface, tree_implementation) =
                    type_tree.to_type_definition_code(&module_name);
                tree_interfaces.push(format!(
                    "{}\n{}",
                    ebnf_doc_comment,
                    tree_interface.to_string()
                ));
                tree_implementations.push(tree_implementation);

                let parser_name = slang_name.to_parser_name_ident();
                parser_names.push(parser_name.clone());
                let parser_expression = combinator_tree.to_parser_combinator_code();
                let parser_implementation = if backlinked.contains(&name) {
                    quote!( #parser_name.define(#parser_expression.boxed()); )
                } else {
                    quote!( let #parser_name = #parser_expression.boxed(); )
                };
                // TODO: Move to strings so this can be inserted
                // parser_implementations.push(ebnf_comment.clone());
                parser_implementations.push(format!(
                    "{}\n{}",
                    ebnf_comment,
                    parser_implementation.to_string()
                ));

                parsers_field_names.push(slang_name.to_field_name_ident());
            }
        }

        let parser_interface = quote!(
            use chumsky::prelude::{Simple, BoxedParser};

            pub type ErrorType = Simple<char>;
            pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;

            #[allow(dead_code)]
            pub struct Parsers { #(pub #parsers_field_names: ParserType<#module_names::N>),* }
        );

        let parser_implementation = {
            vec![
                quote!(
                    use chumsky::Parser;
                    use chumsky::prelude::*;
                    use chumsky::primitive::Just;

                    use super::parser_interface::*;
                    use super::tree_interface::*;

                    #[allow(dead_code)]
                    fn repetition_mapper<E, S>((e, es): (E, Vec<(S, E)>)) -> (Vec<E>, Vec<S>) {
                        let mut expressions = vec![e];
                        let mut separators = vec![];
                        for (s, e) in es.into_iter() {
                            separators.push(s);
                            expressions.push(e);
                        }
                        (expressions, separators)
                    }

                    #[allow(dead_code)]
                    fn difference<M, MO, S, SO>(
                        minuend: M,
                        subtrahend: S,
                    ) -> impl Parser<char, MO, Error = ErrorType>
                    where
                        M: Clone + Parser<char, MO, Error = ErrorType>,
                        S: Parser<char, SO, Error = ErrorType>,
                    {
                        // TODO This could be much more efficient if we were able
                        // to conditionally rewind
                        let minuend_end =
                            minuend.clone().map_with_span(|_, span| span.end).rewind();
                        let subtrahend_end = subtrahend
                            .map_with_span(|_, span| span.end)
                            .rewind()
                            .or_else(|_| Ok(0));
                        minuend_end
                            .then(subtrahend_end)
                            .validate(|(m, s), span, emit| {
                                if m == s {
                                    emit(Simple::custom(span, "subtrahend matches minuend"))
                                }
                            })
                            .ignore_then(minuend)
                    }

                    #[allow(dead_code)]
                    #[inline]
                    fn terminal(str: &str) -> Just<char, &str, ErrorType> {
                        just(str)
                    }
                )
                .to_string(),
                "impl Parsers { pub fn new() -> Self {".to_owned(),
                parser_implementations.join("\n\n"),
                quote!(
                    Self {
                        #(#parsers_field_names: #parser_names.boxed()),*
                    }
                )
                .to_string(),
                "}}".to_owned(),
            ]
            .join("\n\n")
        };

        GeneratedOutput {
            tree_interface: format!(
                "{}\n\n{}",
                quote!(
                    #[allow(unused_imports)]
                    use serde::{Serialize, Deserialize};
                )
                .to_string(),
                tree_interfaces.join("\n\n")
            ),
            parser_interface: quote!(
                use super::tree_interface::*;
                #parser_interface
            )
            .to_string(),
            tree_implementation: quote!(
                use super::tree_interface::*;
                #(#tree_implementations)*
            )
            .to_string(),
            parser_implementation: parser_implementation,
        }
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
