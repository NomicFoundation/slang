use std::path::PathBuf;
use std::{collections::BTreeMap, rc::Rc};

use codegen_ebnf::ProductionEBNFExtensions;
use codegen_schema::*;

use super::code_fragments::CodeFragments;
use super::combinator_node::CombinatorNode;
use super::combinator_tree::{CombinatorTree, CombinatorTreeRef};

#[derive(Clone, Debug)]
pub struct CombinatorForest {
    pub grammar: GrammarRef,
    pub trees_by_production_name: BTreeMap<String, CombinatorTreeRef>,
}

pub type CombinatorForestRef = Rc<CombinatorForest>;

impl CombinatorForest {
    pub fn from_grammar(grammar: GrammarRef) -> CombinatorForestRef {
        let mut trees_by_production_name = BTreeMap::new();
        let mut expression_trees = Vec::new();

        for production in grammar.productions.iter().map(|(_, v)| v).flatten() {
            let tree = CombinatorTree::from_production(&grammar, &production);
            if let CombinatorNode::Expression { members, .. } = tree.root_node.as_ref() {
                expression_trees.push((tree.clone(), members.clone()));
            }
            trees_by_production_name.insert(production.name.clone(), tree);
        }

        for (tree, mut members) in expression_trees {
            members.reverse();
            let mut next_sibling = None;
            for production in members {
                let mut member_tree = trees_by_production_name
                    .get_mut(&production.name)
                    .expect("TODO: production not found");
                let member_tree =
                    Rc::get_mut(&mut member_tree).expect("There cannot be any other references");
                member_tree.convert_to_expression_member(tree.production.clone(), next_sibling);
                next_sibling = Some(production);
            }
        }

        Rc::new(CombinatorForest {
            grammar,
            trees_by_production_name,
        })
    }

    pub fn generate(&self, output_dir: PathBuf) {
        let mut code_fragments = CodeFragments::default();

        for (name, combinator_tree) in &self.trees_by_production_name {
            let production = self.grammar.get_production(&name);
            let ebnf_comment = format!(
                "\n\n{}\n",
                production
                    .generate_ebnf(&self.grammar)
                    .iter()
                    .map(|s| format!("// {}", s))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            code_fragments.add_parser_definition_fragment(ebnf_comment);
            let ebnf_doc_comment = format!(
                "\n\n{}\n",
                production
                    .generate_ebnf(&self.grammar)
                    .iter()
                    .map(|s| format!("/// {}", s))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            code_fragments.add_parser_field_definition_fragment(ebnf_doc_comment);
            combinator_tree.add_to_code_fragments(self, &mut code_fragments);
        }

        if code_fragments.has_errors() {
            // TODO: This currently has way too many false positives
            for error in code_fragments.get_errors() {
                eprintln!("{}", error);
            }
        }

        code_fragments.write_to_source_files(output_dir);
    }
}
