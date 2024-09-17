//! This crate is responsible for creating the Solidity language definition and exposing it to downstream crates.

mod definition;

use codegen_language_definition::model::BuiltIn;
pub use definition::SolidityDefinition;

pub fn render_built_ins(built_ins: &[&BuiltIn]) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push("library $$ {".into());
    for item in built_ins {
        match item {
            BuiltIn::BuiltInFunction { item } => {
                lines.push(format!("function {name}();", name = item.name));
            }
            BuiltIn::BuiltInType { item } => {
                lines.push(format!("struct {name} {{}}", name = item.name));
            }
            BuiltIn::BuiltInVariable { item } => {
                lines.push(format!(
                    "{typ} {name};",
                    typ = item.slot_type,
                    name = item.name
                ));
            }
        }
    }
    lines.push("}".into());
    lines.join("\n")
}
