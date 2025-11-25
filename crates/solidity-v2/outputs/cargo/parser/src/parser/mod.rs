use lalrpop_util::lalrpop_mod;

// TODO: How do I get rid of the squiggly line here?
lalrpop_mod!(
    #[allow(non_snake_case)]
    pub grammar, "/parser/grammar.generated.rs"); // synthesized by LALRPOP

#[cfg(test)]
mod tests;
