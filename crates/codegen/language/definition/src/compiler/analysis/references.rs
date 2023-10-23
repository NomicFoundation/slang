use crate::{
    compiler::{
        analysis::Analysis,
        versions::{VersionRange, VersionSet},
    },
    internals::Spanned,
    spanned::{
        EnumItem, EnumVariant, Field, FieldReference, FragmentItem, Item, ItemKind, KeywordItem,
        PrecedenceExpression, PrecedenceItem, PrecedenceOperator, PrimaryExpression, RepeatedItem,
        Scanner, SeparatedItem, StructItem, TokenDefinition, TokenItem, TriviaParser,
    },
    Identifier,
};
use indexmap::IndexMap;
use itertools::Itertools;
use semver::Version;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::Display;

impl Analysis {
    pub fn analyze_references(&mut self) {
        let language = self.language.clone();
        let enablement = VersionRange::starting_from(&language.versions[0]);

        self.check_reference(
            None,
            &language.root_item,
            &enablement,
            ReferenceFilter::NonTerminals,
        );

        self.check_trivium(&language.leading_trivia, &enablement);
        self.check_trivium(&language.trailing_trivia, &enablement);

        for item in language.items() {
            self.check_item(item, &enablement);
        }
    }

    fn check_item(&mut self, item: &Item, enablement: &VersionRange) {
        match item {
            Item::Struct { item } => {
                self.check_struct(item, &enablement);
            }
            Item::Enum { item } => {
                self.check_enum(item, &enablement);
            }
            Item::Repeated { item } => {
                self.check_repeated(item, &enablement);
            }
            Item::Separated { item } => {
                self.check_separated(item, &enablement);
            }
            Item::Precedence { item } => {
                self.check_precedence(item, &enablement);
            }
            Item::Keyword { item } => {
                self.check_keyword(item, &enablement);
            }
            Item::Token { item } => {
                self.check_token(item, &enablement);
            }
            Item::Fragment { item } => {
                self.check_fragment(item, &enablement);
            }
        }
    }

    fn check_struct(&mut self, item: &StructItem, enablement: &VersionRange) {
        let StructItem {
            name,
            enabled_in,
            disabled_in,
            fields,
        } = item;

        let enablement = self.update_enablement(enablement, &enabled_in, &disabled_in);

        self.check_fields(Some(name), fields, &enablement);
    }

    fn check_enum(&mut self, item: &EnumItem, enablement: &VersionRange) {
        let EnumItem {
            name,
            enabled_in,
            disabled_in,
            variants,
        } = item;

        let enablement = self.update_enablement(enablement, &enabled_in, &disabled_in);

        for variant in variants {
            let EnumVariant {
                name: _,
                enabled_in,
                disabled_in,
                fields,
            } = &**variant;

            let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

            self.check_fields(Some(name), &fields, &enablement);
        }
    }

    fn check_repeated(&mut self, item: &RepeatedItem, enablement: &VersionRange) {
        let RepeatedItem {
            name,
            repeated,
            allow_empty: _,
            enabled_in,
            disabled_in,
        } = item;

        let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

        self.check_reference(Some(name), repeated, &enablement, ReferenceFilter::Nodes);
    }

    fn check_separated(&mut self, item: &SeparatedItem, enablement: &VersionRange) {
        let SeparatedItem {
            name,
            separated,
            separator,
            allow_empty: _,
            enabled_in,
            disabled_in,
        } = item;

        let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

        self.check_reference(Some(name), separated, &enablement, ReferenceFilter::Nodes);
        self.check_reference(Some(name), separator, &enablement, ReferenceFilter::Tokens);
    }

    fn check_precedence(&mut self, item: &PrecedenceItem, enablement: &VersionRange) {
        let PrecedenceItem {
            name,
            enabled_in,
            disabled_in,
            precedence_expressions,
            primary_expressions,
        } = item;

        let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

        for precedence_expression in precedence_expressions {
            let PrecedenceExpression { name: _, operators } = &**precedence_expression;

            for operator in operators {
                let PrecedenceOperator {
                    model: _,
                    enabled_in,
                    disabled_in,
                    fields,
                } = &**operator;

                let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

                self.check_fields(Some(name), &fields, &enablement);
            }
        }

        for primary_expression in primary_expressions {
            let PrimaryExpression {
                expression,
                enabled_in,
                disabled_in,
            } = &**primary_expression;

            let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

            self.check_reference(Some(name), &expression, &enablement, ReferenceFilter::Nodes);
        }
    }

