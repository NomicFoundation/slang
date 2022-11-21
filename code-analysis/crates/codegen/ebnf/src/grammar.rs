use std::{io::Write, path::PathBuf};

use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;

use super::production::ProductionEBNFGeneratorExtensions;

pub trait GrammarEBNFGeneratorExtensions {
    fn generate_ebnf(&self, codegen: &mut CodegenContext, output_dir: &PathBuf);
}

impl GrammarEBNFGeneratorExtensions for Grammar {
    fn generate_ebnf(&self, codegen: &mut CodegenContext, output_dir: &PathBuf) {
        let mut w: Vec<u8> = Vec::new();

        writeln!(w, "(*").unwrap();
        writeln!(w, " * {}", self.manifest.title).unwrap();
        writeln!(w, " *)").unwrap();

        for section in &self.manifest.sections {
            for topic in &section.topics {
                match &topic.definition {
                    None => {}
                    Some(definition) => {
                        writeln!(w).unwrap();
                        writeln!(w, "(*").unwrap();
                        writeln!(w, " * {}: {}", section.title, topic.title).unwrap();
                        writeln!(w, " *)").unwrap();

                        for production in self.productions.get(definition).unwrap() {
                            writeln!(w).unwrap();
                            writeln!(w, "{}", production.generate_ebnf(self).join("\n")).unwrap();
                        }
                    }
                }
            }
        }

        let output_path = output_dir.join("grammar.ebnf");
        codegen
            .write_file(&output_path, &String::from_utf8(w).unwrap())
            .unwrap();
    }
}
