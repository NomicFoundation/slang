use codegen_schema::types::{
    parser::ParserDefinition,
    production::Production,
    scanner::{ScannerDefinition, ScannerRef},
};

use super::{combinator_tree::CombinatorTree, naming};
use std::{collections::BTreeMap, fmt::Debug};

#[derive(Clone, Debug, Default)]
pub struct Trie<P: Clone + Debug> {
    pub subtries: BTreeMap<char, Self>,
    // TODO: => entries: Option<(String, P)> for better constraint
    pub key: Option<String>,
    pub payload: Option<P>,
}

impl<P: Clone + Debug> Trie<P> {
    pub fn new() -> Self {
        Self {
            subtries: BTreeMap::new(),
            key: None,
            payload: None,
        }
    }

    pub fn insert(&mut self, key: &str, payload: P) {
        let chars = key.chars().collect::<Vec<_>>();
        let mut node = self;
        for i in 0..chars.len() {
            node = node.subtries.entry(chars[i]).or_insert_with(Self::new);
        }
        node.payload = Some(payload);
        node.key = Some(key.to_string());
    }

    pub fn extend(&mut self, other: Self) {
        // TODO: This is not efficient
        for (key, payload) in other.iter() {
            self.insert(&key, payload.clone());
        }
    }

    pub fn keys(&self) -> Vec<String> {
        // TODO: This is not efficient
        self.iter().map(|(k, _)| k.clone()).collect()
    }

    pub fn next_interesting_node(&self, prefix: Option<char>) -> (Vec<char>, &Trie<P>) {
        let mut path = prefix.map(|c| vec![c]).unwrap_or_default();
        let mut node = self;
        while node.payload.is_none() && node.subtries.len() == 1 {
            let (key, subtrie) = node.subtries.iter().next().unwrap();
            path.push(*key);
            node = subtrie;
        }
        (path, node)
    }

    pub fn is_empty(&self) -> bool {
        self.subtries.is_empty() && self.payload.is_none()
    }

    pub fn iter(&self) -> TrieIterator<P> {
        TrieIterator {
            subtrie_iterators: vec![self.subtries.iter()],
            leaf_iterator: self
                .payload
                .as_ref()
                .map(|p| (self.key.as_ref().unwrap(), p)),
        }
    }
}

pub struct TrieIterator<'iter, P: Clone + Debug> {
    subtrie_iterators: Vec<std::collections::btree_map::Iter<'iter, char, Trie<P>>>,
    leaf_iterator: Option<(&'iter String, &'iter P)>,
}

impl<'iter, P: Clone + Debug> Iterator for TrieIterator<'iter, P> {
    type Item = (&'iter String, &'iter P);

    fn next(&mut self) -> Option<Self::Item> {
        if self.leaf_iterator.is_some() {
            let value = self.leaf_iterator;
            self.leaf_iterator = None;
            return value;
        }

        if let Some(subtrie_iterator) = self.subtrie_iterators.last_mut() {
            if let Some((_, subtrie)) = subtrie_iterator.next() {
                self.subtrie_iterators.push(subtrie.subtries.iter());
                self.leaf_iterator = subtrie
                    .payload
                    .as_ref()
                    .map(|p| (subtrie.key.as_ref().unwrap(), p));
            } else {
                self.subtrie_iterators.pop();
            }
            return self.next();
        }

        None
    }
}

pub type TerminalTriePayload = Option<String>;
pub type TerminalTrie = Trie<TerminalTriePayload>;

pub fn from_scanner(tree: &CombinatorTree, scanner: ScannerRef) -> Option<TerminalTrie> {
    fn collect_terminals(
        trie: &mut TerminalTrie,
        tree: &CombinatorTree,
        scanner: ScannerRef,
    ) -> bool {
        match &scanner.definition {
            ScannerDefinition::Choice(exprs) => exprs.iter().fold(true, |accum, scanner| {
                accum && collect_terminals(trie, tree, scanner.clone())
            }),

            ScannerDefinition::Terminal(string) => {
                trie.insert(
                    &string,
                        None
                );
                true
            }

            ScannerDefinition::Reference(name) => collect_terminals(
                trie,
                tree,
                match tree.context.get_tree_by_name(name).production.as_ref() {
                    Production::Scanner { name, version_map } => {
                        version_map.get_for_version(&tree.context.version).expect(
                            &format!("Validation should have ensured: no version of {} exists for version {}", name, tree.context.version)
                        ).clone()
                    }
                    Production::TriviaParser { .. }
                    | Production::Parser { .. }
                    | Production::PrecedenceParser { .. } => {
                        unreachable!(
                            "Validation should have ensured: scanners can only reference scanners"
                        )
                    }
                },
            ),

            _ => false,
        }
    }

    let mut trie = TerminalTrie::new();
    if collect_terminals(&mut trie, tree, scanner.clone()) && !trie.is_empty() {
        Some(trie)
    } else {
        None
    }
}

pub fn from_parser_definition(
    tree: &CombinatorTree,
    name: Option<String>,
    parser_definition: &ParserDefinition,
) -> Option<TerminalTrie> {
    fn collect_terminals(
        trie: &mut TerminalTrie,
        tree: &CombinatorTree,
        name: Option<String>,
        parser_definition: &ParserDefinition,
        force_all_entries_to_be_named: bool,
    ) -> bool {
        match parser_definition {
            ParserDefinition::Choice(exprs) => exprs.iter().fold(true, |accum, e| {
                accum
                    && collect_terminals(
                        trie,
                        tree,
                        e.name.clone(),
                        &e.definition,
                        force_all_entries_to_be_named,
                    )
            }),

            ParserDefinition::Terminal(string) => {
                trie.insert(
                    &string,
                    name.clone()
                        .or_else(|| Some(naming::name_of_terminal_string(&string))),
                );
                true
            }

            _ => false,
        }
    }

    let mut trie = TerminalTrie::new();
    if collect_terminals(&mut trie, tree, name, parser_definition, true) && !trie.is_empty() {
        Some(trie)
    } else {
        None
    }
}

#[test]
fn test_prefix_trie() {
    use std::fmt::Write;

    let mut trie = Trie::<u8>::new();
    trie.insert("a", 1);
    trie.insert("afghb", 2);
    trie.insert("afghc", 3);
    trie.insert("b", 4);

    let mut buffer = String::new();
    for (key, value) in trie.iter() {
        write!(&mut buffer, "{}={},", key, value).unwrap();
    }

    assert!(
        buffer == r#"a=1,afghb=2,afghc=3,b=4,"#,
        "trace = {:?}",
        buffer
    );
}
