// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{
    super::{cst, kinds::RuleKind},
    parser_result::{
        ParserResult,
        PrattElement::{self, Binary, Expression, Postfix, Prefix},
    },
};

pub struct PrecedenceHelper;

impl PrecedenceHelper {
    pub fn to_prefix_operator(kind: RuleKind, right: u8, result: ParserResult) -> ParserResult {
        match result {
            ParserResult::Match(r#match) => ParserResult::pratt_operator_match(vec![Prefix {
                nodes: r#match.nodes,
                kind,
                right,
            }]),
            ParserResult::PrattOperatorMatch(_) => {
                unreachable!("This is already a PrattOperatorMatch")
            }
            _ => result,
        }
    }

    pub fn to_postfix_operator(kind: RuleKind, left: u8, result: ParserResult) -> ParserResult {
        match result {
            ParserResult::Match(r#match) => ParserResult::pratt_operator_match(vec![Postfix {
                nodes: r#match.nodes,
                kind,
                left,
            }]),
            ParserResult::PrattOperatorMatch(_) => {
                unreachable!("This is already a PrattOperatorMatch")
            }
            _ => result,
        }
    }

    pub fn to_binary_operator(
        kind: RuleKind,
        left: u8,
        right: u8,
        result: ParserResult,
    ) -> ParserResult {
        match result {
            ParserResult::Match(r#match) => ParserResult::pratt_operator_match(vec![Binary {
                nodes: r#match.nodes,
                kind,
                left,
                right,
            }]),
            ParserResult::PrattOperatorMatch(_) => {
                unreachable!("This is already a PrattOperatorMatch")
            }
            _ => result,
        }
    }
    pub fn reduce_precedence_result(
        child_kind: Option<RuleKind>,
        result: ParserResult,
    ) -> ParserResult {
        // This requires some careful thinking. It could be more compact,
        // but I'm favouring obviousness here. That is also why there are
        // so many `unreachable!` - not only should they never be reached,
        // they also tell the reader what invariants should apply.

        // If the input is valid this should be correct by construction.

        match result {
            ParserResult::PrattOperatorMatch(pratt_operator_match) => {
                let mut pratt_elements = pratt_operator_match.elements;

                let mut i = 0usize;
                while pratt_elements.len() > 1 {
                    // 1. Find the next, highest priority reducable operator
                    // The elements are guaranteed to match the following:
                    // `prefixop* expr postfixop* ( binaryop prefixop* expr postfixop* )*`

                    let slice = &pratt_elements[i..];
                    match slice {
                        [Expression { .. }, Postfix { .. }, ..] => {
                            i += 1;
                            continue;
                        }

                        [Expression { .. }, Binary { .. }, ..] => {
                            i += 1;
                            continue;
                        }

                        [Prefix { right, .. }, Expression { .. }, Binary { left, .. }, ..] => {
                            if right <= left {
                                i += 2;
                                continue;
                            } else {
                                /* Reduce */
                            }
                        }

                        [Prefix { right, .. }, Expression { .. }, Postfix { left, .. }, ..] => {
                            if right <= left {
                                i += 2;
                                continue;
                            } else {
                                /* Reduce */
                            }
                        }

                        [Prefix { .. }, Expression { .. }] => { /* Reduce */ }

                        [Prefix { .. }, Prefix { .. }, ..] => {
                            i += 1;
                            continue;
                        }

                        [Binary { right, .. }, Expression { .. }, Binary { left, .. }, ..] => {
                            if right <= left {
                                i += 2;
                                continue;
                            } else {
                                /* Reduce */
                            }
                        }

                        [Binary { right, .. }, Expression { .. }, Postfix { left, .. }, ..] => {
                            if right <= left {
                                i += 2;
                                continue;
                            } else {
                                /* Reduce */
                            }
                        }

                        [Binary { .. }, Prefix { .. }, ..] => {
                            i += 1;
                            continue;
                        }

                        [Binary { .. }, Expression { .. }] => { /* Reduce */ }

                        [Postfix { .. }, ..] => { /* Reduce */ }

                        _ => {
                            unreachable!(
                                "Unmatched precedence pattern at index {} in: {:#?}",
                                i, pratt_elements
                            )
                        }
                    };

                    // 2. Reduce the operator and it's child expressions to a new expression

                    let make_expression =
                        |left: Option<PrattElement>,
                         kind: RuleKind,
                         nodes: Vec<cst::Node>,
                         right: Option<PrattElement>| {
                            let wrap_children = |children: Vec<cst::Node>| {
                                if children.is_empty() {
                                    children
                                } else if let Some(kind) = child_kind {
                                    vec![cst::Node::rule(kind, children)]
                                } else {
                                    children
                                }
                            };

                            let left_nodes = match left {
                                Some(Expression { nodes }) => nodes,
                                None => vec![],
                                _ => unreachable!("Operator not preceeded by expression"),
                            };

                            let right_nodes = match right {
                                Some(Expression { nodes }) => nodes,
                                None => vec![],
                                _ => unreachable!("Operator not followed by expression"),
                            };

                            let mut children = Vec::with_capacity(
                                left_nodes.len() + nodes.len() + right_nodes.len(),
                            );
                            children.extend(wrap_children(left_nodes));
                            children.extend(nodes);
                            children.extend(wrap_children(right_nodes));
                            Expression {
                                nodes: vec![cst::Node::rule(kind, children)],
                            }
                        };

                    match pratt_elements.remove(i) {
                        Prefix { kind, nodes, .. } => {
                            let expr = pratt_elements.remove(i);
                            pratt_elements
                                .insert(i, make_expression(None, kind, nodes, Some(expr)));
                        }

                        Postfix { kind, nodes, .. } => {
                            let expr = pratt_elements.remove(i - 1);
                            i -= 1;
                            pratt_elements
                                .insert(i, make_expression(Some(expr), kind, nodes, None));
                        }

                        Binary { kind, nodes, .. } => {
                            let right_expr = pratt_elements.remove(i);
                            let left_expr = pratt_elements.remove(i - 1);
                            i -= 1;
                            pratt_elements.insert(
                                i,
                                make_expression(Some(left_expr), kind, nodes, Some(right_expr)),
                            );
                        }

                        Expression { .. } => {
                            unreachable!(
                                "Expected an operator at index {}: {:#?}",
                                i, pratt_elements
                            )
                        }
                    }

                    i = i.saturating_sub(1);
                }

                // 3. Until we have a single expression.

                if pratt_elements.len() != 1 {
                    unreachable!("Expected a single element: {:#?}", pratt_elements)
                }

                if let Expression { nodes } = pratt_elements.pop().unwrap() {
                    ParserResult::r#match(nodes, vec![])
                } else {
                    unreachable!("Expected an expression: {:#?}", pratt_elements)
                }
            }

            result => result,
        }
    }
}
