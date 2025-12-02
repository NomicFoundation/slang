use lalrpop_util::lalrpop_mod;

#[path = "nodes.generated.rs"]
mod nodes;

// TODO: How do I get rid of the squiggly line here?
lalrpop_mod!(
    #[allow(non_snake_case)]
    pub grammar, "/parser/grammar.modified.rs"); // synthesized by LALRPOP

#[cfg(test)]
mod tests;
