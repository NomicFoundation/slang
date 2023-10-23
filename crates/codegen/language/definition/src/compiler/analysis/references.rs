use crate::{
    compiler::{
        analysis::Analysis,
        versions::{VersionRange, VersionSet},
    },
    internals::Spanned,
    spanned::{
        EnumItem, EnumVariant, Field, FieldKind, FragmentItem, Item, ItemKind, KeywordItem,
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

pub fn analyze_references(analysis: &mut Analysis) {
    let language = analysis.language.clone();
    let enablement = VersionRange::starting_from(&language.versions[0]);

    check_reference(
        analysis,
        None,
        &language.root_item,
        &enablement,
        ReferenceFilter::NonTerminals,
    );

    check_trivium(analysis, &language.leading_trivia, &enablement);
    check_trivium(analysis, &language.trailing_trivia, &enablement);

    for item in language.items() {
        check_item(analysis, item, &enablement);
    }
}

fn check_item(analysis: &mut Analysis, item: &Item, enablement: &VersionRange) {
    match item {
        Item::Struct { item } => {
            check_struct(analysis, item, &enablement);
        }
        Item::Enum { item } => {
            check_enum(analysis, item, &enablement);
        }
        Item::Repeated { item } => {
            check_repeated(analysis, item, &enablement);
        }
        Item::Separated { item } => {
            check_separated(analysis, item, &enablement);
        }
        Item::Precedence { item } => {
            check_precedence(analysis, item, &enablement);
        }
        Item::Keyword { item } => {
            check_keyword(analysis, item, &enablement);
        }
        Item::Token { item } => {
            check_token(analysis, item, &enablement);
        }
        Item::Fragment { item } => {
            check_fragment(analysis, item, &enablement);
        }
    }
}

fn check_struct(analysis: &mut Analysis, item: &StructItem, enablement: &VersionRange) {
    let StructItem {
        name,
        enabled_in,
        disabled_in,
        error_recovery: _,
        fields,
    } = item;

    let enablement = update_enablement(analysis, enablement, &enabled_in, &disabled_in);

    check_fields(analysis, Some(name), fields, &enablement);
}

fn check_enum(analysis: &mut Analysis, item: &EnumItem, enablement: &VersionRange) {
    let EnumItem {
        name,
        enabled_in,
        disabled_in,
        variants,
    } = item;

    let enablement = update_enablement(analysis, enablement, &enabled_in, &disabled_in);

    for variant in variants {
        let EnumVariant {
            name: _,
            enabled_in,
            disabled_in,
            error_recovery: _,
            fields,
        } = &**variant;

        let enablement = update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

        check_fields(analysis, Some(name), &fields, &enablement);
    }
}

fn check_repeated(analysis: &mut Analysis, item: &RepeatedItem, enablement: &VersionRange) {
    let RepeatedItem {
        name,
        repeated,
        allow_empty: _,
        enabled_in,
        disabled_in,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

    check_reference(
        analysis,
        Some(name),
        repeated,
        &enablement,
        ReferenceFilter::Nodes,
    );
}

fn check_separated(analysis: &mut Analysis, item: &SeparatedItem, enablement: &VersionRange) {
    let SeparatedItem {
        name,
        separated,
        separator,
        allow_empty: _,
        enabled_in,
        disabled_in,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

    check_reference(
        analysis,
        Some(name),
        separated,
        &enablement,
        ReferenceFilter::Nodes,
    );
    check_reference(
        analysis,
        Some(name),
        separator,
        &enablement,
        ReferenceFilter::Tokens,
    );
}

fn check_precedence(analysis: &mut Analysis, item: &PrecedenceItem, enablement: &VersionRange) {
    let PrecedenceItem {
        name,
        enabled_in,
        disabled_in,
        precedence_expressions,
        primary_expressions,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

    for precedence_expression in precedence_expressions {
        let PrecedenceExpression { name: _, operators } = &**precedence_expression;

        for operator in operators {
            let PrecedenceOperator {
                model: _,
                enabled_in,
                disabled_in,
                error_recovery: _,
                fields,
            } = &**operator;

            let enablement = update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

            check_fields(analysis, Some(name), &fields, &enablement);
        }
    }

    for primary_expression in primary_expressions {
        let PrimaryExpression {
            expression,
            enabled_in,
            disabled_in,
        } = &**primary_expression;

        let enablement = update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

        check_reference(
            analysis,
            Some(name),
            &expression,
            &enablement,
            ReferenceFilter::Nodes,
        );
    }
}

fn check_fields(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    fields: &IndexMap<Spanned<Identifier>, Spanned<Field>>,
    enablement: &VersionRange,
) {
    for field in fields.values() {
        match &**field {
            Field::Required { kind } => {
                check_field_kind(analysis, source, &kind, &enablement);
            }
            Field::Optional {
                kind,
                enabled_in,
                disabled_in,
            } => {
                let enablement =
                    update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

                check_field_kind(analysis, source, &kind, &enablement);
            }
        };
    }
}

fn check_field_kind(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    kind: &FieldKind,
    enablement: &VersionRange,
) {
    match kind {
        FieldKind::NonTerminal { item } => {
            check_reference(
                analysis,
                source,
                item,
                &enablement,
                ReferenceFilter::NonTerminals,
            );
        }
        FieldKind::Terminal { items } => {
            for item in items {
                check_reference(
                    analysis,
                    source,
                    item,
                    &enablement,
                    ReferenceFilter::Terminals,
                );
            }
        }
    };
}

fn check_trivium(analysis: &mut Analysis, parser: &TriviaParser, enablement: &VersionRange) {
    match parser {
        TriviaParser::Sequence { parsers } | TriviaParser::Choice { parsers } => {
            for parser in parsers {
                check_trivium(analysis, parser, &enablement);
            }
        }
        TriviaParser::ZeroOrMore { parser } | TriviaParser::Optional { parser } => {
            check_trivium(analysis, parser, &enablement);
        }
        TriviaParser::Token { token } => {
            check_reference(analysis, None, token, &enablement, ReferenceFilter::Tokens);
        }
    };
}

fn check_keyword(analysis: &mut Analysis, item: &KeywordItem, enablement: &VersionRange) {
    let KeywordItem {
        name,
        identifier,
        enabled_in,
        disabled_in,
        reserved_in,
        unreserved_in,
        value: _,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

    check_reference(
        analysis,
        Some(name),
        identifier,
        &enablement,
        ReferenceFilter::Tokens,
    );

    check_version_pair(analysis, reserved_in, unreserved_in);
}

fn check_token(analysis: &mut Analysis, item: &TokenItem, enablement: &VersionRange) {
    let TokenItem { name, definitions } = item;

    for definition in definitions {
        let TokenDefinition {
            enabled_in,
            disabled_in,
            scanner,
        } = &**definition;

        let enablement = update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

        check_scanner(analysis, Some(name), &scanner, &enablement);
    }
}

fn check_fragment(analysis: &mut Analysis, item: &FragmentItem, enablement: &VersionRange) {
    let FragmentItem {
        name,
        enabled_in,
        disabled_in,
        scanner,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled_in, &disabled_in);

    check_scanner(analysis, Some(name), scanner, &enablement);
}

fn check_scanner(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    scanner: &Scanner,
    enablement: &VersionRange,
) {
    match scanner {
        Scanner::Sequence { scanners } | Scanner::Choice { scanners } => {
            for scanner in scanners {
                check_scanner(analysis, source, scanner, &enablement);
            }
        }
        Scanner::Optional { scanner }
        | Scanner::ZeroOrMore { scanner }
        | Scanner::OneOrMore { scanner } => {
            check_scanner(analysis, source, scanner, &enablement);
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
            check_scanner(analysis, source, scanner, &enablement);
            check_scanner(analysis, source, not_followed_by, &enablement);
        }
        Scanner::Fragment { reference } => {
            check_reference(
                analysis,
                source,
                reference,
                enablement,
                ReferenceFilter::Fragments,
            );
        }
    };
}

fn update_enablement(
    analysis: &mut Analysis,
    existing: &VersionRange,
    enabled_in: &Option<Spanned<Version>>,
    disabled_in: &Option<Spanned<Version>>,
) -> VersionRange {
    check_version_pair(analysis, enabled_in, disabled_in);

    let enabled_in = match enabled_in {
        None => &existing.inclusive_start,
        Some(enabled_in) => {
            if **enabled_in <= existing.inclusive_start {
                analysis.errors.add(
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
                analysis.errors.add(
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
    analysis: &mut Analysis,
    first: &Option<Spanned<Version>>,
    second: &Option<Spanned<Version>>,
) {
    for version in [first, second].into_iter().flatten() {
        if !analysis.language.versions.contains(version) {
            analysis
                .errors
                .add(version, &Errors::VersionNotFound(version));
        }
    }

    if let (Some(first), Some(second)) = (first, second) {
        if first >= second {
            analysis
                .errors
                .add(first, &Errors::UnorderedVersionPair(first, second));
        }
    }
}

fn check_reference(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    reference: &Spanned<Identifier>,
    enablement: &VersionRange,
    filter: ReferenceFilter,
) {
    let target = match analysis.metadata.get_mut(&**reference) {
        Some(target) => target,
        None => {
            analysis
                .errors
                .add(reference, &Errors::UnknownReference(reference));
            return;
        }
    };

    let defined_in = VersionSet::from_range(enablement.to_owned());
    let not_defined_in = defined_in.difference(&target.defined_in);

    if !not_defined_in.is_empty() {
        analysis.errors.add(
            reference,
            &Errors::InvalidReferenceVersion(reference, &not_defined_in),
        );
    }

    if !filter.apply(&target.kind) {
        let expected = &ItemKind::iter()
            .filter(|kind| filter.apply(kind))
            .collect_vec()[..];

        analysis.errors.add(
            reference,
            &Errors::InvalidReferenceFilter(reference, &target.kind, expected),
        );
    }

    target.used_in.add(enablement);

    target.referenced_from.push(reference.span());

    if let Some(source) = source {
        analysis.metadata[source]
            .referenced_items
            .push((**reference).to_owned());
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
