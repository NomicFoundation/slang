use num_bigint::BigInt;
use num_bigint::Sign;
use slang_solidity_v2_semantic::types::literals;

use super::super::YulLiteral;

impl YulLiteral {
    /// Returns the 256-bit word this Yul literal denotes: booleans as 0 / 1,
    /// decimal and hex numbers by their integer value, and string / `hex"…"`
    /// literals packed left-aligned (most-significant end) into the word,
    /// zero-padded on the right (the Yul literal-evaluation semantics).
    pub fn value(&self) -> BigInt {
        match self {
            YulLiteral::TrueKeyword(_) => BigInt::from(1u32),
            YulLiteral::FalseKeyword(_) => BigInt::from(0u32),
            YulLiteral::DecimalLiteral(decimal) => {
                BigInt::parse_bytes(decimal.unparse().trim().as_bytes(), 10)
                    .expect("the parser validated this decimal literal")
            }
            YulLiteral::HexLiteral(hex) => {
                let text = hex.unparse();
                let trimmed = text.trim();
                let digits = trimmed
                    .strip_prefix("0x")
                    .or_else(|| trimmed.strip_prefix("0X"))
                    .unwrap_or(trimmed);
                BigInt::parse_bytes(digits.as_bytes(), 16)
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
        let mut padded = [0u8; 32];
        let len = bytes.len().min(32);
        padded[..len].copy_from_slice(&bytes[..len]);
        BigInt::from_bytes_be(Sign::Plus, &padded)
    }
}