    fn check_fields(
        &mut self,
        source: Option<&Identifier>,
        fields: &IndexMap<Spanned<Identifier>, Spanned<Field>>,
        enablement: &VersionRange,
    ) {
        for field in fields.values() {
            match &**field {
                Field::Required { reference } => {
                    self.check_field_reference(source, &reference, &enablement);
                }
                Field::Optional {
                    reference,
                    enabled_in,
                    disabled_in,
                } => {
                    let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

                    self.check_field_reference(source, &reference, &enablement);
                }
            };
        }
    }

    fn check_field_reference(
        &mut self,
        source: Option<&Identifier>,
        reference: &FieldReference,
        enablement: &VersionRange,
    ) {
        match reference {
            FieldReference::NonTerminal { item } => {
                self.check_reference(source, item, &enablement, ReferenceFilter::NonTerminals);
            }
            FieldReference::Terminal { items } => {
                for item in items {
                    self.check_reference(source, item, &enablement, ReferenceFilter::Terminals);
                }
            }
        };
    }

    fn check_trivium(&mut self, parser: &TriviaParser, enablement: &VersionRange) {
        match parser {
            TriviaParser::Sequence { parsers } | TriviaParser::Choice { parsers } => {
                for parser in parsers {
                    self.check_trivium(parser, &enablement);
                }
            }
            TriviaParser::ZeroOrMore { parser } | TriviaParser::Optional { parser } => {
                self.check_trivium(parser, &enablement);
            }
            TriviaParser::Token { token } => {
                self.check_reference(None, token, &enablement, ReferenceFilter::Tokens);
            }
        };
    }

    fn check_keyword(&mut self, item: &KeywordItem, enablement: &VersionRange) {
        let KeywordItem {
            name,
            identifier,
            enabled_in,
            disabled_in,
            reserved_in,
            unreserved_in,
            value: _,
        } = item;

        let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

        self.check_reference(Some(name), identifier, &enablement, ReferenceFilter::Tokens);

        self.check_version_pair(reserved_in, unreserved_in);
    }

    fn check_token(&mut self, item: &TokenItem, enablement: &VersionRange) {
        let TokenItem {
            name,
            is_terminator: _,
            is_open_delimiter_for,
            is_close_delimiter_for,
            definitions,
        } = item;

        if let Some(is_open_delimiter_for) = is_open_delimiter_for {
            self.check_reference(
                Some(name),
                is_open_delimiter_for,
                &enablement,
                ReferenceFilter::Tokens,
            );
        }

        if let Some(is_close_delimiter_for) = is_close_delimiter_for {
            self.check_reference(
                Some(name),
                is_close_delimiter_for,
                &enablement,
                ReferenceFilter::Tokens,
            );
        }

        for definition in definitions {
            let TokenDefinition {
                enabled_in,
                disabled_in,
                scanner,
            } = &**definition;

            let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

            self.check_scanner(Some(name), &scanner, &enablement);
        }
    }

    fn check_fragment(&mut self, item: &FragmentItem, enablement: &VersionRange) {
        let FragmentItem {
            name,
            enabled_in,
            disabled_in,
            scanner,
        } = item;

        let enablement = self.update_enablement(&enablement, &enabled_in, &disabled_in);

        self.check_scanner(Some(name), scanner, &enablement);
    }

    fn check_scanner(
        &mut self,
        source: Option<&Identifier>,
        scanner: &Scanner,
        enablement: &VersionRange,
    ) {
        match scanner {
            Scanner::Sequence { scanners } | Scanner::Choice { scanners } => {
                for scanner in scanners {
                    self.check_scanner(source, scanner, &enablement);
                }
            }
            Scanner::Optional { scanner }
            | Scanner::ZeroOrMore { scanner }
            | Scanner::OneOrMore { scanner } => {
                self.check_scanner(source, scanner, &enablement);
            }
            Scanner::Not { chars: _ }
            | Scanner::Range {
                inclusive_start: _,
                inclusive_end: _,
            }
            | Scanner::Atom { atom: _ } => {
                // Nothing to check for now.
            }
            Scanner::TrailingContext {
                scanner,
                not_followed_by,
            } => {
                self.check_scanner(source, scanner, &enablement);
                self.check_scanner(source, not_followed_by, &enablement);
            }
            Scanner::Fragment { reference } => {
                self.check_reference(source, reference, enablement, ReferenceFilter::Fragments);
            }
        };
    }

