use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::compiler::analysis::Analysis;
use crate::internals::WriteOutputTokens;

pub(crate) struct LanguageEmitter;

impl LanguageEmitter {
    pub fn emit(analysis: &Analysis) -> TokenStream {
        let language_name = analysis.language.name.to_string();
        let container_mod = format_ident!("{}", language_name.to_snake_case());

        let definition_struct = format_ident!("{}Definition", language_name);
        let definition = analysis.language.write_output_tokens();

        let errors = analysis.errors.to_compile_errors();

        let intellisense_types = Self::emit_intellisense_types(analysis);

        quote! {
            mod #container_mod {
                // Main entry-point to create language definitions:
                pub struct #definition_struct;

                impl #definition_struct {
                    pub fn create() -> std::rc::Rc<language_definition::model::Language> {
                        return std::rc::Rc::new(#definition);
                    }
                }

                // Any validation errors, as `std::compile_error!()` macro invocations:
                #errors

                // `intellisense_types` are hidden from the user. Only used by Rust Analyzer:
                mod intellisense {
                    #intellisense_types
                }
            }
        }
    }

    fn emit_intellisense_types(analysis: &Analysis) -> TokenStream {
        let mut result = TokenStream::new();

        if analysis.errors.has_errors() {
            // No need to generate intellisense types if there are errors.
            // They need to be fixed first, so that we don't get noise in the IDE.
            return result;
        }

        for metadata in analysis.metadata.values() {
            let name = &metadata.name;
            let type_name = format_ident!("{name}", span = name.span());

            result.extend(quote! {
                struct #type_name ;
            });

            for (i, reference) in metadata.referenced_from.iter().enumerate() {
                let reference_name = format_ident!("{name}Reference{i}");
                let reference_value = format_ident!("{name}", span = *reference);

                result.extend(quote! {
                    type #reference_name = #reference_value;
                });
            }
        }

        result
    }
}
