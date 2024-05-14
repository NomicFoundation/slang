#![allow(clippy::unnecessary_wraps)]

use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, Result};
use inflector::Inflector;
use serde_json::{Map, Value};
use tera::Tera;

use crate::paths::PathExtensions;

pub const TEMPLATES_GLOB: &str = "**/*.jinja2";

pub fn create_tera_instance(input_dir: &Path) -> Result<Tera> {
    let templates_glob = input_dir.join(TEMPLATES_GLOB);

    let mut instance = Tera::new(templates_glob.unwrap_str()).map_err(|error| {
        // Stringify and wrap with newlines, as the default error serialization
        // does not render the error report correctly:
        anyhow!("\n{error}\n")
    })?;

    instance.register_filter("camel_case", camel_case_filter);
    instance.register_filter("pascal_case", pascal_case_filter);
    instance.register_filter("snake_case", snake_case_filter);

    instance.register_filter("default_array", default_array_filter);
    instance.register_filter("default_object", default_object_filter);

    instance.register_function("panic", panic_function);

    instance.autoescape_on(vec![]); // disable autoescaping

    Ok(instance)
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

fn snake_case_filter(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    Ok(Value::String(
        value
            .as_str()
            .expect("Expected a string value")
            .to_snake_case(),
    ))
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
