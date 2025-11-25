#[path = "contexts.generated.rs"]
mod contexts;
pub(crate) mod definition;
#[path = "lexemes.generated.rs"]
pub(crate) mod lexemes;

#[cfg(test)]
mod tests;

pub mod temp_sourcify;
