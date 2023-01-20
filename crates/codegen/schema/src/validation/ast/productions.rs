use indexmap::IndexMap;
use semver::Version;

use crate::{types, validation::ast::Node, yaml::cst};

pub type ProductionRef = std::rc::Rc<Production>;

pub struct Production {
    pub name: Node<String>,
    pub kind: Node<types::productions::ProductionKind>,
    pub versioning: Node<ProductionVersioning>,
}

impl Production {
    pub fn new(syntax: &cst::NodeRef, value: types::productions::ProductionRef) -> ProductionRef {
        return ProductionRef::new(Self {
            name: Node::new(syntax.get("name"), value.name.to_owned()),
            kind: Node::new(syntax.get("kind"), value.kind),
            versioning: ProductionVersioning::new(syntax, value.versioning.to_owned()),
        });
    }
}

pub enum ProductionVersioning {
    Unversioned(ExpressionRef),
    Versioned(IndexMap<Node<Version>, ExpressionRef>),
}

impl ProductionVersioning {
    pub fn new(
        syntax: &cst::NodeRef,
        value: types::productions::ProductionVersioning,
    ) -> Node<Self> {
        match value {
            types::productions::ProductionVersioning::Unversioned(value) => {
                let syntax = &syntax.get("unversioned");
                let value = Self::Unversioned(Expression::new(&syntax, value));

                return Node::new(syntax, value);
            }

            types::productions::ProductionVersioning::Versioned(value) => {
                let syntax = &syntax.get("versioned");
                let value = Self::Versioned(
                    value
                        .into_iter()
                        .map(|(version, expression)| {
                            let field = syntax.field(&version.to_string());
                            (
                                Node::new(&field.key, version),
                                Expression::new(&field.value, expression),
                            )
                        })
                        .collect(),
                );

                return Node::new(syntax, value);
            }
        };
    }
}

pub type ExpressionRef = std::rc::Rc<Expression>;

pub struct Expression {
    pub config: ExpressionConfig,
    pub parser: Node<ExpressionParser>,
}

impl Expression {
    pub fn new(syntax: &cst::NodeRef, value: types::productions::ExpressionRef) -> ExpressionRef {
        return ExpressionRef::new(Self {
            config: ExpressionConfig::new(syntax, value.config.to_owned()),
            parser: ExpressionParser::new(syntax, value.parser.to_owned()),
        });
    }
}

pub enum ExpressionParser {
    Choice(Vec<ExpressionRef>),
    DelimitedBy {
        open: Node<String>,
        expression: ExpressionRef,
        close: Node<String>,
    },
    Difference {
        minuend: ExpressionRef,
        subtrahend: ExpressionRef,
    },
    Not(ExpressionRef),
    OneOrMore(ExpressionRef),
    Optional(ExpressionRef),
    Range {
        from: Node<char>,
        to: Node<char>,
    },
    Reference(Node<String>),
    Repeat {
        min: Node<usize>,
        max: Node<usize>,
        expression: ExpressionRef,
    },
    SeparatedBy {
        separator: Node<String>,
        expression: ExpressionRef,
    },
    Sequence(Vec<ExpressionRef>),
    Terminal(Node<String>),
    ZeroOrMore(ExpressionRef),
}

