use super::super::{ModifierDefinitionStruct, Type};

impl ModifierDefinitionStruct {
    /// Returns the types of the modifier's parameters, in declaration order. A
    /// modifier is not a value, so it has no type of its own: this is the
    /// part of its signature an invocation's arguments are checked against.
    ///
    /// Returns `None` if a parameter type could not be resolved.
    pub fn parameter_types(&self) -> Option<Vec<Type>> {
        self.parameters()
            .iter()
            .map(|parameter| parameter.get_type())
            .collect()
    }
}
