use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{bail, Context, Result};

pub fn run_command(cwd: &PathBuf, parts: &[&str], stdin: Option<&str>) -> Result<String> {
    return wrap_command_errors(cwd, parts, Stdio::piped, |command| {
        let mut process = command.spawn()?;

        if let Some(stdin) = stdin {
            process
                .stdin
                .as_mut()
                .context("Failed to capture stdin")?
                .write_all(stdin.as_bytes())
                .context("Failed to write to stdin")?;
        }

        let output = process.wait_with_output()?;

        let stdout = String::from_utf8(output.stdout).context("Failed to read stdout")?;

        let status = output.status;

        if status.success() {
            return Ok(stdout);
        } else {
            // Only read/return stderr on failure, as sometimes successful invocations write
            // irrelevant information (like progress indicators) to stderr, which we don't need.
            let stderr = String::from_utf8(output.stderr).context("Failed to read stderr")?;

            bail!("Command failed with status: {status}\n{stdout}\n{stderr}")
        }
    });
}

fn wrap_command_errors<
    TResult,
    TStdio: Fn() -> Stdio,
    TCallback: FnOnce(&mut Command) -> Result<TResult>,
>(
    cwd: &PathBuf,
    parts: &[&str],
    stdio: TStdio,
    callback: TCallback,
) -> Result<TResult> {
    let mut command = Command::new(&parts[0]);

    let command = command
        .args(&parts[1..])
        .current_dir(cwd)
        .stdin(stdio())
        .stdout(stdio())
        .stderr(stdio());

    return callback(command)
        .context(format!("Cwd: {cwd:?}"))
        .context(format!("Command failed: {parts:#?}"));
}
