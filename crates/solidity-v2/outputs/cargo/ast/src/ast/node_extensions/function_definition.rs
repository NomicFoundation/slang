use super::super::{FunctionDefinition, FunctionDefinitionStruct};

impl FunctionDefinitionStruct {
    /// Whether this function overrides `base`: the binder's override relation,
    /// the relation `super` and virtual dispatch resolve on. Directional: `self`
    /// is the more-derived function, `base` the one it overrides.
    pub fn overrides(&self, base: &FunctionDefinition) -> bool {
        self.semantic
            .function_overrides(self.ir_node.id(), base.node_id())
    }
}
