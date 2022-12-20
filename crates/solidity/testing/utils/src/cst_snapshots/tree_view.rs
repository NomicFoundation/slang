use std::{self, fmt::Write, ops::Range};

use anyhow::Result;

use crate::cst_snapshots::nodes::TestNode;

struct TreeLine {
    pub branch: String,
    pub preview: Option<String>,
}

impl TestNode {
    pub fn to_tree_view(&self, source: &str) -> Result<String> {
        let lines = self.render_tree_lines(source);

        let preview_offset = 4 + lines
            .iter()
            .map(|line| line.branch.chars().count())
            .max()
            .expect("Node must produce at least one tree line.");

        let mut result = String::new();

        for line in lines {
            let TreeLine { branch, preview } = line;
            write!(&mut result, "{branch: <preview_offset$} │")?;

            if let Some(preview) = preview {
                writeln!(&mut result, " {preview}")?;
            } else {
                writeln!(&mut result)?;
            }
        }

        return Ok(result.trim().to_owned());
    }

    fn render_tree_lines(&self, source: &str) -> Vec<TreeLine> {
        let first_line = {
            let icon = if self.children.is_empty() {
                "►"
            } else {
                "█"
            };

            let kind = &self.kind;

            let (range, preview) = if let Some(range) = &self.range {
                let preview = if self.children.is_empty() {
                    Some(Self::render_preview(source, range.to_owned()))
                } else {
                    None
                };

                (format!("{range:?}"), preview)
            } else {
                (format!("<empty>"), None)
            };

            let branch = format!("{icon} {kind:?} - ({range})");

            TreeLine { branch, preview }
        };

        let mut lines = vec![first_line];

        for (child_index, child) in self.children.iter().enumerate() {
            let is_last_child = child_index == self.children.len() - 1;

            for (child_line_index, child_line) in
                child.render_tree_lines(source).into_iter().enumerate()
            {
                let is_first_line = child_line_index == 0;

                let prefix = if is_last_child {
                    if is_first_line {
                        "└──"
                    } else {
                        "   "
                    }
                } else {
                    if is_first_line {
                        "├──"
                    } else {
                        "│  "
                    }
                };

                lines.push(TreeLine {
                    branch: format!("{}{}", prefix, child_line.branch),
                    preview: child_line.preview,
                });
            }
        }

        return lines;
    }

    fn render_preview(source: &str, range: Range<usize>) -> String {
        let contents = source[range].replace("\r", "\\r").replace("\n", "\\n");

        let max_length = 50;
        if contents.len() <= max_length {
            return contents;
        }

        let separator = "...";
        return contents[0..(max_length - separator.len())].to_owned() + separator;
    }
}
