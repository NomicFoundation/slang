use std::{io::Write, path::PathBuf};

use codegen_schema::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use crate::{
    productions::{write_production, SpecProductionContext},
    NavigationEntry,
};

pub fn generate_spec_grammar(
    codegen: &mut CodegenContext,
    grammar: &Grammar,
    generated_folder: &PathBuf,
    entries: &mut Vec<NavigationEntry>,
) {
    let context = generate_context(grammar);
    let grammar_path = generated_folder.join("00-grammar").join("index.md");

    entries.push(NavigationEntry {
        indentation_level: 1,
        title: grammar.manifest.title.clone(),
        file_path: Some(grammar_path.clone()),
    });

    let mut w: Vec<u8> = Vec::new();
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

    codegen
        .write_file(&grammar_path, &String::from_utf8(w).unwrap())
        .unwrap();
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
                    .map(|production| (production.name.clone(), ".".to_string()))
            })
            .collect(),
    };
    context
}
