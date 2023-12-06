use crate::{
    compiler::{
        analysis::{Analysis, ItemMetadata},
        version_set::VersionSet,
    },
    internals::Spanned,
    model::{Identifier, SpannedItem, SpannedVersionSpecifier},
};
use std::collections::HashSet;

pub(crate) fn analyze_definitions(analysis: &mut Analysis) {
    collect_top_level_items(analysis);

    check_enum_items(analysis);
    check_precedence_items(analysis);
}

fn collect_top_level_items(analysis: &mut Analysis) {
    let language = analysis.language.clone();

    for item in language.items() {
        let name = get_item_name(item);
        let defined_in = calculate_defined_in(analysis, item);

        if analysis.metadata.contains_key(&**name) {
            analysis.errors.add(name, &Errors::ExistingItem(name));
        }

        analysis.metadata.insert(
            (**name).to_owned(),
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
    for item in analysis.language.clone().items() {
        let SpannedItem::Enum { item } = item.as_ref() else {
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

    for item in analysis.language.clone().items() {
        let SpannedItem::Precedence { item } = item.as_ref() else {
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
            let reference = &primary_expression.reference;

            if !current_expressions.insert(reference) {
                analysis
                    .errors
                    .add(reference, &Errors::ExistingExpression(reference));
            }
        }
    }
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
            analysis.add_specifier(&mut defined_in, specifier);
        } else {
            analysis.add_all_versions(&mut defined_in);
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
            for definition in &item.definitions {
                try_add_specifier(&definition.enabled);
            }
        }
        SpannedItem::Token { item } => {
            for definition in &item.definitions {
                try_add_specifier(&definition.enabled);
            }
        }
        SpannedItem::Fragment { item } => {
            try_add_specifier(&item.enabled);
        }
    };

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
}
