use std::{io::Write, path::PathBuf};

use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;

use super::{
    productions::{write_production, SpecProductionContext},
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
        .manifest
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
                    let topic_slug = grammar.generate_topic_slug(section_index, topic_index);
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

                    match &topic.definition {
                        None => {}
                        Some(definition) => {
                            writeln!(w).unwrap();
                            writeln!(w, "<div class=\"admonition summary\">").unwrap();
                            writeln!(w, "<p class=\"admonition-title\">Grammar</p>").unwrap();

                            grammar
                                .productions
                                .get(definition)
                                .unwrap()
                                .iter()
                                .for_each(|production| {
                                    writeln!(w).unwrap();
                                    write_production(&mut w, production, &context);
                                });

                            writeln!(w).unwrap();
                            writeln!(w, "</div>").unwrap();
                        }
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
    let context = SpecProductionContext {
        grammar: grammar,
        productions_location: grammar
            .manifest
            .sections
            .iter()
            .enumerate()
            .flat_map(|(section_index, section)| {
                section
                    .topics
                    .iter()
                    .enumerate()
                    .flat_map(move |(topic_index, topic)| {
                        topic.definition.iter().flat_map(move |definition| {
                            grammar.productions.get(definition).unwrap().iter().map(
                                move |production| {
                                    (
                                        production.name.clone(),
                                        format!(
                                            "../../{}",
                                            grammar.generate_topic_slug(section_index, topic_index)
                                        ),
                                    )
                                },
                            )
                        })
                    })
            })
            .collect(),
    };
    context
}
