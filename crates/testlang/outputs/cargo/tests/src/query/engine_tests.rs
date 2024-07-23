use std::collections::BTreeMap;

use slang_testlang::cst::{Edge, Node};
use slang_testlang::cursor::Cursor;
use slang_testlang::kinds::{EdgeLabel, NonterminalKind, TerminalKind};
use slang_testlang::query::{Query, QueryMatch};
use slang_testlang::text_index::TextIndex;

fn terminal(label: Option<EdgeLabel>, kind: TerminalKind, text: &str) -> Edge {
    Edge {
        label,
        node: Node::terminal(kind, text.to_string()),
    }
}

fn nonterminal<const N: usize>(
    label: Option<EdgeLabel>,
    kind: NonterminalKind,
    children: [Edge; N],
) -> Edge {
    Edge {
        label,
        node: Node::nonterminal(kind, children.into_iter().collect()),
    }
}

fn capture_cursors_to_strings(
    captures: BTreeMap<String, Vec<Cursor>>,
) -> BTreeMap<String, Vec<String>> {
    captures
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

    ( @inner [ $($child:expr)* ] $label:ident : $terminal_kind:ident $text:literal $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* terminal(Some(EdgeLabel::$label), TerminalKind::$terminal_kind, $text) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $terminal_kind:ident $text:literal $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* terminal(None, TerminalKind::$terminal_kind, $text) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $label:ident : $non_terminal_kind:ident [ $($children:tt)* ] $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* nonterminal(Some(EdgeLabel::$label), NonterminalKind::$non_terminal_kind, cst_tree!(@inner [] $($children)*)) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $non_terminal_kind:ident [ $($children:tt)* ] $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* nonterminal(None, NonterminalKind::$non_terminal_kind, cst_tree!(@inner [] $($children)*)) ] $($($rest)*)?)
    };

    // Start with a nonterminal
    ( $label:ident : $non_terminal_kind:ident [ $($children:tt)* ] ) => {
        nonterminal(Some(EdgeLabel::$label), NonterminalKind::$non_terminal_kind, cst_tree!(@inner [] $($children)*))
    };

    ( $non_terminal_kind:ident [ $($children:tt)* ] ) => {
        nonterminal(None, NonterminalKind::$non_terminal_kind, cst_tree!(@inner [] $($children)*))
    };

}

macro_rules! query_matches {
    ( $( { $( $key:ident : [ $($value:literal),* ] ),* } )* ) => {
        vec![ $( {
            #[allow(unused_mut)]
            let mut captures = BTreeMap::new();
            $( captures.insert( stringify!($key).to_string(), vec![ $( $value.to_string() ),* ]); )*
            captures
        } ),* ]
    };

}

fn run_query_test(tree: &Edge, query: &str, matches: Vec<BTreeMap<String, Vec<String>>>) {
    let cursor = tree.cursor_with_offset(TextIndex::ZERO);
    let query = vec![Query::parse(query).unwrap()];
    let mut matches = matches.into_iter();
    for QueryMatch { captures, .. } in cursor.query(query) {
        let captures = capture_cursors_to_strings(captures);
        if let Some(expected_captures) = matches.next() {
            assert_eq!(captures, expected_captures);
        } else {
            panic!("Unexpected query match: {captures:?}");
        }
    }
    if let Some(expected_captures) = matches.next() {
        panic!("Missing query match: {expected_captures:?}");
    }
}

fn common_test_tree() -> Edge {
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

fn common_test_tree_with_trivia() -> Edge {
    cst_tree!(
        TreeNode [
            Node: DelimitedIdentifier "A",
            Whitespace " ",
            DelimitedIdentifier "B",
            Whitespace " ",
            EndOfLine "\n",
            DelimitedIdentifier "C",
            TreeNodeChild [
                Whitespace " ",
                DelimitedIdentifier "D",
                EndOfLine "\n",
                Whitespace " ",
                Node: DelimitedIdentifier "E",
                Whitespace " ",
            ],
        ]
    )
}

#[test]
fn test_spread() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode @x1 [DelimitedIdentifier] @x2 [DelimitedIdentifier]]",
        query_matches! {
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
        "[TreeNode @y1 [DelimitedIdentifier] . @y2 [DelimitedIdentifier]]",
        query_matches! {
            {y1: ["A"], y2: ["B"]}
            {y1: ["B"], y2: ["C"]}
        },
    );
}

#[test]
fn test_anchor_skips_trivia() {
    run_query_test(
        &common_test_tree_with_trivia(),
        "[TreeNode @y1 [DelimitedIdentifier] . @y2 [DelimitedIdentifier]]",
        query_matches! {
            {y1: ["A"], y2: ["B"]}
            {y1: ["B"], y2: ["C"]}
        },
    );
}

