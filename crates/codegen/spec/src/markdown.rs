use std::{fmt::Write, path::Path};

use infra_utils::paths::PathExtensions;

pub struct MarkdownWriter {
    w: String,
}

impl MarkdownWriter {
    pub fn new() -> Self {
        return Self { w: String::new() };
    }

    pub fn into_string(self) -> String {
        return self.w;
    }

    pub fn write_header(&mut self, level: usize, value: &str) {
        writeln!(self.w, "{prefix} {value}", prefix = "#".repeat(level)).unwrap();
    }

    pub fn write_snippet(&mut self, snippet_path: &Path) {
        writeln!(
            self.w,
            "--8<-- \"{snippet_path}\"",
            snippet_path = snippet_path.strip_repo_root().unwrap().unwrap_str()
        )
        .unwrap();
    }

    pub fn write_code_block(&mut self, language: &str, class: &str, id: &str, contents: &str) {
        writeln!(self.w, "```{{ .{language} .{class} #{id} }}").unwrap();
        writeln!(self.w, "{contents}").unwrap();
        writeln!(self.w, "```").unwrap();
    }

    pub fn write_text(&mut self, text: &str) {
        writeln!(self.w, "{text}").unwrap();
    }

    pub fn write_list_link(&mut self, title: &str, path: &str) {
        writeln!(self.w, "-   [{title}]({path})").unwrap();
    }

    pub fn write_newline(&mut self) {
        writeln!(self.w).unwrap();
    }
}
