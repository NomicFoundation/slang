use crate::ir;

impl ir::TupleExpressionStruct {
    /// Whether this tuple omits a component, however deeply nested, eg.
    /// `(1, )`.
    pub fn has_omitted_component(&self) -> bool {
        self.items.iter().any(|item| match &item.expression {
            None => true,
            Some(ir::Expression::TupleExpression(inner)) => inner.has_omitted_component(),
            Some(_) => false,
        })
    }
}
