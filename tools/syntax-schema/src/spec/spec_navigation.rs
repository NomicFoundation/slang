use crate::schema::Grammar;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

impl Grammar {
    pub fn generate_spec_navigation(&self, output_path: &PathBuf) {
        let mut w = File::create(output_path).expect("Unable to create file");

        writeln!(w, "nav:").unwrap();
        writeln!(w, "  - Grammar: grammar.md").unwrap();
    }
}
