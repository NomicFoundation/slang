use std::path::PathBuf;

use anyhow::Result;
use codegen_utils::context::CodegenContext;

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
    pub fn write_files(&self, codegen: &mut CodegenContext, parent_dir: &PathBuf) -> Result<()> {
        let current_dir = parent_dir.join(self.path());

        match self {
            NavigationEntry::Directory { children, .. } => {
                let mut nav_page = MarkdownWriter::new();

                nav_page.write_comment("markdownlint-disable first-line-h1");
                nav_page.write_newline();

                for child in children {
                    child.write_files(codegen, &current_dir)?;
                    nav_page.write_list_link(child.title(), &child.nav_path());
                }

                codegen.write_file(&current_dir.join("NAV.md"), &nav_page.to_string())?;
            }
            NavigationEntry::Page { contents, .. } => {
                codegen.write_file(&current_dir.join("index.md"), &contents)?;
            }
        };

        return Ok(());
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
