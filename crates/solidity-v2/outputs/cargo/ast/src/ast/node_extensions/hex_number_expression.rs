use num_bigint::BigInt;
use slang_solidity_v2_semantic::types::Number;

use super::super::HexNumberExpressionStruct;
use super::common::number_value_of_node;

impl HexNumberExpressionStruct {
    /// Returns the integer value of this literal.
    ///
    /// Hex literals always carry an integer value as a property of the
    /// literal text itself, regardless of whether the binder folded the
    /// surrounding context as a numeric or non-numeric target (e.g.
    /// `address`, `bytesN`).
    pub fn integer_value(&self) -> Option<BigInt> {
        let hex = self.ir_node.literal.unparse();
        BigInt::parse_bytes(hex.strip_prefix("0x")?.as_bytes(), 16)
    }

    /// Returns the literal number carried by this node's type. Hex number
    /// expressions only ever fold to integers.
    pub fn number_value(&self) -> Option<Number> {
        number_value_of_node(&self.semantic, self.ir_node.id())
    }
}
