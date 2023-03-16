use std::path::PathBuf;

use anyhow::Result;
use codegen_ebnf::EbnfSerializer;
use codegen_schema::types::{
    grammar::Grammar,
    production::{Production, ProductionRef},
};
use codegen_utils::context::CodegenContext;
use inflector::Inflector;
use semver::Version;

use crate::{grammar::GrammarSpecPrivateExtensions, markdown::MarkdownWriter};

pub struct Snippets<'context> {
    grammar: &'context Grammar,
    output_dir: &'context PathBuf,
}

impl<'context> Snippets<'context> {
    pub fn new(grammar: &'context Grammar, output_dir: &'context PathBuf) -> Self {
        return Self {
            grammar,
            output_dir,
        };
    }

    pub fn write_files(&self, codegen: &mut CodegenContext) -> Result<()> {
        let last_version = self.grammar.versions.last().unwrap();
        for production in self.grammar.productions.values() {
            let versions = match production.versions() {
                None => vec![last_version],
                Some(versions) => versions,
            };

            for version in versions {
                let snippet_path = self.get_snippet_path(production, &version).unwrap();
                let snippet = self.get_snippet(production, &version).unwrap();
                codegen.write_file(&snippet_path, &snippet)?;
            }
        }

        return Ok(());
    }

    pub fn get_snippet_path(
        &self,
        production: &ProductionRef,
        version: &Version,
    ) -> Option<PathBuf> {
        let production_name = &production.name();
        let (section, topic) = self.grammar.locate_production(production_name);

        let file_name = match production.versions() {
            None => "unversioned".to_owned(),
            Some(versions) => {
                Production::get_exact_version(versions.into_iter(), &version)?.to_string()
            }
        };

        return Some(
            self.output_dir
                .join("ebnf")
                .join(&section.path)
                .join(&topic.path)
                .join(production_name.to_kebab_case())
                .join(format!("{file_name}.md")),
        );
    }

    fn get_snippet(&self, production: &ProductionRef, version: &Version) -> Option<String> {
        let mut snippet = MarkdownWriter::new();

        snippet.write_comment("markdownlint-disable first-line-h1");
        snippet.write_newline();

        let language = "ebnf"; // https://pygments.org/languages/
        let class = "slang-ebnf"; // used to select code blocks via JS during runtime
        let id = production.name(); // used for navigation (generarating URL hashes)

        let contents = &EbnfSerializer::serialize_version(self.grammar, production, version)?;
        snippet.write_code_block(language, class, id, contents);

        return Some(snippet.to_string());
    }
}
