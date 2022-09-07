mod chumsky;

pub trait GrammarRefChumskyExtensions {
    fn generate_chumsky(&self, output_dir: std::path::PathBuf);
}

impl GrammarRefChumskyExtensions for codegen_schema::GrammarRef {
    fn generate_chumsky(&self, output_dir: std::path::PathBuf) {
        let combinator_forest =
            chumsky::combinator_forest::CombinatorForest::from_grammar(self.clone());
        combinator_forest.generate(output_dir);
    }
}
