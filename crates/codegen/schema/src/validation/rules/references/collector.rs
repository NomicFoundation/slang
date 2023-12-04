use crate::{
    types::{LanguageDefinitionRef, ProductionRef},
    validation::{
        rules::references::metadata::Metadata,
        visitors::{run_visitor, LocationRef, Reporter, VersionSet, Visitor},
    },
};

pub struct Collector<'collector> {
    metadata: &'collector mut Metadata,
    current_production: Option<ProductionRef>,
}

impl<'collector> Collector<'collector> {
    pub fn collect<'call: 'collector>(
        language: &'call LanguageDefinitionRef,
        metadata: &'call mut Metadata,
        reporter: &'call mut Reporter,
    ) {
        let mut instance = Self {
            metadata,
            current_production: None,
        };

        run_visitor(&mut instance, language, reporter);
    }
}

impl Visitor for Collector<'_> {
    fn visit_production(
        &mut self,
        production: &ProductionRef,
        location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.metadata.add_production(&production.name, location);
        self.current_production = Some(production.to_owned());

        true
    }

    fn visit_version(
        &mut self,
        version_set: &VersionSet,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        let production = self.current_production.as_ref().unwrap();
        self.metadata.add_version(&production.name, version_set);

        false
    }
}
