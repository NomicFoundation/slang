use crate::{
    compiler::{analysis::Analysis, version_set::VersionSet},
    internals::Spanned,
    model::{
        Identifier, SpannedEnumItem, SpannedEnumVariant, SpannedField, SpannedFieldKind,
        SpannedFragmentItem, SpannedItem, SpannedKeywordDefinition, SpannedKeywordItem,
        SpannedPrecedenceExpression, SpannedPrecedenceItem, SpannedPrecedenceOperator,
        SpannedPrimaryExpression, SpannedRepeatedItem, SpannedScanner, SpannedSeparatedItem,
        SpannedStructItem, SpannedTokenDefinition, SpannedTokenItem, SpannedTriviaItem,
        SpannedTriviaParser, SpannedVersionSpecifier,
    },
};
use indexmap::IndexMap;
use semver::Version;
use std::fmt::Debug;
use strum_macros::Display;

pub(crate) fn analyze_references(analysis: &mut Analysis) {
    let language = analysis.language.clone();

    let mut enablement = VersionSet::new();
    analysis.add_all_versions(&mut enablement);

    check_reference(
        analysis,
        None,
        &language.root_item,
        &enablement,
        ReferenceFilter::NonTerminals,
    );

    check_trivia_parser(analysis, &language.leading_trivia, &enablement);
    check_trivia_parser(analysis, &language.trailing_trivia, &enablement);

    for item in language.items() {
        check_item(analysis, item, &enablement);
    }
}

fn check_item(analysis: &mut Analysis, item: &SpannedItem, enablement: &VersionSet) {
    match item {
        SpannedItem::Struct { item } => {
            check_struct(analysis, item, enablement);
        }
        SpannedItem::Enum { item } => {
            check_enum(analysis, item, enablement);
        }
        SpannedItem::Repeated { item } => {
            check_repeated(analysis, item, enablement);
        }
        SpannedItem::Separated { item } => {
            check_separated(analysis, item, enablement);
        }
        SpannedItem::Precedence { item } => {
            check_precedence(analysis, item, enablement);
        }
        SpannedItem::Trivia { item } => {
            check_trivia(analysis, item, enablement);
        }
        SpannedItem::Keyword { item } => {
            check_keyword(analysis, item, enablement);
        }
        SpannedItem::Token { item } => {
            check_token(analysis, item, enablement);
        }
        SpannedItem::Fragment { item } => {
            check_fragment(analysis, item, enablement);
        }
    }
}

fn check_struct(analysis: &mut Analysis, item: &SpannedStructItem, enablement: &VersionSet) {
    let SpannedStructItem {
        name,
        enabled,
        error_recovery: _,
        fields,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled);

    check_fields(analysis, Some(name), fields, &enablement);
}

fn check_enum(analysis: &mut Analysis, item: &SpannedEnumItem, enablement: &VersionSet) {
    let SpannedEnumItem {
        name,
        enabled,
        variants,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled);

    for variant in variants {
        let SpannedEnumVariant {
            name: _,
            enabled,
            reference,
        } = variant;

        let enablement = update_enablement(analysis, &enablement, enabled);

        check_reference(
            analysis,
            Some(name),
            reference,
            &enablement,
            ReferenceFilter::Nodes,
        );
    }
}

fn check_repeated(analysis: &mut Analysis, item: &SpannedRepeatedItem, enablement: &VersionSet) {
    let SpannedRepeatedItem {
        name,
        repeated,
        allow_empty: _,
        enabled,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled);

    check_reference(
        analysis,
        Some(name),
        repeated,
        &enablement,
        ReferenceFilter::Nodes,
    );
}

