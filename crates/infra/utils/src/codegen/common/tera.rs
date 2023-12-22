#![allow(clippy::unnecessary_wraps)]

use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, Result};
use inflector::Inflector;
use serde_json::Value;
use tera::Tera;

use crate::paths::PathExtensions;

pub fn create_tera_instance(templates_glob: &Path) -> Result<Tera> {
    let mut instance = Tera::new(templates_glob.unwrap_str()).map_err(|error| {
        // Stringify and wrap with newlines, as the default error serialization
        // does not render the error report correctly:
        anyhow!("\n{error}\n")
    })?;

    instance.register_filter("camel_case", camel_case);
    instance.register_filter("snake_case", snake_case);

    instance.register_function("panic", panic);

    instance.autoescape_on(vec![]); // disable autoescaping

    Ok(instance)
}

fn camel_case(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    Ok(Value::String(
        value
            .as_str()
            .expect("Expected a string value")
            .to_camel_case(),
    ))
}

fn snake_case(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 0, "Expected no arguments");

    Ok(Value::String(
        value
            .as_str()
            .expect("Expected a string value")
            .to_snake_case(),
    ))
}

fn panic(args: &HashMap<String, Value>) -> tera::Result<Value> {
    assert_eq!(args.len(), 1, "Expected one argument");

    let message = args
        .get("message")
        .expect("Expected a 'message' argument")
        .as_str()
        .expect("Expected a string message");

    panic!("{message}");
}
