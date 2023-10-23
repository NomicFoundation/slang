use crate::{
    compiler::{
        analysis::{Analysis, ItemMetadata},
        versions::{VersionRange, VersionSet},
    },
    internals::Spanned,
    spanned::Item,
    Identifier,
};
use indexmap::IndexSet;
use semver::Version;
use std::{collections::HashSet, fmt::Debug};

impl Analysis {
    pub fn analyze_definitions(&mut self) {
        self.collect_top_level_items();
        self.check_inner_definitions();
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

            self.check_existing_item(name);

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

    fn check_inner_definitions(&mut self) {
        // Multiple operators can have the same name, but under the same item.
        // Check if they conflict with top level items, or operators under different items.
        // They cannot be referenced from anywhere else, so no need to add them to top-level definitions.
        let mut all_expressions = HashSet::new();

        for item in self.language.clone().items() {
            match item {
                Item::Enum { item } => {
                    let mut current_variants = HashSet::new();

                    for variant in &item.variants {
                        let name = &variant.name;
                        if !current_variants.insert(&**name) {
                            self.errors.add(name, &Errors::VariantAlreadyDefined(name));
                        }
                    }
                }

                Item::Precedence { item } => {
                    let mut expression_names: IndexSet<_> = item
                        .operators
                        .iter()
                        .map(|operator| &operator.expression_name)
                        .collect();

                    for expression_name in &expression_names {
                        self.check_existing_item(&expression_name);

                        if !all_expressions.insert(expression_name.to_owned()) {
                            self.errors.add(
                                *expression_name,
                                &Errors::ExpressionAlreadyDefined(expression_name),
                            );
                        }
                    }

                    for primary_expression in &item.primary_expressions {
                        let expression_name = &primary_expression.name;
                        if !expression_names.insert(expression_name) {
                            self.errors.add(
                                expression_name,
                                &Errors::VariantAlreadyDefined(expression_name),
                            );
                        }
                    }
                }

                _ => {}
            };
        }
    }

    fn check_existing_item(&mut self, name: &Spanned<Identifier>) {
        if self.metadata.contains_key(&**name) {
            self.errors.add(name, &Errors::ItemAlreadyDefined(name));
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
            Item::Token { item } => {
                let mut defined_in = VersionSet::empty();
                for definition in &item.definitions {
                    defined_in.add(
                        &self.calculate_enablement(&definition.enabled_in, &definition.disabled_in),
                    );
                }
                return defined_in;
            }
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
    #[error("Item '{0}' is already defined.")]
    ItemAlreadyDefined(&'err Identifier),
    #[error("Variant '{0}' is already defined.")]
    VariantAlreadyDefined(&'err Identifier),
    #[error("Expression '{0}' is already defined under a different item.")]
    ExpressionAlreadyDefined(&'err Identifier),
}
