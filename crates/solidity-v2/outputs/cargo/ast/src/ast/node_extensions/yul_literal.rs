use ruint::aliases::U256;
use slang_solidity_v2_semantic::types::literals;

use super::super::YulLiteral;

impl YulLiteral {
    /// The 256-bit word this Yul literal denotes: booleans as 0 / 1, decimal and
    /// hex numbers by their integer value, and string / `hex"…"` literals packed
    /// left-aligned (most-significant end) into the word, keeping only the first
    /// 32 bytes.
    pub fn integer_value(&self) -> U256 {
        match self {
            YulLiteral::TrueKeyword(_) => U256::from(1u8),
            YulLiteral::FalseKeyword(_) => U256::ZERO,
            YulLiteral::DecimalLiteral(decimal) => decimal
                .ir_node
                .unparse()
                .parse()
                .expect("the parser validated this decimal literal"),
            YulLiteral::HexLiteral(hex) => hex
                .ir_node
                .unparse()
                .parse()
                .expect("the parser validated this hex literal"),
            YulLiteral::StringLiteral(string) => Self::pack_word(
                &literals::value_of_string_literals(std::slice::from_ref(&string.ir_node)),
            ),
            YulLiteral::HexStringLiteral(hex_string) => Self::pack_word(
                &literals::value_of_hex_string_literals(std::slice::from_ref(&hex_string.ir_node)),
            ),
        }
    }

    /// Packs up to 32 bytes left-aligned into a big-endian 256-bit word,
    /// zero-padded on the right.
    fn pack_word(bytes: &[u8]) -> U256 {
        let len = bytes.len().min(32);
        U256::from_be_slice(&bytes[..len]) << (8 * (32 - len))
    }
}
