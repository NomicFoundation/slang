use std::{fs::File, io::Write, path::PathBuf};

use crate::schema::*;

impl Grammar {
    pub fn generate_ebnf(&self, output_path: &PathBuf) {
        let mut w = File::create(output_path).expect("Unable to create file");

        let mut first = true;
        for p in self.productions.iter().flat_map(|(_, p)| p) {
            if first {
                first = false;
            } else {
                writeln!(w).unwrap();
            }
            writeln!(w, "{}", p.generate_ebnf(self).join("\n")).unwrap();
        }
    }
}
