use num_bigint::BigInt;
use num_rational::BigRational;

use super::super::DecimalNumberExpressionStruct;
use super::common::{integer_value_of_node, rational_value_of_node};

impl DecimalNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` for rationals
    /// that do not reduce to an integer after unit multiplication.
    pub fn integer_value(&self) -> Option<BigInt> {
        integer_value_of_node(&self.semantic, self.ir_node.id())
    }

    /// Returns the rational value of this literal, or `None` if the literal
    /// cannot be evaluated. Integer literals lift to a rational with
    /// denominator one.
    pub fn rational_value(&self) -> Option<BigRational> {
        rational_value_of_node(&self.semantic, self.ir_node.id())
    }
}
