use crate::ir;

impl ir::ModifierInvocationStruct {
    /// Whether the name is qualified, as in `A.m`, rather than bare, as in `m`.
    pub fn is_qualified(&self) -> bool {
        self.name.len() > 1
    }
}
