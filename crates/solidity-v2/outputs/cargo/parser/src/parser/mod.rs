use lalrpop_util::lalrpop_mod;

// TODO: How do I get rid of the squiggly line here?
lalrpop_mod!(
    #[allow(non_snake_case)]
    pub calculator, "/parser/grammar.generated.rs"); // synthesized by LALRPOP
