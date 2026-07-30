use crate::ir;

impl ir::TupleExpressionStruct {
    /// Whether this is the empty tuple `()`. The grammar represents it as a
    /// single component with no expression, which distinguishes it from a
    /// parenthesised expression `(x)` (a single filled component) and from a
    /// tuple with omitted slots such as `(x,)` (more than one component).
    pub fn is_empty_tuple(&self) -> bool {
        self.items.len() == 1 && self.items[0].expression.is_none()
    }
}
