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
            .find(|p| p.name == self.manifest.root_production)
            .expect("Didn't find root production");
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
        Self::format_and_write_source(
            &context.output_dir.join("mod.rs"),
            generated_output.mod_file,
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
            let production = grammar.get_production(name);
            let expr = production.expression_to_generate();
            let mut identifiers = expr.referenced_identifiers();
            if !production.is_token {
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
            order += 1;
            ordering.insert(name.clone(), order);
            order
        }

        let mut ordering = BTreeMap::new();
        visit_production(self, root, &mut ordering);

        ordering
    }

    // Detect backlinked (recursively-referenced) productions

    fn recursive_production_names(&self, ordering: &BTreeMap<String, usize>) -> BTreeSet<String> {
        let mut backlinked = BTreeSet::new();

        for (name, order) in ordering {
            let production = self.get_production(name);
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

        backlinked
    }
}

pub struct GeneratedOutput {
    pub parser_interface: String,
    pub tree_interface: String,
    pub parser_implementation: String,
    pub tree_implementation: String,
    pub mod_file: String,
}

impl Production {
    pub fn expression_to_generate(&self) -> ExpressionRef {
        self.versions.iter().last().map(|(_, e)| e.clone()).unwrap()
    }

    fn generate_chumsky(&self, grammar: &Grammar, _context: &GenerationContext) -> GeneratedOutput {
        let production_ordering = grammar.production_ordering(&self.name);
        let recursive_production_names = grammar.recursive_production_names(&production_ordering);

        let mut module_names = vec![];

        let mut tree_interfaces = vec![];
        let mut tree_implementations = vec![];

        let mut parser_names = vec![];
        let mut parsers_field_names = vec![];
        let mut parser_implementations = vec![];

        for name in &recursive_production_names {
            let parser_name = SlangName::from_string(name).to_parser_name_ident();
            parser_implementations
                .push(quote!( let mut #parser_name = Recursive::declare(); ).to_string());
        }

        let mut ordered_productions = production_ordering.keys().cloned().collect::<Vec<String>>();
        ordered_productions.sort_by(|a, b| (&production_ordering[a]).cmp(&production_ordering[b]));
        for name in ordered_productions {
            let production = grammar.get_production(&name);
            let slang_name = production.slang_name();
            let module_name = slang_name.to_module_name_ident();
            module_names.push(module_name.clone());

            // Generate the tree

            let combinator_tree = production.combinator_tree();
            let type_tree = combinator_tree.to_type_tree();
            let (tree_interface, tree_implementation) =
                type_tree.to_type_definition_code(&module_name);
            let ebnf_doc_comment = production
                .generate_ebnf(grammar)
                .iter()
                .map(|s| format!("/// {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            tree_interfaces.push(format!(
                "{}\n{}",
                ebnf_doc_comment,
                tree_interface.to_string()
            ));
            tree_implementations.push(tree_implementation);

            // Generate the parser

            let parser_name = slang_name.to_parser_name_ident();
            parser_names.push(parser_name.clone());
            let parser_expression = combinator_tree.to_parser_combinator_code();
            let parser_implementation = if recursive_production_names.contains(&name) {
                quote!( #parser_name.define(#parser_expression.boxed()); )
            } else {
                quote!( let #parser_name = #parser_expression.boxed(); )
            };
            let ebnf_comment = production
                .generate_ebnf(grammar)
                .iter()
                .map(|s| format!("// {}", s))
                .collect::<Vec<_>>()
                .join("\n");
            parser_implementations.push(format!(
                "{}\n{}",
                ebnf_comment,
                parser_implementation.to_string()
            ));
            parsers_field_names.push(slang_name.to_field_name_ident());
        }

        let tree_interface = format!(
            "{}\n\n{}",
            quote!(
                #[allow(unused_imports)]
                use serde::{Serialize, Deserialize};

                pub trait DefaultTest {
                    fn is_default(&self) -> bool {
                        false
                    }
                }

                #[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
                pub struct FixedTerminal<const N: usize>();
            )
            .to_string(),
            tree_interfaces.join("\n\n")
        );

        let tree_implementation = format!(
            "{}\n\n{}",
            quote!(
                use super::tree_interface::*;

                impl<T: DefaultTest> DefaultTest for Box<T> {
                    fn is_default(&self) -> bool {
                        self.as_ref().is_default()
                    }
                }

                impl<T> DefaultTest for Vec<T> {
                    fn is_default(&self) -> bool {
                      self.is_empty()
                    }
                }

                impl<T> DefaultTest for Option<T> {
                    fn is_default(&self) -> bool {
                      self.is_none()
                    }
                }
                impl DefaultTest for () {
                    fn is_default(&self) -> bool {
                      true
                    }
                }

                impl DefaultTest for usize {
                    fn is_default(&self) -> bool {
                        *self == 0
                    }
                }

                impl<const N: usize> DefaultTest for FixedTerminal<N> {
                    fn is_default(&self) -> bool {
                        true
                    }
                }
            )
            .to_string(),
            tree_implementations.join("\n\n")
        );

        let parser_interface = quote!(
            use chumsky::prelude::{Simple, BoxedParser};

            pub type ErrorType = Simple<char>;
            pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;

            use super::tree_interface::*;

            #[allow(dead_code)]
            pub struct Parsers { #(pub #parsers_field_names: ParserType<#module_names::N>),* }
        )
        .to_string();

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

        let mod_file = quote!(
            pub mod parser_implementation;
            pub mod parser_interface;
            pub mod tree_implementation;
            pub mod tree_interface;
        )
        .to_string();

        GeneratedOutput {
            tree_interface,
            tree_implementation,
            parser_interface,
            parser_implementation,
            mod_file,
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
