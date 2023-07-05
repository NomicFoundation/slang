#[derive(Clone)]
pub enum EbnfNode {
    BaseProduction,
    Choices {
        choices: Vec<EbnfNode>,
    },
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
    ProductionRef {
        name: String,
    },
    Range {
        from: char,
        to: char,
    },
    Sequence {
        elements: Vec<EbnfNode>,
    },
    SubStatement {
        name: String,
        comment: Option<String>,
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
    pub fn choices(choices: Vec<EbnfNode>) -> Self {
        Self::Choices { choices }
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

    pub fn production_ref(name: String) -> Self {
        Self::ProductionRef { name }
    }

    pub fn range(from: char, to: char) -> Self {
        Self::Range { from, to }
    }

    pub fn sequence(elements: Vec<EbnfNode>) -> Self {
        Self::Sequence { elements }
    }

    pub fn sub_statement(name: String, comment: Option<String>, body: EbnfNode) -> Self {
        Self::SubStatement {
            name,
            comment,
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
