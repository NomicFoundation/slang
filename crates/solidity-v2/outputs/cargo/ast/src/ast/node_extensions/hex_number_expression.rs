use num_bigint::BigInt;
use slang_solidity_v2_semantic::types::integer_value_of_hex_number_expression;

use super::super::HexNumberExpressionStruct;

impl HexNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` if the literal
    /// cannot be evaluated (e.g. a malformed hex digit sequence).
    pub fn integer_value(&self) -> Option<BigInt> {
        integer_value_of_hex_number_expression(&self.ir_node)
    }
}
