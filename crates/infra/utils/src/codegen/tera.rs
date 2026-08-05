use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use inflector::Inflector;
use tera::{Kwargs, State, Tera, TeraResult, Value};

use crate::cargo::CargoWorkspace;
use crate::codegen::snapshots::collect_snapshot_tests;
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

        let mut instance = Tera::new();

        // Everything must be registered before the templates are loaded below: tera validates at
        // load time that every filter/function/component a template references exists.
        instance.register_filter("camel_case", camel_case_filter);
        instance.register_filter("pascal_case", pascal_case_filter);
        instance.register_filter("snake_case", snake_case_filter);
        instance.register_filter("wit_case", wit_case_filter);

        instance.register_function("collect_snapshot_tests", collect_snapshot_tests_function);

        instance.autoescape_on(Vec::<&str>::new()); // disable autoescaping

        instance
            .load_from_glob(templates_glob.unwrap_str())
            // Wrap with newlines, so that the multi-line error report starts on its own line:
            .map_err(|error| anyhow!("\n{error}\n"))?;

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

        self.instance
            .render(template_relative_path, context)
            .map_err(|error| anyhow!("\n{error}\n"))
    }
}

// The callbacks below take `Kwargs` by value, as tera's `Filter`/`Function` traits require, hence
// the `needless_pass_by_value` allows.

/// Tera silently ignores unexpected keyword arguments, so a typo would otherwise be dropped.
fn assert_kwarg_count(kwargs: &Kwargs, expected: usize) {
    let keys = kwargs.iter().map(|(key, _)| key).collect::<Vec<_>>();
    assert_eq!(keys.len(), expected, "Unexpected arguments: {keys:?}");
}

#[allow(clippy::needless_pass_by_value)]
fn camel_case_filter(value: &str, kwargs: Kwargs, _: &State<'_>) -> String {
    assert_kwarg_count(&kwargs, 0);

    value.to_camel_case()
}

#[allow(clippy::needless_pass_by_value)]
fn pascal_case_filter(value: &str, kwargs: Kwargs, _: &State<'_>) -> String {
    assert_kwarg_count(&kwargs, 0);

    value.to_pascal_case()
}

#[allow(clippy::needless_pass_by_value)]
fn snake_case_filter(value: &str, kwargs: Kwargs, _: &State<'_>) -> String {
    assert_kwarg_count(&kwargs, 0);

    value.to_snake_case()
}

#[allow(clippy::needless_pass_by_value)]
fn wit_case_filter(value: &str, kwargs: Kwargs, _: &State<'_>) -> String {
    assert_kwarg_count(&kwargs, 0);

    let mut result = String::new();
    result.push('%');

    for (i, c) in value.chars().enumerate() {
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

    result
}

#[allow(clippy::needless_pass_by_value)]
fn collect_snapshot_tests_function(kwargs: Kwargs, _: &State<'_>) -> TeraResult<Value> {
    assert_kwarg_count(&kwargs, 2);

    let crate_name = kwargs.must_get::<&str>("crate_name")?;
    let path = kwargs.must_get::<&str>("path")?;

    let data_dir = CargoWorkspace::locate_source_crate(crate_name)
        .unwrap()
        .join(path);

    let entries = collect_snapshot_tests(&data_dir);

    Ok(Value::from_serializable(&entries))
}
