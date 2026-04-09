use std::fmt::Debug;
use std::rc::Rc;

use indexmap::IndexMap;
use semver::Version;

use crate::compiler::analysis::Analysis;
use crate::internals::Spanned;
use crate::model::{
    Identifier, SpannedBuiltInContext, SpannedBuiltInDefinition, SpannedBuiltInScope,
    SpannedEnumItem, SpannedEnumVariant, SpannedField, SpannedFragmentItem, SpannedItem,
    SpannedKeywordItem, SpannedPrecedenceExpression, SpannedPrecedenceItem,
    SpannedPrecedenceOperator, SpannedPrimaryExpression, SpannedRepeatedItem, SpannedSeparatedItem,
    SpannedStructItem, SpannedTokenItem, SpannedVersionSpecifier,
};

pub(crate) fn run(analysis: &mut Analysis) {
    let language = Rc::clone(&analysis.language);

    for item in language.items() {
        check_item(analysis, item);
    }

    for built_in_context in &language.built_ins {
        check_built_in_context(analysis, built_in_context);
    }
}

fn check_item(analysis: &mut Analysis, item: &SpannedItem) {
    match item {
        SpannedItem::Struct { item } => {
            check_struct(analysis, item);
        }
        SpannedItem::Enum { item } => {
            check_enum(analysis, item);
        }
        SpannedItem::Repeated { item } => {
            check_repeated(analysis, item);
        }
        SpannedItem::Separated { item } => {
            check_separated(analysis, item);
        }
        SpannedItem::Precedence { item } => {
            check_precedence(analysis, item);
        }
        SpannedItem::Trivia { item: _ } => {
            // Unversioned
        }
        SpannedItem::Keyword { item } => {
            check_keyword(analysis, item);
        }
        SpannedItem::Token { item } => {
            check_token(analysis, item);
        }
        SpannedItem::Fragment { item } => {
            check_fragment(analysis, item);
        }
    }
}

fn check_struct(analysis: &mut Analysis, item: &SpannedStructItem) {
    let SpannedStructItem {
        name: _,
        enabled,
        switch_lexical_context: _,
        fields,
        parser_options: _,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());
    check_fields(analysis, fields);
}

fn check_enum(analysis: &mut Analysis, item: &SpannedEnumItem) {
    let SpannedEnumItem {
        name: _,
        enabled,
        variants,
        parser_options: _,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());

    for variant in variants {
        let SpannedEnumVariant {
            reference: _,
            enabled,
        } = variant;

        check_version_specifier(analysis, enabled.as_ref());
    }
}

fn check_repeated(analysis: &mut Analysis, item: &SpannedRepeatedItem) {
    let SpannedRepeatedItem {
        name: _,
        reference: _,
        allow_empty: _,
        enabled,
        parser_options: _,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());
}

fn check_separated(analysis: &mut Analysis, item: &SpannedSeparatedItem) {
    let SpannedSeparatedItem {
        name: _,
        reference: _,
        separator: _,
        allow_empty: _,
        enabled,
        parser_options: _,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());
}

fn check_precedence(analysis: &mut Analysis, item: &SpannedPrecedenceItem) {
    let SpannedPrecedenceItem {
        name: _,
        enabled,
        precedence_expressions,
        primary_expressions,
        parser_options: _,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());

    for precedence_expression in precedence_expressions {
        let SpannedPrecedenceExpression { name: _, operators } = precedence_expression.as_ref();

        for operator in operators {
            let SpannedPrecedenceOperator {
                model: _,
                enabled,
                fields,
            } = operator;

            check_version_specifier(analysis, enabled.as_ref());
            check_fields(analysis, fields);
        }
    }

    for primary_expression in primary_expressions {
        let SpannedPrimaryExpression {
            reference: _,
            enabled,
        } = primary_expression;

        check_version_specifier(analysis, enabled.as_ref());
    }
}

