use std::path::Path;

use anyhow::Result;
use infra_utils::codegen::CodegenWriteOnly;

use crate::markdown::MarkdownWriter;

pub enum NavigationEntry {
    Directory {
        title: String,
        path: String,
        children: Vec<NavigationEntry>,
    },
    Page {
        title: String,
        path: String,
        contents: String,
    },
}

impl NavigationEntry {
    pub fn write_files(&self, codegen: &mut CodegenWriteOnly, parent_dir: &Path) -> Result<()> {
        let current_dir = parent_dir.join(self.path());

        match self {
            NavigationEntry::Directory {
                title, children, ..
            } => {
                let mut nav_page = MarkdownWriter::new();
                nav_page.write_list_link(title, "./index.md");

                let mut index_page = MarkdownWriter::new();
                index_page.write_header(1, title);
                index_page.write_newline();

                for child in children {
                    child.write_files(codegen, &current_dir)?;
                    nav_page.write_list_link(child.title(), &child.nav_path());
                    index_page.write_list_link(child.title(), &child.nav_path());
                }

                codegen.write_file(current_dir.join("NAV.md"), nav_page.into_string())?;
                codegen.write_file(current_dir.join("index.md"), index_page.into_string())?;
            }
            NavigationEntry::Page { contents, .. } => {
                codegen.write_file(current_dir.join("index.md"), contents)?;
            }
        };

        Ok(())
    }

    fn title(&self) -> &str {
        match self {
            NavigationEntry::Directory { title, .. } | NavigationEntry::Page { title, .. } => title,
        }
    }

    fn path(&self) -> &str {
        match self {
            NavigationEntry::Directory { path, .. } | NavigationEntry::Page { path, .. } => path,
        }
    }

    fn nav_path(&self) -> String {
        // `mkdocs-literate-nav` expects this exact format to differentiate between directories and pages.
        match self {
            NavigationEntry::Directory { path, .. } => format!("./{path}/"),
            NavigationEntry::Page { path, .. } => format!("./{path}/index.md"),
        }
    }
}
