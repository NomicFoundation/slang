use std::collections::BTreeSet;

use codegen_schema::grammar::Grammar;
use codegen_schema::manifest::ProductionVersioning;
use codegen_utils::context::CodegenContext;

use super::code_generator::CodeGenerator;
use super::combinator_context::CombinatorContext;
use super::combinator_tree::CombinatorTree;

pub trait GrammarParserGeneratorExtensions {
    fn generate_rust_lib_sources(
        &self,
        context: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    );
    fn generate_typescript_lib_sources(
        &self,
        context: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    );
}

trait PrivateGrammarParserGeneratorExtensions {
    fn create_code_generator(&self) -> CodeGenerator;
}

impl GrammarParserGeneratorExtensions for Grammar {
    fn generate_rust_lib_sources(
        &self,
        context: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    ) {
        self.create_code_generator()
            .write_rust_lib_sources(self, context, output_dir);
    }

    fn generate_typescript_lib_sources(
        &self,
        context: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    ) {
        self.create_code_generator()
            .write_typescript_lib_sources(self, context, output_dir);
    }
}

impl PrivateGrammarParserGeneratorExtensions for Grammar {
    fn create_code_generator(&self) -> CodeGenerator {
        let mut version_breaks = BTreeSet::new();
        version_breaks.insert(self.manifest.versions.first().unwrap());

        for production in self.productions.values().flatten() {
            match &production.versioning {
                ProductionVersioning::Unversioned(_) => {}
                ProductionVersioning::Versioned(versions) => {
                    version_breaks.extend(versions.keys());
                }
            }
        }

        let mut generated_code = CodeGenerator::default();

        for version in version_breaks {
            let context = CombinatorContext::new(self, version.clone());

            {
                // Scoped to isolate the borrow
                let mut trees_by_name = context.trees_by_name.borrow_mut();

                for production in self.productions.values().flatten() {
                    trees_by_name.insert(
                        production.name.clone(),
                        CombinatorTree::new(&context, &production),
                    );
                }
            }

            {
                // Scoped to isolate the borrow
                let trees_by_name = context.trees_by_name.borrow();

                // Do this after all trees are created, but before any are
                // generated, so that e.g. precedence rules can transform their
                // members
                for tree in trees_by_name.values() {
                    tree.ensure_tree_is_built();
                }

                for tree in trees_by_name.values() {
                    tree.add_to_generated_code(&mut generated_code);
                }
            }
        }

        if generated_code.has_errors() {
            eprintln!("Errors:");
            for error in generated_code.get_errors() {
                eprintln!("  {}", error);
            }
        }

        generated_code
    }
}
