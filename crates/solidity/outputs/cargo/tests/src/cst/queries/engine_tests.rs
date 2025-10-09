use std::collections::BTreeMap;

use slang_solidity::cst::{
    Edge, EdgeLabel, Node, NonterminalKind, Query, QueryMatch, TerminalKind, TextIndex,
};

fn terminal(label: EdgeLabel, kind: TerminalKind, text: &str) -> Edge {
    Edge {
        label,
        node: Node::terminal(kind, text.to_string()),
    }
}

fn nonterminal<const N: usize>(
    label: EdgeLabel,
    kind: NonterminalKind,
    children: [Edge; N],
) -> Edge {
    Edge {
        label,
        node: Node::nonterminal(kind, children.into_iter().collect()),
    }
}

fn capture_cursors_to_strings(query_match: &QueryMatch) -> BTreeMap<String, Vec<String>> {
    query_match
        .captures()
        .map(|capture| {
            (
                capture.name().to_string(),
                capture
                    .cursors()
                    .iter()
                    .map(|c| c.node().unparse())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>()
}

#[allow(unused_macro_rules)]
macro_rules! cst_tree {
    ( @inner [ $($child:expr)* ]) => { [ $($child),* ] };

    ( @inner [ $($child:expr)* ] $label:ident : $terminal_kind:ident $text:literal $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* terminal(EdgeLabel::$label, TerminalKind::$terminal_kind, $text) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $terminal_kind:ident $text:literal $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* terminal(EdgeLabel::Root, TerminalKind::$terminal_kind, $text) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $label:ident : $non_terminal_kind:ident [ $($children:tt)* ] $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* nonterminal(EdgeLabel::$label, NonterminalKind::$non_terminal_kind, cst_tree!(@inner [] $($children)*)) ] $($($rest)*)?)
    };

    ( @inner [ $($child:expr)* ] $non_terminal_kind:ident [ $($children:tt)* ] $(, $($rest:tt)*)? ) => {
        cst_tree!(@inner [ $($child)* nonterminal(EdgeLabel::Root, NonterminalKind::$non_terminal_kind, cst_tree!(@inner [] $($children)*)) ] $($($rest)*)?)
    };

    // Start with a nonterminal
    ( $label:ident : $non_terminal_kind:ident [ $($children:tt)* ] ) => {
        nonterminal(EdgeLabel::$label, NonterminalKind::$non_terminal_kind, cst_tree!(@inner [] $($children)*))
    };

    ( $non_terminal_kind:ident [ $($children:tt)* ] ) => {
        nonterminal(EdgeLabel::Root, NonterminalKind::$non_terminal_kind, cst_tree!(@inner [] $($children)*))
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

fn run_query_test(tree: Edge, query: &str, expected_matches: Vec<BTreeMap<String, Vec<String>>>) {
    let cursor = tree.node.create_cursor(TextIndex::ZERO);
    let query = vec![Query::create(query).unwrap()];
    let mut expected_matches = expected_matches.into_iter();
    for query_match in cursor.query(query) {
        let captures = capture_cursors_to_strings(&query_match);
        if let Some(expected_captures) = expected_matches.next() {
            assert_eq!(captures, expected_captures);
        } else {
            panic!("Unexpected query match: {captures:?}");
        }
    }
    if let Some(expected_captures) = expected_matches.next() {
        panic!("Missing query match: {expected_captures:?}");
    }
}

fn common_test_tree() -> Edge {
    // This definition doesn't make sense as Solidity grammar. That's OK, we just want to test queries.
    cst_tree!(
        ContractDefinition [
            Name: Identifier "A",
            Identifier "B",
            Identifier "C",
            ContractMembers [
                Identifier "D",
                Name: Identifier "E",
            ],
        ]
    )
}

fn common_test_tree_with_trivia() -> Edge {
    cst_tree!(
        ContractDefinition [
            Name: Identifier "A",
            Whitespace " ",
            Identifier "B",
            Whitespace " ",
            EndOfLine "\n",
            Identifier "C",
            ContractMembers [
                Whitespace " ",
                Identifier "D",
                EndOfLine "\n",
                Whitespace " ",
                Name: Identifier "E",
                Whitespace " ",
            ],
        ]
    )
}

#[test]
fn test_spread() {
    run_query_test(
        common_test_tree(),
        "[ContractDefinition @x1 [Identifier] @x2 [Identifier]]",
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
        common_test_tree(),
        "[ContractDefinition @y1 [Identifier] . @y2 [Identifier]]",
        query_matches! {
            {y1: ["A"], y2: ["B"]}
            {y1: ["B"], y2: ["C"]}
        },
    );
}

#[test]
fn test_adjacency_skips_trivia() {
    run_query_test(
        common_test_tree_with_trivia(),
        "[ContractDefinition @y1 [Identifier] . @y2 [Identifier]]",
        query_matches! {
            {y1: ["A"], y2: ["B"]}
            {y1: ["B"], y2: ["C"]}
        },
    );
}

#[test]
fn test_anonymous_node_matcher_skips_trivia() {
    run_query_test(
        common_test_tree_with_trivia(),
        "[ContractMembers @x [_]]",
        query_matches! {
            {x: ["D"]}
            {x: ["E"]}
        },
    );
}

#[test]
fn test_child() {
    run_query_test(
        common_test_tree(),
        "[ContractMembers @x [Identifier]]",
        query_matches! {
            {x: ["D"]}
            {x: ["E"]}
        },
    );
}

#[test]
fn test_parent_and_child() {
    run_query_test(
        common_test_tree(),
        "[ContractDefinition @p name:[_] [ContractMembers @c [Identifier]]]",
        query_matches! {
            {c: ["D"], p: ["A"]}
            {c: ["E"], p: ["A"]}
        },
    );
}

#[test]
fn test_named() {
    run_query_test(
        common_test_tree(),
        "[ContractDefinition @x name:[Identifier]]",
        query_matches! {
            {x: ["A"]}
        },
    );
}

#[test]
fn test_multilevel_adjacent() {
    run_query_test(
        common_test_tree(),
        "[_ @x [Identifier] . @y [Identifier]]",
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
        common_test_tree(),
        "[_ @x name:[_]]",
        query_matches! {
            {x: ["A"]}
            {x: ["E"]}
        },
    );
}

#[test]
fn test_text_value() {
    run_query_test(
        common_test_tree(),
        r#"[ContractDefinition @z1 [Identifier] . ["B"] . @z2 [Identifier]]"#,
        query_matches! {
            {z1: ["A"], z2: ["C"]}
        },
    );
}

#[test]
fn test_one_or_more() {
    run_query_test(
        common_test_tree(),
        "[ContractDefinition (@x [Identifier])+ . [_] .]",
        query_matches! {
            {x: ["A", "B", "C"]}
            {x: ["B", "C"]}
            {x: ["C"]}
        },
    );
}

#[test]
fn test_one_or_more_anonymous() {
    run_query_test(
        common_test_tree(),
        "[ContractDefinition (@x [_])+ .]",
        query_matches! {
            {x: ["A", "B", "C", "DE"]}
            {x: ["B", "C", "DE"]}
            {x: ["C", "DE"]}
            {x: ["DE"]}
        },
    );
}

#[test]
fn test_one_or_more_anonymous_both_adjacent() {
    run_query_test(
        common_test_tree(),
        "[ContractDefinition . (@x [_])+ .]",
        query_matches! {
            {x: ["A", "B", "C", "DE"]}
        },
    );
}

#[test]
fn test_one_or_more_anonymous_both_adjacent_with_trivia() {
    run_query_test(
        common_test_tree_with_trivia(),
        "[ContractMembers . @children [_]+ .]",
        query_matches! {
            {children: ["D", "E"]}
        },
    );
}

#[test]
fn test_zero_or_more() {
    run_query_test(
        common_test_tree(),
        "[ContractDefinition (@y [Identifier])* . [_] .]",
        query_matches! {
            {y: ["A", "B", "C"]}
            {y: ["B", "C"]}
            {y: ["C"]}
            {y: []}
        },
    );
}

#[test]
fn test_optional() {
    run_query_test(
        common_test_tree(),
        "[ContractDefinition (@z [Identifier])? . [_] .]",
        query_matches! {
            {z: ["C"]}
            {z: []}
        },
    );
}

#[test]
fn test_nested() {
    run_query_test(
        common_test_tree(),
        "@root [ContractDefinition @z [Identifier] . [_] .]",
        query_matches! {
            {root: ["ABCDE"], z: ["C"]}
        },
    );
}

#[test]
fn test_alternatives() {
    run_query_test(
        common_test_tree(),
        "(@x name:[_] | @y [Identifier] . @z [Identifier])",
        query_matches! {
            {x: ["A"], y: [], z: []}
            {x: [], y: ["A"], z: ["B"]}
            {x: [], y: ["B"], z: ["C"]}
            {x: [], y: ["D"], z: ["E"]}
            {x: ["E"], y: [], z: []}
        },
    );
}

#[test]
fn test_adjacency_at_beginning_skips_trivia() {
    run_query_test(
        common_test_tree_with_trivia(),
        "[ContractMembers . @x [Identifier]]",
        query_matches! {
            {x: ["D"]}
        },
    );
}

#[test]
fn test_adjacency_at_end_skips_trivia() {
    run_query_test(
        common_test_tree_with_trivia(),
        "[ContractMembers @x [Identifier] .]",
        query_matches! {
            {x: ["E"]}
        },
    );
}

fn flat_tree() -> Edge {
    cst_tree!(
        ContractDefinition [
            Name: Identifier "A",
            Whitespace " ",
            Identifier "B",
            Identifier "C",
            Identifier "D",
        ]
    )
}

#[test]
fn test_ellipsis_followed_by_optional_grouping() {
    run_query_test(
        flat_tree(),
        "[ContractDefinition @x [Identifier] (@y [Identifier] . @z [Identifier])?]",
        query_matches! {
            {x: ["A"], y: ["B"], z: ["C"]}
            {x: ["A"], y: ["C"], z: ["D"]}
            {x: ["A"], y: [], z:[]}
            {x: ["B"], y: ["C"], z: ["D"]}
            {x: ["B"], y: [], z:[]}
            {x: ["C"], y: [], z:[]}
            {x: ["D"], y: [], z:[]}
        },
    );
}

#[test]
fn test_adjacency_followed_by_optional_grouping() {
    run_query_test(
        flat_tree(),
        "[ContractDefinition @x [Identifier] . (@y [Identifier] . @z [Identifier])?]",
        query_matches! {
            {x: ["A"], y: [], z:[]}
            {x: ["A"], y: ["B"], z: ["C"]}
            {x: ["B"], y: [], z:[]}
            {x: ["B"], y: ["C"], z: ["D"]}
            {x: ["C"], y: [], z:[]}
            {x: ["D"], y: [], z:[]}
        },
    );
}

#[test]
fn test_captures_followed_by_non_captured_matchers() {
    run_query_test(
        flat_tree(),
        "[ContractDefinition @x [Identifier] [Identifier]]",
        query_matches! {
            {x: ["A"]}
            {x: ["A"]}
            {x: ["A"]}
            {x: ["B"]}
            {x: ["B"]}
            {x: ["C"]}
        },
    );
}

#[test]
fn test_captures_followed_by_anonymous_matchers() {
    run_query_test(
        flat_tree(),
        "[ContractDefinition @x [Identifier] [_]]",
        query_matches! {
            {x: ["A"]}
            {x: ["A"]}
            {x: ["A"]}
            {x: ["B"]}
            {x: ["B"]}
            {x: ["C"]}
        },
    );
}

#[test]
fn test_captures_followed_by_non_captured_optional_matchers() {
    run_query_test(
        flat_tree(),
        "[ContractDefinition @x [Identifier] [Identifier]?]",
        query_matches! {
            {x: ["A"]}
            {x: ["A"]}
            {x: ["A"]}
            {x: ["A"]}
            {x: ["B"]}
            {x: ["B"]}
            {x: ["B"]}
            {x: ["C"]}
            {x: ["C"]}
            {x: ["D"]}
        },
    );
}

#[test]
fn test_captures_followed_by_captured_optional_matchers() {
    run_query_test(
        flat_tree(),
        "[ContractDefinition @x [Identifier] @y [Identifier]?]",
        query_matches! {
            {x: ["A"], y: ["B"]}
            {x: ["A"], y: ["C"]}
            {x: ["A"], y: ["D"]}
            {x: ["A"], y: []}
            {x: ["B"], y: ["C"]}
            {x: ["B"], y: ["D"]}
            {x: ["B"], y: []}
            {x: ["C"], y: ["D"]}
            {x: ["C"], y: []}
            {x: ["D"], y: []}
        },
    );
}

fn sample_deep_tree() -> Edge {
    cst_tree!(
        SourceUnit [
            ContractKeyword: ContractKeyword "contract",
            Name: Identifier "$t1",
            Name: ContractDefinition [
                OpenBracket "[",
                Members: ContractMembers [
                    ContractMembers [
                        Variant: Identifier "A",
                    ],
                    ContractMembers [
                        Variant: ContractDefinition [
                            OpenBracket "[",
                            Members: ContractMembers [
                                ContractMembers [
                                    Variant: Identifier "B"
                                ],
                                ContractMembers [
                                    Variant: Identifier "C"
                                ],
                            ],
                            CloseBracket "]",
                        ]
                    ]
                ],
                CloseBracket "]",
            ],
            Semicolon: Semicolon ";"
        ]
    )
}

#[test]
fn test_deeply_nested_matchers() {
    run_query_test(
        sample_deep_tree(),
        "@parent [ContractDefinition members: [ContractMembers [ContractMembers @child variant: [ContractDefinition]]]]",
        query_matches! {
            {parent: ["[A[BC]]"], child: ["[BC]"]}
        },
    );
}
