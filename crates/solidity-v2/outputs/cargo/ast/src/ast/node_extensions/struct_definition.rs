use slang_solidity_v2_common::collections::Set;

use super::super::visitor::{self, Visitor};
use super::super::{Definition, IdentifierPath, StructDefinition, StructDefinitionStruct};

impl StructDefinitionStruct {
    /// Whether the struct's type refers to itself through its members' types.
    pub fn is_recursive(&self) -> bool {
        let mut visited = Set::default();
        let mut pending = NamedStructs(Vec::new());
        visitor::accept_struct_definition(self, &mut pending);
        while let Some(definition) = pending.0.pop() {
            if definition.node_id() == self.node_id() {
                return true;
            }
            if visited.insert(definition.node_id()) {
                visitor::accept_struct_definition(&definition, &mut pending);
            }
        }
        false
    }
}

/// The struct definitions the visited identifier paths resolve to.
struct NamedStructs(Vec<StructDefinition>);

impl Visitor for NamedStructs {
    fn enter_identifier_path(&mut self, node: &IdentifierPath) -> bool {
        if let Some(Definition::Struct(definition)) = node.resolve_to_definition() {
            self.0.push(definition);
        }
        true
    }
}
