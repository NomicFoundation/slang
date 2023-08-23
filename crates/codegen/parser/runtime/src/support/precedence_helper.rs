use super::{
    super::{cst, kinds::RuleKind},
    parser_result::ParserResult,
};

pub struct PrecedenceHelper;

impl PrecedenceHelper {
    pub fn to_precedence_result(
        operator_kind: RuleKind,
        left_binding_power: u8,
        right_binding_power: u8,
        result: ParserResult,
    ) -> ParserResult {
        match result {
            ParserResult::Match(r#match) => ParserResult::pratt_operator_match(vec![(
                left_binding_power,
                vec![cst::Node::rule(operator_kind, r#match.nodes)],
                right_binding_power,
            )]),
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
        match result {
            ParserResult::PrattOperatorMatch(pratt_operator_match) => {
                let mut nodes = pratt_operator_match.nodes;

                let mut i = 0usize;
                while nodes.len() > 1 {
                    // prefixop* expr postfixop* ( binaryop prefixop* expr postfixop* )*
                    // This requires some careful thinking
                    let slice = &nodes[i..];
                    match slice {
                        [(0, _, 0), ..] => {
                            i += 1;
                            continue;
                        }
                        [_, (0, _, 0)] => {}
                        [(_, _, 255), ..] => {}
                        [(_, _, right1), (0, _, 0), (left2, _, _), ..] => {
                            if right1 <= left2 {
                                i += 2;
                                continue;
                            }
                        }
                        [(255, _, _), ..] => {
                            i += 1;
                            continue;
                        }
                        [(_, _, right1), (left2, _, _), ..] => {
                            if right1 <= left2 {
                                i += 1;
                                continue;
                            }
                        }

                        _ => {
                            unreachable!("This is a malformed pratt parser sequence: {:#?}", nodes)
                        }
                    };

                    let wrap_children = |nodes: Vec<cst::Node>| {
                        if let Some(kind) = child_kind {
                            vec![cst::Node::rule(kind, nodes)]
                        } else {
                            nodes
                        }
                    };

                    if nodes[i].1.len() != 1 {
                        unreachable!("This is a malformed operator node ({}): {:#?}", i, nodes);
                    }

                    if nodes[i].0 == 255 {
                        // Prefix
                        let expr = nodes.remove(i + 1);
                        if let cst::Node::Rule(ref mut rule_node) = nodes[i].1[0] {
                            let mut children = rule_node.children.clone();
                            children.extend(wrap_children(expr.1));
                            nodes[i] = (0, vec![cst::Node::rule(rule_node.kind, children)], 0);
                        } else {
                            unreachable!("This is a malformed operator node: not a RuleNode");
                        }
                    } else if nodes[i].2 == 255 {
                        // Postfix
                        let expr = nodes.remove(i - 1);
                        i -= 1;
                        if let cst::Node::Rule(ref mut rule_node) = nodes[i].1[0] {
                            let mut children = rule_node.children.clone();
                            for child in wrap_children(expr.1).into_iter().rev() {
                                children.insert(0, child);
                            }
                            nodes[i] = (0, vec![cst::Node::rule(rule_node.kind, children)], 0);
                        } else {
                            unreachable!("This is a malformed operator node: not a RuleNode");
                        }
                    } else {
                        // Binary
                        let expr_right = nodes.remove(i + 1);
                        let expr_left = nodes.remove(i - 1);
                        i -= 1;
                        if let cst::Node::Rule(ref mut rule_node) = nodes[i].1[0] {
                            let mut children = rule_node.children.clone();
                            for child in wrap_children(expr_left.1).into_iter().rev() {
                                children.insert(0, child);
                            }
                            children.extend(wrap_children(expr_right.1));
                            nodes[i] = (0, vec![cst::Node::rule(rule_node.kind, children)], 0);
                        } else {
                            unreachable!("This is a malformed operator node: not a RuleNode");
                        }
                    }

                    i = i.saturating_sub(1);
                }

                ParserResult::r#match(nodes.pop().unwrap().1, vec![])
            }
            result => result,
        }
    }
}
