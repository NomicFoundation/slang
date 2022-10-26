use std::{collections::BTreeMap, rc::Rc};

use codegen_ebnf::ProductionEBNFExtensions;
use codegen_schema::*;
use semver::Version;

use super::code_fragments::CodeFragments;
use super::combinator_node::CombinatorNode;
use super::combinator_tree::{CombinatorTree, CombinatorTreeRef};

#[derive(Clone, Debug)]
pub struct CombinatorForest {
    pub version: Version,
    pub grammar: GrammarRef,
    pub trees_by_name: BTreeMap<String, CombinatorTreeRef>,
}

pub type CombinatorForestRef = Rc<CombinatorForest>;

impl CombinatorForest {
    pub fn from_grammar(grammar: GrammarRef, version: &Version) -> CombinatorForestRef {
        let mut trees_by_name = BTreeMap::new();
        let mut precedence_rule_trees_and_members = Vec::new();

        for production in grammar.productions.iter().map(|(_, v)| v).flatten() {
            let tree = CombinatorTree::from_production(&grammar, &production, version);
            if let CombinatorNode::PrecedenceRule { members, .. } = tree.root_node.as_ref() {
                precedence_rule_trees_and_members.push((tree.clone(), members.clone()));
            }
            trees_by_name.insert(production.name.clone(), tree);
        }

        // Transform precedence rule members
        for (tree, mut members) in precedence_rule_trees_and_members {
            members.reverse();
            let mut next_member = None;
            for production in members {
                let mut precedence_rule_member_tree = trees_by_name
                    .get_mut(&production.name)
                    .expect("TODO: production not found");
                let member_tree = Rc::get_mut(&mut precedence_rule_member_tree)
                    .expect("There cannot be any other references");
                member_tree.convert_to_precedence_rule_member(
                    tree.production.clone(),
                    version,
                    next_member,
                );
                next_member = Some(production);
            }
        }

        Rc::new(CombinatorForest {
            version: version.clone(),
            grammar,
            trees_by_name,
        })
    }

    pub fn add_to_code_fragments(&self, code_fragments: &mut CodeFragments) {
        for (name, tree) in &self.trees_by_name {
            let production = self.grammar.get_production(&name);
            if production.versions.contains_key(&self.version) {
                code_fragments
                    .add_parser_comment(name.clone(), production.generate_ebnf(&self.grammar));
                tree.add_to_code_fragments(self, code_fragments);
            }
        }
    }
}