    fn update_enablement(
        &mut self,
        existing: &VersionRange,
        enabled_in: &Option<Spanned<Version>>,
        disabled_in: &Option<Spanned<Version>>,
    ) -> VersionRange {
        self.check_version_pair(enabled_in, disabled_in);

        let enabled_in = match enabled_in {
            None => &existing.inclusive_start,
            Some(enabled_in) => {
                if **enabled_in <= existing.inclusive_start {
                    self.errors.add(
                        enabled_in,
                        &Errors::EnabledTooEarly(&existing.inclusive_start),
                    );
                }
                &**enabled_in
            }
        };

        let disabled_in = match disabled_in {
            None => &existing.exclusive_end,
            Some(disabled_in) => {
                if **disabled_in >= existing.exclusive_end {
                    self.errors.add(
                        disabled_in,
                        &Errors::DisabledTooLate(&existing.exclusive_end),
                    );
                }
                &**disabled_in
            }
        };

        return VersionRange::between(enabled_in, disabled_in);
    }

    fn check_version_pair(
        &mut self,
        first: &Option<Spanned<Version>>,
        second: &Option<Spanned<Version>>,
    ) {
        for version in [first, second].into_iter().flatten() {
            if !self.language.versions.contains(version) {
                self.errors.add(version, &Errors::VersionNotFound(version));
            }
        }

        if let (Some(first), Some(second)) = (first, second) {
            if first >= second {
                self.errors
                    .add(first, &Errors::UnorderedVersionPair(first, second));
            }
        }
    }

    fn check_reference(
        &mut self,
        source: Option<&Identifier>,
        reference: &Spanned<Identifier>,
        enablement: &VersionRange,
        filter: ReferenceFilter,
    ) {
        let target = match self.metadata.get_mut(&**reference) {
            Some(target) => target,
            None => {
                self.errors
                    .add(reference, &Errors::UnknownReference(reference));
                return;
            }
        };

        let defined_in = VersionSet::single(enablement.to_owned());
        let not_defined_in = defined_in.difference(&target.defined_in);

        if !not_defined_in.is_empty() {
            self.errors.add(
                reference,
                &Errors::InvalidReferenceVersion(reference, &not_defined_in),
            );
        }

        if !filter.apply(&target.kind) {
            let expected = &ItemKind::iter()
                .filter(|kind| filter.apply(kind))
                .collect_vec()[..];

            self.errors.add(
                reference,
                &Errors::InvalidReferenceFilter(reference, &target.kind, expected),
            );
        }

        target.used_in.add(enablement);

        target.referenced_from.push(reference.span());

        if let Some(source) = source {
            self.metadata[source]
                .referenced_items
                .push((**reference).to_owned());
        }
    }
}

#[derive(Debug, Display, PartialEq, Eq)]
enum ReferenceFilter {
    Nodes,

    NonTerminals,
    Terminals,

    Tokens,
    Fragments,
}

impl ReferenceFilter {
    fn apply(&self, target_kind: &ItemKind) -> bool {
        // A bit explicit here on purpose, to make sure we review
        // this list whenever filters or items are changed:

        return match target_kind {
            ItemKind::Struct
            | ItemKind::Enum
            | ItemKind::Repeated
            | ItemKind::Separated
            | ItemKind::Precedence => match self {
                Self::Nodes | Self::NonTerminals => true,
                Self::Terminals | Self::Tokens | Self::Fragments => false,
            },
            ItemKind::Keyword | ItemKind::Token => match self {
                Self::Nodes | Self::Terminals | Self::Tokens => true,
                Self::NonTerminals | Self::Fragments => false,
            },
            ItemKind::Fragment => match self {
                Self::Fragments => true,
                Self::Nodes | Self::NonTerminals | Self::Terminals | Self::Tokens => false,
            },
        };
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Version '{0}' does not exist in the language definition.")]
    VersionNotFound(&'err Version),
    #[error("Version '{0}' must be less than corresponding version '{1}'.")]
    UnorderedVersionPair(&'err Version, &'err Version),
    #[error("Parent scope is enabled in '{0}'.")]
    EnabledTooEarly(&'err Version),
    #[error("Parent scope is disabled in '{0}'.")]
    DisabledTooLate(&'err Version),
    #[error("Reference to unknown item '{0}'.")]
    UnknownReference(&'err Identifier),
    #[error("Reference '{0}' is not defined in versions '{1}'.")]
    InvalidReferenceVersion(&'err Identifier, &'err VersionSet),
    #[error("Reference '{0}' of kind '{1}' is not valid. Expected: {2:?}.")]
    InvalidReferenceFilter(&'err Identifier, &'err ItemKind, &'err [ItemKind]),
}
