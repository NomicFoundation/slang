#[path = "contexts.generated.rs"]
mod contexts;
mod definition;
#[path = "lexemes.generated.rs"]
mod lexemes;
#[cfg(test)]
mod tests;

pub use definition::Lexer;
pub(crate) use lexemes::LexemeKind;