#[test]
fn test_child() {
    run_query_test(
        &common_test_tree(),
        "[TreeNodeChild @x [DelimitedIdentifier]]",
        query_matches! {
            {x: ["D"]}
            {x: ["E"]}
        },
    );
}

#[test]
fn test_parent_and_child() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode @p node:[_] [TreeNodeChild @c [DelimitedIdentifier]]]",
        query_matches! {
            {c: ["D"], p: ["A"]}
            {c: ["E"], p: ["A"]}
        },
    );
}

#[test]
fn test_named() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode @x node:[DelimitedIdentifier]]",
        query_matches! {
            {x: ["A"]}
        },
    );
}

#[test]
fn test_multilevel_adjacent() {
    run_query_test(
        &common_test_tree(),
        "[_ @x [DelimitedIdentifier] . @y [DelimitedIdentifier]]",
        query_matches! {
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
        "[_ @x node:[_]]",
        query_matches! {
            {x: ["A"]}
            {x: ["E"]}
        },
    );
}

#[test]
fn test_text_value() {
    run_query_test(
        &common_test_tree(),
        r#"[TreeNode @z1 [DelimitedIdentifier] . ["B"] . @z2 [DelimitedIdentifier]]"#,
        query_matches! {
            {z1: ["A"], z2: ["C"]}
        },
    );
}

#[test]
fn test_one_or_more() {
    run_query_test(
        &common_test_tree(),
        "[TreeNode (@x [DelimitedIdentifier])+ . [_] .]",
        query_matches! {
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
        "[TreeNode (@y [DelimitedIdentifier])* . [_] .]",
        query_matches! {
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
        "[TreeNode (@z [DelimitedIdentifier])? . [_] .]",
        query_matches! {
            {z: ["C"]}
            {}
        },
    );
}

#[test]
fn test_nested() {
    run_query_test(
        &common_test_tree(),
        "@root [TreeNode @z [DelimitedIdentifier] . [_] .]",
        query_matches! {
            {root: ["ABCDE"], z: ["C"]}
        },
    );
}

#[test]
fn test_alternatives() {
    run_query_test(
        &common_test_tree(),
        "(@x node:[_] | @y [DelimitedIdentifier] . @z [DelimitedIdentifier])",
        query_matches! {
            {x: ["A"]}
            {y: ["A"], z: ["B"]}
            {y: ["B"], z: ["C"]}
            {y: ["D"], z: ["E"]}
            {x: ["E"]}
        },
    );
}

#[test]
fn test_anchor_at_beginning_skips_trivia() {
    run_query_test(
        &common_test_tree_with_trivia(),
        "[TreeNodeChild . @x [DelimitedIdentifier]]",
        query_matches! {
            {x: ["D"]}
        },
    );
}

#[test]
fn test_anchor_at_end_skips_trivia() {
    run_query_test(
        &common_test_tree_with_trivia(),
        "[TreeNodeChild @x [DelimitedIdentifier] .]",
        query_matches! {
            {x: ["E"]}
        },
    );
}

fn other_tree() -> Edge {
    cst_tree!(
        TreeNode [
            Node: DelimitedIdentifier "A",
            Whitespace " ",
            DelimitedIdentifier "B",
            DelimitedIdentifier "C",
            DelimitedIdentifier "D",
        ]
    )
}

#[test]
fn test_grouping_interaction_with_trivia() {
    run_query_test(
        &other_tree(),
        "[TreeNode @x [DelimitedIdentifier] (@y [DelimitedIdentifier] . @z [DelimitedIdentifier])?]",
        query_matches! {
            {x: ["A"]}
            {x: ["A"]}
            {x: ["A"], y: ["B"], z: ["C"]}
            {x: ["A"]}
            {x: ["A"], y: ["C"], z: ["D"]}
            {x: ["A"]}
            {x: ["A"]}
            {x: ["B"]}
            {x: ["B"], y: ["C"], z: ["D"]}
            {x: ["B"]}
            {x: ["B"]}
            {x: ["C"]}
            {x: ["C"]}
            {x: ["D"]}
        },
    );
}

#[test]
fn test_anchored_grouping_interaction_with_trivia() {
    run_query_test(
        &other_tree(),
        "[TreeNode @x [DelimitedIdentifier] . (@y [DelimitedIdentifier] . @z [DelimitedIdentifier])?]",
        query_matches! {
            {x: ["A"]}
            {x: ["A"], y: ["B"], z: ["C"]}
            {x: ["B"]}
            {x: ["B"], y: ["C"], z: ["D"]}
            {x: ["C"]}
            {x: ["D"]}
        },
    );
}
