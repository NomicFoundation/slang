use std::path::PathBuf;

use codegen_schema::types::LanguageDefinition;
use semver::Version;

use crate::{markdown::MarkdownWriter, navigation::NavigationEntry, snippets::Snippets};

pub fn generate_supported_versions_page(language: &LanguageDefinition) -> NavigationEntry {
    let versions = &language.versions;
    let version_breaks = language.collect_version_breaks();

    let mut page = MarkdownWriter::new();

    page.write_header(1, "Supported Versions");

    page.write_newline();
    page.write_text(&format!("This specification compiles information from {all_count} publicly released versions of {language}. Among which, {breaks_count} versions had syntax-related changes:", all_count = versions.len(), language = language.title, breaks_count = version_breaks.len()));

    page.write_newline();
    for version in version_breaks {
        page.write_list_link(&version.to_string(), &format!("../grammar/v{version}/"));
    }

    page.write_newline();
    page.write_text("Here is the full list of supported versions:");

    page.write_newline();
    for version in versions {
        page.write_text(&format!("`{version}`"));
    }

    NavigationEntry::Page {
        title: "Supported Versions".to_owned(),
        path: "supported-versions".to_owned(),
        contents: page.into_string(),
    }
}

pub fn generate_grammar_dir(language: &LanguageDefinition, snippets: &Snippets) -> NavigationEntry {
    let mut pages = Vec::<NavigationEntry>::new();

    for version in language.collect_version_breaks() {
        pages.push(NavigationEntry::Page {
            title: format!("v{version}"),
            path: format!("v{version}"),
            contents: generate_grammar_page(language, snippets, &version),
        });
    }

    NavigationEntry::Directory {
        title: "Grammar".to_owned(),
        path: "grammar".to_owned(),
        children: pages,
    }
}

fn generate_grammar_page(
    language: &LanguageDefinition,
    snippets: &Snippets,
    version: &Version,
) -> String {
    let mut page = MarkdownWriter::new();

    page.write_header(1, "Grammar");

    for section in &language.sections {
        page.write_newline();
        page.write_header(2, &section.title);

        for topic in &section.topics {
            let snippets: Vec<PathBuf> = topic
                .productions
                .iter()
                .filter_map(|production| snippets.get_snippet_path(production, version))
                .collect();

            if snippets.is_empty() {
                continue;
            }

            page.write_newline();
            page.write_header(3, &topic.title);

            page.write_newline();
            for snippet_path in snippets {
                page.write_snippet(&snippet_path);
            }
        }
    }

    page.into_string()
}