fn check_separated(analysis: &mut Analysis, item: &SpannedSeparatedItem, enablement: &VersionSet) {
    let SpannedSeparatedItem {
        name,
        separated,
        separator,
        allow_empty: _,
        enabled,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled);

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

fn check_precedence(
    analysis: &mut Analysis,
    item: &SpannedPrecedenceItem,
    enablement: &VersionSet,
) {
    let SpannedPrecedenceItem {
        name,
        enabled,
        precedence_expressions,
        primary_expressions,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled);

    for precedence_expression in precedence_expressions {
        let SpannedPrecedenceExpression {
            name: _,
            rule_name: _,
            operators,
        } = precedence_expression;

        for operator in operators {
            let SpannedPrecedenceOperator {
                model: _,
                enabled,
                error_recovery: _,
                fields,
            } = operator;

            let enablement = update_enablement(analysis, &enablement, enabled);

            check_fields(analysis, Some(name), fields, &enablement);
        }
    }

    for primary_expression in primary_expressions {
        let SpannedPrimaryExpression {
            expression,
            enabled,
        } = primary_expression;

        let enablement = update_enablement(analysis, &enablement, enabled);

        check_reference(
            analysis,
            Some(name),
            expression,
            &enablement,
            ReferenceFilter::Nodes,
        );
    }
}

fn check_fields(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    fields: &IndexMap<Spanned<Identifier>, SpannedField>,
    enablement: &VersionSet,
) {
    for field in fields.values() {
        match field {
            SpannedField::Required { kind } => {
                check_field_kind(analysis, source, kind, enablement);
            }
            SpannedField::Optional { kind, enabled } => {
                let enablement = update_enablement(analysis, enablement, enabled);

                check_field_kind(analysis, source, kind, &enablement);
            }
        };
    }
}

fn check_field_kind(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    kind: &SpannedFieldKind,
    enablement: &VersionSet,
) {
    match kind {
        SpannedFieldKind::NonTerminal { item } => {
            check_reference(
                analysis,
                source,
                item,
                enablement,
                ReferenceFilter::NonTerminals,
            );
        }
        SpannedFieldKind::Terminal { items } => {
            for item in items {
                check_reference(
                    analysis,
                    source,
                    item,
                    enablement,
                    ReferenceFilter::Terminals,
                );
            }
        }
    };
}

fn check_trivia_parser(
    analysis: &mut Analysis,
    parser: &SpannedTriviaParser,
    enablement: &VersionSet,
) {
    match parser {
        SpannedTriviaParser::Sequence { parsers } | SpannedTriviaParser::Choice { parsers } => {
            for parser in parsers {
                check_trivia_parser(analysis, parser, enablement);
            }
        }
        SpannedTriviaParser::OneOrMore { parser }
        | SpannedTriviaParser::ZeroOrMore { parser }
        | SpannedTriviaParser::Optional { parser } => {
            check_trivia_parser(analysis, parser, enablement);
        }
        SpannedTriviaParser::Trivia { trivia } => {
            check_reference(analysis, None, trivia, enablement, ReferenceFilter::Trivia);
        }
    };
}

fn check_trivia(analysis: &mut Analysis, item: &SpannedTriviaItem, enablement: &VersionSet) {
    let SpannedTriviaItem { name, scanner } = item;

    check_scanner(analysis, Some(name), scanner, enablement);
}

fn check_keyword(analysis: &mut Analysis, item: &SpannedKeywordItem, enablement: &VersionSet) {
    let SpannedKeywordItem {
        name,
        identifier,
        definitions,
    } = item;

    check_reference(
        analysis,
        Some(name),
        identifier,
        enablement,
        ReferenceFilter::Tokens,
    );

    for definition in definitions {
        let SpannedKeywordDefinition {
            enabled,
            reserved,
            value: _,
        } = definition;

        let _ = update_enablement(analysis, enablement, enabled);

        if let Some(reserved) = reserved {
            check_version_specifier(analysis, reserved);
        }
    }
}

fn check_token(analysis: &mut Analysis, item: &SpannedTokenItem, enablement: &VersionSet) {
    let SpannedTokenItem { name, definitions } = item;

    for definition in definitions {
        let SpannedTokenDefinition { enabled, scanner } = definition;

        let enablement = update_enablement(analysis, enablement, enabled);

        check_scanner(analysis, Some(name), scanner, &enablement);
    }
}

fn check_fragment(analysis: &mut Analysis, item: &SpannedFragmentItem, enablement: &VersionSet) {
    let SpannedFragmentItem {
        name,
        enabled,
        scanner,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled);

    check_scanner(analysis, Some(name), scanner, &enablement);
}

fn check_scanner(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    scanner: &SpannedScanner,
    enablement: &VersionSet,
) {
    match scanner {
        SpannedScanner::Sequence { scanners } | SpannedScanner::Choice { scanners } => {
            for scanner in scanners {
                check_scanner(analysis, source, scanner, enablement);
            }
        }
        SpannedScanner::Optional { scanner }
        | SpannedScanner::ZeroOrMore { scanner }
        | SpannedScanner::OneOrMore { scanner } => {
            check_scanner(analysis, source, scanner, enablement);
        }
        SpannedScanner::Not { chars: _ }
        | SpannedScanner::Range {
            inclusive_start: _,
            inclusive_end: _,
        }
        | SpannedScanner::Atom { atom: _ } => {
            // Nothing to check for now.
        }
        SpannedScanner::TrailingContext {
            scanner,
            not_followed_by,
        } => {
            check_scanner(analysis, source, scanner, enablement);
            check_scanner(analysis, source, not_followed_by, enablement);
        }
        SpannedScanner::Fragment { reference } => {
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

#[derive(Debug, Display, PartialEq, Eq)]
enum ReferenceFilter {
    Nodes,

    NonTerminals,
    Terminals,

    Trivia,
    Tokens,

    Fragments,
}

impl ReferenceFilter {
    fn apply(&self, item: &SpannedItem) -> bool {
        match (self, item) {
            (Self::Nodes, item)
                if Self::NonTerminals.apply(item) || Self::Terminals.apply(item) =>
            {
                return true;
            }
            (
                Self::NonTerminals,
                SpannedItem::Struct { .. }
                | SpannedItem::Enum { .. }
                | SpannedItem::Repeated { .. }
                | SpannedItem::Separated { .. }
                | SpannedItem::Precedence { .. },
            )
            | (
                Self::Terminals,
                SpannedItem::Trivia { .. }
                | SpannedItem::Keyword { .. }
                | SpannedItem::Token { .. },
            )
            | (Self::Trivia, SpannedItem::Trivia { .. })
            | (Self::Tokens, SpannedItem::Token { .. })
            | (Self::Fragments, SpannedItem::Fragment { .. }) => {
                return true;
            }

            _ => {
                return false;
            }
        };
    }
}

fn check_reference(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    reference: &Spanned<Identifier>,
    enablement: &VersionSet,
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

    let not_defined_in = enablement.difference(&target.defined_in);
    if !not_defined_in.is_empty() {
        analysis.errors.add(
            reference,
            &Errors::InvalidReferenceVersion(reference, &target.defined_in, &not_defined_in),
        );
    }

    if !filter.apply(&target.item) {
        analysis.errors.add(
            reference,
            &Errors::InvalidReferenceFilter(reference, &filter),
        );
    }

    target.used_in.add_version_set(enablement);

    target.referenced_from.push(reference.span());

    if let Some(source) = source {
        analysis.metadata[source]
            .referenced_items
            .push((**reference).to_owned());
    }
}

fn update_enablement(
    analysis: &mut Analysis,
    existing_enablement: &VersionSet,
    new_specifier: &Option<Spanned<SpannedVersionSpecifier>>,
) -> VersionSet {
    let Some(new_specifier) = new_specifier else {
        return existing_enablement.to_owned();
    };

    if !check_version_specifier(analysis, new_specifier) {
        return existing_enablement.to_owned();
    }

    let mut new_enablement = VersionSet::new();
    analysis.add_specifier(&mut new_enablement, new_specifier);

    let not_defined_in = new_enablement.difference(existing_enablement);
    if !not_defined_in.is_empty() {
        analysis
            .errors
            .add(new_specifier, &Errors::EnabledTooWide(existing_enablement));
    }

    return new_enablement;
}

fn check_version_specifier(analysis: &mut Analysis, specifier: &SpannedVersionSpecifier) -> bool {
    match specifier {
        SpannedVersionSpecifier::Never => {
            return true;
        }
        SpannedVersionSpecifier::From { from } => {
            return check_version(analysis, from);
        }
        SpannedVersionSpecifier::Till { till } => {
            return check_version(analysis, till);
        }
        SpannedVersionSpecifier::Range { from, till } => {
            if from >= till {
                analysis
                    .errors
                    .add(from, &Errors::UnorderedVersionPair(from, till));
                return false;
            }

            return check_version(analysis, from) || check_version(analysis, till);
        }
    };
}

fn check_version(analysis: &mut Analysis, version: &Spanned<Version>) -> bool {
    if !analysis.language.versions.contains(version) {
        analysis
            .errors
            .add(version, &Errors::VersionNotFound(version));

        return false;
    }

    return true;
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Version '{0}' does not exist in the language definition.")]
    VersionNotFound(&'err Version),
    #[error("Version '{0}' must be less than corresponding version '{1}'.")]
    UnorderedVersionPair(&'err Version, &'err Version),
    #[error("Parent scope is only enabled in '{0}'.")]
    EnabledTooWide(&'err VersionSet),
    #[error("Reference to unknown item '{0}'.")]
    UnknownReference(&'err Identifier),
    #[error("Reference '{0}' is only defined in '{1}', but not in '{2}'.")]
    InvalidReferenceVersion(&'err Identifier, &'err VersionSet, &'err VersionSet),
    #[error("Reference '{0}' is not valid. Expected: {1}.")]
    InvalidReferenceFilter(&'err Identifier, &'err ReferenceFilter),
}
