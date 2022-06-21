use std::{fs::File, io::Write, path::PathBuf};

use crate::schema::*;

impl Grammar {
    pub fn generate_ebnf(&self, output_path: &PathBuf) {
        let mut w = File::create(output_path).expect("Unable to create file");

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
    }
}
