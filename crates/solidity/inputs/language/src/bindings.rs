use std::fmt::{Error, Write};

use codegen_language_definition::model::{BuiltIn, BuiltInContext};

pub fn render_built_ins(built_in_contexts: &[BuiltInContext]) -> String {
    try_render_built_ins(built_in_contexts).expect("Failed to render built-ins")
}

fn try_render_built_ins(built_in_contexts: &[BuiltInContext]) -> Result<String, Error> {
    let mut buffer = String::new();
    for context in built_in_contexts {
        writeln!(buffer, "contract {context} {{", context = context.name)?;
        for item in &context.definitions {
            match item {
                BuiltIn::BuiltInFunction { item } => {
                    writeln!(
                        buffer,
                        "  function {name}({parameters}) public{return_type};",
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
                    writeln!(buffer, "  struct {name} {{", name = item.name)?;
                    for field in &item.fields {
                        writeln!(buffer, "    {field_def};", field_def = field.definition)?;
                    }
                    for function in &item.functions {
                        writeln!(
                            buffer,
                            "    function({parameters}){return_type} {name};",
                            name = function.name,
                            return_type = function
                                .return_type
                                .as_deref()
                                .map(|return_type| format!(" returns ({return_type})"))
                                .unwrap_or_default(),
                            parameters = function.parameters.join(", "),
                        )?;
                    }
                    writeln!(buffer, "  }}")?;
                }
                BuiltIn::BuiltInVariable { item } => {
                    writeln!(buffer, "  {var_def};", var_def = item.definition)?;
                }
            }
        }
        writeln!(buffer, "}}")?;
    }
    Ok(buffer)
}
