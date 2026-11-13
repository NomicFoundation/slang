use crate::ast::ModifierDefinitionStruct;

impl ModifierDefinitionStruct {
    /// Returns the signature for this modifier using internal type names for
    /// parameters. A modifier is never part of the ABI, so this is its only
    /// signature form.
    pub fn compute_internal_signature(&self) -> Option<String> {
        let name = self.ir_node.name.unparse();
        let parameters = self.parameters().compute_internal_signature()?;
        Some(format!("{name}({parameters})"))
    }
}
