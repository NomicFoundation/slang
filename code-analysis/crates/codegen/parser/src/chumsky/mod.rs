pub mod boilerplate;
pub mod character_filter;
pub mod combinator_forest;
pub mod combinator_node;
pub mod combinator_tree;
pub mod naming;
pub mod rustfmt;
pub mod terminal_trie;

// This is temporary until we deal properly with versions
pub(crate) trait ProductionChumskyExtensions {
    fn expression_to_generate(&self) -> codegen_schema::ExpressionRef;
}

impl ProductionChumskyExtensions for codegen_schema::Production {
    fn expression_to_generate(&self) -> codegen_schema::ExpressionRef {
        self.versions.iter().last().map(|(_, e)| e.clone()).unwrap()
    }
}
