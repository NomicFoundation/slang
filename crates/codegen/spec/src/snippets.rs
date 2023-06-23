use std::path::PathBuf;

use anyhow::Result;
use codegen_ebnf::EbnfSerializer;
use codegen_schema::types::{
    LanguageDefinitionRef, LanguageSection, LanguageTopic, ProductionDefinition, ProductionRef,
    VersionMap,
};
use codegen_utils::context::CodegenContext;
use inflector::Inflector;
use semver::Version;

use crate::markdown::MarkdownWriter;

pub struct Snippets {
    language: LanguageDefinitionRef,
    output_dir: PathBuf,
}

impl Snippets {
    pub fn new(language: &LanguageDefinitionRef, output_dir: &PathBuf) -> Self {
        return Self {
            language: language.to_owned(),
            output_dir: output_dir.to_owned(),
        };
    }

    pub fn write_files(&self, codegen: &mut CodegenContext) -> Result<()> {
        let last_version = self.language.versions.last().unwrap();
        for production in self.language.productions.values() {
            let versions = match production.versions() {
                None => vec![last_version],
                Some(versions) => versions,
            };

            for version in versions {
                if let Some(snippet_path) = self.get_snippet_path(production, &version) {
                    let snippet = self.get_snippet(production, &version).unwrap_or_default();
                    codegen.write_file(&snippet_path, &snippet)?
                };
            }
        }

        return Ok(());
    }

    pub fn get_snippet_path(
        &self,
        production: &ProductionRef,
        version: &Version,
    ) -> Option<PathBuf> {
        let (section, topic) = self.locate_production(&production.name);

        let file_name = match &production.definition {
            ProductionDefinition::Scanner { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => Some("unversioned".to_owned()),
                VersionMap::Versioned(versions) => versions
                    .keys()
                    .rev()
                    .find(|v| *v <= version)
                    .and_then(|v| versions.get(v).unwrap().as_ref().map(|_| v.to_string())),
            },
            ProductionDefinition::TriviaParser { version_map, .. }
            | ProductionDefinition::Parser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => Some("unversioned".to_owned()),
                VersionMap::Versioned(versions) => versions
                    .keys()
                    .rev()
                    .find(|v| *v <= version)
                    .and_then(|v| versions.get(v).unwrap().as_ref().map(|_| v.to_string())),
            },
            ProductionDefinition::PrecedenceParser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => Some("unversioned".to_owned()),
                VersionMap::Versioned(versions) => versions
                    .keys()
                    .rev()
                    .find(|v| *v <= version)
                    .and_then(|v| versions.get(v).unwrap().as_ref().map(|_| v.to_string())),
            },
        };

        return file_name.map(|file_name| {
            self.output_dir
                .join("ebnf")
                .join(&section.path)
                .join(&topic.path)
                .join(production.name.to_kebab_case())
                .join(format!("{file_name}.md"))
        });
    }

    fn get_snippet(&self, production: &ProductionRef, version: &Version) -> Option<String> {
        let mut snippet = MarkdownWriter::new();

        // https://pygments.org/languages/
        let language = "ebnf";
        // used to select code blocks via JS during runtime
        let class = "slang-ebnf";
        // used for navigation (generarating URL hashes)
        let id = if matches!(production.definition, ProductionDefinition::Scanner { .. }) {
            production.name.to_screaming_snake_case()
        } else {
            production.name.to_owned()
        };

        let contents = &EbnfSerializer::serialize_version(&self.language, production, version)?;
        snippet.write_code_block(language, class, &id, contents);

        return Some(snippet.to_string());
    }

    fn locate_production(&self, name: &str) -> (&LanguageSection, &LanguageTopic) {
        for section in &self.language.sections {
            for topic in &section.topics {
                for production in &topic.productions {
                    if name == production.name {
                        return (section, topic);
                    }
                }
            }
        }

        unreachable!("Cannot locate production '{name}'.");
    }
}
