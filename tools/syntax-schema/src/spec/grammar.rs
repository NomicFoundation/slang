use std::{fs::File, io::Write, path::PathBuf};

use crate::schema::Grammar;

use super::{
    productions::{write_production, SpecProductionContext},
    NavigationEntry,
};

pub fn generate_spec_grammar(
    grammar: &Grammar,
    generated_folder: &PathBuf,
    entries: &mut Vec<NavigationEntry>,
) {
    let context = generate_context(grammar);
    let grammar_path = generated_folder.join("00-grammar").join("index.md");

    std::fs::create_dir_all(grammar_path.parent().unwrap()).unwrap();

    entries.push(NavigationEntry {
        indentation_level: 1,
        title: grammar.manifest.title.clone(),
        file_path: Some(grammar_path.clone()),
    });

    let mut w = File::create(grammar_path).expect("Unable to create file");
    writeln!(w, "# {}", grammar.manifest.title).unwrap();
    writeln!(w).unwrap();
    writeln!(w, "<!-- markdownlint-disable no-inline-html -->").unwrap();
    writeln!(w, "<!-- markdownlint-disable no-space-in-emphasis -->").unwrap();
    writeln!(w, "<!-- cSpell:disable -->").unwrap();

    grammar.manifest.sections.iter().for_each(|section| {
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

                    grammar
                        .productions
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

    writeln!(w).unwrap();
    writeln!(w, "--8<-- \"specification/notes/00-grammar/index.md\"").unwrap();
}

fn generate_context(grammar: &Grammar) -> SpecProductionContext {
    let context = SpecProductionContext {
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
