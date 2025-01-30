use crate::codegen::CodegenFileSystem;
use crate::commands::Command;
use crate::paths::{FileWalker, PathExtensions};

use anyhow::Result;
use std::path::Path;

/// Invoke `ldw process-models` on `input_dir` to generate Rust code in
/// `output_dir`.
pub fn process_models(
    file_system: &mut CodegenFileSystem,
    input_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<()> {
    let input_dir = input_dir.as_ref();
    let output_dir = output_dir.as_ref();

    let mut model_fqns = find_model_fqns(input_dir)?;
    let mut name_args: Vec<String> = vec!["--name".into()];
    name_args.append(&mut model_fqns);

    Command::new("ts-node")
        .current_dir(Path::repo_path("crates/codegen/ldw"))
        .args(["-P", "./tsconfig.json"])
        .arg("src-ts/cli/ldw.ts")
        .arg("process-model")
        .args(["--in-dir", input_dir.to_str().unwrap()])
        .args(["--out-dir", output_dir.to_str().unwrap()])
        .args(["--language", "rust"])
        .args(name_args)
        .run();

    file_system.mark_generated_dir(output_dir)?;

    Ok(())
}

/// Search the specified directory for model files (`*.model`). Returns a list of
/// fully qualified names (FQNs) for these models which can be passed to the `--name`
/// option of the `ldw process-models` command.
fn find_model_fqns(input_dir: &Path) -> Result<Vec<String>> {
    let mut model_fqns = Vec::new();

    for file_path in FileWalker::from_directory(input_dir).find_all()? {
        if file_path.extension().unwrap() == "model" {
            let model_path = file_path.parent().unwrap().strip_prefix(input_dir)?;
            let fqn = model_path.to_str().unwrap().replace('/', "::");

            model_fqns.push(fqn);
        }
    }

    Ok(model_fqns)
}
