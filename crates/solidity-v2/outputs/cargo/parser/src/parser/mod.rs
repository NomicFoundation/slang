use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator, "/parser/calculator.rs"); // synthesized by LALRPOP

// #[test]
// fn calculator() {
//     assert!(calculator::TermParser::new().parse("22").is_ok());
//     assert!(calculator::TermParser::new().parse("(22)").is_ok());
//     assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
//     assert!(calculator::TermParser::new().parse("((22)").is_err());
// }
