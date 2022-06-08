use super::generator::{write_production, SpecGeneratorContext};
use crate::schema::Grammar;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

impl Grammar {
    pub fn generate_grammar_spec(&self, output_path: &PathBuf) {
        let context = generate_context(self);

        let mut w = File::create(output_path).expect("Unable to create file");

        writeln!(w, "# Grammar").unwrap();
        writeln!(w).unwrap();
        writeln!(w, "<!-- markdownlint-disable no-inline-html -->").unwrap();
        writeln!(w, "<!-- markdownlint-disable no-space-in-emphasis -->").unwrap();
        writeln!(w, "<!-- cSpell:disable -->").unwrap();

        self.manifest.sections.iter().for_each(|section| {
            writeln!(w).unwrap();
            writeln!(w, "## {}", section.title).unwrap();

            section
                .topics
                .iter()
                .for_each(|topic| match &topic.definition {
                    None => {}
                    Some(definition) => {
                        writeln!(w).unwrap();
                        writeln!(w, "### {}", topic.title).unwrap();

                        self.productions
                            .get(definition)
                            .unwrap()
                            .iter()
                            .for_each(|production| {
                                writeln!(w).unwrap();
                                write_production(&mut w, production, &context);
                            });
                    }
                })
        });
    }
}

fn generate_context(grammar: &Grammar) -> SpecGeneratorContext {
    let context = SpecGeneratorContext {
        grammar: &grammar,
        productions_location: grammar
            .productions
            .iter()
            .flat_map(|(_, productions)| {
                productions
                    .iter()
                    .map(|production| (production.name.clone(), "".to_string()))
            })
            .collect(),
    };
    context
}
