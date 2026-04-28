use num_bigint::BigInt;
use slang_solidity_v2_semantic::types::integer_value_of_decimal_number_expression;

use super::super::DecimalNumberExpressionStruct;

impl DecimalNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` for rationals
    /// that do not reduce to an integer after unit multiplication.
    pub fn integer_value(&self) -> Option<BigInt> {
        integer_value_of_decimal_number_expression(&self.ir_node)
    }
}
