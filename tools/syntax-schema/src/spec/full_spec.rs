use crate::schema::Grammar;
use std::io::Write;
use std::{fs::File, path::PathBuf};

use super::generator::{write_production, SpecGeneratorContext};

impl Grammar {
    pub fn generate_full_spec(&self, output_folder: &PathBuf) {
        let spec_dir = output_folder;
        if spec_dir.exists() {
            std::fs::remove_dir_all(&spec_dir).unwrap();
        }

        let context = generate_context(self);

        self.manifest
            .sections
            .iter()
            .enumerate()
            .for_each(|(section_index, section)| {
                section
                    .topics
                    .iter()
                    .enumerate()
                    .for_each(|(topic_index, topic)| {
                        let topic_file = spec_dir
                            .join(self.generate_topic_slug(section_index, topic_index))
                            .join("index.md");

                        std::fs::create_dir_all(topic_file.parent().unwrap()).unwrap();

                        let mut w = File::create(topic_file).expect("Unable to create file");
                        writeln!(w, "# {}", topic.title).unwrap();
                        writeln!(w).unwrap();
                        writeln!(w, "<!-- markdownlint-disable no-inline-html -->").unwrap();
                        writeln!(w, "<!-- markdownlint-disable no-space-in-emphasis -->").unwrap();
                        writeln!(w, "<!-- cSpell:disable -->").unwrap();

                        match &topic.definition {
                            None => {}
                            Some(definition) => {
                                writeln!(w).unwrap();
                                writeln!(w, "<div class=\"admonition summary\">").unwrap();
                                writeln!(w, "<p class=\"admonition-title\">Grammar</p>").unwrap();

                                self.productions.get(definition).unwrap().iter().for_each(
                                    |production| {
                                        writeln!(w).unwrap();
                                        write_production(&mut w, production, &context);
                                    },
                                );

                                writeln!(w).unwrap();
                                writeln!(w, "</div>").unwrap();
                            }
                        }

                        match &topic.notes {
                            None => {}
                            Some(notes) => {
                                writeln!(w).unwrap();
                                writeln!(&w, "--8<-- \"{}\"", notes).unwrap();
                            }
                        }
                    });
            });
    }
}

fn generate_context(grammar: &Grammar) -> SpecGeneratorContext {
    let context = SpecGeneratorContext {
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
