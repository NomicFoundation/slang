use std::{io::Write, path::PathBuf};

use codegen_ebnf::ebnf_writer::EBNFWritable;
use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;

use super::{
    html_ebnf_writer::{HTMLEBNFWriter, SpecProductionContext},
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
        title: grammar.title.clone(),
        file_path: Some(grammar_path.clone()),
    });

    let mut w: Vec<u8> = Vec::new();
    writeln!(w, "# {}", grammar.title).unwrap();
    writeln!(w).unwrap();
    writeln!(w, "<!-- markdownlint-disable no-inline-html -->").unwrap();
    writeln!(w, "<!-- markdownlint-disable no-space-in-emphasis -->").unwrap();
    writeln!(w, "<!-- cSpell:disable -->").unwrap();

    grammar.sections.iter().for_each(|section| {
        writeln!(w).unwrap();
        writeln!(w, "## {}", section.title).unwrap();

        for topic in &section.topics {
            if topic.productions.is_empty() {
                continue;
            }

            writeln!(w).unwrap();
            writeln!(w, "### {}", topic.title).unwrap();

            for production in &topic.productions {
                writeln!(w).unwrap();
                let mut writer = HTMLEBNFWriter {
                    w: &mut w,
                    context: &context,
                };
                production.write_ebnf("", &mut writer);
            }
        }
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
            .map(|(_, production)| (production.name().clone(), ".".to_string()))
            .collect(),
    };
    context
}
