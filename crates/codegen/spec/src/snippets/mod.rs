mod ebnf;

use std::path::PathBuf;

use anyhow::Result;
use codegen_ebnf::ebnf_writer::EBNFWritable;
use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;
use inflector::Inflector;

use crate::{grammar::GrammarSpecPrivateExtensions, snippets::ebnf::HTMLWriter};

pub struct Snippets<'a> {
    grammar: &'a Grammar,
    output_dir: &'a PathBuf,
}

impl<'a> Snippets<'a> {
    pub fn new(grammar: &'a Grammar, output_dir: &'a PathBuf) -> Self {
        return Self {
            grammar,
            output_dir,
        };
    }

    pub fn write_files(&self, codegen: &mut CodegenContext) -> Result<()> {
        for production in self.grammar.productions.values() {
            let mut writer = HTMLWriter::new(self.grammar);
            production.write_ebnf("", &mut writer);

            let snippet = writer.to_string();
            let snippet_path = self.get_snippet_path(production.name());

            codegen.write_file(&snippet_path, &snippet)?;
        }

        return Ok(());
    }

    pub fn get_snippet_path(&self, name: &str) -> PathBuf {
        let (section, topic) = self.grammar.locate_production(name);
        return self
            .output_dir
            .join("ebnf")
            .join(&section.path)
            .join(&topic.path)
            .join(format!("{name}.html", name = name.to_kebab_case()));
    }
}
