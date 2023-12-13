mod collector;
mod validator;

use crate::types::LanguageDefinitionRef;
use crate::validation::rules::definitions::keywords::collector::KeywordsCollector;
use crate::validation::rules::definitions::keywords::validator::KeywordsValidator;
use crate::validation::visitors::Reporter;

pub struct Keywords;

impl Keywords {
    pub fn validate(language: &LanguageDefinitionRef, reporter: &mut Reporter) {
        let keywords = KeywordsCollector::collect(language, reporter);

        KeywordsValidator::validate(language, keywords, reporter);
    }
}
