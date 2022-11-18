use std::collections::BTreeSet;

use codegen_schema::*;
use codegen_utils::context::CodegenContext;

use super::combinator_context::CombinatorContext;
use super::combinator_tree::CombinatorTree;
use super::generated_code::GeneratedCode;

pub trait GrammarChumskyExtensions {
    fn generate_rust_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    );
    fn generate_typescript_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    );
}

impl GrammarChumskyExtensions for Grammar {
    fn generate_rust_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    ) {
        let mut version_breaks = BTreeSet::new();
        for production in self.productions.values().flatten() {
            for version in production.versions.keys() {
                version_breaks.insert(version.clone());
            }
        }

        let mut generated_code = GeneratedCode::default();

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

        generated_code.generate_rust_lib_sources(codegen, output_dir);
    }
    fn generate_typescript_lib_sources(
        &self,
        codegen: &mut CodegenContext,
        output_dir: &std::path::PathBuf,
    ) {
        let mut version_breaks = BTreeSet::new();
        for production in self.productions.values().flatten() {
            for version in production.versions.keys() {
                version_breaks.insert(version.clone());
            }
        }

        let mut generated_code = GeneratedCode::default();

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

        generated_code.generate_typescript_lib_sources(codegen, output_dir);
    }
}
