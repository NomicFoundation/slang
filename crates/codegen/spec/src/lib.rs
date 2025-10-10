//! Generates a human-readable specification for [`language_definition`].
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
use infra_utils::codegen::CodegenFileSystem;
use language_definition::model::Language;

use crate::generators::grammar_ebnf::generate_grammar_ebnf;
use crate::generators::navigation::{SpecDir, SpecPage};
use crate::generators::supported_versions::generate_supported_versions;
use crate::generators::topic_page::generate_topic_page;
use crate::model::SpecModel;

pub struct Spec;

impl Spec {
    pub fn generate(language: Rc<Language>, output_dir: &Path) -> Result<()> {
        let mut fs = CodegenFileSystem::default();

        fs.write_file_formatted(
            output_dir.join("language-definition.json"),
            serde_json::to_string(&language)?,
        )?;

        let model = SpecModel::build(language);
        let public_dir = Self::generate_public_dir(&model)?;

        public_dir.write_to_disk(&mut fs, output_dir)?;

        fs.write_file_raw(
            output_dir.join("grammar.ebnf"),
            generate_grammar_ebnf(&model)?,
        )?;

        Ok(())
    }

    fn generate_public_dir(model: &SpecModel) -> Result<SpecDir> {
        let mut public_dir = SpecDir::new(
            format!("{name} Grammar", name = model.language.name),
            "public",
        );

        public_dir.add_page(SpecPage::new(
            "Supported Versions",
            "supported-versions",
            generate_supported_versions(model)?,
        ));

        for (section_index, section) in model.sections.iter().enumerate() {
            let mut section_dir = SpecDir::new(&section.title, &section.slug);

            for (topic_index, topic) in section.topics.iter().enumerate() {
                section_dir.add_page(SpecPage::new(
                    &topic.title,
                    &topic.slug,
                    generate_topic_page(model, section_index, topic_index)?,
                ));
            }

            public_dir.add_dir(section_dir);
        }

        Ok(public_dir)
    }
}
