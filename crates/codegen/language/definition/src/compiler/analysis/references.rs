use std::fmt::Debug;
use std::rc::Rc;

use indexmap::IndexMap;
use semver::Version;

use crate::compiler::analysis::Analysis;
use crate::compiler::version_set::VersionSet;
use crate::internals::Spanned;
use crate::model::SpannedItemDiscriminants::{
    self, Enum, Fragment, Keyword, Precedence, Repeated, Separated, Struct, Token, Trivia,
};
use crate::model::{
    Identifier, SpannedBuiltIn, SpannedBuiltInField, SpannedBuiltInFunction, SpannedBuiltInType,
    SpannedEnumItem, SpannedEnumVariant, SpannedField, SpannedFragmentItem, SpannedItem,
    SpannedKeywordDefinition, SpannedKeywordItem, SpannedPrecedenceExpression,
    SpannedPrecedenceItem, SpannedPrecedenceOperator, SpannedPrimaryExpression,
    SpannedRepeatedItem, SpannedScanner, SpannedSeparatedItem, SpannedStructItem,
    SpannedTokenDefinition, SpannedTokenItem, SpannedTriviaItem, SpannedTriviaParser,
    SpannedVersionSpecifier,
};

pub(crate) fn analyze_references(analysis: &mut Analysis) {
    let language = Rc::clone(&analysis.language);

    let mut enablement = VersionSet::new();
    analysis.add_all_versions(&mut enablement);

    check_reference(
        analysis,
        None,
        &language.root_item,
        &enablement,
        &[Struct, Enum, Repeated, Separated, Precedence],
    );

    check_trivia_parser(analysis, &language.leading_trivia, &enablement);
    check_trivia_parser(analysis, &language.trailing_trivia, &enablement);

    for item in language.items() {
        check_item(analysis, item, &enablement);
    }

    for built_in_context in &language.built_ins {
        for built_in in &built_in_context.definitions {
            check_built_in(analysis, built_in, &enablement);
        }
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

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    check_fields(analysis, Some(name), fields, &enablement);
}

fn check_enum(analysis: &mut Analysis, item: &SpannedEnumItem, enablement: &VersionSet) {
    let SpannedEnumItem {
        name,
        enabled,
        variants,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    for variant in variants {
        let SpannedEnumVariant { reference, enabled } = variant;

        let enablement = update_enablement(analysis, &enablement, enabled.as_ref());

        check_reference(
            analysis,
            Some(name),
            reference,
            &enablement,
            &[
                Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
            ],
        );
    }
}

fn check_repeated(analysis: &mut Analysis, item: &SpannedRepeatedItem, enablement: &VersionSet) {
    let SpannedRepeatedItem {
        name,
        reference,
        allow_empty: _,
        enabled,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    check_reference(
        analysis,
        Some(name),
        reference,
        &enablement,
        &[
            Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
        ],
    );
}

fn check_separated(analysis: &mut Analysis, item: &SpannedSeparatedItem, enablement: &VersionSet) {
    let SpannedSeparatedItem {
        name,
        reference,
        separator,
        allow_empty: _,
        enabled,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    check_reference(
        analysis,
        Some(name),
        reference,
        &enablement,
        &[
            Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
        ],
    );

    check_reference(analysis, Some(name), separator, &enablement, &[Token]);
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

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    for precedence_expression in precedence_expressions {
        let SpannedPrecedenceExpression { name: _, operators } = precedence_expression.as_ref();

        for operator in operators {
            let SpannedPrecedenceOperator {
                model: _,
                enabled,
                error_recovery: _,
                fields,
            } = operator;

            let enablement = update_enablement(analysis, &enablement, enabled.as_ref());

            check_fields(analysis, Some(name), fields, &enablement);
        }
    }

    for primary_expression in primary_expressions {
        let SpannedPrimaryExpression { reference, enabled } = primary_expression;

        let enablement = update_enablement(analysis, &enablement, enabled.as_ref());

        check_reference(
            analysis,
            Some(name),
            reference,
            &enablement,
            &[
                Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
            ],
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
            SpannedField::Required { reference } => {
                check_reference(
                    analysis,
                    source,
                    reference,
                    enablement,
                    &[
                        Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
                    ],
                );
            }
            SpannedField::Optional { reference, enabled } => {
                let enablement = update_enablement(analysis, enablement, enabled.as_ref());

                check_reference(
                    analysis,
                    source,
                    reference,
                    &enablement,
                    &[
                        Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
                    ],
                );

                if let Some(target) = analysis.metadata.get_mut(&**reference) {
                    match &target.item {
                        SpannedItem::Repeated { item: child } => {
                            if child.allow_empty.as_ref().is_some_and(|b| **b) {
                                analysis
                                    .errors
                                    .add(reference, &Errors::OptionalFieldAllowsEmpty);
                            }
                        }
                        SpannedItem::Separated { item: child } => {
                            if child.allow_empty.as_ref().is_some_and(|b| **b) {
                                analysis
                                    .errors
                                    .add(reference, &Errors::OptionalFieldAllowsEmpty);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
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
        SpannedTriviaParser::Trivia { reference } => {
            check_reference(analysis, None, reference, enablement, &[Trivia]);
        }
    }
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

    check_reference(analysis, Some(name), identifier, enablement, &[Token]);

    for definition in definitions {
        let SpannedKeywordDefinition {
            enabled,
            reserved,
            value: _,
        } = definition;

        let _ = update_enablement(analysis, enablement, enabled.as_ref());

        if let Some(reserved) = reserved {
            check_version_specifier(analysis, reserved);
        }
    }
}

fn check_token(analysis: &mut Analysis, item: &SpannedTokenItem, enablement: &VersionSet) {
    let SpannedTokenItem { name, definitions } = item;

    for definition in definitions {
        let SpannedTokenDefinition { enabled, scanner } = definition;

        let enablement = update_enablement(analysis, enablement, enabled.as_ref());

        check_scanner(analysis, Some(name), scanner, &enablement);
    }
}

fn check_fragment(analysis: &mut Analysis, item: &SpannedFragmentItem, enablement: &VersionSet) {
    let SpannedFragmentItem {
        name,
        enabled,
        scanner,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

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
            check_reference(analysis, source, reference, enablement, &[Fragment]);
        }
    }
}

fn check_reference(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    reference: &Spanned<Identifier>,
    enablement: &VersionSet,
    expected_kinds: &[SpannedItemDiscriminants],
) {
    let Some(target) = analysis.metadata.get_mut(&**reference) else {
        analysis
            .errors
            .add(reference, &Errors::UnknownReference(reference));
        return;
    };

    let not_defined_in = enablement.difference(&target.defined_in);
    if !not_defined_in.is_empty() {
        analysis.errors.add(
            reference,
            &Errors::InvalidReferenceVersion(reference, &target.defined_in, &not_defined_in),
        );
    }

    let actual_kind: SpannedItemDiscriminants = target.item.clone().into();
    if !expected_kinds.contains(&actual_kind) {
        analysis.errors.add(
            reference,
            &Errors::InvalidReferenceFilter(reference, &actual_kind, expected_kinds),
        );
    }

    target.used_in.add_version_set(enablement);

    target.referenced_from.push(reference.span());

    if let Some(source) = source {
        analysis.metadata[source]
            .referenced_items
            .push((**reference).clone());
    }
}

fn check_built_in(analysis: &mut Analysis, built_in: &SpannedBuiltIn, enablement: &VersionSet) {
    match built_in {
        SpannedBuiltIn::BuiltInFunction { item } => {
            check_built_in_function(analysis, item, enablement);
        }
        SpannedBuiltIn::BuiltInType { item } => {
            check_built_in_type(analysis, item, enablement);
        }
        SpannedBuiltIn::BuiltInVariable { item } => {
            check_built_in_field(analysis, item, enablement);
        }
    }
}

fn check_built_in_function(
    analysis: &mut Analysis,
    built_in: &SpannedBuiltInFunction,
    enablement: &VersionSet,
) {
    let SpannedBuiltInFunction {
        name: _,
        return_type: _,
        parameters: _,
        enabled,
    } = built_in;

    let _ = update_enablement(analysis, enablement, enabled.as_ref());
}

fn check_built_in_type(
    analysis: &mut Analysis,
    built_in: &SpannedBuiltInType,
    enablement: &VersionSet,
) {
    let SpannedBuiltInType {
        name: _,
        fields,
        functions,
        enabled,
    } = built_in;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    for field in fields {
        check_built_in_field(analysis, field, &enablement);
    }
    for function in functions {
        check_built_in_function(analysis, function, &enablement);
    }
}

fn check_built_in_field(
    analysis: &mut Analysis,
    built_in: &SpannedBuiltInField,
    enablement: &VersionSet,
) {
    let SpannedBuiltInField {
        definition: _,
        enabled,
    } = built_in;

    let _ = update_enablement(analysis, enablement, enabled.as_ref());
}

fn update_enablement(
    analysis: &mut Analysis,
    existing_enablement: &VersionSet,
    new_specifier: Option<&Spanned<SpannedVersionSpecifier>>,
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

    new_enablement
}

fn check_version_specifier(analysis: &mut Analysis, specifier: &SpannedVersionSpecifier) -> bool {
    match specifier {
        SpannedVersionSpecifier::Never => true,
        SpannedVersionSpecifier::From { from } => check_version(analysis, from),
        SpannedVersionSpecifier::Till { till } => check_version(analysis, till),
        SpannedVersionSpecifier::Range { from, till } => {
            if from >= till {
                analysis
                    .errors
                    .add(from, &Errors::UnorderedVersionPair(from, till));
                return false;
            }

            check_version(analysis, from) || check_version(analysis, till)
        }
    }
}

fn check_version(analysis: &mut Analysis, version: &Spanned<Version>) -> bool {
    if !analysis.language.versions.contains(version) {
        analysis
            .errors
            .add(version, &Errors::VersionNotFound(version));

        return false;
    }

    true
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
    #[error("Optional field points to a container that allows empty children. Should this be required instead?")]
    OptionalFieldAllowsEmpty,
    #[error("Reference '{0}' of kind '{1:?}' is not valid. Expected: {2:?}")]
    InvalidReferenceFilter(
        &'err Identifier,
        &'err SpannedItemDiscriminants,
        &'err [SpannedItemDiscriminants],
    ),
}
