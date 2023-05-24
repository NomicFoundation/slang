use crate::{
    types::{
        ParserDefinition, ParserRef, PrecedenceParserRef, Production, ProductionRef,
        ScannerDefinition, ScannerRef, SchemaRef, VersionMap,
    },
    validation::visitors::{location::LocationRef, reporter::Reporter, visitor::Visitor},
};

pub trait Receiver {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        schema: &SchemaRef,
        location: LocationRef,
        reporter: &mut Reporter,
    );
}

impl Receiver for ProductionRef {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        schema: &SchemaRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        if !visitor.visit_production(self, &location, reporter) {
            return;
        }

        match self.as_ref() {
            Production::Scanner { version_map, .. } => match version_map {
                VersionMap::Unversioned(scanner) => {
                    let location = location.field("unversioned");
                    scanner.receive(visitor, schema, location, reporter);
                }
                VersionMap::Versioned(versions) => {
                    let location = location.field("versioned");
                    for (version, scanner) in versions {
                        if let Some(scanner) = scanner {
                            let location = location.field(version.to_string());
                            scanner.receive(visitor, schema, location, reporter);
                        }
                    }
                }
            },

            Production::TriviaParser { version_map, .. }
            | Production::Parser { version_map, .. } => match version_map {
                VersionMap::Unversioned(parser) => {
                    let location = location.field("unversioned");
                    parser.receive(visitor, schema, location, reporter);
                }
                VersionMap::Versioned(versions) => {
                    let location = location.field("versioned");
                    for (version, parser) in versions {
                        if let Some(parser) = parser {
                            let location = location.field(version.to_string());
                            parser.receive(visitor, schema, location, reporter);
                        }
                    }
                }
            },

            Production::PrecedenceParser { version_map, .. } => match version_map {
                VersionMap::Unversioned(parser) => {
                    let location = location.field("unversioned");
                    parser.receive(visitor, schema, location, reporter);
                }
                VersionMap::Versioned(versions) => {
                    let location = location.field("versioned");
                    for (version, parser) in versions {
                        if let Some(parser) = parser {
                            let location = location.field(version.to_string());
                            parser.receive(visitor, schema, location, reporter);
                        }
                    }
                }
            },
        };
    }
}

impl Receiver for ScannerRef {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        schema: &SchemaRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        if !visitor.visit_scanner(self, &location, reporter) {
            return;
        }

        self.definition.receive(visitor, schema, location, reporter);
    }
}

impl Receiver for ScannerDefinition {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        schema: &SchemaRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        match self {
            ScannerDefinition::Choice(expressions) => {
                let location = location.field("choice");
                for (i, expression) in expressions.iter().enumerate() {
                    expression.receive(visitor, schema, location.index(i), reporter);
                }
            }
            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                let location = location.field("difference");
                minuend.receive(visitor, schema, location.field("minuend"), reporter);
                subtrahend.receive(visitor, schema, location.field("subtrahend"), reporter);
            }
            ScannerDefinition::Not(expression) => {
                expression.receive(visitor, schema, location.field("not"), reporter);
            }
            ScannerDefinition::OneOrMore(expression) => {
                expression.receive(visitor, schema, location.field("oneOrMore"), reporter);
            }
            ScannerDefinition::Optional(expression) => {
                expression.receive(visitor, schema, location.field("optional"), reporter);
            }
            ScannerDefinition::Range { .. } => {}
            ScannerDefinition::Reference(_) => {}
            ScannerDefinition::Repeat { expression, .. } => {
                expression.receive(visitor, schema, location.field("repeat"), reporter);
            }
            ScannerDefinition::Sequence(expressions) => {
                let location = location.field("sequence");
                for (i, expression) in expressions.iter().enumerate() {
                    expression.receive(visitor, schema, location.index(i), reporter);
                }
            }
            ScannerDefinition::Terminal(_) => {}
            ScannerDefinition::TrailingContext {
                expression,
                not_followed_by,
            } => {
                let location = location.field("trailingContext");
                expression.receive(visitor, schema, location.field("expression"), reporter);
                not_followed_by.receive(visitor, schema, location.field("notFollowedBy"), reporter);
            }
            ScannerDefinition::ZeroOrMore(expression) => {
                expression.receive(visitor, schema, location.field("zeroOrMore"), reporter);
            }
        };
    }
}

impl Receiver for ParserRef {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        schema: &SchemaRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        if !visitor.visit_parser(self, &location, reporter) {
            return;
        }

        self.definition.receive(visitor, schema, location, reporter);
    }
}

impl Receiver for ParserDefinition {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        schema: &SchemaRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        match self {
            ParserDefinition::Choice(expressions) => {
                let location = location.field("choice");
                for (i, expression) in expressions.iter().enumerate() {
                    expression.receive(visitor, schema, location.index(i), reporter);
                }
            }
            ParserDefinition::DelimitedBy { expression, .. } => {
                let location = location.field("delimitedBy");
                expression.receive(visitor, schema, location.field("expression"), reporter);
            }
            ParserDefinition::OneOrMore(expression) => {
                expression.receive(visitor, schema, location.field("oneOrMore"), reporter);
            }
            ParserDefinition::Optional(expression) => {
                expression.receive(visitor, schema, location.field("optional"), reporter);
            }
            ParserDefinition::Reference(_) => {}
            ParserDefinition::Repeat { expression, .. } => {
                let location = location.field("repeat");
                expression.receive(visitor, schema, location.field("expression"), reporter);
            }
            ParserDefinition::SeparatedBy { expression, .. } => {
                let location = location.field("separatedBy");
                expression.receive(visitor, schema, location.field("expression"), reporter);
            }
            ParserDefinition::Sequence(expressions) => {
                let location = location.field("sequence");
                for (i, expression) in expressions.iter().enumerate() {
                    expression.receive(visitor, schema, location.index(i), reporter);
                }
            }
            ParserDefinition::TerminatedBy { expression, .. } => {
                let location = location.field("terminatedBy");
                expression.receive(visitor, schema, location.field("expression"), reporter);
            }
            ParserDefinition::ZeroOrMore(expression) => {
                expression.receive(visitor, schema, location.field("zeroOrMore"), reporter);
            }
        };
    }
}

impl Receiver for PrecedenceParserRef {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        schema: &SchemaRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        if !visitor.visit_precedence_parser(self, &location, reporter) {
            return;
        }

        {
            let location = location.field("definitions");
            for (i, definition) in self.definition.definitions.iter().enumerate() {
                let location = location.index(i).field("operator");
                definition
                    .operator
                    .receive(visitor, schema, location, reporter);
            }
        }

        {
            let location = location.field("primaryExpression");
            self.definition
                .primary_expression
                .receive(visitor, schema, location, reporter);
        }
    }
}
