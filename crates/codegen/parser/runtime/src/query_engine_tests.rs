use std::collections::{BTreeMap, HashMap};

use super::cst::{NamedNode, Node};
use super::cursor::Cursor;
use super::kinds::{RuleKind, TokenKind};
use super::query_model::Query;
use super::text_index::TextIndex;

fn token(kind: TokenKind, text: &str) -> NamedNode {
    NamedNode {
        name: None,
        node: Node::token(kind, text.to_string()),
    }
}

fn rule<const N: usize>(kind: RuleKind, children: [NamedNode; N]) -> NamedNode {
    NamedNode {
        name: None,
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

    ( @inner [ $($child:expr)* ] $kind:ident $text:literal $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* token(TokenKind::$kind, $text) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $kind:ident [ $($children:tt)* ] $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* rule(RuleKind::$kind, cst_tree!(@inner [] $($children)*)) ] $($($rest)*)?)
    };

    // Start with a rule
    ( $kind:ident [ $($children:tt)* ] ) => {
        rule(RuleKind::$kind, cst_tree!(@inner [] $($children)*))
    };

}

macro_rules! query_result {
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
    for (_query_no, bindings) in cursor.query(query) {
        let bindings = binding_cursors_to_strings(bindings);
        if let Some(expected_bindings) = results.next() {
            assert_eq!(bindings, expected_bindings);
        } else {
            panic!("Unexpected query result");
        }
    }
    assert!(results.next().is_none(), "Missing query result");
}

fn common_test_tree() -> NamedNode {
    cst_tree!(
        A [
            X "t1",
            X "t2",
            X "t3",
        ]
    )
}

#[test]
fn test_spread() {
    run_query_test(
        &common_test_tree(),
        "[A ... @x1 [X] ... @x2 [X] ...]",
        query_result! {
            {x1: ["t1"], x2: ["t2"]}
            {x1: ["t1"], x2: ["t3"]}
            {x1: ["t2"], x2: ["t3"]}
        },
    );
}

#[test]
fn test_adjacent() {
    run_query_test(
        &common_test_tree(),
        "[A ... @y1 [X] @y2 [X] ...]",
        query_result! {
            {y1: ["t1"], y2: ["t2"]}
            {y1: ["t2"], y2: ["t3"]}
        },
    );
}

#[test]
fn test_text_value() {
    run_query_test(
        &common_test_tree(),
        r#"[A ... @z1 [X] ["t2"] @z2 [X] ...]"#,
        query_result! {
            {z1: ["t1"], z2: ["t3"]}
        },
    );
}

#[test]
fn test_one_or_more() {
    run_query_test(
        &common_test_tree(),
        "[A ... (@x [X])+ ]",
        query_result! {
            {x: ["t1", "t2", "t3"]}
            {x: ["t2", "t3"]}
            {x: ["t3"]}
        },
    );
}

#[test]
fn test_zero_or_more() {
    run_query_test(
        &common_test_tree(),
        "[A ... (@y [X])* ]",
        query_result! {
            {y: ["t1", "t2", "t3"]}
            {y: ["t2", "t3"]}
            {y: ["t3"]}
            {}
        },
    );
}

#[test]
fn test_optional() {
    run_query_test(
        &common_test_tree(),
        "[A ... (@z [X])? ]",
        query_result! {
            {z: ["t3"]}
            {}
        },
    );
}
