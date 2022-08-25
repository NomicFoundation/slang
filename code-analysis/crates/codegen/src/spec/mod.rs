use self::{grammar::generate_spec_grammar, topics::generate_spec_sections};
use crate::schema::Grammar;
use std::{collections::HashSet, fs::File, io::Write, path::PathBuf};

pub mod grammar;
pub mod productions;
pub mod topics;

pub struct NavigationEntry {
    pub indentation_level: usize,
    pub title: String,
    pub file_path: Option<PathBuf>,
}

impl Grammar {
    pub fn generate_spec(&self, documentation_folder: &PathBuf) {
        let mut entries: Vec<NavigationEntry> = Vec::new();

        let generated_folder = documentation_folder
            .join("docs")
            .join("specification")
            .join("generated");

        generate_spec_grammar(&self, &generated_folder, &mut entries);
        generate_spec_sections(&self, &generated_folder, &mut entries);

        generate_spec_navigation(&documentation_folder, &entries);

        let generated_files: HashSet<&PathBuf> = entries
            .iter()
            .filter_map(|entry| entry.file_path.as_ref())
            .collect();

        delete_orphaned_files(&generated_folder, &generated_files)
    }
}

fn generate_spec_navigation(documentation_folder: &PathBuf, entries: &Vec<NavigationEntry>) {
    let navigation_file = documentation_folder.join("mkdocs.specification.yml");
    let docs_folder = documentation_folder.join("docs");

    std::fs::create_dir_all(navigation_file.parent().unwrap()).unwrap();
    let mut w = File::create(navigation_file).expect("Unable to create file");

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
                    " {}",
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
}

fn delete_orphaned_files(root_folder: &PathBuf, generated_files: &HashSet<&PathBuf>) {
    root_folder.read_dir().unwrap().for_each(|child| {
        let child_path = child.unwrap().path();

        if child_path.is_dir() {
            delete_orphaned_files(&child_path, generated_files);
        } else if generated_files.contains(&child_path) {
            // Keep file
        } else {
            std::fs::remove_file(&child_path).unwrap();
        }
    })
}
