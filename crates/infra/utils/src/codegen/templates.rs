//! Here, `input_dir` will have three kinds of files:
//!
//! 1) `input_dir/template.rs.jinja2`     -> a template file to render
//! 2) `input_dir/generated/template.rs`  -> the stub file rendered by this template in the source crate
//! 3) `input_dir/other.rs`               -> other source files to be copied as-is
//!
//! For `render_standalone_template()`:
//! It will render each template (#1) to its stub file (#2) under the same directory.
//!
//! For `render_runtime_output()`:
//! The first pass below will render the template (#1) to `output_dir/generated/template.rs`, and skip the stub (#2).
//! Then the second pass will copy the remaining sources (#3) to `output_dir/other.rs` as-is.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use regex::Regex;
use serde::Serialize;
use tera::Tera;

use crate::codegen::tera::{create_tera_instance, TEMPLATES_GLOB};
use crate::codegen::CodegenFileSystem;
use crate::paths::{FileWalker, PathExtensions};

pub struct CodegenTemplates {
    input_dir: PathBuf,
    fs: CodegenFileSystem,
    tera: Tera,
}

impl CodegenTemplates {
    pub fn new(input_dir: impl Into<PathBuf>) -> Result<Self> {
        let input_dir = input_dir.into();
        let fs = CodegenFileSystem::new(&input_dir)?;
        let tera = create_tera_instance(&input_dir)?;

        Ok(Self {
            input_dir,
            fs,
            tera,
        })
    }

    pub fn render_stubs(&mut self, model: impl Serialize) -> Result<()> {
        let context = tera::Context::from_serialize(model)?;

        for template_path in FileWalker::from_directory(&self.input_dir).find([TEMPLATES_GLOB])? {
            let stub_path = Self::get_stub_path(&template_path);

            self.render_aux(&template_path, &context, &stub_path)?;
        }

        Ok(())
    }

    pub fn render_directory(
        &mut self,
        model: impl Serialize,
        output_dir: impl AsRef<Path>,
    ) -> Result<()> {
        let context = tera::Context::from_serialize(model)?;
        let output_dir = output_dir.as_ref();

        let mut template_files = HashSet::new();

        for template_path in FileWalker::from_directory(&self.input_dir).find([TEMPLATES_GLOB])? {
            let stub_path = Self::get_stub_path(&template_path);

            let output_path = output_dir.join(stub_path.strip_prefix(&self.input_dir)?);

            self.render_aux(&template_path, &context, &output_path)?;

            assert!(template_files.insert(template_path));
            assert!(template_files.insert(stub_path));
        }

        for source_path in FileWalker::from_directory(&self.input_dir).find_all()? {
            // Skip templates already handled in the first pass:
            if template_files.contains(&source_path) {
                continue;
            }

            let output_path = output_dir.join(source_path.strip_prefix(&self.input_dir)?);

            // Preserve the source of otherwise-generated files:
            if source_path.generated_dir().is_ok() {
                self.fs.mark_generated_file(output_path)?;
            } else {
                self.fs.copy_file(source_path, output_path)?;
            }
        }

        Ok(())
    }

    pub fn render_single(
        &mut self,
        template_path: impl AsRef<Path>,
        model: impl Serialize,
        output_path: impl AsRef<Path>,
    ) -> Result<()> {
        let context = tera::Context::from_serialize(model)?;

        self.render_aux(template_path.as_ref(), &context, output_path.as_ref())
    }

    fn render_aux(
        &mut self,
        template_path: &Path,
        context: &tera::Context,
        output_path: &Path,
    ) -> Result<()> {
        // tera expects the template path to be relative to the input directory:
        let template_relative_path = template_path.strip_prefix(&self.input_dir)?.unwrap_str();

        let rendered = match self.tera.render(template_relative_path, context) {
            Ok(rendered) => rendered,
            Err(error) => {
                self.try_report_error(error)?;

                // Exit process, to skip printing irrelevant backtraces:
                #[allow(clippy::exit)]
                std::process::exit(1);
            }
        };

        self.fs.write_file(output_path, rendered)
    }

    fn get_stub_path(template_path: &Path) -> PathBuf {
        let without_extension = template_path.with_extension("");
        let template_name = without_extension.unwrap_name();

        template_path
            .unwrap_parent()
            .join("generated")
            .join(template_name)
    }

    fn try_report_error(&self, error: tera::Error) -> Result<()> {
        // Unfortunately, tera's error details are hidden in the debug output.
        // It should be fixed in v2: https://github.com/Keats/tera/issues/885
        let error_details = format!("{error:?}");

        let Some(captures) = Regex::new(
            r"Variable `(?<variable>.+)` not found in context while rendering '(?<template>.+)'",
        )?
        .captures(&error_details) else {
            // Another (unrecognized) error: just propagate it as-is:
            bail!(error);
        };

        let message = &captures[0];
        let variable = captures.name("variable").unwrap().as_str();
        let template = captures.name("template").unwrap().as_str();

        let template_path = self.input_dir.join(template);
        let source = template_path.read_to_string()?;

        let variable_location = Regex::new(&format!(
            "\\{{.*[^a-zA-Z0-9_\\.](?<variable>{variable}).*\\}}"
        ))?
        .captures(&source)
        .unwrap()
        .name("variable")
        .unwrap()
        .start();

        let start = source[..variable_location].chars().count();
        let end = start + variable.chars().count();

        let source_id = template_path.unwrap_str();
        let label = ariadne::Label::new((source_id, start..end)).with_message(message);

        ariadne::Report::build(ariadne::ReportKind::Error, source_id, start)
            .with_message(message)
            .with_label(label)
            .finish()
            .write(
                (source_id, ariadne::Source::from(&source)),
                &mut std::io::stdout(),
            )?;

        Ok(())
    }
}
