pub enum EbnfNode {
    Choice {
        nodes: Vec<EbnfNode>,
    },
    Difference {
        minuend: Box<EbnfNode>,
        subtrahend: Box<EbnfNode>,
    },
    Not {
        node: Box<EbnfNode>,
    },
    OneOrMore {
        node: Box<EbnfNode>,
    },
    Optional {
        node: Box<EbnfNode>,
    },
    ProductionRef {
        name: String,
    },
    Range {
        from: char,
        to: char,
    },
    Sequence {
        nodes: Vec<EbnfNode>,
    },
    Terminal {
        terminal: String,
    },
    WithComment {
        node: Box<EbnfNode>,
        comment: String,
    },
    ZeroOrMore {
        node: Box<EbnfNode>,
    },
}

impl EbnfNode {
    pub fn choice(nodes: Vec<EbnfNode>) -> Self {
        let mut results = vec![];

        for node in nodes {
            match node {
                EbnfNode::Choice { nodes } => results.extend(nodes),
                _ => results.push(node),
            }
        }

        Self::Choice { nodes: results }
    }

    pub fn difference(minuend: EbnfNode, subtrahend: EbnfNode) -> Self {
        Self::Difference {
            minuend: Box::new(minuend),
            subtrahend: Box::new(subtrahend),
        }
    }

    pub fn not(node: EbnfNode) -> Self {
        Self::Not {
            node: Box::new(node),
        }
    }

    pub fn one_or_more(node: EbnfNode) -> Self {
        Self::OneOrMore {
            node: Box::new(node),
        }
    }

    pub fn optional(node: EbnfNode) -> Self {
        Self::Optional {
            node: Box::new(node),
        }
    }

    pub fn production_ref(name: &str) -> Self {
        Self::ProductionRef {
            name: name.to_owned(),
        }
    }

    pub fn range(from: char, to: char) -> Self {
        Self::Range { from, to }
    }

    pub fn sequence(nodes: Vec<EbnfNode>) -> Self {
        let mut results = vec![];

        for node in nodes {
            match node {
                EbnfNode::Sequence { nodes } => results.extend(nodes),
                _ => results.push(node),
            }
        }

        Self::Sequence { nodes: results }
    }

    pub fn terminal(terminal: &str) -> Self {
        Self::Terminal {
            terminal: terminal.to_owned(),
        }
    }

    pub fn with_comment(node: EbnfNode, comment: String) -> Self {
        Self::WithComment {
            node: Box::new(node),
            comment,
        }
    }

    pub fn zero_or_more(node: EbnfNode) -> Self {
        Self::ZeroOrMore {
            node: Box::new(node),
        }
    }
}

impl EbnfNode {
    pub fn precedence(&self) -> u8 {
        // We are specifying precedence "groups" instead of a flat list.
        // This separates members of the same precedence, like both "a b (c | d)" and "a | b | (c d)".
        match self {
            // Not an operator
            EbnfNode::WithComment { .. } => 0,

            // Binary
            EbnfNode::Choice { .. } | EbnfNode::Difference { .. } | EbnfNode::Sequence { .. } => 1,

            // Prefix
            EbnfNode::Not { .. } => 2,

            // Postfix
            EbnfNode::OneOrMore { .. }
            | EbnfNode::Optional { .. }
            | EbnfNode::ZeroOrMore { .. } => 3,

            // Primary
            EbnfNode::ProductionRef { .. } | EbnfNode::Range { .. } | EbnfNode::Terminal { .. } => {
                4
            }
        }
    }
}
