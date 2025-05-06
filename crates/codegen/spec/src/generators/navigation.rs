use std::fmt::Write;
use std::path::Path;

use anyhow::Result;
use infra_utils::codegen::CodegenFileSystem;

pub enum SpecEntry {
    Dir(SpecDir),
    Page(SpecPage),
}

pub struct SpecDir {
    title: String,
    slug: String,
    entries: Vec<SpecEntry>,
}

impl SpecDir {
    pub fn new(title: impl Into<String>, slug: impl Into<String>) -> Self {
        SpecDir {
            title: title.into(),
            slug: slug.into(),
            entries: vec![],
        }
    }

    pub fn add_dir(&mut self, dir: SpecDir) {
        self.entries.push(SpecEntry::Dir(dir));
    }

    pub fn add_page(&mut self, page: SpecPage) {
        self.entries.push(SpecEntry::Page(page));
    }

    pub fn write_to_disk(&self, fs: &mut CodegenFileSystem, parent_dir: &Path) -> Result<()> {
        let SpecDir {
            title,
            slug,
            entries,
        } = self;

        let current_dir = parent_dir.join(slug);

        let mut index = String::new();
        writeln!(index, "# {title}")?;
        writeln!(index)?;

        let mut navigation = String::new();
        writeln!(navigation, "- [{title}](./index.md)")?;

        for entry in entries {
            match entry {
                SpecEntry::Dir(
                    child @ SpecDir {
                        title: child_title,
                        slug: child_slug,
                        entries: _,
                    },
                ) => {
                    writeln!(index, "- [{child_title}](./{child_slug}/index.md)")?;
                    writeln!(navigation, "- [{child_title}](./{child_slug}/)")?;

                    child.write_to_disk(fs, &current_dir)?;
                }
                SpecEntry::Page(SpecPage {
                    title: child_title,
                    slug: child_slug,
                    contents,
                }) => {
                    writeln!(index, "- [{child_title}](./{child_slug}.md)")?;
                    writeln!(navigation, "- [{child_title}](./{child_slug}.md)")?;

                    fs.write_file_raw(current_dir.join(format!("{child_slug}.md")), contents)?;
                }
            }
        }

        fs.write_file_raw(current_dir.join("index.md"), index)?;
        fs.write_file_raw(current_dir.join(".navigation.md"), navigation)?;
        Ok(())
    }
}

pub struct SpecPage {
    title: String,
    slug: String,
    contents: String,
}

impl SpecPage {
    pub fn new(
        title: impl Into<String>,
        slug: impl Into<String>,
        contents: impl Into<String>,
    ) -> Self {
        SpecPage {
            title: title.into(),
            slug: slug.into(),
            contents: contents.into(),
        }
    }
}
