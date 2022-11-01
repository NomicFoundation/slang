use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{bail, Context, Result};

use crate::context::CodegenContext;

pub(crate) fn run_command(
    codegen: &CodegenContext,
    parts: &Vec<&str>,
    stdin: Option<&str>,
) -> Result<String> {
    let mut command = Command::new(&parts[0]);
    let command = command
        .args(&parts[1..])
        .current_dir(&codegen.repo_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());

    let mut process = command.spawn()?;

    if let Some(stdin) = stdin {
        process
            .stdin
            .as_mut()
            .context(format!("failed to capture stdin of command: {command:?}"))?
            .write_all(stdin.as_bytes())?;
    }

    let output = process.wait_with_output()?;

    let stdout = String::from_utf8(output.stdout)
        .context(format!("Failed to read stdout from command: {command:?}"))?;

    let status = output.status;
    if !status.success() {
        let stderr = String::from_utf8(output.stderr)
            .context(format!("Failed to read stderr from command: {command:?}"))?;

        bail!("Command failed with {status}: {command:?}\n{stdout}\n{stderr}");
    }

    return Ok(stdout);
}
