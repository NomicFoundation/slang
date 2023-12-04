use crate::{
    types::{
        LanguageDefinition, LanguageDefinitionRef, LanguageTopic, ParserRef, PrecedenceParserRef,
        ProductionRef, ScannerRef,
    },
    validation::visitors::{
        location::{Location, LocationRef},
        receivers::Receiver,
        reporter::Reporter,
        VersionSet,
    },
};

pub trait Visitor {
    fn visit_manifest(&mut self, _location: &LocationRef, _reporter: &mut Reporter) -> bool {
        true
    }

    fn visit_production(
        &mut self,
        _production: &ProductionRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        true
    }

    fn visit_version(
        &mut self,
        _version_set: &VersionSet,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        true
    }

    fn visit_scanner(
        &mut self,
        _scanner: &ScannerRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        true
    }

    fn visit_parser(
        &mut self,
        _parser: &ParserRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        true
    }

    fn visit_precedence_parser(
        &mut self,
        _precedence_parser: &PrecedenceParserRef,
        _location: &LocationRef,
        _reporter: &mut Reporter,
    ) -> bool {
        true
    }

    fn finish(&mut self, _reporter: &mut Reporter) {}
}

pub fn run_visitor(
    visitor: &mut impl Visitor,
    language: &LanguageDefinitionRef,
    reporter: &mut Reporter,
) {
    let manifest_location = Location::root(
        language
            .language_dir
            .join(LanguageDefinition::MANIFEST_FILE_NAME),
    );
    if !visitor.visit_manifest(&manifest_location, reporter) {
        return;
    }

    for section in &language.sections {
        for topic in &section.topics {
            let productions_path = language
                .language_dir
                .join(&section.path)
                .join(&topic.path)
                .join(LanguageTopic::PRODUCTIONS_FILE_NAME);

            let location = Location::root(productions_path);

            for (i, production) in topic.productions.iter().enumerate() {
                production.receive(visitor, language, location.index(i), reporter);
            }
        }
    }

    visitor.finish(reporter);
}
