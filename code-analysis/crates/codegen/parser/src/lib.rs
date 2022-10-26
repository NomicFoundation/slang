use std::collections::BTreeSet;

use chumsky::code_fragments::CodeFragments;

mod chumsky;

pub trait GrammarRefChumskyExtensions {
    fn generate_chumsky(&self, output_dir: std::path::PathBuf);
}

impl GrammarRefChumskyExtensions for codegen_schema::GrammarRef {
    fn generate_chumsky(&self, output_dir: std::path::PathBuf) {
        let mut version_breaks = BTreeSet::new();
        for productions in self.productions.values() {
            for production in productions {
                for version in production.versions.keys() {
                    version_breaks.insert(version.clone());
                }
            }
        }

        let mut code_fragments = CodeFragments::default();
        for version in version_breaks {
            let combinator_forest =
                chumsky::combinator_forest::CombinatorForest::from_grammar(self.clone(), &version);
            combinator_forest.add_to_code_fragments(&mut code_fragments);
        }

        if code_fragments.has_errors() {
            eprintln!("Errors:");
            for error in code_fragments.get_errors() {
                eprintln!("  {}", error);
            }
        }

        code_fragments.write_to_source_files(output_dir);
    }
}
