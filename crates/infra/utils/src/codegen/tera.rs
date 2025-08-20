#![allow(clippy::unnecessary_wraps)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Result};
use inflector::Inflector;
use regex::Regex;
use serde_json::{Map, Value};
use tera::Tera;

use crate::paths::{FileWalker, PathExtensions};

const JINJA_GLOB: &str = "**/*.jinja2";

pub struct TeraWrapper {
    input_dir: PathBuf,

    instance: Tera,
}

impl TeraWrapper {
    pub fn new(input_dir: impl Into<PathBuf>) -> Result<Self> {
        let input_dir = input_dir.into();

        let templates_glob = input_dir.join(JINJA_GLOB);

        let mut instance = Tera::new(templates_glob.unwrap_str()).map_err(|error| {
            // Stringify and wrap with newlines, as the default error serialization
            // does not render the error report correctly:
            anyhow!("\n{error}\n")
        })?;

        instance.register_filter("camel_case", camel_case_filter);
        instance.register_filter("pascal_case", pascal_case_filter);
        instance.register_filter("kebab_case", kebab_case_filter);
        instance.register_filter("snake_case", snake_case_filter);
        instance.register_filter("wit_case", wit_case_filter);

        instance.register_filter("default_array", default_array_filter);
        instance.register_filter("default_object", default_object_filter);

        instance.register_function("panic", panic_function);

        instance.autoescape_on(vec![]); // disable autoescaping

        Ok(Self {
            input_dir,
            instance,
        })
    }

    pub fn find_all_templates(&self) -> Result<impl Iterator<Item = PathBuf>> {
        FileWalker::from_directory(&self.input_dir).find([JINJA_GLOB])
    }

    pub fn render(&self, template_path: &Path, context: &tera::Context) -> Result<String> {
        // tera expects the template path to be relative to the input directory:
        let template_relative_path = template_path.strip_prefix(&self.input_dir)?.unwrap_str();

        let rendered = match self.instance.render(template_relative_path, context) {
            Ok(rendered) => rendered,
            Err(error) => {
                self.try_report_error(error)?;

                // Exit process, to skip printing irrelevant backtraces:
                #[allow(clippy::exit)]
                std::process::exit(1);
            }
        };

        Ok(rendered)
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

fn camel_case_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    Ok(Value::String(
        value
            .as_str()
            .expect("Expected a string value")
            .to_camel_case(),
    ))
}

fn pascal_case_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    Ok(Value::String(
        value
            .as_str()
            .expect("Expected a string value")
            .to_pascal_case(),
    ))
}

fn kebab_case_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    Ok(Value::String(
        value
            .as_str()
            .expect("Expected a string value")
            .to_kebab_case(),
    ))
}

fn snake_case_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    Ok(Value::String(
        value
            .as_str()
            .expect("Expected a string value")
            .to_snake_case(),
    ))
}

fn wit_case_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    let input = value.as_str().expect("Expected a string value");

    let mut result = String::new();
    result.push('%');

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('-');
            }
            for c in c.to_lowercase() {
                result.push(c);
            }
        } else if c.is_alphanumeric() {
            result.push(c);
        } else {
            result.push('-');
        }
    }

    Ok(Value::String(result))
}

fn default_array_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    if value.is_null() {
        Ok(Value::Array(Vec::default()))
    } else {
        Ok(value.clone())
    }
}

fn default_object_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    if value.is_null() {
        Ok(Value::Object(Map::default()))
    } else {
        Ok(value.clone())
    }
}

fn panic_function(args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 1, "Expected one argument");

    let message = args
        .get("message")
        .expect("Expected a 'message' argument")
        .as_str()
        .expect("Expected a string message");

    panic!("{message}");
}
