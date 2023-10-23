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

impl Analysis {
    pub fn analyze_definitions(&mut self) {
        self.collect_top_level_items();

        self.check_enum_items();
        self.check_precedence_items();
    }

    fn collect_top_level_items(&mut self) {
        for item in self.language.clone().items() {
            let name = match item {
                Item::Struct { item } => &item.name,
                Item::Enum { item } => &item.name,
                Item::Repeated { item } => &item.name,
                Item::Separated { item } => &item.name,
                Item::Precedence { item } => &item.name,
                Item::Token { item } => &item.name,
                Item::Keyword { item } => &item.name,
                Item::Fragment { item } => &item.name,
            };

            let defined_in = self.calculate_defined_in(item);

            if self.metadata.contains_key(&**name) {
                self.errors.add(name, &Errors::ExistingItem(name));
            }

            self.metadata.insert(
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

    fn check_enum_items(&mut self) {
        for item in self.language.clone().items() {
            let Item::Enum { item } = item else {
                continue;
            };

            let mut variants = HashSet::new();

            for variant in &item.variants {
                let name = &variant.name;
                if !variants.insert(&**name) {
                    self.errors.add(name, &Errors::ExistingVariant(name));
                }
            }
        }
    }

    fn check_precedence_items(&mut self) {
        // Make sure that all expressions have unique names across the entire language, since they produce their own kinds.
        // However, they cannot be referenced from anywhere else, so no need to add them to top-level definitions.
        let mut all_expressions = HashSet::new();

        for item in self.language.clone().items() {
            let Item::Precedence { item } = item else {
                continue;
            };

            // Additionally, make sure that both precedence and primary expressions under
            // the same precedence item are unique, as they will produce enum variants.
            let mut current_expressions = HashSet::new();

            for precedence_expression in &item.precedence_expressions {
                let name = &precedence_expression.name;
                if self.metadata.contains_key(&**name) {
                    self.errors.add(name, &Errors::ExistingItem(name));
                }

                if !all_expressions.insert(name) {
                    self.errors.add(name, &Errors::ExistingExpression(name));
                }

                current_expressions.insert(name);
            }

            for primary_expression in &item.primary_expressions {
                let expression = &primary_expression.expression;

                if !current_expressions.insert(expression) {
                    self.errors
                        .add(expression, &Errors::ExistingExpression(expression));
                }
            }
        }
    }

    fn calculate_defined_in(&self, item: &Item) -> VersionSet {
        return match item {
            Item::Struct { item } => {
                VersionSet::single(self.calculate_enablement(&item.enabled_in, &item.disabled_in))
            }
            Item::Enum { item } => {
                VersionSet::single(self.calculate_enablement(&item.enabled_in, &item.disabled_in))
            }
            Item::Repeated { item } => {
                VersionSet::single(self.calculate_enablement(&item.enabled_in, &item.disabled_in))
            }
            Item::Separated { item } => {
                VersionSet::single(self.calculate_enablement(&item.enabled_in, &item.disabled_in))
            }
            Item::Precedence { item } => {
                VersionSet::single(self.calculate_enablement(&item.enabled_in, &item.disabled_in))
            }
            Item::Keyword { item } => {
                VersionSet::single(self.calculate_enablement(&item.enabled_in, &item.disabled_in))
            }
            Item::Token { item } => VersionSet::union(item.definitions.iter().map(|definition| {
                self.calculate_enablement(&definition.enabled_in, &definition.disabled_in)
            })),
            Item::Fragment { item } => {
                VersionSet::single(self.calculate_enablement(&item.enabled_in, &item.disabled_in))
            }
        };
    }

    fn calculate_enablement(
        &self,
        enabled_in: &Option<Spanned<Version>>,
        disabled_in: &Option<Spanned<Version>>,
    ) -> VersionRange {
        let enabled_in = match enabled_in {
            Some(enabled_in) => enabled_in,
            None => &self.language.versions[0],
        };

        return match disabled_in {
            Some(disabled_in) => VersionRange::between(enabled_in, disabled_in),
            None => VersionRange::starting_from(enabled_in),
        };
    }
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