fn check_fields(analysis: &mut Analysis, fields: &IndexMap<Spanned<Identifier>, SpannedField>) {
    for field in fields.values() {
        match field {
            SpannedField::Required { reference: _ } => {}
            SpannedField::Optional {
                reference: _,
                enabled,
            } => {
                check_version_specifier(analysis, enabled.as_ref());
            }
        }
    }
}

fn check_keyword(analysis: &mut Analysis, item: &SpannedKeywordItem) {
    let SpannedKeywordItem {
        name: _,
        enabled,
        reserved,
        scanner: _,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());
    check_version_specifier(analysis, reserved.as_ref());
}

fn check_token(analysis: &mut Analysis, item: &SpannedTokenItem) {
    let SpannedTokenItem {
        name: _,
        enabled,
        not_followed_by: _,
        scanner: _,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());
}

fn check_fragment(analysis: &mut Analysis, item: &SpannedFragmentItem) {
    let SpannedFragmentItem {
        name: _,
        enabled,
        scanner: _,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());
}

fn check_built_in_context(analysis: &mut Analysis, context: &SpannedBuiltInContext) {
    let SpannedBuiltInContext { name: _, scopes } = context;

    for scope in scopes {
        check_built_in_scope(analysis, scope);
    }
}

fn check_built_in_scope(analysis: &mut Analysis, scope: &SpannedBuiltInScope) {
    let SpannedBuiltInScope {
        name: _,
        definitions,
    } = scope;

    for definition in definitions {
        check_built_in_definition(analysis, definition);
    }
}

fn check_built_in_definition(analysis: &mut Analysis, definition: &SpannedBuiltInDefinition) {
    let SpannedBuiltInDefinition { name: _, enabled } = definition;

    check_version_specifier(analysis, enabled.as_ref());
}

fn check_version_specifier(
    analysis: &mut Analysis,
    specifier: Option<&Spanned<SpannedVersionSpecifier>>,
) {
    let Some(specifier) = specifier else {
        return;
    };

    let first_version = analysis.language.versions.first().unwrap().clone();

    match &**specifier {
        SpannedVersionSpecifier::Always => {
            analysis.errors.add(specifier, &Errors::RedundantAlways);
        }
        SpannedVersionSpecifier::Never => {}
        SpannedVersionSpecifier::From { from } => {
            check_version(analysis, from);

            if **from == *first_version {
                analysis.errors.add(from, &Errors::RedundantFrom(from));
            }
        }
        SpannedVersionSpecifier::Till { till } => {
            check_version(analysis, till);

            if **till == *first_version {
                analysis.errors.add(till, &Errors::RedundantTill(till));
            }
        }
        SpannedVersionSpecifier::Range { from, till } => {
            if from >= till {
                analysis
                    .errors
                    .add(from, &Errors::UnorderedVersionPair(from, till));
                return;
            }

            check_version(analysis, from);
            check_version(analysis, till);

            if **from == *first_version {
                analysis.errors.add(from, &Errors::RedundantRangeFrom(from));
            }
        }
    }
}

fn check_version(analysis: &mut Analysis, version: &Spanned<Version>) {
    if !analysis.language.versions.contains(version) {
        analysis
            .errors
            .add(version, &Errors::VersionNotFound(version));
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Version '{0}' does not exist in the language definition.")]
    VersionNotFound(&'err Version),
    #[error("Version '{0}' must be less than corresponding version '{1}'.")]
    UnorderedVersionPair(&'err Version, &'err Version),
    #[error(
        "Explicit 'Always' is redundant, since it is the default when 'enabled' is not specified."
    )]
    RedundantAlways,
    #[error("'From' with the first supported version '{0}' is equivalent to 'Always', and can be removed.")]
    RedundantFrom(&'err Version),
    #[error("'Till' with the first supported version '{0}' is equivalent to 'Never'.")]
    RedundantTill(&'err Version),
    #[error(
        "'Range' starting from the first supported version '{0}' can be simplified to 'Till'."
    )]
    RedundantRangeFrom(&'err Version),
}
