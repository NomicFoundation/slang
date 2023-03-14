pub enum EbnfNode {
    Alternatives {
        alternatives: Vec<EbnfNode>,
    },
    BaseProduction,
    Difference {
        minuend: Box<EbnfNode>,
        subtrahend: Box<EbnfNode>,
    },
    Not {
        body: Box<EbnfNode>,
    },
    OneOrMore {
        body: Box<EbnfNode>,
    },
    Optional {
        body: Box<EbnfNode>,
    },
    Parenthesis {
        body: Box<EbnfNode>,
    },
    Range {
        from: char,
        to: char,
    },
    Reference {
        name: String,
    },
    Repeat {
        min: usize,
        max: usize,
        body: Box<EbnfNode>,
    },
    Sequence {
        elements: Vec<EbnfNode>,
    },
    Statement {
        name: String,
        body: Box<EbnfNode>,
    },
    Terminal {
        value: String,
    },
    ZeroOrMore {
        body: Box<EbnfNode>,
    },
}

impl EbnfNode {
    pub fn alternatives(alternatives: Vec<EbnfNode>) -> Self {
        Self::Alternatives { alternatives }
    }

    pub fn difference(minuend: EbnfNode, subtrahend: EbnfNode) -> Self {
        Self::Difference {
            minuend: Box::new(minuend),
            subtrahend: Box::new(subtrahend),
        }
    }

    pub fn not(body: EbnfNode) -> Self {
        Self::Not {
            body: Box::new(body),
        }
    }

    pub fn one_or_more(body: EbnfNode) -> Self {
        Self::OneOrMore {
            body: Box::new(body),
        }
    }

    pub fn optional(body: EbnfNode) -> Self {
        Self::Optional {
            body: Box::new(body),
        }
    }

    pub fn parenthesis(body: EbnfNode) -> Self {
        Self::Parenthesis {
            body: Box::new(body),
        }
    }

    pub fn range(from: char, to: char) -> Self {
        Self::Range { from, to }
    }

    pub fn reference(name: String) -> Self {
        Self::Reference { name }
    }

    pub fn repeat(min: usize, max: usize, body: EbnfNode) -> Self {
        Self::Repeat {
            min,
            max,
            body: Box::new(body),
        }
    }

    pub fn sequence(elements: Vec<EbnfNode>) -> Self {
        Self::Sequence { elements }
    }

    pub fn statement(name: String, body: EbnfNode) -> Self {
        Self::Statement {
            name,
            body: Box::new(body),
        }
    }

    pub fn terminal(value: String) -> Self {
        Self::Terminal { value }
    }

    pub fn zero_or_more(body: EbnfNode) -> Self {
        Self::ZeroOrMore {
            body: Box::new(body),
        }
    }
}
