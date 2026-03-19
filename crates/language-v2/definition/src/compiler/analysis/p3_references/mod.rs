use std::fmt::Debug;
use std::rc::Rc;

use crate::compiler::analysis::Analysis;
use crate::compiler::utils::version_set::VersionSet;
use crate::internals::Spanned;
use crate::model::SpannedItemDiscriminants::{
    self, Enum, Fragment, Keyword, Precedence, Repeated, Separated, Struct, Token, Trivia,
};
use crate::model::{
    Identifier, SpannedBuiltIn, SpannedBuiltInField, SpannedBuiltInFunction, SpannedBuiltInType,
    SpannedEnumItem, SpannedEnumVariant, SpannedField, SpannedFragmentItem, SpannedItem,
    SpannedKeywordItem, SpannedPrecedenceExpression, SpannedPrecedenceItem,
    SpannedPrecedenceOperator, SpannedPrimaryExpression, SpannedRepeatedItem, SpannedScanner,
    SpannedSeparatedItem, SpannedStructItem, SpannedTokenDefinition, SpannedTokenItem,
    SpannedTriviaItem, SpannedTriviaParser, SpannedVersionSpecifier,
};

pub(crate) fn run(analysis: &mut Analysis) {
    let language = Rc::clone(&analysis.language);

    let mut enablement = VersionSet::new();
    enablement.add_all_versions(&analysis.language);

    check_reference(
        analysis,
        None,
        &language.root_item,
        &enablement,
        &[Struct, Enum, Repeated, Separated, Precedence],
        None,
    );

    check_trivia_parser(analysis, &language.leading_trivia, &enablement);
    check_trivia_parser(analysis, &language.trailing_trivia, &enablement);

    for lexical_context in &language.contexts {
        for section in &lexical_context.sections {
            for topic in &section.topics {
                for item in &topic.items {
                    check_item(analysis, item, &enablement, &lexical_context.name);
                }
            }
        }
    }

    for built_in_context in &language.built_ins {
        for built_in in &built_in_context.definitions {
            check_built_in(analysis, built_in, &enablement);
        }
    }
}

fn check_item(
    analysis: &mut Analysis,
    item: &SpannedItem,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    match item {
        SpannedItem::Struct { item } => {
            check_struct(analysis, item, enablement, lexical_context);
        }
        SpannedItem::Enum { item } => {
            check_enum(analysis, item, enablement, lexical_context);
        }
        SpannedItem::Repeated { item } => {
            check_repeated(analysis, item, enablement, lexical_context);
        }
        SpannedItem::Separated { item } => {
            check_separated(analysis, item, enablement, lexical_context);
        }
        SpannedItem::Precedence { item } => {
            check_precedence(analysis, item, enablement, lexical_context);
        }
        SpannedItem::Trivia { item } => {
            check_trivia(analysis, item, enablement, lexical_context);
        }
        SpannedItem::Keyword { item } => {
            check_keyword(analysis, item, enablement);
        }
        SpannedItem::Token { item } => {
            check_token(analysis, item, enablement, lexical_context);
        }
        SpannedItem::Fragment { item } => {
            check_fragment(analysis, item, enablement, lexical_context);
        }
    }
}

fn check_struct(
    analysis: &mut Analysis,
    item: &SpannedStructItem,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    let SpannedStructItem {
        name,
        enabled,
        switch_lexical_context,
        error_recovery: _,
        fields,
        parser_options: _,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    if let Some(gateway_lexical_context) = switch_lexical_context {
        if **gateway_lexical_context == *lexical_context {
            analysis.errors.add(
                gateway_lexical_context,
                &Errors::UnnecessaryLexicalContextSwitch,
            );
        }

        let mut fields = fields.values();
        if let Some(gateway_field) = fields.next() {
            check_gateway_field(analysis, name, gateway_field, &enablement, lexical_context);

            for field in fields {
                check_field(analysis, name, field, &enablement, gateway_lexical_context);
            }
        }
    } else {
        for field in fields.values() {
            check_field(analysis, name, field, &enablement, lexical_context);
        }
    }
}

fn check_enum(
    analysis: &mut Analysis,
    item: &SpannedEnumItem,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    let SpannedEnumItem {
        name,
        enabled,
        variants,
        parser_options: _,
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
            Some(lexical_context),
        );
    }
}

fn check_repeated(
    analysis: &mut Analysis,
    item: &SpannedRepeatedItem,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    let SpannedRepeatedItem {
        name,
        reference,
        allow_empty: _,
        enabled,
        parser_options: _,
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
        Some(lexical_context),
    );
}

fn check_separated(
    analysis: &mut Analysis,
    item: &SpannedSeparatedItem,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    let SpannedSeparatedItem {
        name,
        reference,
        separator,
        allow_empty: _,
        enabled,
        parser_options: _,
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
        Some(lexical_context),
    );

    check_reference(
        analysis,
        Some(name),
        separator,
        &enablement,
        &[Token],
        Some(lexical_context),
    );
}

fn check_precedence(
    analysis: &mut Analysis,
    item: &SpannedPrecedenceItem,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    let SpannedPrecedenceItem {
        name,
        enabled,
        precedence_expressions,
        primary_expressions,
        parser_options: _,
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

            for field in fields.values() {
                check_field(analysis, name, field, &enablement, lexical_context);
            }
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
            Some(lexical_context),
        );
    }
}

