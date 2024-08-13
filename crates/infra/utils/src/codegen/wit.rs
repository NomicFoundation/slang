use std::path::Path;

use anyhow::Result;
use itertools::Itertools;

use crate::cargo::CargoWorkspace;
use crate::commands::Command;
use crate::paths::{FileWalker, PathExtensions};

pub fn generate_rust_bindings(input_wit: &Path) -> Result<String> {
    CargoWorkspace::install_binary("wit-bindgen-cli")?;

    let output_dir = tempfile::tempdir()?;

    Command::new("wit-bindgen")
        .args(["rust", input_wit.unwrap_str()])
        .flag("--pub-export-macro")
        .property("--default-bindings-module", "$crate::wit::slang")
        .property("--out-dir", output_dir.path().unwrap_str())
        .run();

    let generated_files = FileWalker::from_directory(output_dir.path())
        .find_all()?
        .collect_vec();

    assert_eq!(generated_files.len(), 1);

    generated_files[0].read_to_string()
}
