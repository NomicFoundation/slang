use std::path::Path;

use anyhow::Result;
use infra_utils::codegen::{Codegen, CodegenWriteOnly};
use schemars::{schema_for, JsonSchema};
use serde_json::Value;

use crate::types::{LanguageDefinition, ManifestFile, ProductionsFile};

impl LanguageDefinition {
    pub fn generate_json_schema(output_dir: &Path) -> Result<()> {
        let mut codegen = Codegen::write_only()?;

        write_schema_file::<ManifestFile>(&mut codegen, &output_dir.join("manifest.schema.json"))?;

        write_schema_file::<ProductionsFile>(
            &mut codegen,
            &output_dir.join("productions.schema.json"),
        )?;

        return Ok(());
    }
}

fn write_schema_file<TSchema: JsonSchema>(
    codegen: &mut CodegenWriteOnly,
    file_path: &Path,
) -> Result<()> {
    let schema = schema_for!(TSchema);

    let schema_json = {
        // The serde validation is our source of truth, and would run strict validation.
        // Relax JSON schema to enable auto-completion, even for in-progress/incomplete snippets.
        let strict = serde_json::to_value(&schema)?;
        let relaxed = relax_schema(strict);
        serde_json::to_string_pretty(&relaxed)?
    };

    return codegen.write_file(file_path, schema_json);
}

fn relax_schema(value: Value) -> Value {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            return value;
        }

        Value::Array(nodes) => {
            return Value::Array(nodes.into_iter().map(relax_schema).collect());
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
