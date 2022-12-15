use std::path::PathBuf;

use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;
use quote::{format_ident, quote};

use super::{
    code_generator::{CodeGenerator, ParserResultType},
    naming, typescript_lib_boilerplate,
};

impl CodeGenerator {
    pub fn write_typescript_lib_sources(
        &self,
        grammar: &Grammar,
        codegen: &mut CodegenContext,
        output_dir: &PathBuf,
    ) {
        codegen
            .write_file(
                &output_dir.join("mod.rs"),
                &typescript_lib_boilerplate::mod_head().to_string(),
            )
            .unwrap();

        codegen
            .write_file(
                &output_dir.join("lex.rs"),
                &typescript_lib_boilerplate::lex_head().to_string(),
            )
            .unwrap();

        codegen
            .write_file(
                &output_dir.join("cst.rs"),
                &typescript_lib_boilerplate::cst_head().to_string(),
            )
            .unwrap();

        self.write_parser_source(grammar, codegen, output_dir);

        codegen
            .write_file(
                &output_dir.join("language.rs"),
                &format!(
                    "{}

                    #[napi]
                    impl Language {{
                        {}
                    }}
                    ",
                    typescript_lib_boilerplate::language_head(),
                    self
                        .parsers
                        .iter()
                        .map(|(name, parser)| {
                            let field_name = naming::to_field_name_ident(&name);
                            let method_name = format_ident!("parse_{}", field_name);
                            let result_type = match parser.result_type {
                                ParserResultType::Token => quote! { "CSTTokenNode" },
                                ParserResultType::Rule => quote! { "CSTRuleNode" },
                            };
                            format!(
                                "{}\n{}",
                                parser
                                    .comment
                                    .iter()
                                    .map(|s| format!("// {}", s))
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                                quote!(
                                    #[napi(ts_return_type = #result_type)]
                                    // pub fn #method_name(&self, env: Env, source: String) -> napi::JsUnknown {
                                    pub fn #method_name(&self, env: Env, source: String) -> napi::JsObject {
                                        let (node, _errs) = self.parsers.#field_name.parse_recovery(source.as_str());
                                        // env.to_js_value(&node).unwrap()
                                        node.unwrap().to_js(&env)
                                    }
                                )
                            )
                        })
                        .collect::<Vec<_>>().join("\n\n"),
                ),
            )
            .unwrap();

        // Do the kinds last, because the code generation steps above may have added new kinds
        codegen
            .write_file(
                &output_dir.join("kinds.rs"),
                &format!(
                    "{}

                    #[derive(Debug, PartialEq, Eq, Serialize)]
                    #[napi]
                    pub enum Token {{
                        {}
                    }}

                    #[derive(Debug, PartialEq, Eq, Serialize)]
                    #[napi]
                    pub enum Rule {{
                        {}
                    }}
                    ",
                    typescript_lib_boilerplate::kinds_head(),
                    self.token_kinds
                        .keys()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                    self.rule_kinds
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            )
            .unwrap();
    }
}
