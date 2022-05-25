// Chumsky types get very big when not boxing
#![recursion_limit = "256"]

pub mod chumsky;
pub mod ebnf;
pub mod schema;
pub mod solidity;
pub mod validation;
