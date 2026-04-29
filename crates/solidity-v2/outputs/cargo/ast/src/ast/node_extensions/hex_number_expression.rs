use num_bigint::BigInt;

use super::super::HexNumberExpressionStruct;
use super::common::integer_value_of_node;

impl HexNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` if the literal
    /// cannot be evaluated (e.g. a malformed hex digit sequence).
    pub fn integer_value(&self) -> Option<BigInt> {
        integer_value_of_node(&self.semantic, self.ir_node.id())
    }
}
