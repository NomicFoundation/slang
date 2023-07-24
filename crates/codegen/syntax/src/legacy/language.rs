use codegen_schema::types::LanguageDefinitionRef;

use crate::legacy::{
    code_generator::CodeGenerator, combinator_context::CombinatorContext,
    combinator_tree::CombinatorTree,
};

pub trait PrivateSyntaxGeneratorExtensions {
    fn create_code_generator(&self) -> CodeGenerator;
}

impl PrivateSyntaxGeneratorExtensions for LanguageDefinitionRef {
    fn create_code_generator(&self) -> CodeGenerator {
        let version_breaks = self.collect_version_breaks();

        let mut generated_code = CodeGenerator::new(self);

        for version in version_breaks {
            let context = CombinatorContext::new(self, version.clone());

            {
                // Scoped to isolate the borrow
                let mut trees_by_name = context.trees_by_name.borrow_mut();

                for production in self.productions.values() {
                    trees_by_name.insert(
                        production.name.clone(),
                        CombinatorTree::new(&context, production),
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
                eprintln!("  {error}");
            }
        }

        generated_code
    }
}
