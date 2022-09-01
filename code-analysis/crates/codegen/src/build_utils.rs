use std::{path::PathBuf, process::Command};

pub fn read_source_file(file_path: &PathBuf) -> String {
    rerun_if_changed(file_path);

    let contents = std::fs::read_to_string(file_path)
        .expect(&format!("Cannot read source file: {file_path:?}"));

    return contents;
}

pub fn write_generated_file(file_path: &PathBuf, contents: &str) {
    rerun_if_changed(file_path);

    let contents = format!("{}\n\n{}", generate_header(file_path), contents);

    if file_path.exists() {
        let existing_contents = std::fs::read_to_string(file_path)
            .expect(&format!("Cannot read existing file: {file_path:?}"));

        // To respect Cargo incrementability, don't touch the file if it is already up to date.
        if contents == existing_contents {
            return;
        }
    }

    std::fs::create_dir_all(
        file_path
            .parent()
            .expect(&format!("Cannot get parent directory of: {file_path:?}")),
    )
    .expect(&format!("Cannot create parent directory of: {file_path:?}"));

    std::fs::write(file_path, contents)
        .expect(&format!("Cannot write generated file: {file_path:?}"));
}

pub fn delete_generated_file(file_path: &PathBuf) {
    rerun_if_changed(file_path);

    if file_path.exists() {
        std::fs::remove_file(file_path)
            .expect(&format!("Cannot delete generated file: {file_path:?}"));
    }
}

pub fn assert_no_changes_in_ci() {
    if !should_verify_generated_files() {
        return;
    }

    run_git_command(&["status"]);
    run_git_command(&["diff-index", "HEAD", "--quiet"]);

    fn run_git_command(args: &[&str]) {
        let result = Command::new("git")
            .args(args)
            .spawn()
            .expect("Failed to invoke `git`")
            .wait()
            .expect("Failed to invoke `git`");

        assert!(
            result.success(),
            "Found codegen changes. Please rerun `cargo build` locally and review changes."
        );
    }
}

fn should_verify_generated_files() -> bool {
    std::env::var("SLANG_VERIFY_GENERATED_FILES").is_ok()
}

fn generate_header(file_path: &PathBuf) -> String {
    let warning_line = "This file is generated via `cargo build`. Please don't edit by hand.";

    let extension = file_path
        .extension()
        .expect(&format!("Cannot get extension of file: {file_path:?}"))
        .to_str()
        .expect(&format!("Cannot get extension of file: {file_path:?}"));

    return match extension {
        "ebnf" => format!("(* {warning_line} *)"),
        "md" => format!("<!-- {warning_line} -->"),
        "rs" => format!("// {warning_line}"),
        "yml" => format!("# {warning_line}"),
        _ => panic!("Unsupported extension: {extension}"),
    };
}

fn rerun_if_changed(file_path: &PathBuf) {
    let file_path = file_path.to_str().unwrap();
    println!("cargo:rerun-if-changed={file_path}");
}
