#[derive(Clone, Eq, Hash, PartialEq)]
pub enum EbnfNode {
    BaseProduction,
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
    SubStatement {
        name: String,
        comment: Option<String>,
        root_node: Box<EbnfNode>,
    },
    Terminal {
        value: String,
    },
    ZeroOrMore {
        node: Box<EbnfNode>,
    },
}

impl EbnfNode {
    pub fn choice(nodes: Vec<EbnfNode>) -> Self {
        Self::Choice { nodes }
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

    pub fn production_ref(name: String) -> Self {
        Self::ProductionRef { name }
    }

    pub fn range(from: char, to: char) -> Self {
        Self::Range { from, to }
    }

    pub fn sequence(nodes: Vec<EbnfNode>) -> Self {
        Self::Sequence { nodes }
    }

    pub fn sub_statement(name: String, comment: Option<String>, root_node: EbnfNode) -> Self {
        Self::SubStatement {
            name,
            comment,
            root_node: Box::new(root_node),
        }
    }

    pub fn terminal(value: String) -> Self {
        Self::Terminal { value }
    }

    pub fn zero_or_more(node: EbnfNode) -> Self {
        Self::ZeroOrMore {
            node: Box::new(node),
        }
    }
}
