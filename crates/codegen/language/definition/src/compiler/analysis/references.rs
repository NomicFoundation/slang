use crate::{
    compiler::analysis::Analysis,
    internals::Spanned,
    model::{
        spanned::{
            EnumItem, EnumVariant, Field, FieldKind, FragmentItem, Item, ItemKind,
            KeywordDefinition, KeywordItem, PrecedenceExpression, PrecedenceItem,
            PrecedenceOperator, PrimaryExpression, RepeatedItem, Scanner, SeparatedItem,
            StructItem, TokenDefinition, TokenItem, TriviaItem, TriviaParser, VersionSpecifier,
        },
        Identifier,
    },
    utils::VersionSet,
};
use indexmap::IndexMap;
use itertools::Itertools;
use semver::Version;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::Display;

pub fn analyze_references(analysis: &mut Analysis) {
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

fn check_item(analysis: &mut Analysis, item: &Item, enablement: &VersionSet) {
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
        Item::Trivia { item } => {
            check_trivia(analysis, item, &enablement);
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

fn check_struct(analysis: &mut Analysis, item: &StructItem, enablement: &VersionSet) {
    let StructItem {
        name,
        enabled,
        error_recovery: _,
        fields,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled);

    check_fields(analysis, Some(name), fields, &enablement);
}

fn check_enum(analysis: &mut Analysis, item: &EnumItem, enablement: &VersionSet) {
    let EnumItem {
        name,
        enabled,
        variants,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled);

    for variant in variants {
        let EnumVariant {
            name: _,
            enabled,
            error_recovery: _,
            fields,
        } = &**variant;

        let enablement = update_enablement(analysis, &enablement, &enabled);

        check_fields(analysis, Some(name), &fields, &enablement);
    }
}

fn check_repeated(analysis: &mut Analysis, item: &RepeatedItem, enablement: &VersionSet) {
    let RepeatedItem {
        name,
        repeated,
        allow_empty: _,
        enabled,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled);

    check_reference(
        analysis,
        Some(name),
        repeated,
        &enablement,
        ReferenceFilter::Nodes,
    );
}

fn check_separated(analysis: &mut Analysis, item: &SeparatedItem, enablement: &VersionSet) {
    let SeparatedItem {
        name,
        separated,
        separator,
        allow_empty: _,
        enabled,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled);

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

fn check_precedence(analysis: &mut Analysis, item: &PrecedenceItem, enablement: &VersionSet) {
    let PrecedenceItem {
        name,
        enabled,
        precedence_expressions,
        primary_expressions,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled);

    for precedence_expression in precedence_expressions {
        let PrecedenceExpression { name: _, operators } = &**precedence_expression;

        for operator in operators {
            let PrecedenceOperator {
                model: _,
                enabled,
                error_recovery: _,
                fields,
            } = &**operator;

            let enablement = update_enablement(analysis, &enablement, &enabled);

            check_fields(analysis, Some(name), &fields, &enablement);
        }
    }

    for primary_expression in primary_expressions {
        let PrimaryExpression {
            expression,
            enabled,
        } = &**primary_expression;

        let enablement = update_enablement(analysis, &enablement, &enabled);

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
    enablement: &VersionSet,
) {
    for field in fields.values() {
        match &**field {
            Field::Required { kind } => {
                check_field_kind(analysis, source, &kind, &enablement);
            }
            Field::Optional { kind, enabled } => {
                let enablement = update_enablement(analysis, &enablement, &enabled);

                check_field_kind(analysis, source, &kind, &enablement);
            }
        };
    }
}

fn check_field_kind(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    kind: &FieldKind,
    enablement: &VersionSet,
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

fn check_trivia_parser(analysis: &mut Analysis, parser: &TriviaParser, enablement: &VersionSet) {
    match parser {
        TriviaParser::Sequence { parsers } | TriviaParser::Choice { parsers } => {
            for parser in parsers {
                check_trivia_parser(analysis, parser, &enablement);
            }
        }
        TriviaParser::ZeroOrMore { parser } | TriviaParser::Optional { parser } => {
            check_trivia_parser(analysis, parser, &enablement);
        }
        TriviaParser::Trivia { trivia } => {
            check_reference(analysis, None, trivia, &enablement, ReferenceFilter::Trivia);
        }
        TriviaParser::EndOfInput => {}
    };
}

fn check_trivia(analysis: &mut Analysis, item: &TriviaItem, enablement: &VersionSet) {
    let TriviaItem { name, scanner } = item;

    check_scanner(analysis, Some(name), &scanner, &enablement);
}

fn check_keyword(analysis: &mut Analysis, item: &KeywordItem, enablement: &VersionSet) {
    let KeywordItem {
        name,
        identifier,
        definitions,
    } = item;

    check_reference(
        analysis,
        Some(name),
        identifier,
        &enablement,
        ReferenceFilter::Tokens,
    );

    for definition in definitions {
        let KeywordDefinition {
            enabled,
            reserved,
            value: _,
        } = &**definition;

        let _ = update_enablement(analysis, &enablement, &enabled);

        if let Some(reserved) = reserved {
            check_version_specifier(analysis, reserved);
        }
    }
}

fn check_token(analysis: &mut Analysis, item: &TokenItem, enablement: &VersionSet) {
    let TokenItem { name, definitions } = item;

    for definition in definitions {
        let TokenDefinition { enabled, scanner } = &**definition;

        let enablement = update_enablement(analysis, &enablement, &enabled);

        check_scanner(analysis, Some(name), &scanner, &enablement);
    }
}

fn check_fragment(analysis: &mut Analysis, item: &FragmentItem, enablement: &VersionSet) {
    let FragmentItem {
        name,
        enabled,
        scanner,
    } = item;

    let enablement = update_enablement(analysis, &enablement, &enabled);

    check_scanner(analysis, Some(name), scanner, &enablement);
}

fn check_scanner(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    scanner: &Scanner,
    enablement: &VersionSet,
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
    fn apply(&self, target_kind: &ItemKind) -> bool {
        match target_kind {
            ItemKind::Struct
            | ItemKind::Enum
            | ItemKind::Repeated
            | ItemKind::Separated
            | ItemKind::Precedence => {
                return matches!(self, Self::Nodes | Self::NonTerminals);
            }
            ItemKind::Trivia => {
                return matches!(self, Self::Nodes | Self::Trivia);
            }
            ItemKind::Keyword => {
                return matches!(self, Self::Nodes | Self::Terminals);
            }
            ItemKind::Token => {
                return matches!(self, Self::Nodes | Self::Terminals | Self::Tokens);
            }
            ItemKind::Fragment => {
                return matches!(self, Self::Fragments);
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

    if !filter.apply(&target.kind) {
        let expected = &ItemKind::iter()
            .filter(|kind| filter.apply(kind))
            .collect_vec()[..];

        analysis.errors.add(
            reference,
            &Errors::InvalidReferenceFilter(reference, &target.kind, expected),
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
    new_specifier: &Option<Spanned<VersionSpecifier>>,
) -> VersionSet {
    let Some(new_specifier) = new_specifier else {
        return existing_enablement.to_owned();
    };

    if !check_version_specifier(analysis, new_specifier) {
        return existing_enablement.to_owned();
    }

    let mut new_enablement = VersionSet::new();
    analysis.add_specifier(&mut new_enablement, new_specifier);

    let not_defined_in = new_enablement.difference(&existing_enablement);
    if !not_defined_in.is_empty() {
        analysis
            .errors
            .add(new_specifier, &Errors::EnabledTooWide(&existing_enablement));
    }

    return new_enablement;
}

fn check_version_specifier(analysis: &mut Analysis, specifier: &VersionSpecifier) -> bool {
    match specifier {
        VersionSpecifier::Never => {
            return true;
        }
        VersionSpecifier::From { from } => {
            return check_version(analysis, from);
        }
        VersionSpecifier::Till { till } => {
            return check_version(analysis, till);
        }
        VersionSpecifier::Range { from, till } => {
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
    #[error("Reference '{0}' of kind '{1}' is not valid. Expected: {2:?}.")]
    InvalidReferenceFilter(&'err Identifier, &'err ItemKind, &'err [ItemKind]),
}
