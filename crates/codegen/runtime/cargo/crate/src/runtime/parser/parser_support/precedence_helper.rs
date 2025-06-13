use crate::cst::{Edge, EdgeLabel, Node, NonterminalKind};
use crate::parser::parser_support::parser_result::PrattElement::{
    self, Binary, Expression, Postfix, Prefix,
};
use crate::parser::parser_support::parser_result::{ParserResult, PrattOperatorMatch};

pub struct PrecedenceHelper;

impl PrecedenceHelper {
    pub fn to_prefix_operator(
        kind: NonterminalKind,
        right: u8,
        result: ParserResult,
    ) -> ParserResult {
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

    pub fn to_postfix_operator(
        kind: NonterminalKind,
        left: u8,
        result: ParserResult,
    ) -> ParserResult {
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
        kind: NonterminalKind,
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

    #[allow(clippy::too_many_lines, clippy::redundant_else)] // Explicit on purpose, see below.
    pub fn reduce_precedence_result(
        child_kind: NonterminalKind,
        result: ParserResult,
    ) -> ParserResult {
        // This requires some careful thinking. It could be more compact,
        // but I'm favouring obviousness here. That is also why there are
        // so many `unreachable!` - not only should they never be reached,
        // they also tell the reader what invariants should apply.

        // If the input is valid this should be correct by construction.

        let ParserResult::PrattOperatorMatch(PrattOperatorMatch { mut elements }) = result else {
            return result;
        };

        let mut i = 0usize;
        while elements.len() > 1 {
            // 1. Find the next, highest priority reducable operator
            // The elements are guaranteed to match the following:
            // `prefixop* expr postfixop* ( binaryop prefixop* expr postfixop* )*`

            match &elements[i..] {
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
                        i, elements
                    )
                }
            }

            // 2. Reduce the operator and it's child expressions to a new expression

            let make_expression = |left: Option<PrattElement>,
                                   kind: NonterminalKind,
                                   nodes: Vec<Edge>,
                                   right: Option<PrattElement>| {
                assert!(left.is_some() || right.is_some());

                let left_label = right
                    .as_ref()
                    .map_or(EdgeLabel::Operand, |_| EdgeLabel::LeftOperand);
                let right_label = left
                    .as_ref()
                    .map_or(EdgeLabel::Operand, |_| EdgeLabel::RightOperand);

                let left_nodes = match left {
                    Some(Expression { nodes }) => {
                        vec![Edge {
                            label: left_label,
                            node: Node::nonterminal(child_kind, nodes),
                        }]
                    }
                    None => vec![],
                    _ => unreachable!("Operator not preceeded by expression"),
                };

                let right_nodes = match right {
                    Some(Expression { nodes }) => {
                        vec![Edge {
                            label: right_label,
                            node: Node::nonterminal(child_kind, nodes),
                        }]
                    }
                    None => vec![],
                    _ => unreachable!("Operator not followed by expression"),
                };

                let children = [left_nodes, nodes, right_nodes].concat();

                Expression {
                    nodes: vec![Edge {
                        label: EdgeLabel::Variant,
                        node: Node::nonterminal(kind, children),
                    }],
                }
            };

            match elements.remove(i) {
                Prefix { kind, nodes, .. } => {
                    let expr = elements.remove(i);
                    elements.insert(i, make_expression(None, kind, nodes, Some(expr)));
                }

                Postfix { kind, nodes, .. } => {
                    let expr = elements.remove(i - 1);
                    i -= 1;
                    elements.insert(i, make_expression(Some(expr), kind, nodes, None));
                }

                Binary { kind, nodes, .. } => {
                    let right_expr = elements.remove(i);
                    let left_expr = elements.remove(i - 1);
                    i -= 1;
                    elements.insert(
                        i,
                        make_expression(Some(left_expr), kind, nodes, Some(right_expr)),
                    );
                }

                Expression { .. } => {
                    unreachable!("Expected an operator at index {}: {:#?}", i, elements)
                }
            }

            i = i.saturating_sub(1);
        }

        // 3. Until we have a single expression.

        match <[_; 1]>::try_from(elements) {
            Ok([Expression { nodes }]) => ParserResult::r#match(nodes, vec![]),
            Ok([head]) => unreachable!("Expected an expression: {:#?}", head),
            Err(elems) => unreachable!("Expected a single element: {:#?}", elems),
        }
    }
}
