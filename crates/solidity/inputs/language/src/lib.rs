//! This crate is responsible for creating the Solidity language definition and exposing it to downstream crates.

mod definition;

use std::fmt::{Error, Write};

use codegen_language_definition::model::BuiltIn;
pub use definition::SolidityDefinition;

pub fn render_built_ins(built_ins: &[BuiltIn]) -> String {
    try_render_built_ins(built_ins).expect("Failed to render built-ins")
}

fn try_render_built_ins(built_ins: &[BuiltIn]) -> Result<String, Error> {
    let mut buffer = String::new();
    writeln!(buffer, "contract $$ {{")?;
    for item in built_ins {
        match item {
            BuiltIn::BuiltInFunction { item } => {
                writeln!(
                    buffer,
                    "function {name}({parameters}) public{return_type};",
                    name = item.name,
                    parameters = item.parameters.join(", "),
                    return_type = item
                        .return_type
                        .as_ref()
                        .map(|return_type| format!(" returns ({return_type})"))
                        .unwrap_or_default(),
                )?;
            }
            BuiltIn::BuiltInType { item } => {
                writeln!(buffer, "struct {name} {{", name = item.name)?;
                for field in &item.fields {
                    writeln!(buffer, "  {field_def};", field_def = field.definition)?;
                }
                for function in &item.functions {
                    writeln!(
                        buffer,
                        "  function({parameters}){return_type} {name};",
                        name = function.name,
                        return_type = function
                            .return_type
                            .as_deref()
                            .map(|return_type| format!(" returns ({return_type})"))
                            .unwrap_or_default(),
                        parameters = function.parameters.join(", "),
                    )?;
                }
                writeln!(buffer, "}}")?;
            }
            BuiltIn::BuiltInVariable { item } => {
                writeln!(buffer, "{var_def};", var_def = item.definition)?;
            }
        }
    }
    writeln!(buffer, "}}")?;
    Ok(buffer)
}
