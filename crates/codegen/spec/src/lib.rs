use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;

use self::{grammar::generate_spec_grammar, topics::generate_spec_sections};
use std::{io::Write, path::PathBuf};

mod grammar;
mod productions;
mod topics;

pub struct NavigationEntry {
    pub indentation_level: usize,
    pub title: String,
    pub file_path: Option<PathBuf>,
}

pub trait GrammarSpecGeneratorExtensions {
    fn generate_spec(&self, codegen: &mut CodegenContext, documentation_folder: &PathBuf);
}

impl GrammarSpecGeneratorExtensions for Grammar {
    fn generate_spec(&self, codegen: &mut CodegenContext, documentation_folder: &PathBuf) {
        let generated_folder = documentation_folder.join("docs/specification/generated");

        let mut entries: Vec<NavigationEntry> = Vec::new();
        generate_spec_grammar(codegen, self, &generated_folder, &mut entries);
        generate_spec_sections(codegen, self, &generated_folder, &mut entries);

        generate_spec_navigation(codegen, &documentation_folder, &entries);
    }
}

fn generate_spec_navigation(
    codegen: &mut CodegenContext,
    documentation_folder: &PathBuf,
    entries: &Vec<NavigationEntry>,
) {
    let docs_folder = documentation_folder.join("docs");

    let mut w: Vec<u8> = Vec::new();

    writeln!(w, "nav:").unwrap();
    writeln!(w, "  - Specification:").unwrap();

    entries.iter().for_each(|entry| {
        writeln!(
            w,
            "  {}- {}:{}",
            " ".repeat(entry.indentation_level * 4),
            entry.title,
            match &entry.file_path {
                Some(file_path) => format!(
                    " \"{}\"",
                    file_path
                        .strip_prefix(&docs_folder)
                        .unwrap()
                        .to_str()
                        .unwrap()
                ),
                None => "".to_string(),
            }
        )
        .unwrap();
    });

    let navigation_file = documentation_folder
        .join("docs")
        .join("specification")
        .join("generated")
        .join("mkdocs.navigation.yml");

    codegen
        .write_file(&navigation_file, &String::from_utf8(w).unwrap())
        .unwrap();
}
