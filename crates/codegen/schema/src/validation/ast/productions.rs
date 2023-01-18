use indexmap::IndexMap;
use semver::Version;

use crate::{ast_optional, ast_value, types, validation::ast::Node, yaml::cst};

pub type ProductionRef = std::rc::Rc<Production>;

pub struct Production {
    pub name: Node<String>,
    pub kind: Node<types::productions::ProductionKind>,
    pub versioning: Node<ProductionVersioning>,
}

impl Production {
    pub fn new(syntax: &cst::NodeRef, value: types::productions::ProductionRef) -> ProductionRef {
        return ProductionRef::new(Self {
            name: ast_value!(syntax, value, name),
            kind: ast_value!(syntax, value, kind),
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
                let syntax = &syntax.unwrap_field("unversioned").value;
                let value = Self::Unversioned(Expression::new(&syntax, value));

                return Node::new(syntax, value);
            }

            types::productions::ProductionVersioning::Versioned(value) => {
                let syntax = &syntax.unwrap_field("versioned").value;
                let value = Self::Versioned(
                    value
                        .into_iter()
                        .map(|(version, expression)| {
                            let field = syntax.unwrap_field(&version.to_string());
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
    pub ebnf: EBNF,
}

impl Expression {
    pub fn new(syntax: &cst::NodeRef, value: types::productions::ExpressionRef) -> ExpressionRef {
        return ExpressionRef::new(Self {
            config: ExpressionConfig::new(syntax, value.config.to_owned()),
            ebnf: EBNF::new(syntax, value.ebnf.to_owned()),
        });
    }
}

pub enum EBNF {
    Choice(Vec<ExpressionRef>),
    DelimitedBy(EBNFDelimitedBy),
    Difference(EBNFDifference),
    Not(ExpressionRef),
    OneOrMore(ExpressionRef),
    Optional(ExpressionRef),
    Range(EBNFRange),
    Reference(Node<String>),
    Repeat(EBNFRepeat),
    SeparatedBy(EBNFSeparatedBy),
    Sequence(Vec<ExpressionRef>),
    Terminal(Node<String>),
    ZeroOrMore(ExpressionRef),
}

pub struct EBNFDelimitedBy {
    pub open: Node<String>,
    pub expression: ExpressionRef,
    pub close: Node<String>,
}

pub struct EBNFDifference {
    pub minuend: ExpressionRef,
    pub subtrahend: ExpressionRef,
}

pub struct EBNFRange {
    pub from: Node<char>,
    pub to: Node<char>,
}

pub struct EBNFRepeat {
    pub min: Node<usize>,
    pub max: Node<usize>,
    pub expression: ExpressionRef,
}

pub struct EBNFSeparatedBy {
    pub separator: Node<String>,
    pub expression: ExpressionRef,
}

impl EBNF {
    pub fn new(syntax: &cst::NodeRef, value: types::productions::EBNF) -> Self {
        return match value {
            types::productions::EBNF::Choice(value) => {
                let syntax = &syntax.unwrap_field("choice").value;
                Self::Choice(syntax.zip_array(value, Expression::new))
            }
            types::productions::EBNF::DelimitedBy(value) => {
                let syntax = &syntax.unwrap_field("delimitedBy").value;
                Self::DelimitedBy(EBNFDelimitedBy {
                    open: ast_value!(syntax, value, open),
                    expression: ast_value!(syntax, value, expression, Expression),
                    close: ast_value!(syntax, value, close),
                })
            }
            types::productions::EBNF::Difference(value) => {
                let syntax = &syntax.unwrap_field("difference").value;
                Self::Difference(EBNFDifference {
                    minuend: ast_value!(syntax, value, minuend, Expression),
                    subtrahend: ast_value!(syntax, value, subtrahend, Expression),
                })
            }
            types::productions::EBNF::Not(value) => {
                let syntax = &syntax.unwrap_field("not").value;
                Self::Not(Expression::new(syntax, value))
            }
            types::productions::EBNF::OneOrMore(value) => {
                let syntax = &syntax.unwrap_field("oneOrMore").value;
                Self::OneOrMore(Expression::new(syntax, value))
            }
            types::productions::EBNF::Optional(value) => {
                let syntax = &syntax.unwrap_field("optional").value;
                Self::Optional(Expression::new(syntax, value))
            }
            types::productions::EBNF::Range(value) => {
                let syntax = &syntax.unwrap_field("range").value;
                Self::Range(EBNFRange {
                    from: ast_value!(syntax, value, from),
                    to: ast_value!(syntax, value, to),
                })
            }
            types::productions::EBNF::Reference(value) => {
                let syntax = &syntax.unwrap_field("reference").value;
                Self::Reference(Node::new(syntax, value))
            }
            types::productions::EBNF::Repeat(value) => {
                let syntax = &syntax.unwrap_field("repeat").value;
                Self::Repeat(EBNFRepeat {
                    min: ast_value!(syntax, value, min),
                    max: ast_value!(syntax, value, max),
                    expression: ast_value!(syntax, value, expression, Expression),
                })
            }
            types::productions::EBNF::SeparatedBy(value) => {
                let syntax = &syntax.unwrap_field("separatedBy").value;
                Self::SeparatedBy(EBNFSeparatedBy {
                    separator: ast_value!(syntax, value, separator),
                    expression: ast_value!(syntax, value, expression, Expression),
                })
            }
            types::productions::EBNF::Sequence(value) => {
                let syntax = &syntax.unwrap_field("sequence").value;
                Self::Sequence(syntax.zip_array(value, Expression::new))
            }
            types::productions::EBNF::Terminal(value) => {
                let syntax = &syntax.unwrap_field("terminal").value;
                Self::Terminal(Node::new(syntax, value))
            }
            types::productions::EBNF::ZeroOrMore(value) => {
                let syntax = &syntax.unwrap_field("zeroOrMore").value;
                Self::ZeroOrMore(Expression::new(syntax, value))
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
            name: ast_optional!(syntax, value, name),
            parser_type: ast_optional!(syntax, value, parser_type),
            lookahead: ast_optional!(syntax, value, lookahead, Expression),
            associativity: ast_optional!(syntax, value, associativity),
        };
    }
}
