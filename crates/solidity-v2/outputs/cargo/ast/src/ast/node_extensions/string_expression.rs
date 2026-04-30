use slang_solidity_v2_semantic::types::literals;

use super::super::StringExpression;
use crate::ast::{HexStringLiteralsStruct, StringLiteralsStruct, UnicodeStringLiteralsStruct};

impl StringExpression {
    /// Returns the concatenated decoded string value as bytes.
    pub fn value(&self) -> Vec<u8> {
        match self {
            StringExpression::StringLiterals(literals) => literals.value(),
            StringExpression::HexStringLiterals(literals) => literals.value(),
            StringExpression::UnicodeStringLiterals(literals) => literals.value(),
        }
    }
}

impl StringLiteralsStruct {
    pub fn value(&self) -> Vec<u8> {
        literals::value_of_string_literals(&self.ir_nodes)
    }
}

impl HexStringLiteralsStruct {
    pub fn value(&self) -> Vec<u8> {
        literals::value_of_hex_string_literals(&self.ir_nodes)
    }
}

impl UnicodeStringLiteralsStruct {
    pub fn value(&self) -> Vec<u8> {
        literals::value_of_unicode_string_literals(&self.ir_nodes)
    }
}
