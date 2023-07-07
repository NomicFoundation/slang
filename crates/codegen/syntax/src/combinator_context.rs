use std::{cell::RefCell, collections::BTreeMap};

use codegen_schema::types::LanguageDefinitionRef;
use semver::Version;
use typed_arena::Arena;

use super::{combinator_node::CombinatorNode, combinator_tree::CombinatorTree};

pub struct CombinatorContext<'context> {
    pub language: LanguageDefinitionRef,
    pub version: Version,
    pub trees_by_name: RefCell<BTreeMap<String, &'context CombinatorTree<'context>>>,
    tree_arena: Arena<CombinatorTree<'context>>,
    node_arena: Arena<CombinatorNode<'context>>,
}

impl<'context> CombinatorContext<'context> {
    pub fn new(language: &LanguageDefinitionRef, version: Version) -> Self {
        CombinatorContext {
            language: language.to_owned(),
            version,
            trees_by_name: Default::default(),
            tree_arena: Arena::new(),
            node_arena: Arena::new(),
        }
    }

    pub fn alloc_tree(
        &'context self,
        tree: CombinatorTree<'context>,
    ) -> &'context mut CombinatorTree<'context> {
        self.tree_arena.alloc(tree)
    }

    pub fn alloc_node(
        &'context self,
        node: CombinatorNode<'context>,
    ) -> &'context mut CombinatorNode<'context> {
        self.node_arena.alloc(node)
    }

    pub fn get_tree_by_name(&'context self, name: &str) -> &'context CombinatorTree<'context> {
        self.trees_by_name
            .borrow()
            .get(name)
            .expect(&format!("Production {name} not found"))
    }
}
