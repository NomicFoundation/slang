// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use core::fmt;

use semver::Version;

use super::CommandError;
use crate::bindings::{self, Bindings, Definition, Reference};
use crate::cursor::Cursor;

pub fn execute(file_path_string: &str, version: Version) -> Result<(), CommandError> {
    // TODO: we probably want the language default path resolver here, not the default one
    // That is if we want to keep supporting this command.
    let mut bindings = bindings::create(version.clone());
    let parse_output = super::parse::parse_source_file(file_path_string, version, |_| ())?;
    let tree_cursor = parse_output.create_tree_cursor();

    bindings.add_file(file_path_string, tree_cursor);

    print_definitions(&bindings);
    print_references(&bindings);

    Ok(())
}

fn print_definitions(bindings: &Bindings) {
    println!("\nAll definitions found:");
    for definition in bindings.all_definitions() {
        println!("{}", DisplayDefinition(&definition));
    }
}

fn print_references(bindings: &Bindings) {
    println!("\nAll references found:");
    for reference in bindings.all_references() {
        println!("{}", DisplayReference(&reference));
        if let Some(def) = reference.jump_to_definition() {
            println!("  -> {}", DisplayDefinition(&def));
        } else {
            println!("  -> No definition found");
        }
    }
}

struct DisplayRange<'a>(&'a Cursor);

impl<'a> fmt::Display for DisplayRange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let range = self.0.text_range();
        write!(f, "{}..{}", range.start, range.end)
    }
}

struct DisplayDefinition<'a>(&'a Definition<'a>);

impl<'a> fmt::Display for DisplayDefinition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = self.0.get_file().unwrap_or("<unkwown>");
        if let Some(cursor) = self.0.get_cursor() {
            let location = DisplayRange(&cursor);
            let identifier = cursor.node().unparse();
            write!(f, "`{identifier}` defined at {location} in {file}")
        } else {
            write!(
                f,
                "Definition without available cursor: {definition:?} in {file}",
                definition = self.0
            )
        }
    }
}

struct DisplayReference<'a>(&'a Reference<'a>);

impl<'a> fmt::Display for DisplayReference<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = self.0.get_file().unwrap_or("<unkwown>");
        if let Some(cursor) = self.0.get_cursor() {
            let location = DisplayRange(&cursor);
            let identifier = cursor.node().unparse();
            write!(f, "`{identifier}` referenced at {location} in {file}")
        } else {
            write!(
                f,
                "Reference without available cursor: {reference:?} in {file}",
                reference = self.0
            )
        }
    }
}
