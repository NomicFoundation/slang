use std::path::PathBuf;

use codegen_schema::types::grammar::Grammar;
use semver::Version;

use crate::{markdown::MarkdownWriter, navigation::NavigationEntry, snippets::Snippets};

pub fn generate_grammar_dir(
    grammar: &Grammar,
    snippets: &Snippets,
    repo_root: &PathBuf,
) -> NavigationEntry {
    let mut pages = Vec::<NavigationEntry>::new();

    pages.push(NavigationEntry::Page {
        title: "Versions".to_owned(),
        path: "versions".to_owned(),
        contents: generate_versions_page(grammar),
    });

    for version in grammar.collect_version_breaks() {
        let version_path = version_path(&version);
        pages.push(NavigationEntry::Page {
            title: version_path.to_owned(),
            path: version_path,
            contents: generate_grammar_page(grammar, snippets, repo_root, &version),
        });
    }

    return NavigationEntry::Directory {
        title: "Grammar".to_owned(),
        path: "grammar".to_owned(),
        children: pages,
    };
}

fn generate_versions_page(grammar: &Grammar) -> String {
    let versions = &grammar.versions;
    let version_breaks = grammar.collect_version_breaks();

    let mut page = MarkdownWriter::new();

    page.write_header(1, "Versions");

    page.write_newline();
    page.write_text(&format!("This specification compiles information from {all_count} publicly released versions of {language}. Among which, {breaks_count} versions had syntax-related changes:", all_count = versions.len(), language = grammar.title, breaks_count = version_breaks.len()));

    page.write_newline();
    for version in version_breaks {
        let version_path = version_path(&version);
        page.write_list_link(&version_path, &format!("../{version_path}/"));
    }

    page.write_newline();
    page.write_text("Here is the full list of supported versions:");

    page.write_newline();
    for version in versions {
        page.write_text(&format!("`{version}`"));
    }

    return page.to_string();
}

fn generate_grammar_page(
    grammar: &Grammar,
    snippets: &Snippets,
    repo_root: &PathBuf,
    version: &Version,
) -> String {
    let mut page = MarkdownWriter::new();

    page.write_header(1, "Grammar");

    for section in &grammar.sections {
        page.write_newline();
        page.write_header(2, &section.title);

        for topic in &section.topics {
            let snippets: Vec<PathBuf> = topic
                .productions
                .values()
                .filter_map(|production| snippets.get_snippet_path(production, version))
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

fn version_path(version: &Version) -> String {
    return format!("v{version}", version = version.to_string());
}
