use std::{io::Write, path::PathBuf};

use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;

use super::production::ProductionEBNFExtensions;

pub trait GrammarEBNFExtensions {
    fn generate_ebnf(&self, codegen: &CodegenContext, path: &PathBuf);
}

impl GrammarEBNFExtensions for Grammar {
    fn generate_ebnf(&self, codegen: &CodegenContext, output_path: &PathBuf) {
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

        codegen
            .write_file(output_path, &String::from_utf8(w).unwrap())
            .unwrap();
    }
}
