#![allow(clippy::unnecessary_wraps)]

use std::collections::HashMap;
use std::path::Path;

use inflector::Inflector;
use serde_json::Value;
use tera::Tera;

use crate::paths::PathExtensions;

pub fn create_tera_instance(input_dir: &Path) -> Tera {
    let templates_glob = input_dir.join("**/*.jinja2");

    let mut instance = Tera::new(templates_glob.unwrap_str()).unwrap_or_else(|error| {
        panic!("\n{error}\n"); // render error report on a separate line
    });

    instance.register_filter("camel_case", camel_case);
    instance.register_filter("snake_case", snake_case);

    instance.register_function("panic", panic);

    instance.autoescape_on(vec![]); // disable autoescaping

    instance
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
