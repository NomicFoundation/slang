#[path = "src/types/mod.rs"]
mod types;

use anyhow::Result;
use codegen_utils::context::CodegenContext;
use schemars::{schema_for, JsonSchema};
use serde_json::Value;
use types::{manifest::Manifest, production::ProductionRef};

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        write_schema_file::<Manifest>(
            codegen,
            "crates/codegen/schema/generated/manifest.schema.json",
        )?;

        write_schema_file::<Vec<ProductionRef>>(
            codegen,
            "crates/codegen/schema/generated/productions.schema.json",
        )?;

        return Ok(());
    });
}

fn write_schema_file<S: JsonSchema>(
    codegen: &mut CodegenContext,
    relative_path: &str,
) -> Result<()> {
    let schema = schema_for!(S);

    let schema_json = {
        // The serde validation is our source of truth, and would run strict validation.
        // Relax JSON schema to enable auto-completion, even for in-progress/incomplete snippets.
        let strict = serde_json::to_value(&schema)?;
        let relaxed = relax_schema(strict);
        serde_json::to_string_pretty(&relaxed)?
    };

    let schema_path = codegen.repo_root.join(relative_path);
    codegen.write_file(&schema_path, &schema_json)?;

    return Ok(());
}

fn relax_schema(value: Value) -> Value {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            return value;
        }

        Value::Array(items) => {
            return Value::Array(items.into_iter().map(relax_schema).collect());
        }

        Value::Object(entries) => {
            return Value::Object(
                entries
                    .into_iter()
                    .filter_map(|(key, value)| {
                        // Remove `additionalProperties: false`
                        if key == "additionalProperties" {
                            if let Value::Bool(false) = value {
                                return None;
                            }
                        }

                        // Replace `oneOf` and `allOf` with `anyOf`
                        let key = if key == "oneOf" || key == "allOf" {
                            "anyOf".to_string()
                        } else {
                            key
                        };

                        return Some((key, relax_schema(value)));
                    })
                    .collect(),
            );
        }
    };
}
