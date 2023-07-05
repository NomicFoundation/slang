use std::rc::Rc;

use crate::{
    types::{
        LanguageDefinitionRef, ParserDefinition, ParserRef, PrecedenceParserRef,
        ProductionDefinition, ProductionRef, ScannerDefinition, ScannerRef, VersionMap,
    },
    validation::visitors::{
        location::LocationRef, reporter::Reporter, visitor::Visitor, VersionSet,
    },
};

pub trait Receiver {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        language: &LanguageDefinitionRef,
        location: LocationRef,
        reporter: &mut Reporter,
    );
}

impl Receiver for ProductionRef {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        language: &LanguageDefinitionRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        if !visitor.visit_production(self, &location, reporter) {
            return;
        }

        self.definition
            .receive(visitor, language, location, reporter);
    }
}

impl Receiver for ProductionDefinition {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        language: &LanguageDefinitionRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        match self {
            Self::Scanner { version_map, .. } => {
                version_map.receive(visitor, language, location, reporter);
            }
            Self::TriviaParser { version_map, .. } => {
                version_map.receive(visitor, language, location, reporter);
            }
            Self::Parser { version_map, .. } => {
                version_map.receive(visitor, language, location, reporter);
            }
            Self::PrecedenceParser { version_map, .. } => {
                version_map.receive(visitor, language, location, reporter);
            }
        };
    }
}

impl<T> Receiver for VersionMap<T>
where
    Rc<T>: Receiver,
{
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        language: &LanguageDefinitionRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        let max_version = VersionSet::max_version();

        match self {
            VersionMap::Unversioned(instance) => {
                let location = location.field("unversioned");

                let first_version = language.versions[0].to_owned();
                let version_set = VersionSet::range(first_version..max_version);

                if visitor.visit_version(&version_set, &location, reporter) {
                    instance.receive(visitor, language, location, reporter);
                }
            }
            VersionMap::Versioned(versioned) => {
                let location = location.field("versioned");

                let mut versions = versioned.keys().collect::<Vec<_>>();
                versions.push(&max_version);

                let versioned = versions
                    .windows(2)
                    .map(|window| window[0].to_owned()..window[1].to_owned())
                    .zip(versioned.values());

                for (range, instance) in versioned {
                    if let Some(value) = instance {
                        let location = location.field(range.start.to_string());

                        let version_set = VersionSet::range(range);

                        if visitor.visit_version(&version_set, &location, reporter) {
                            value.receive(visitor, language, location, reporter);
                        }
                    }
                }
            }
        };
    }
}

impl Receiver for ScannerRef {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        language: &LanguageDefinitionRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        if !visitor.visit_scanner(self, &location, reporter) {
            return;
        }

        self.definition
            .receive(visitor, language, location, reporter);
    }
}

impl Receiver for ScannerDefinition {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        language: &LanguageDefinitionRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        match self {
            ScannerDefinition::Choice(expressions) => {
                let location = location.field("choice");
                for (i, expression) in expressions.iter().enumerate() {
                    expression.receive(visitor, language, location.index(i), reporter);
                }
            }
            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                let location = location.field("difference");
                minuend.receive(visitor, language, location.field("minuend"), reporter);
                subtrahend.receive(visitor, language, location.field("subtrahend"), reporter);
            }
            ScannerDefinition::Not(expression) => {
                expression.receive(visitor, language, location.field("not"), reporter);
            }
            ScannerDefinition::OneOrMore(expression) => {
                expression.receive(visitor, language, location.field("oneOrMore"), reporter);
            }
            ScannerDefinition::Optional(expression) => {
                expression.receive(visitor, language, location.field("optional"), reporter);
            }
            ScannerDefinition::Range { .. } => {}
            ScannerDefinition::Reference(_) => {}
            ScannerDefinition::Sequence(expressions) => {
                let location = location.field("sequence");
                for (i, expression) in expressions.iter().enumerate() {
                    expression.receive(visitor, language, location.index(i), reporter);
                }
            }
            ScannerDefinition::Terminal(_) => {}
            ScannerDefinition::TrailingContext {
                expression,
                not_followed_by,
            } => {
                let location = location.field("trailingContext");
                expression.receive(visitor, language, location.field("expression"), reporter);
                not_followed_by.receive(
                    visitor,
                    language,
                    location.field("notFollowedBy"),
                    reporter,
                );
            }
            ScannerDefinition::ZeroOrMore(expression) => {
                expression.receive(visitor, language, location.field("zeroOrMore"), reporter);
            }
        };
    }
}

impl Receiver for ParserRef {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        language: &LanguageDefinitionRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        if !visitor.visit_parser(self, &location, reporter) {
            return;
        }

        self.definition
            .receive(visitor, language, location, reporter);
    }
}

impl Receiver for ParserDefinition {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        language: &LanguageDefinitionRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        match self {
            ParserDefinition::Choice(expressions) => {
                let location = location.field("choice");
                for (i, expression) in expressions.iter().enumerate() {
                    expression.receive(visitor, language, location.index(i), reporter);
                }
            }
            ParserDefinition::DelimitedBy { expression, .. } => {
                let location = location.field("delimitedBy");
                expression.receive(visitor, language, location.field("expression"), reporter);
            }
            ParserDefinition::OneOrMore(expression) => {
                expression.receive(visitor, language, location.field("oneOrMore"), reporter);
            }
            ParserDefinition::Optional(expression) => {
                expression.receive(visitor, language, location.field("optional"), reporter);
            }
            ParserDefinition::Reference(_) => {}
            ParserDefinition::SeparatedBy { expression, .. } => {
                let location = location.field("separatedBy");
                expression.receive(visitor, language, location.field("expression"), reporter);
            }
            ParserDefinition::Sequence(expressions) => {
                let location = location.field("sequence");
                for (i, expression) in expressions.iter().enumerate() {
                    expression.receive(visitor, language, location.index(i), reporter);
                }
            }
            ParserDefinition::TerminatedBy { expression, .. } => {
                let location = location.field("terminatedBy");
                expression.receive(visitor, language, location.field("expression"), reporter);
            }
            ParserDefinition::ZeroOrMore(expression) => {
                expression.receive(visitor, language, location.field("zeroOrMore"), reporter);
            }
        };
    }
}

impl Receiver for PrecedenceParserRef {
    fn receive(
        &self,
        visitor: &mut impl Visitor,
        language: &LanguageDefinitionRef,
        location: LocationRef,
        reporter: &mut Reporter,
    ) {
        if !visitor.visit_precedence_parser(self, &location, reporter) {
            return;
        }

        {
            let location = location.field("definitions");
            for (i, definition) in self.operators.iter().enumerate() {
                let location = location.index(i).field("operator");
                definition
                    .operator
                    .receive(visitor, language, location, reporter);
            }
        }

        {
            let location = location.field("primaryExpression");
            self.primary_expression
                .receive(visitor, language, location, reporter);
        }
    }
}
