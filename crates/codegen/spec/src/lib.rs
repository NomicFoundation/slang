//! Generates a human-readable specification for [`codegen_language_definition`].
//!
//! At the time of writing, the generated pages include:
//! - A page containing a list of supported versions.
//! - A page for each topic, containing both the grammar and documentation.
//!
//! And the auxiliary snippet files included by the grammar mkdocs pages.

mod generators;
mod model;

use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use codegen_language_definition::model::Language;
use infra_utils::codegen::CodegenWriteOnly;

use crate::generators::grammar_ebnf::generate_grammar_ebnf;
use crate::generators::navigation::{generate_section_navigation, generate_top_navigation};
use crate::generators::supported_versions::generate_supported_versions;
use crate::generators::topic_page::generate_topic_page;
use crate::model::SpecModel;

pub struct Spec;

impl Spec {
    pub fn generate(language: Rc<Language>, output_dir: &Path) -> Result<()> {
        let model = SpecModel::build(language);

        let mut codegen = CodegenWriteOnly::new()?;

        codegen.write_file(
            output_dir.join("grammar.ebnf"),
            generate_grammar_ebnf(&model)?,
        )?;

        codegen.write_file(
            output_dir.join("public/NAV.md"),
            generate_top_navigation(&model)?,
        )?;

        codegen.write_file(
            output_dir.join("public/supported-versions.md"),
            generate_supported_versions(&model)?,
        )?;

        for (section_index, section) in model.sections.iter().enumerate() {
            let section_slug = &section.slug;

            codegen.write_file(
                output_dir.join(format!("public/{section_slug}/NAV.md")),
                generate_section_navigation(&model, section_index)?,
            )?;

            for (topic_index, topic) in section.topics.iter().enumerate() {
                let topic_slug = &topic.slug;

                codegen.write_file(
                    output_dir.join(format!("public/{section_slug}/{topic_slug}.md")),
                    generate_topic_page(&model, section_index, topic_index)?,
                )?;
            }
        }

        Ok(())
    }
}
