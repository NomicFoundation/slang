use crate::{
    types::ProductionRef,
    validation::{
        rules::references::metadata::Metadata,
        visitors::{LocationRef, Reporter, VersionSet, Visitor},
    },
};

pub struct Collector {
    metadata: Metadata,
    current_production: Option<ProductionRef>,
}

impl Collector {
    pub fn new() -> Self {
        return Self {
            metadata: Metadata::new(),
            current_production: None,
        };
    }

    pub fn metadata(self) -> Metadata {
        return self.metadata;
    }
}

impl Visitor for Collector {
    fn visit_production(
        &mut self,
        production: &ProductionRef,
        location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        self.metadata.add_production(&production.name, location);
        self.current_production = Some(production.to_owned());

        return true;
    }

    fn visit_version(
        &mut self,
        version_set: &VersionSet,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        let production = self.current_production.as_ref().unwrap();
        self.metadata.add_version(&production.name, version_set);

        return false;
    }
}
