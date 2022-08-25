use self::{grammar::generate_spec_grammar, topics::generate_spec_sections};
use crate::{
    build_utils::{delete_generated_file, write_generated_file},
    schema::Grammar,
};
use std::{collections::HashSet, io::Write, path::PathBuf};

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
        let generated_folder = documentation_folder
            .join("docs")
            .join("specification")
            .join("generated");

        let mut entries: Vec<NavigationEntry> = Vec::new();
        generate_spec_grammar(&self, &generated_folder, &mut entries);
        generate_spec_sections(&self, &generated_folder, &mut entries);

        let mut generated_files: HashSet<&PathBuf> = entries
            .iter()
            .filter_map(|entry| entry.file_path.as_ref())
            .collect();

        let navigation_file = generate_spec_navigation(&documentation_folder, &entries);
        generated_files.insert(&navigation_file);

        delete_orphaned_files(&generated_folder, &generated_files)
    }
}

fn generate_spec_navigation(
    documentation_folder: &PathBuf,
    entries: &Vec<NavigationEntry>,
) -> PathBuf {
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

    let navigation_file = documentation_folder
        .join("docs")
        .join("specification")
        .join("generated")
        .join("mkdocs.navigation.yml");

    write_generated_file(&navigation_file, &String::from_utf8(w).unwrap());
    return navigation_file;
}

fn delete_orphaned_files(root_folder: &PathBuf, generated_files: &HashSet<&PathBuf>) {
    root_folder.read_dir().unwrap().for_each(|child| {
        let child_path = child.unwrap().path();

        if child_path.is_dir() {
            delete_orphaned_files(&child_path, generated_files);
        } else if generated_files.contains(&child_path) {
            // Keep file
        } else {
            delete_generated_file(&child_path);
        }
    })
}
