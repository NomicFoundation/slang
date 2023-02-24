use std::{collections::HashMap, io::Write, path::PathBuf};

use codegen_ebnf::ebnf_writer::EBNFWritable;
use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use super::{
    html_ebnf_writer::{HTMLEBNFWriter, SpecProductionContext},
    NavigationEntry,
};

pub fn generate_spec_sections(
    codegen: &mut CodegenContext,
    grammar: &Grammar,
    generated_folder: &PathBuf,
    entries: &mut Vec<NavigationEntry>,
) {
    let context = generate_context(grammar);

    grammar
        .sections
        .iter()
        .enumerate()
        .for_each(|(section_index, section)| {
            entries.push(NavigationEntry {
                indentation_level: 1,
                title: format!("{}. {}", section_index + 1, &section.title),
                file_path: None,
            });

            section
                .topics
                .iter()
                .enumerate()
                .for_each(|(topic_index, topic)| {
                    let topic_slug = generate_topic_slug(grammar, section_index, topic_index);
                    let topic_file = generated_folder.join(&topic_slug).join("index.md");

                    entries.push(NavigationEntry {
                        indentation_level: 2,
                        title: format!(
                            "{}.{}. {}",
                            section_index + 1,
                            topic_index + 1,
                            &topic.title
                        ),
                        file_path: Some(topic_file.clone()),
                    });

                    let notes_file = generated_folder
                        .parent()
                        .unwrap()
                        .join("notes")
                        .join(&topic_slug)
                        .join("index.md");

                    entries.push(NavigationEntry {
                        indentation_level: 2,
                        title: "\"\"".to_string(),
                        file_path: Some(notes_file.clone()),
                    });

                    let mut w : Vec<u8> = Vec::new();

                    writeln!(w, "<!-- markdownlint-disable no-inline-html -->").unwrap();
                    writeln!(w, "<!-- markdownlint-disable no-space-in-emphasis -->").unwrap();
                    writeln!(w, "<!-- cSpell:disable -->").unwrap();

                    writeln!(w).unwrap();
                    writeln!(w, "# {}", topic.title).unwrap();

                    if !topic.productions.is_empty() {
                        writeln!(w).unwrap();
                        writeln!(w, "<div class=\"admonition summary\">").unwrap();
                        writeln!(w, "<p class=\"admonition-title\">Grammar</p>").unwrap();

                        topic.productions
                            .iter()
                            .for_each(|production| {
                                writeln!(w).unwrap();
                                let mut writer = HTMLEBNFWriter { w: &mut w, context: &context };
                                production.write_ebnf("", &mut writer);
                            });

                        writeln!(w).unwrap();
                        writeln!(w, "</div>").unwrap();
                    }

                    assert!(notes_file.exists(), "Notes file does not exist: {notes_file:?}");

                    assert_eq!(
                        codegen.read_file(&notes_file).unwrap().lines().nth(0),
                        Some("<!-- markdownlint-configure-file { \"first-line-heading\": { \"level\": 2 } } -->"),
                        "First line of notes file should be the markdownlint configuration: {:?}", &notes_file
                    );

                    writeln!(w).unwrap();
                    writeln!(w, "--8<-- \"specification/notes/{topic_slug}/index.md\"").unwrap();

                    codegen.write_file(&topic_file, &String::from_utf8(w).unwrap()).unwrap();
                });
        });
}

fn generate_context(grammar: &Grammar) -> SpecProductionContext {
    let mut productions_location = HashMap::<String, String>::new();

    for (section_index, section) in grammar.sections.iter().enumerate() {
        for (topic_index, topic) in section.topics.iter().enumerate() {
            let topic_slug = generate_topic_slug(grammar, section_index, topic_index);

            for production in &topic.productions {
                productions_location
                    .insert(production.name().clone(), format!("../../{topic_slug}"));
            }
        }
    }

    return SpecProductionContext {
        grammar,
        productions_location,
    };
}

fn generate_topic_slug(grammar: &Grammar, section_index: usize, topic_index: usize) -> String {
    let section = grammar.sections.get(section_index).unwrap();
    let topic = section.topics.get(topic_index).unwrap();

    return format!(
        "{:0>2}-{}/{:0>2}-{}",
        section_index + 1,
        section.title.to_ascii_lowercase().replace(" ", "-"),
        topic_index + 1,
        topic.title.to_ascii_lowercase().replace(" ", "-"),
    );
}
