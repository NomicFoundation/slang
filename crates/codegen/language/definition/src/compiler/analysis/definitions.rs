use crate::{
    compiler::{
        analysis::{Analysis, ItemMetadata},
        versions::{VersionRange, VersionSet},
    },
    internals::Spanned,
    spanned::Item,
    Identifier,
};
use semver::Version;
use std::{collections::HashSet, fmt::Debug};

pub fn analyze_definitions(analysis: &mut Analysis) {
    collect_top_level_items(analysis);

    check_enum_items(analysis);
    check_precedence_items(analysis);
}

fn collect_top_level_items(analysis: &mut Analysis) {
    for item in analysis.language.clone().items() {
        let name = get_item_name(item);
        let defined_in = calculate_defined_in(analysis, item);

        if analysis.metadata.contains_key(&**name) {
            analysis.errors.add(name, &Errors::ExistingItem(name));
        }

        analysis.metadata.insert(
            (**name).to_owned(),
            ItemMetadata {
                name: name.clone(),
                kind: item.into(),

                defined_in,
                used_in: VersionSet::empty(),

                referenced_from: Vec::new(),
                referenced_items: Vec::new(),
            },
        );
    }
}

fn check_enum_items(analysis: &mut Analysis) {
    for item in analysis.language.clone().items() {
        let Item::Enum { item } = item else {
            continue;
        };

        let mut variants = HashSet::new();

        for variant in &item.variants {
            let name = &variant.name;
            if !variants.insert(&**name) {
                analysis.errors.add(name, &Errors::ExistingVariant(name));
            }
        }
    }
}

fn check_precedence_items(analysis: &mut Analysis) {
    // Make sure that all expressions have unique names across the entire language, since they produce their own kinds.
    // However, they cannot be referenced from anywhere else, so no need to add them to top-level definitions.
    let mut all_expressions = HashSet::new();

    for item in analysis.language.clone().items() {
        let Item::Precedence { item } = item else {
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
        }

        for primary_expression in &item.primary_expressions {
            let expression = &primary_expression.expression;

            if !current_expressions.insert(expression) {
                analysis
                    .errors
                    .add(expression, &Errors::ExistingExpression(expression));
            }
        }
    }
}

fn get_item_name(item: &Item) -> &Spanned<Identifier> {
    match item {
        Item::Struct { item } => &item.name,
        Item::Enum { item } => &item.name,
        Item::Repeated { item } => &item.name,
        Item::Separated { item } => &item.name,
        Item::Precedence { item } => &item.name,
        Item::Trivia { item } => &item.name,
        Item::Keyword { item } => &item.name,
        Item::Token { item } => &item.name,
        Item::Fragment { item } => &item.name,
    }
}

fn calculate_defined_in(analysis: &mut Analysis, item: &Item) -> VersionSet {
    return match item {
        Item::Struct { item } => VersionSet::from_range(calculate_enablement(
            analysis,
            &item.enabled_in,
            &item.disabled_in,
        )),
        Item::Enum { item } => VersionSet::from_range(calculate_enablement(
            analysis,
            &item.enabled_in,
            &item.disabled_in,
        )),
        Item::Repeated { item } => VersionSet::from_range(calculate_enablement(
            analysis,
            &item.enabled_in,
            &item.disabled_in,
        )),
        Item::Separated { item } => VersionSet::from_range(calculate_enablement(
            analysis,
            &item.enabled_in,
            &item.disabled_in,
        )),
        Item::Precedence { item } => VersionSet::from_range(calculate_enablement(
            analysis,
            &item.enabled_in,
            &item.disabled_in,
        )),
        Item::Trivia { item: _ } => {
            VersionSet::from_range(calculate_enablement(analysis, &None, &None))
        }
        Item::Keyword { item } => VersionSet::from_range(calculate_enablement(
            analysis,
            &item.enabled_in,
            &item.disabled_in,
        )),
        Item::Token { item } => {
            VersionSet::from_ranges(item.definitions.iter().map(|definition| {
                calculate_enablement(analysis, &definition.enabled_in, &definition.disabled_in)
            }))
        }
        Item::Fragment { item } => VersionSet::from_range(calculate_enablement(
            analysis,
            &item.enabled_in,
            &item.disabled_in,
        )),
    };
}

fn calculate_enablement(
    analysis: &mut Analysis,
    enabled_in: &Option<Spanned<Version>>,
    disabled_in: &Option<Spanned<Version>>,
) -> VersionRange {
    let enabled_in = match enabled_in {
        Some(enabled_in) => enabled_in,
        None => &analysis.language.versions[0],
    };

    return match disabled_in {
        Some(disabled_in) => VersionRange::between(enabled_in, disabled_in),
        None => VersionRange::starting_from(enabled_in),
    };
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("An item with the name '{0}' already exists.")]
    ExistingItem(&'err Identifier),
    #[error("A variant with the name '{0}' already exists.")]
    ExistingVariant(&'err Identifier),
    #[error("An expression with the name '{0}' already exists.")]
    ExistingExpression(&'err Identifier),
}
