use num_bigint::{BigInt, Sign};
use slang_solidity_v2_semantic::types::literals;

use super::super::YulLiteral;

impl YulLiteral {
    /// The 256-bit word this Yul literal denotes: booleans as 0 / 1, decimal and
    /// hex numbers by their integer value, and string / `hex"…"` literals packed
    /// left-aligned (most-significant end) into the word, zero-padded on the right.
    ///
    /// Numeric literals are returned verbatim, without enforcing the 2^256-1 range
    /// (an out-of-range literal yields a `BigInt` wider than a word). String /
    /// `hex"…"` literals longer than 32 bytes are truncated to the first 32.
    pub fn integer_value(&self) -> BigInt {
        match self {
            YulLiteral::TrueKeyword(_) => BigInt::from(1u8),
            YulLiteral::FalseKeyword(_) => BigInt::from(0u8),
            YulLiteral::DecimalLiteral(decimal) => {
                let text = decimal.ir_node.unparse();
                BigInt::parse_bytes(text.as_bytes(), 10)
                    .expect("the parser validated this decimal literal")
            }
            YulLiteral::HexLiteral(hex) => {
                let text = hex.ir_node.unparse();
                BigInt::parse_bytes(&text.as_bytes()[2..], 16)
                    .expect("the parser validated this hex literal")
            }
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
    fn pack_word(bytes: &[u8]) -> BigInt {
        let mut word = [0u8; 32];
        let len = bytes.len().min(32);
        word[..len].copy_from_slice(&bytes[..len]);
        BigInt::from_bytes_be(Sign::Plus, &word)
    }
}
