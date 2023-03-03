mod markdown;
mod navigation;

use std::path::PathBuf;

use codegen_schema::types::grammar::{Grammar, GrammarSection, GrammarTopic};
use codegen_utils::context::CodegenContext;

use crate::{
    public::{markdown::MarkdownWriter, navigation::NavigationEntry},
    snippets::Snippets,
};

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
        let root_entry = NavigationEntry::Directory {
            title: "",
            path: "public",
            children: vec![
                NavigationEntry::Page {
                    title: "Grammar",
                    path: "grammar",
                    contents: self.grammar_page(&codegen.repo_root),
                },
                NavigationEntry::Directory {
                    title: "Reference",
                    path: "reference",
                    children: Self::map_entries(&self.grammar.sections, |section| {
                        NavigationEntry::Directory {
                            title: &section.title,
                            path: &section.path,
                            children: Self::map_entries(&section.topics, |topic| {
                                NavigationEntry::Page {
                                    title: &topic.title,
                                    path: &topic.path,
                                    contents: self.topic_page(section, topic, &codegen.repo_root),
                                }
                            }),
                        }
                    }),
                },
            ],
        };

        return root_entry.write_files(codegen, output_dir);
    }

    fn grammar_page(&self, repo_root: &PathBuf) -> String {
        let mut page = MarkdownWriter::new();

        for section in &self.grammar.sections {
            page.write_header(2, &section.title);
            page.write_newline();

            for topic in &section.topics {
                if topic.productions.is_empty() {
                    continue;
                }

                page.write_header(3, &topic.title);
                page.write_newline();

                for production in topic.productions.keys() {
                    let snippet_path = self.snippets.get_snippet_path(production);
                    page.write_snippet(repo_root, &snippet_path);
                }

                page.write_newline();
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

        if !topic.productions.is_empty() {
            page.write_header(2, "Grammar");
            page.write_newline();

            for production in topic.productions.keys() {
                let snippet_path = self.snippets.get_snippet_path(production);
                page.write_snippet(repo_root, &snippet_path);
            }

            page.write_newline();
        }

        let notes_path = self
            .grammar
            .manifest_dir
            .join(&section.path)
            .join(&topic.path)
            .join(GrammarTopic::notes_file());

        page.write_snippet(repo_root, &notes_path);

        return page.to_string();
    }

    fn map_entries<T>(
        entries: &'a Vec<T>,
        mapper: impl Fn(&'a T) -> NavigationEntry<'a>,
    ) -> Vec<NavigationEntry<'a>> {
        return entries.iter().map(mapper).collect();
    }
}
