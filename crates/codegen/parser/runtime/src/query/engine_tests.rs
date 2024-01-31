use std::collections::{BTreeMap, HashMap};

// This crate is copied to another crate, so all imports should be relative
use super::super::cst::{NamedNode, Node};
use super::super::cursor::Cursor;
use super::super::kinds::{FieldName, RuleKind, TokenKind};
use super::super::text_index::TextIndex;
use super::engine::QueryResult;
use super::model::Query;

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
        Rule1 [
            Name1: Token1 "t1",
            Token1 "t2",
            Token1 "t3",
            Rule2 [
                Token1 "t5",
                Name1: Token1 "t6",
            ],
        ]
    )
}

#[test]
fn test_spread() {
    run_query_test(
        &common_test_tree(),
        "[Rule1 ... @x1 [Token1] ... @x2 [Token1] ...]",
        query_results! {
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
        "[Rule1 ... @y1 [Token1] @y2 [Token1] ...]",
        query_results! {
            {y1: ["t1"], y2: ["t2"]}
            {y1: ["t2"], y2: ["t3"]}
        },
    );
}

#[test]
fn test_child() {
    run_query_test(
        &common_test_tree(),
        "[Rule2 ... @x [Token1] ...]",
        query_results! {
            {x: ["t5"]}
            {x: ["t6"]}
        },
    );
}

#[test]
fn test_parent_and_child() {
    run_query_test(
        &common_test_tree(),
        "[Rule1 ... @p [Name1:_] ...  [Rule2 ... @c [Token1] ...]]",
        query_results! {
            {c: ["t5"], p: ["t1"]}
            {c: ["t6"], p: ["t1"]}
        },
    );
}

#[test]
fn test_named() {
    run_query_test(
        &common_test_tree(),
        "[Rule1 ... @x [Name1:Token1] ...]",
        query_results! {
            {x: ["t1"]}
        },
    );
}

#[test]
fn test_multilevel_adjacent() {
    run_query_test(
        &common_test_tree(),
        "[_ ... @x [Token1] @y [Token1] ...]",
        query_results! {
            {x: ["t1"], y: ["t2"]}
            {x: ["t2"], y: ["t3"]}
            {x: ["t5"], y: ["t6"]}
        },
    );
}

#[test]
fn test_multilevel_named() {
    run_query_test(
        &common_test_tree(),
        "[_ ... @x [Name1:_] ...]",
        query_results! {
            {x: ["t1"]}
            {x: ["t6"]}
        },
    );
}

#[test]
fn test_text_value() {
    run_query_test(
        &common_test_tree(),
        r#"[Rule1 ... @z1 [Token1] ["t2"] @z2 [Token1] ...]"#,
        query_results! {
            {z1: ["t1"], z2: ["t3"]}
        },
    );
}

#[test]
fn test_one_or_more() {
    run_query_test(
        &common_test_tree(),
        "[Rule1 ... (@x [Token1])+ [_] ]",
        query_results! {
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
        "[Rule1 ... (@y [Token1])* [_] ]",
        query_results! {
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
        "[Rule1 ... (@z [Token1])? [_] ]",
        query_results! {
            {z: ["t3"]}
            {}
        },
    );
}
