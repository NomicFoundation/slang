use codegen::{build_utils::assert_no_changes_in_ci, schema::Grammar};
use std::path::PathBuf;

fn main() {
    assert_no_changes_in_ci();

    let this_crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // Load Manifest

    let manifest_input = this_crate_dir.join("grammar").join("manifest.yml");
    let grammar = Grammar::from_manifest(&manifest_input);

    // Generate Grammar

    let grammar_dir = this_crate_dir.join("src").join("generated");
    grammar.generate_chumsky(&grammar_dir);
    grammar.generate_ebnf(&grammar_dir.join("derived.ebnf"));

    // Generate Spec

    let documentation_dir = this_crate_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("documentation");
    grammar.generate_spec(&documentation_dir);

    assert_no_changes_in_ci();
}
