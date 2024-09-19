//! This crate is responsible for creating the Solidity language definition and exposing it to downstream crates.

mod definition;

use codegen_language_definition::model::BuiltIn;
pub use definition::SolidityDefinition;

pub fn render_built_ins(built_ins: &[BuiltIn]) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push("contract $$ {".into());
    for item in built_ins {
        match item {
            BuiltIn::BuiltInFunction { item } => {
                let return_type = item
                    .return_type
                    .as_ref()
                    .map(|return_type| format!(" returns ({return_type})"))
                    .unwrap_or_default();
                let parameters = item
                    .parameters
                    .iter()
                    .map(|parameter| parameter.def.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                lines.push(format!(
                    "function {name}({parameters}) public{return_type};",
                    name = item.name
                ));
            }
            BuiltIn::BuiltInType { item } => {
                let fields = item
                    .fields
                    .iter()
                    .map(|field| format!("  {field_def};", field_def = field.def))
                    .collect::<Vec<_>>()
                    .join("\n");
                lines.push(format!("struct {name} {{\n{fields}\n}}", name = item.name));
            }
            BuiltIn::BuiltInVariable { item } => {
                lines.push(format!("{var_def};", var_def = item.def));
            }
        }
    }
    lines.push("}".into());
    lines.join("\n")
}
