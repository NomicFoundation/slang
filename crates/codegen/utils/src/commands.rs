use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{Context, Result};

pub fn run_command(cwd: &PathBuf, parts: &Vec<&str>, stdin: Option<&str>) -> Result<String> {
    let result = {
        let mut command = Command::new(&parts[0]);
        let command = command
            .args(&parts[1..])
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());

        let mut process = command.spawn()?;

        if let Some(stdin) = stdin {
            process
                .stdin
                .as_mut()
                .context("failed to capture stdin")?
                .write_all(stdin.as_bytes())?;
        }

        let output = process.wait_with_output()?;

        let stdout = String::from_utf8(output.stdout).context("Failed to read stdout")?;

        let status = output.status;
        if status.success() {
            Ok(stdout)
        } else {
            let stderr = String::from_utf8(output.stderr).context("Failed to read stderr")?;

            Err(anyhow::anyhow!("Command failed with {status}")
                .context(format!("{stdout}\n{stderr}")))
        }
    };

    return result
        .context(format!("{parts:#?}",))
        .context("Failed to run command");
}
