use crate::ast::{
    AdditiveExpressionStruct, BitwiseAndExpressionStruct, BitwiseOrExpressionStruct,
    BitwiseXorExpressionStruct, Definition, EqualityExpressionStruct, FunctionDefinition,
    InequalityExpressionStruct, MultiplicativeExpressionStruct, PrefixExpressionStruct,
};

/// An operator expression a `using {f as op} for T global;` directive can
/// rebind to a free function.
pub trait UserDefinedOperatorExpression {
    /// The function this operator is bound to, or `None` when it keeps its
    /// built-in meaning.
    fn resolve_operator_to_function(&self) -> Option<FunctionDefinition>;
}

/// The reference pass resolves every operator expression that a `using`
/// directive rebinds, so the AST reads the function back from the binder.
macro_rules! impl_resolve_operator_to_function {
    ($($node:ident),* $(,)?) => {
        $(
            impl UserDefinedOperatorExpression for $node {
                fn resolve_operator_to_function(&self) -> Option<FunctionDefinition> {
                    let definition_id = self
                        .semantic
                        .binder()
                        .resolved_operator_function(self.ir_node.id())?;
                    let Some(Definition::Function(function)) =
                        Definition::try_create(definition_id, &self.semantic)
                    else {
                        unreachable!("a using directive binds an operator to a function definition");
                    };
                    Some(function)
                }
            }
        )*
    };
}

impl_resolve_operator_to_function!(
    AdditiveExpressionStruct,
    BitwiseAndExpressionStruct,
    BitwiseOrExpressionStruct,
    BitwiseXorExpressionStruct,
    EqualityExpressionStruct,
    InequalityExpressionStruct,
    MultiplicativeExpressionStruct,
    PrefixExpressionStruct,
);
