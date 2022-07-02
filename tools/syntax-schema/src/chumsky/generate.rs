use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
};

use quote::quote;

use super::{combinator_tree::ProductionGeneratedCode, rustfmt::rustfmt};
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
            let mut identifiers = production.combinator_tree().referenced_identifiers();
            if !production.is_token() {
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
            let mut identifiers = production.combinator_tree().referenced_identifiers();
            if !production.is_token() {
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

        let mut parser_field_definitions = vec![];
        let mut parser_field_initializations = vec![];
        let mut parser_implementation_predeclarations = vec![];
        let mut parser_implementations = vec![];
        let mut tree_interfaces = vec![];
        let mut tree_implementations = vec![];

        let mut ordered_productions = production_ordering.keys().cloned().collect::<Vec<String>>();
        ordered_productions.sort_by(|a, b| (&production_ordering[a]).cmp(&production_ordering[b]));
        for name in ordered_productions {
            let production = grammar.get_production(&name);
            let combinator_tree = production.combinator_tree();
            let is_recursive = recursive_production_names.contains(&name);
            let ProductionGeneratedCode {
                parser_field_definition,
                parser_field_initialization,
                parser_implementation_predeclaration,
                parser_implementation,
                tree_interface,
                tree_implementation,
            } = combinator_tree.to_generated_code(is_recursive);

            parser_field_definitions.push(parser_field_definition);
            parser_field_initializations.push(parser_field_initialization);
            parser_implementation_predeclarations
                .push(parser_implementation_predeclaration.to_string());

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

            tree_implementations.push(tree_implementation.to_string());
        }

        GeneratedOutput {
            tree_interface: vec![
                boilerplate::tree_interface().to_string(),
                tree_interfaces.join("\n\n"),
            ]
            .join("\n\n"),

            tree_implementation: vec![
                boilerplate::tree_implementation().to_string(),
                tree_implementations.join("\n\n"),
            ]
            .join("\n\n"),

            parser_interface: vec![
                boilerplate::parser_interface().to_string(),
                quote!(
                    #[allow(dead_code)]
                    pub struct Parsers { #(#parser_field_definitions),* }
                )
                .to_string(),
            ]
            .join("\n\n"),

            parser_implementation: vec![
                boilerplate::parser_implementation().to_string(),
                "impl Parsers { pub fn new() -> Self {".to_owned(),
                parser_implementation_predeclarations.join("\n\n"),
                parser_implementations.join("\n\n"),
                quote!(
                    Self {
                        #(#parser_field_initializations),*
                    }
                )
                .to_string(),
                "}}".to_owned(),
            ]
            .join("\n\n"),

            mod_file: quote!(
                pub mod parser_implementation;
                pub mod parser_interface;
                pub mod tree_implementation;
                pub mod tree_interface;
            )
            .to_string(),
        }
    }
}

mod boilerplate {
    use proc_macro2::TokenStream;
    use quote::quote;

    pub fn tree_interface() -> TokenStream {
        quote!(
            #[allow(unused_imports)]
            use serde::{Serialize, Deserialize};

            pub trait DefaultTest {
                fn is_default(&self) -> bool {
                    false
                }
            }

            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
            pub struct FixedSizeTerminal<const N: usize>();

            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
            pub struct FixedSizeTerminalWithNoise<const N: usize> {
                #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                pub leading: Ignore,
                #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                pub content: FixedSizeTerminal<N>,
                #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                pub trailing: Ignore,
            }

            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
            pub struct VariableSizeTerminal(pub usize);

            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
            pub struct VariableSizeTerminalWithNoise {
                #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                pub leading: Ignore,
                #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                pub content: VariableSizeTerminal,
                #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
                pub trailing: Ignore,
            }
        )
    }

    pub fn tree_implementation() -> TokenStream {
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

            impl DefaultTest for VariableSizeTerminal {
                fn is_default(&self) -> bool {
                    self.0 == 0
                }
            }

            impl DefaultTest for VariableSizeTerminalWithNoise {
                fn is_default(&self) -> bool {
                    self.content.is_default() && self.leading.is_default() && self.trailing.is_default()
                }
            }

            impl<const N: usize> DefaultTest for FixedSizeTerminal<N> {
                fn is_default(&self) -> bool {
                    true
                }
            }

            impl<const N: usize> DefaultTest for FixedSizeTerminalWithNoise<N> {
                fn is_default(&self) -> bool {
                    self.leading.is_empty() && self.trailing.is_empty()
                }
            }
        )
    }

    pub fn parser_interface() -> TokenStream {
        quote!(
            use chumsky::prelude::{Simple, BoxedParser};

            pub type ErrorType = Simple<char>;
            pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;

            use super::tree_interface::*;
        )
    }

    pub fn parser_implementation() -> TokenStream {
        quote!(
            use chumsky::Parser;
            use chumsky::prelude::*;
            use chumsky::primitive::Just;

            use super::parser_interface::*;
            use super::tree_interface::*;

            #[allow(dead_code)]
            fn repetition_mapper<E, S>((e, es): (E, Vec<(S, E)>)) -> (Vec<E>, Vec<S>) {
                let mut elements = vec![e];
                let mut separators = vec![];
                for (s, e) in es.into_iter() {
                    separators.push(s);
                    elements.push(e);
                }
                (elements, separators)
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
                let minuend_end = minuend.clone().map_with_span(|_, span| span.end).rewind();
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
    }
}