fn check_field(
    analysis: &mut Analysis,
    source: &Identifier,
    field: &SpannedField,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    match field {
        SpannedField::Required { reference } => {
            check_reference(
                analysis,
                Some(source),
                reference,
                enablement,
                &[
                    Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
                ],
                Some(lexical_context),
            );
        }
        SpannedField::Optional { reference, enabled } => {
            let enablement = update_enablement(analysis, enablement, enabled.as_ref());

            check_reference(
                analysis,
                Some(source),
                reference,
                &enablement,
                &[
                    Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
                ],
                Some(lexical_context),
            );

            if let Some(target) = analysis.metadata.get(&**reference) {
                if match &target.item {
                    SpannedItem::Repeated { item: child } => {
                        child.allow_empty.as_ref().is_some_and(|b| **b)
                    }
                    SpannedItem::Separated { item: child } => {
                        child.allow_empty.as_ref().is_some_and(|b| **b)
                    }
                    _ => false,
                } {
                    analysis
                        .errors
                        .add(reference, &Errors::OptionalFieldAllowsEmpty);
                }
            }
        }
    }
}

fn check_gateway_field(
    analysis: &mut Analysis,
    source: &Identifier,
    field: &SpannedField,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    match field {
        SpannedField::Required { reference } => {
            check_reference(
                analysis,
                Some(source),
                reference,
                enablement,
                &[Keyword],
                Some(lexical_context),
            );
        }
        SpannedField::Optional {
            reference,
            enabled: _,
        } => {
            analysis
                .errors
                .add(reference, &Errors::OptionalGatewayField);
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
            check_reference(analysis, None, reference, enablement, &[Trivia], None);
        }
    }
}

fn check_trivia(
    analysis: &mut Analysis,
    item: &SpannedTriviaItem,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    let SpannedTriviaItem { name, scanner } = item;

    check_scanner(analysis, Some(name), scanner, enablement, lexical_context);
}

fn check_keyword(analysis: &mut Analysis, item: &SpannedKeywordItem, enablement: &VersionSet) {
    let SpannedKeywordItem {
        name: _,
        enabled,
        definitions: _,
    } = item;

    let _ = update_enablement(analysis, enablement, enabled.as_ref());
}

fn check_token(
    analysis: &mut Analysis,
    item: &SpannedTokenItem,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    let SpannedTokenItem {
        name,
        enabled,
        not_followed_by,
        definitions,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    if let Some(scanner) = not_followed_by {
        check_scanner(analysis, Some(name), scanner, &enablement, lexical_context);
    }

    for definition in definitions {
        let SpannedTokenDefinition { scanner } = definition;

        check_scanner(analysis, Some(name), scanner, &enablement, lexical_context);
    }
}

fn check_fragment(
    analysis: &mut Analysis,
    item: &SpannedFragmentItem,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    let SpannedFragmentItem {
        name,
        enabled,
        scanner,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    check_scanner(analysis, Some(name), scanner, &enablement, lexical_context);
}

fn check_scanner(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    scanner: &SpannedScanner,
    enablement: &VersionSet,
    lexical_context: &Identifier,
) {
    match scanner {
        SpannedScanner::Sequence { scanners } | SpannedScanner::Choice { scanners } => {
            for scanner in scanners {
                check_scanner(analysis, source, scanner, enablement, lexical_context);
            }
        }
        SpannedScanner::Optional { scanner }
        | SpannedScanner::ZeroOrMore { scanner }
        | SpannedScanner::OneOrMore { scanner } => {
            check_scanner(analysis, source, scanner, enablement, lexical_context);
        }
        SpannedScanner::Not { chars: _ }
        | SpannedScanner::Range {
            inclusive_start: _,
            inclusive_end: _,
        }
        | SpannedScanner::Atom { atom: _ } => {
            // Nothing to check for now.
        }
        SpannedScanner::Fragment { reference } => {
            check_reference(
                analysis,
                source,
                reference,
                enablement,
                &[Fragment],
                Some(lexical_context),
            );
        }
    }
}

fn check_reference(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    reference: &Spanned<Identifier>,
    enablement: &VersionSet,
    expected_kinds: &[SpannedItemDiscriminants],
    lexical_context: Option<&Identifier>,
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

    if let Some(expected) = lexical_context {
        if target.lexical_context != *expected {
            analysis.errors.add(
                reference,
                &Errors::InvalidReferenceContext(reference, &target.lexical_context, expected),
            );
        }
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

    let mut new_enablement = VersionSet::new();
    new_enablement.add_specifier(new_specifier, &analysis.language);

    let not_defined_in = new_enablement.difference(existing_enablement);
    if !not_defined_in.is_empty() {
        analysis
            .errors
            .add(new_specifier, &Errors::EnabledTooWide(existing_enablement));
    }

    new_enablement
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
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
    #[error("Reference '{0}' is in context '{1}', but expected context '{2}'.")]
    InvalidReferenceContext(&'err Identifier, &'err Identifier, &'err Identifier),
    #[error("Unnecessary switch to the same lexical context.")]
    UnnecessaryLexicalContextSwitch,
    #[error("First field in a context switch cannot be optional.")]
    OptionalGatewayField,
}
