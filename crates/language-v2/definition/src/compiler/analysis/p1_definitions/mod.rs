use std::collections::HashSet;
use std::rc::Rc;

use crate::compiler::analysis::{Analysis, ItemMetadata};
use crate::compiler::utils::version_set::VersionSet;
use crate::internals::Spanned;
use crate::model::{
    Identifier, SpannedField, SpannedItem, SpannedOperatorModel, SpannedPrecedenceOperator,
    SpannedVersionSpecifier,
};

pub(crate) fn run(analysis: &mut Analysis) {
    collect_top_level_items(analysis);

    check_enum_items(analysis);
    check_precedence_items(analysis);
}

fn collect_top_level_items(analysis: &mut Analysis) {
    let language = Rc::clone(&analysis.language);

    for item in language.items() {
        let name = get_item_name(item);
        let defined_in = calculate_defined_in(analysis, item);

        if analysis.metadata.contains_key(&**name) {
            analysis.errors.add(name, &Errors::ExistingItem(name));
        }

        analysis.metadata.insert(
            (**name).clone(),
            ItemMetadata {
                name: name.clone(),
                item: item.clone(),

                defined_in,
                used_in: VersionSet::new(),

                referenced_from: Vec::new(),
                referenced_items: Vec::new(),
            },
        );
    }
}

fn check_enum_items(analysis: &mut Analysis) {
    for item in Rc::clone(&analysis.language).items() {
        let SpannedItem::Enum { item } = item else {
            continue;
        };

        let mut variants = HashSet::new();

        for variant in &item.variants {
            let reference = &variant.reference;
            if !variants.insert(&**reference) {
                analysis
                    .errors
                    .add(reference, &Errors::ExistingVariant(reference));
            }
        }
    }
}

fn check_precedence_items(analysis: &mut Analysis) {
    // Make sure that all expressions have unique names across the entire language, since they produce their own kinds.
    // However, they cannot be referenced from anywhere else, so no need to add them to top-level definitions.
    let mut all_expressions = HashSet::new();

    for item in Rc::clone(&analysis.language).items() {
        let SpannedItem::Precedence { item } = item else {
            continue;
        };

        // Additionally, make sure that both precedence and primary expressions under
        // the same precedence item are unique, as they will produce enum variants.
        let mut current_expressions = HashSet::new();

        for precedence_expression in &item.precedence_expressions {
            let name = &precedence_expression.name;
            if analysis.metadata.contains_key(&**name) {
                analysis.errors.add(name, &Errors::ExistingItem(name));
            }

            if !all_expressions.insert(name) {
                analysis.errors.add(name, &Errors::ExistingExpression(name));
            }

            current_expressions.insert(name);

            // Make sure all operators have the same structure, as they are represented using the same AST type:
            let first_op = &precedence_expression.operators[0];
            for second_op in precedence_expression.operators.iter().skip(1) {
                if !compare_operators(analysis, first_op, second_op) {
                    analysis.errors.add(name, &Errors::OperatorMismatch);
                }
            }
        }

        for primary_expression in &item.primary_expressions {
            let reference = &primary_expression.reference;

            if !current_expressions.insert(reference) {
                analysis
                    .errors
                    .add(reference, &Errors::ExistingExpression(reference));
            }
        }
    }
}

fn compare_operators(
    analysis: &mut Analysis,
    first_op: &SpannedPrecedenceOperator,
    second_op: &SpannedPrecedenceOperator,
) -> bool {
    match (&*first_op.model, &*second_op.model) {
        // Allow it if they are both prefixes:
        (SpannedOperatorModel::Prefix, SpannedOperatorModel::Prefix)
        // Allow it if they are both suffixes:
        | (SpannedOperatorModel::Postfix, SpannedOperatorModel::Postfix)
        // Allow it if they are both binary (regardless of associativity):
        | (
            SpannedOperatorModel::BinaryLeftAssociative
            | SpannedOperatorModel::BinaryRightAssociative,
            SpannedOperatorModel::BinaryLeftAssociative
            | SpannedOperatorModel::BinaryRightAssociative,
        ) => {}
        _ => return false,
    }

    // Must have the same number of fields:
    if first_op.fields.len() != second_op.fields.len() {
        return false;
    }

    for ((first_key, first_field), (second_key, second_field)) in
        first_op.fields.iter().zip(second_op.fields.iter())
    {
        // Must have the same labels:
        if first_key != second_key {
            return false;
        }

        let (
            // Allow it if both are required:
            (
                SpannedField::Required { reference: first_ref },
                SpannedField::Required { reference: second_ref },
            )
            // Allow it if both are optional (regardless of enablement):
            | (
                SpannedField::Optional { reference: first_ref, enabled: _ },
                SpannedField::Optional { reference: second_ref, enabled: _ },
            )
        ) = (first_field, second_field)
        else {
            return false;
        };

        // Allow it if they both reference the same exact kind:
        if first_ref == second_ref {
            continue;
        }

        // Otherwise, allow it if both are terminals:
        match (
            &analysis.metadata[&**first_ref].item,
            &analysis.metadata[&**second_ref].item,
        ) {
            (
                SpannedItem::Keyword { .. } | SpannedItem::Token { .. },
                SpannedItem::Keyword { .. } | SpannedItem::Token { .. },
            ) => {}
            _ => return false,
        }
    }

    true
}

fn get_item_name(item: &SpannedItem) -> &Spanned<Identifier> {
    match item {
        SpannedItem::Struct { item } => &item.name,
        SpannedItem::Enum { item } => &item.name,
        SpannedItem::Repeated { item } => &item.name,
        SpannedItem::Separated { item } => &item.name,
        SpannedItem::Precedence { item } => &item.name,
        SpannedItem::Trivia { item } => &item.name,
        SpannedItem::Keyword { item } => &item.name,
        SpannedItem::Token { item } => &item.name,
        SpannedItem::Fragment { item } => &item.name,
    }
}

fn calculate_defined_in(analysis: &mut Analysis, item: &SpannedItem) -> VersionSet {
    let mut defined_in = VersionSet::new();

    let mut try_add_specifier = |specifier: &Option<Spanned<SpannedVersionSpecifier>>| {
        if let Some(specifier) = specifier {
            defined_in.add_specifier(specifier, &analysis.language);
        } else {
            defined_in.add_all_versions(&analysis.language);
        }
    };

    match item {
        SpannedItem::Struct { item } => {
            try_add_specifier(&item.enabled);
        }
        SpannedItem::Enum { item } => {
            try_add_specifier(&item.enabled);
        }
        SpannedItem::Repeated { item } => {
            try_add_specifier(&item.enabled);
        }
        SpannedItem::Separated { item } => {
            try_add_specifier(&item.enabled);
        }
        SpannedItem::Precedence { item } => {
            try_add_specifier(&item.enabled);
        }
        SpannedItem::Trivia { item: _ } => {
            try_add_specifier(&None);
        }
        SpannedItem::Keyword { item } => {
            try_add_specifier(&item.enabled);
        }
        SpannedItem::Token { item } => {
            try_add_specifier(&item.enabled);
        }
        SpannedItem::Fragment { item } => {
            try_add_specifier(&item.enabled);
        }
    }

    defined_in
}

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("An item with the name '{0}' already exists.")]
    ExistingItem(&'err Identifier),
    #[error("A variant referencing '{0}' already exists.")]
    ExistingVariant(&'err Identifier),
    #[error("An expression with the name '{0}' already exists.")]
    ExistingExpression(&'err Identifier),
    #[error("All operators under the same expression must have the same model and type.")]
    OperatorMismatch,
}
