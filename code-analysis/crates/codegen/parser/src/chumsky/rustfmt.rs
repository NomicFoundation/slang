use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use codegen_utils::write_generated_file;

pub fn format_and_write_source(path: &PathBuf, source: String) {
    // Make it possible to debug the code even when rustfmt dies
    // by writing the unformatted code to the file.
    let formatted_src = rustfmt(&source);
    if formatted_src == "" {
        write_generated_file(path, source.as_str());
    } else {
        write_generated_file(path, &formatted_src);
    };
}

fn rustfmt(source: &String) -> String {
    let mut child = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let source = source.clone();
    std::thread::spawn(move || {
        stdin
            .write_all(source.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");

    String::from_utf8(output.stdout).unwrap()
}
