use num_bigint::BigInt;
use slang_solidity_v2_semantic::types::{Number, literals};

use super::super::HexNumberExpressionStruct;
use super::common::integer_value_of_node;

impl HexNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` if the literal
    /// cannot be evaluated (e.g. a malformed hex digit sequence).
    pub fn integer_value(&self) -> Option<BigInt> {
        // A 40-digit literal is typed `address`, which carries no value: read it from the text.
        integer_value_of_node(&self.semantic, self.ir_node.id())
            .or_else(|| literals::value_of_hex_number_expression(&self.ir_node))
    }

    /// Returns the literal number this node spells. Hex number
    /// expressions only ever fold to integers.
    pub fn number_value(&self) -> Option<Number> {
        self.integer_value().map(Number::Integer)
    }
}
