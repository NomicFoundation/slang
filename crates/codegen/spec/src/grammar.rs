use codegen_schema::types::grammar::{Grammar, GrammarSection, GrammarTopic};

pub trait GrammarSpecPrivateExtensions {
    fn locate_production(&self, name: &str) -> (&GrammarSection, &GrammarTopic);
}

impl GrammarSpecPrivateExtensions for Grammar {
    fn locate_production(&self, name: &str) -> (&GrammarSection, &GrammarTopic) {
        for section in &self.sections {
            for topic in &section.topics {
                if topic.productions.contains_key(name) {
                    return (section, topic);
                }
            }
        }

        unreachable!("Cannot locate production '{name}'.");
    }
}
