use crate::ir;

impl ir::ArgumentsDeclaration {
    /// Whether the call passes no arguments, in either of the two forms: `f()`
    /// and `f({})`.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::PositionalArguments(arguments) => arguments.is_empty(),
            Self::NamedArguments(arguments) => arguments.is_empty(),
        }
    }
}