impl ExpressionParser {
    pub fn new(syntax: &cst::NodeRef, value: types::productions::ExpressionParser) -> Node<Self> {
        match value {
            types::productions::ExpressionParser::Choice(value) => {
                let syntax = syntax.field("choice");
                return Node::new(
                    &syntax.key,
                    Self::Choice(syntax.value.zip(value, Expression::new)),
                );
            }
            types::productions::ExpressionParser::DelimitedBy {
                open,
                expression,
                close,
            } => {
                let syntax = syntax.field("delimitedBy");
                return Node::new(
                    &syntax.key,
                    Self::DelimitedBy {
                        open: Node::new(&syntax.value.get("open"), open),
                        expression: Expression::new(&syntax.value.get("expression"), expression),
                        close: Node::new(&syntax.value.get("close"), close),
                    },
                );
            }
            types::productions::ExpressionParser::Difference {
                minuend,
                subtrahend,
            } => {
                let syntax = syntax.field("difference");
                return Node::new(
                    &syntax.key,
                    Self::Difference {
                        minuend: Expression::new(&syntax.value.get("minuend"), minuend),
                        subtrahend: Expression::new(&syntax.value.get("subtrahend"), subtrahend),
                    },
                );
            }
            types::productions::ExpressionParser::Not(value) => {
                let syntax = syntax.field("not");
                return Node::new(
                    &syntax.key,
                    Self::Not(Expression::new(&syntax.value, value)),
                );
            }
            types::productions::ExpressionParser::OneOrMore(value) => {
                let syntax = syntax.field("oneOrMore");
                return Node::new(
                    &syntax.key,
                    Self::OneOrMore(Expression::new(&syntax.value, value)),
                );
            }
            types::productions::ExpressionParser::Optional(value) => {
                let syntax = syntax.field("optional");
                return Node::new(
                    &syntax.key,
                    Self::Optional(Expression::new(&syntax.value, value)),
                );
            }
            types::productions::ExpressionParser::Range { from, to } => {
                let syntax = syntax.field("range");
                return Node::new(
                    &syntax.key,
                    Self::Range {
                        from: Node::new(&syntax.value.get("from"), from),
                        to: Node::new(&syntax.value.get("to"), to),
                    },
                );
            }
            types::productions::ExpressionParser::Reference(value) => {
                let syntax = syntax.field("reference");
                return Node::new(
                    &syntax.key,
                    Self::Reference(Node::new(&syntax.value, value)),
                );
            }
            types::productions::ExpressionParser::Repeat {
                min,
                max,
                expression,
            } => {
                let syntax = syntax.field("repeat");
                return Node::new(
                    &syntax.key,
                    Self::Repeat {
                        min: Node::new(&syntax.value.get("min"), min),
                        max: Node::new(&syntax.value.get("max"), max),
                        expression: Expression::new(&syntax.value.get("expression"), expression),
                    },
                );
            }
            types::productions::ExpressionParser::SeparatedBy {
                separator,
                expression,
            } => {
                let syntax = syntax.field("separatedBy");
                return Node::new(
                    &syntax.key,
                    Self::SeparatedBy {
                        separator: Node::new(&syntax.value.get("separator"), separator),
                        expression: Expression::new(&syntax.value.get("expression"), expression),
                    },
                );
            }
            types::productions::ExpressionParser::Sequence(value) => {
                let syntax = syntax.field("sequence");
                return Node::new(
                    &syntax.key,
                    Self::Sequence(syntax.value.zip(value, Expression::new)),
                );
            }
            types::productions::ExpressionParser::Terminal(value) => {
                let syntax = syntax.field("terminal");
                return Node::new(&syntax.key, Self::Terminal(Node::new(&syntax.value, value)));
            }
            types::productions::ExpressionParser::ZeroOrMore(value) => {
                let syntax = syntax.field("zeroOrMore");
                return Node::new(
                    &syntax.key,
                    Self::ZeroOrMore(Expression::new(&syntax.value, value)),
                );
            }
        };
    }
}

pub struct ExpressionConfig {
    pub name: Option<Node<String>>,
    pub parser_type: Option<Node<types::productions::ParserType>>,
    pub lookahead: Option<ExpressionRef>,
    pub associativity: Option<Node<types::productions::ExpressionAssociativity>>,
}

impl ExpressionConfig {
    pub fn new(syntax: &cst::NodeRef, value: types::productions::ExpressionConfig) -> Self {
        return Self {
            name: value.name.and_then(|name| {
                return Some(Node::new(syntax.get("name"), name));
            }),
            parser_type: value.parser_type.and_then(|parser_type| {
                return Some(Node::new(syntax.get("parserType"), parser_type));
            }),
            lookahead: value.lookahead.and_then(|lookahead| {
                return Some(Expression::new(syntax.get("lookahead"), lookahead));
            }),
            associativity: value.associativity.and_then(|associativity| {
                return Some(Node::new(syntax.get("associativity"), associativity));
            }),
        };
    }
}
