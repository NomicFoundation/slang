use crate::ir;

impl ir::Expression {
    /// Whether this is a literal value expression. A number unit is part of
    /// the literal.
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Self::DecimalNumberExpression(_)
                | Self::HexNumberExpression(_)
                | Self::StringExpression(_)
                | Self::TrueKeyword(_)
                | Self::FalseKeyword(_)
        )
    }
}
