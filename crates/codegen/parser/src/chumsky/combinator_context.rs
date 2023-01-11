use std::cell::RefCell;
use std::collections::BTreeMap;

use codegen_schema::grammar::Grammar;
use semver::Version;
use typed_arena::Arena;

use super::character_filter::CharacterFilter;
use super::combinator_node::CombinatorNode;
use super::combinator_tree::CombinatorTree;

pub struct CombinatorContext<'context> {
    pub grammar: &'context Grammar,
    pub version: Version,
    pub trees_by_name: RefCell<BTreeMap<String, &'context CombinatorTree<'context>>>,
    tree_arena: Arena<CombinatorTree<'context>>,
    node_arena: Arena<CombinatorNode<'context>>,
    character_filter_arena: Arena<CharacterFilter<'context>>,
}

impl<'context> CombinatorContext<'context> {
    pub fn new(grammar: &'context Grammar, version: Version) -> Self {
        CombinatorContext {
            grammar,
            version,
            trees_by_name: Default::default(),
            tree_arena: Arena::new(),
            node_arena: Arena::new(),
            character_filter_arena: Arena::new(),
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

    pub fn alloc_character_filter(
        &'context self,
        filter: CharacterFilter<'context>,
    ) -> &'context mut CharacterFilter<'context> {
        self.character_filter_arena.alloc(filter)
    }

    pub fn get_tree_by_name(&'context self, name: &str) -> &'context CombinatorTree<'context> {
        self.trees_by_name
            .borrow()
            .get(name)
            .expect(&format!("Production {} not found", name))
    }
}
