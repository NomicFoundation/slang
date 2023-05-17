use crate::{
    types::{
        ParserRef, PrecedenceParserRef, ProductionRef, ScannerRef, Schema, SchemaRef, SchemaTopic,
    },
    validation::visitors::{
        location::{Location, LocationRef},
        receivers::Receiver,
        reporter::Reporter,
    },
};

pub trait Visitor {
    fn visit_manifest(&mut self, _location: &LocationRef, _reporter: &mut Reporter) -> bool {
        return true;
    }

    fn visit_production(
        &mut self,
        _production: &ProductionRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        return true;
    }

    fn visit_scanner(
        &mut self,
        _scanner: &ScannerRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        return true;
    }

    fn visit_parser(
        &mut self,
        _parser: &ParserRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        return true;
    }

    fn visit_precedence_parser(
        &mut self,
        _precedence_parser: &PrecedenceParserRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        return true;
    }

    fn finish(&mut self, _reporter: &mut Reporter) {}
}

pub fn run_visitor(visitor: &mut impl Visitor, schema: &SchemaRef, reporter: &mut Reporter) {
    let manifest_location = Location::root(schema.schema_dir.join(Schema::MANIFEST_FILE_NAME));
    if !visitor.visit_manifest(&manifest_location, reporter) {
        return;
    }

    for section in &schema.sections {
        for topic in &section.topics {
            let productions_path = schema
                .schema_dir
                .join(&section.path)
                .join(&topic.path)
                .join(SchemaTopic::PRODUCTIONS_FILE_NAME);

            let location = Location::root(productions_path);

            for (i, production) in topic.productions.iter().enumerate() {
                production.receive(visitor, schema, location.index(i), reporter);
            }
        }
    }

    visitor.finish(reporter);
}
