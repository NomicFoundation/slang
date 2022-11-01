use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::commands::run_command;

pub struct CodegenContext {
    pub repo_dir: PathBuf,
    pub validate: bool,
}

impl CodegenContext {
    pub fn with_context(
        repo_dir: PathBuf,
        operation: impl FnOnce(&Self) -> Result<()>,
    ) -> Result<()> {
        let context = Self {
            repo_dir,
            validate: std::env::var("SLANG_CODEGEN_VALIDATE").is_ok(),
        };

        pre_validation(&context)?;
        operation(&context)?;
        post_validation(&context)?;

        return Ok(());
    }
}

fn pre_validation(codegen: &CodegenContext) -> Result<()> {
    if codegen.validate {
        assert_no_local_changes(codegen).context("Found local changes after running codegen")?;
    }

    return Ok(());
}

fn post_validation(codegen: &CodegenContext) -> Result<()> {
    if codegen.validate {
        assert_no_local_changes(codegen).context("Found local changes after running codegen")?;
    }

    return Ok(());
}

fn assert_no_local_changes(codegen: &CodegenContext) -> Result<()> {
    // Update index status
    run_command(codegen, &vec!["git", "status"], None)?;

    // Command will fail if there are any local changes to the index
    run_command(codegen, &vec!["git", "diff-index", "HEAD", "--quiet"], None)?;

    return Ok(());
}
