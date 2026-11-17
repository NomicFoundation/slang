use ruint::aliases::U256;
use ruint::{BaseConvertError, ParseError};
use slang_solidity_v2_ir::ir;

use super::strings::{value_of_hex_string_literals, value_of_string_literals};

/// Computes the 256-bit word a Yul literal evaluates to.
///
/// Number literals are parsed as integers, booleans as `1`/`0`, and string
/// literals as their bytes left-aligned into a 256-bit word.
///
/// Returns `None` only for a decimal or hex literal whose value does not fit
/// into 256 bits (leading zeros don't count). Every other literal has a value.
pub fn yul_literal_value(literal: &ir::YulLiteral) -> Option<U256> {
    match literal {
        ir::YulLiteral::TrueKeyword(_) => Some(U256::ONE),
        ir::YulLiteral::FalseKeyword(_) => Some(U256::ZERO),
        ir::YulLiteral::DecimalLiteral(literal) => {
            // Yul number literals don't allow `_` separators (unlike Solidity).
            parse_number(literal.unparse(), 10)
        }
        ir::YulLiteral::HexLiteral(literal) => {
            // Skip the `0x` prefix. Yul hex literals don't allow `_` separators.
            parse_number(&literal.unparse()[2..], 16)
        }
        ir::YulLiteral::StringLiteral(literal) => Some(string_bytes_to_value(
            &value_of_string_literals(std::slice::from_ref(literal)),
        )),
        ir::YulLiteral::HexStringLiteral(literal) => Some(string_bytes_to_value(
            &value_of_hex_string_literals(std::slice::from_ref(literal)),
        )),
    }
}

/// Parses the digits of a Yul number literal, returning `None` only when the
/// value overflows 256 bits.
fn parse_number(digits: &str, radix: u64) -> Option<U256> {
    match U256::from_str_radix(digits, radix) {
        Ok(value) => Some(value),
        Err(ParseError::BaseConvertError(BaseConvertError::Overflow)) => None,
        Err(error) => unreachable!(
            "the lexer only produces valid base-{radix} digits, got {digits:?}: {error}"
        ),
    }
}

/// Interprets the bytes of a string literal as a 256-bit word: the bytes are
/// left-aligned into 32 bytes (padded with zeros on the right) and read as a
/// big-endian integer.
fn string_bytes_to_value(bytes: &[u8]) -> U256 {
    let mut word = [0u8; 32];
    let len = bytes.len().min(32);
    word[..len].copy_from_slice(&bytes[..len]);
    U256::from_be_bytes(word)
}
