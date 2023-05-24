use std::path::PathBuf;

use anyhow::Result;
use codegen_ebnf::EbnfSerializer;
use codegen_schema::types::{
    Production, ProductionRef, Schema, SchemaSection, SchemaTopic, VersionMap,
};
use codegen_utils::context::CodegenContext;
use inflector::Inflector;
use semver::Version;

use crate::markdown::MarkdownWriter;

pub struct Snippets<'context> {
    schema: &'context Schema,
    output_dir: &'context PathBuf,
}

impl<'context> Snippets<'context> {
    pub fn new(schema: &'context Schema, output_dir: &'context PathBuf) -> Self {
        return Self { schema, output_dir };
    }

    pub fn write_files(&self, codegen: &mut CodegenContext) -> Result<()> {
        let last_version = self.schema.versions.last().unwrap();
        for production in self.schema.productions.values() {
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
        let production_name = &production.name();
        let (section, topic) = self.locate_production(production_name);

        let file_name = match production.as_ref() {
            Production::Scanner { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => Some("unversioned".to_owned()),
                VersionMap::Versioned(versions) => versions
                    .keys()
                    .rev()
                    .find(|v| *v <= version)
                    .and_then(|v| versions.get(v).unwrap().as_ref().map(|_| v.to_string())),
            },
            Production::TriviaParser { version_map, .. }
            | Production::Parser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => Some("unversioned".to_owned()),
                VersionMap::Versioned(versions) => versions
                    .keys()
                    .rev()
                    .find(|v| *v <= version)
                    .and_then(|v| versions.get(v).unwrap().as_ref().map(|_| v.to_string())),
            },
            Production::PrecedenceParser { version_map, .. } => match version_map {
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
                .join(production_name.to_kebab_case())
                .join(format!("{file_name}.md"))
        });
    }

    fn get_snippet(&self, production: &ProductionRef, version: &Version) -> Option<String> {
        let mut snippet = MarkdownWriter::new();

        let language = "ebnf"; // https://pygments.org/languages/
        let class = "slang-ebnf"; // used to select code blocks via JS during runtime
        let id = production.name(); // used for navigation (generarating URL hashes)

        let contents = &EbnfSerializer::serialize_version(self.schema, production, version)?;
        snippet.write_code_block(language, class, id, contents);

        return Some(snippet.to_string());
    }

    fn locate_production(&self, name: &str) -> (&SchemaSection, &SchemaTopic) {
        for section in &self.schema.sections {
            for topic in &section.topics {
                for production in &topic.productions {
                    if name == production.name() {
                        return (section, topic);
                    }
                }
            }
        }

        unreachable!("Cannot locate production '{name}'.");
    }
}
