use std::collections::{BTreeMap, HashMap};

// This crate is copied to another crate, so all imports should be relative
use slang_testlang::cst::{NamedNode, Node};
use slang_testlang::cursor::Cursor;
use slang_testlang::kinds::{FieldName, RuleKind, TokenKind};
use slang_testlang::query::{Query, QueryResult};
use slang_testlang::text_index::TextIndex;

fn token(name: Option<FieldName>, kind: TokenKind, text: &str) -> NamedNode {
    NamedNode {
        name,
        node: Node::token(kind, text.to_string()),
    }
}

fn rule<const N: usize>(
    name: Option<FieldName>,
    kind: RuleKind,
    children: [NamedNode; N],
) -> NamedNode {
    NamedNode {
        name,
        node: Node::rule(kind, children.into_iter().collect()),
    }
}

fn binding_cursors_to_strings(
    bindings: HashMap<String, Vec<Cursor>>,
) -> BTreeMap<String, Vec<String>> {
    bindings
        .into_iter()
        .map(|(key, values)| {
            (
                key,
                values
                    .iter()
                    .map(|v| v.node().unparse())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>()
}

#[allow(unused_macro_rules)]
macro_rules! cst_tree {
    ( @inner [ $($child:expr)* ]) => { [ $($child),* ] };

    ( @inner [ $($child:expr)* ] $field_name:ident : $token_kind:ident $text:literal $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* token(Some(FieldName::$field_name), TokenKind::$token_kind, $text) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $token_kind:ident $text:literal $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* token(None, TokenKind::$token_kind, $text) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $field_name:ident : $rule_kind:ident [ $($children:tt)* ] $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* rule(Some(FieldName::$field_name), RuleKind::$rule_kind, cst_tree!(@inner [] $($children)*)) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $rule_kind:ident [ $($children:tt)* ] $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* rule(None, RuleKind::$rule_kind, cst_tree!(@inner [] $($children)*)) ] $($($rest)*)?)
    };

    // Start with a rule
    ( $field_name:ident : $rule_kind:ident [ $($children:tt)* ] ) => {
        rule(Some(FieldName::$field_name), RuleKind::$rule_kind, cst_tree!(@inner [] $($children)*))
    };

    ( $rule_kind:ident [ $($children:tt)* ] ) => {
        rule(None, RuleKind::$rule_kind, cst_tree!(@inner [] $($children)*))
    };

}

macro_rules! query_results {
    ( $( { $( $key:ident : [ $($value:literal),* ] ),* } )* ) => {
        vec![ $( {
            #[allow(unused_mut)]
            let mut bindings = BTreeMap::new();
            $( bindings.insert( stringify!($key).to_string(), vec![ $( $value.to_string() ),* ]); )*
            bindings
        } ),* ]
    };

}

fn run_query_test(tree: &NamedNode, query: &str, results: Vec<BTreeMap<String, Vec<String>>>) {
    let cursor = tree.cursor_with_offset(TextIndex::ZERO);
    let query = vec![Query::parse(query).unwrap()];
    let mut results = results.into_iter();
    for QueryResult { bindings, .. } in cursor.query(query) {
        let bindings = binding_cursors_to_strings(bindings);
        if let Some(expected_bindings) = results.next() {
            assert_eq!(bindings, expected_bindings);
        } else {
            panic!("Unexpected query result: {bindings:?}");
        }
    }
    if let Some(expected_bindings) = results.next() {
        panic!("Missing query result: {expected_bindings:?}");
    }
}

fn common_test_tree() -> NamedNode {
    cst_tree!(
        TreeNode [
            Node: DelimitedIdentifier "A",
            DelimitedIdentifier "B",
            DelimitedIdentifier "C",
            TreeNodeChild [
                DelimitedIdentifier "D",
                Node: DelimitedIdentifier "E",
            ],
        ]
    )
}

#[test]
fn test_spread() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode ... @x1 [DelimitedIdentifier] ... @x2 [DelimitedIdentifier] ...]",
        query_results! {
            {x1: ["A"], x2: ["B"]}
            {x1: ["A"], x2: ["C"]}
            {x1: ["B"], x2: ["C"]}
        },
    );
}

#[test]
fn test_adjacent() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode ... @y1 [DelimitedIdentifier] @y2 [DelimitedIdentifier] ...]",
        query_results! {
            {y1: ["A"], y2: ["B"]}
            {y1: ["B"], y2: ["C"]}
        },
    );
}

#[test]
fn test_child() {
    run_query_test(
        &common_test_tree(),
        "[TreeNodeChild ... @x [DelimitedIdentifier] ...]",
        query_results! {
            {x: ["D"]}
            {x: ["E"]}
        },
    );
}

#[test]
fn test_parent_and_child() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode ... @p [node:_] ...  [TreeNodeChild ... @c [DelimitedIdentifier] ...]]",
        query_results! {
            {c: ["D"], p: ["A"]}
            {c: ["E"], p: ["A"]}
        },
    );
}

#[test]
fn test_named() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode ... @x [node:DelimitedIdentifier] ...]",
        query_results! {
            {x: ["A"]}
        },
    );
}

#[test]
fn test_multilevel_adjacent() {
    run_query_test(
        &common_test_tree(),
        "[_ ... @x [DelimitedIdentifier] @y [DelimitedIdentifier] ...]",
        query_results! {
            {x: ["A"], y: ["B"]}
            {x: ["B"], y: ["C"]}
            {x: ["D"], y: ["E"]}
        },
    );
}

#[test]
fn test_multilevel_named() {
    run_query_test(
        &common_test_tree(),
        "[_ ... @x [node:_] ...]",
        query_results! {
            {x: ["A"]}
            {x: ["E"]}
        },
    );
}

#[test]
fn test_text_value() {
    run_query_test(
        &common_test_tree(),
        r#"[TreeNode ... @z1 [DelimitedIdentifier] ["B"] @z2 [DelimitedIdentifier] ...]"#,
        query_results! {
            {z1: ["A"], z2: ["C"]}
        },
    );
}

#[test]
fn test_one_or_more() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode ... (@x [DelimitedIdentifier])+ [_] ]",
        query_results! {
            {x: ["A", "B", "C"]}
            {x: ["B", "C"]}
            {x: ["C"]}
        },
    );
}

#[test]
fn test_zero_or_more() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode ... (@y [DelimitedIdentifier])* [_] ]",
        query_results! {
            {y: ["A", "B", "C"]}
            {y: ["B", "C"]}
            {y: ["C"]}
            {}
        },
    );
}

#[test]
fn test_optional() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode ... (@z [DelimitedIdentifier])? [_] ]",
        query_results! {
            {z: ["C"]}
            {}
        },
    );
}
