mod collector;
mod validator;

use crate::{
    types::LanguageDefinitionRef,
    validation::{
        rules::definitions::keywords::{
            collector::KeywordsCollector, validator::KeywordsValidator,
        },
        visitors::Reporter,
    },
};

pub struct Keywords;

impl Keywords {
    pub fn validate(language: &LanguageDefinitionRef, reporter: &mut Reporter) {
        let keywords = KeywordsCollector::collect(language, reporter);

        KeywordsValidator::validate(language, keywords, reporter);
    }
}
