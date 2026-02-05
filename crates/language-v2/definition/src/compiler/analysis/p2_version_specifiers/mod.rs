use std::fmt::Debug;
use std::rc::Rc;

use indexmap::IndexMap;
use semver::Version;

use crate::compiler::analysis::Analysis;
use crate::internals::Spanned;
use crate::model::{
    Identifier, SpannedBuiltIn, SpannedBuiltInField, SpannedBuiltInFunction, SpannedBuiltInType,
    SpannedEnumItem, SpannedEnumVariant, SpannedField, SpannedFragmentItem, SpannedItem,
    SpannedKeywordDefinition, SpannedKeywordItem, SpannedPrecedenceExpression,
    SpannedPrecedenceItem, SpannedPrecedenceOperator, SpannedPrimaryExpression,
    SpannedRepeatedItem, SpannedSeparatedItem, SpannedStructItem, SpannedTokenItem,
    SpannedVersionSpecifier,
};

pub(crate) fn run(analysis: &mut Analysis) {
    let language = Rc::clone(&analysis.language);

    for item in language.items() {
        check_item(analysis, item);
    }

    for built_in_context in &language.built_ins {
        for built_in in &built_in_context.definitions {
            check_built_in(analysis, built_in);
        }
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
        error_recovery: _,
        fields,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());
    check_fields(analysis, fields);
}

fn check_enum(analysis: &mut Analysis, item: &SpannedEnumItem) {
    let SpannedEnumItem {
        name: _,
        enabled,
        variants,
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
    } = item;

    check_version_specifier(analysis, enabled.as_ref());
}

fn check_precedence(analysis: &mut Analysis, item: &SpannedPrecedenceItem) {
    let SpannedPrecedenceItem {
        name: _,
        enabled,
        precedence_expressions,
        primary_expressions,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());

    for precedence_expression in precedence_expressions {
        let SpannedPrecedenceExpression { name: _, operators } = precedence_expression.as_ref();

        for operator in operators {
            let SpannedPrecedenceOperator {
                model: _,
                enabled,
                error_recovery: _,
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
        definitions,
    } = item;

    check_version_specifier(analysis, enabled.as_ref());

    for definition in definitions {
        let SpannedKeywordDefinition { reserved, value: _ } = definition;

        check_version_specifier(analysis, reserved.as_ref());
    }
}

fn check_token(analysis: &mut Analysis, item: &SpannedTokenItem) {
    let SpannedTokenItem {
        name: _,
        enabled,
        definitions: _,
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

fn check_built_in(analysis: &mut Analysis, built_in: &SpannedBuiltIn) {
    match built_in {
        SpannedBuiltIn::BuiltInFunction { item } => {
            check_built_in_function(analysis, item);
        }
        SpannedBuiltIn::BuiltInType { item } => {
            check_built_in_type(analysis, item);
        }
        SpannedBuiltIn::BuiltInVariable { item } => {
            check_built_in_field(analysis, item);
        }
    }
}

fn check_built_in_function(analysis: &mut Analysis, built_in: &SpannedBuiltInFunction) {
    let SpannedBuiltInFunction {
        name: _,
        return_type: _,
        parameters: _,
        enabled,
    } = built_in;

    check_version_specifier(analysis, enabled.as_ref());
}

fn check_built_in_type(analysis: &mut Analysis, built_in: &SpannedBuiltInType) {
    let SpannedBuiltInType {
        name: _,
        fields,
        functions,
        enabled,
    } = built_in;

    check_version_specifier(analysis, enabled.as_ref());

    for field in fields {
        check_built_in_field(analysis, field);
    }
    for function in functions {
        check_built_in_function(analysis, function);
    }
}

fn check_built_in_field(analysis: &mut Analysis, built_in: &SpannedBuiltInField) {
    let SpannedBuiltInField {
        definition: _,
        enabled,
    } = built_in;

    check_version_specifier(analysis, enabled.as_ref());
}

fn check_version_specifier(
    analysis: &mut Analysis,
    specifier: Option<&Spanned<SpannedVersionSpecifier>>,
) {
    let Some(specifier) = specifier else {
        return;
    };

    match &**specifier {
        SpannedVersionSpecifier::Always => {}
        SpannedVersionSpecifier::Never => {}
        SpannedVersionSpecifier::From { from } => {
            check_version(analysis, from);
        }
        SpannedVersionSpecifier::Till { till } => {
            check_version(analysis, till);
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
}
