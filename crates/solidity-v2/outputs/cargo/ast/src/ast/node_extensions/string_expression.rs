use slang_solidity_v2_semantic::types::literals;

use super::super::{StringExpression, Type};
use crate::ast::{HexStringLiteralsStruct, StringLiteralsStruct, UnicodeStringLiteralsStruct};

impl StringExpression {
    /// Returns the type assigned to this string expression by the typing pass.
    ///
    /// The typing pass binds the resolved type to the first literal in the
    /// sequence; this accessor delegates to that literal. Returns `None` when
    /// the sequence is empty or the typing pass did not record a type.
    pub fn get_type(&self) -> Option<Type> {
        match self {
            StringExpression::StringLiterals(literals) => literals.iter().next()?.get_type(),
            StringExpression::HexStringLiterals(literals) => literals.iter().next()?.get_type(),
            StringExpression::UnicodeStringLiterals(literals) => literals.iter().next()?.get_type(),
        }
    }

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
