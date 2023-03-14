use std::path::PathBuf;

use codegen_schema::types::grammar::{Grammar, GrammarSection, GrammarTopic};
use codegen_utils::context::CodegenContext;
use semver::Version;

use crate::{markdown::MarkdownWriter, navigation::NavigationEntry, snippets::Snippets};

pub struct PublicPages<'a> {
    grammar: &'a Grammar,
    snippets: &'a Snippets<'a>,
}

impl<'a> PublicPages<'a> {
    pub fn new(grammar: &'a Grammar, snippets: &'a Snippets<'a>) -> Self {
        return Self { grammar, snippets };
    }

    pub fn write_files(
        &self,
        codegen: &'a mut CodegenContext,
        output_dir: &'a PathBuf,
    ) -> anyhow::Result<()> {
        let versions = self.grammar.collect_version_breaks();

        let root_entry = NavigationEntry::Directory {
            title: "".to_owned(),
            path: "public".to_owned(),
            children: vec![
                NavigationEntry::Directory {
                    title: "Grammar".to_owned(),
                    path: "grammar".to_owned(),
                    children: Self::map_entries(versions.iter().rev(), |version| {
                        let version_string = format!("v{v}", v = version.to_string());
                        NavigationEntry::Page {
                            title: version_string.to_owned(),
                            path: version_string.to_owned(),
                            contents: self.grammar_page(&codegen.repo_root, version),
                        }
                    }),
                },
                NavigationEntry::Directory {
                    title: "Reference".to_owned(),
                    path: "reference".to_owned(),
                    children: Self::map_entries(self.grammar.sections.iter(), |section| {
                        NavigationEntry::Directory {
                            title: section.title.to_owned(),
                            path: section.path.to_owned(),
                            children: Self::map_entries(section.topics.iter(), |topic| {
                                NavigationEntry::Page {
                                    title: topic.title.to_owned(),
                                    path: topic.path.to_owned(),
                                    contents: self.topic_page(&section, &topic, &codegen.repo_root),
                                }
                            }),
                        }
                    }),
                },
            ],
        };

        return root_entry.write_files(codegen, output_dir);
    }

    fn grammar_page(&self, repo_root: &PathBuf, version: &Version) -> String {
        let mut page = MarkdownWriter::new();

        page.write_header(1, "Grammar");

        for section in &self.grammar.sections {
            page.write_newline();
            page.write_header(2, &section.title);

            for topic in &section.topics {
                let snippets: Vec<PathBuf> = topic
                    .productions
                    .values()
                    .filter_map(|production| self.snippets.get_snippet_path(production, version))
                    .collect();

                if snippets.is_empty() {
                    continue;
                }

                page.write_newline();
                page.write_header(3, &topic.title);

                page.write_newline();
                for snippet_path in snippets {
                    page.write_snippet(repo_root, &snippet_path);
                }
            }
        }

        return page.to_string();
    }

    fn topic_page(
        &self,
        section: &GrammarSection,
        topic: &GrammarTopic,
        repo_root: &PathBuf,
    ) -> String {
        let mut page = MarkdownWriter::new();

        page.write_header(1, &topic.title);

        page.write_newline();
        page.write_snippet(
            repo_root,
            &self
                .grammar
                .manifest_dir
                .join(&section.path)
                .join(&topic.path)
                .join(GrammarTopic::notes_file()),
        );

        return page.to_string();
    }

    fn map_entries<T>(
        entries: impl Iterator<Item = T>,
        mapper: impl FnMut(T) -> NavigationEntry,
    ) -> Vec<NavigationEntry> {
        return entries.into_iter().map(mapper).collect();
    }
}
