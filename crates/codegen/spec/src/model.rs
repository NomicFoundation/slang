use std::rc::Rc;

use codegen_ebnf::EbnfModel;
use codegen_language_definition::model::{Identifier, Language, Section, Topic};
use inflector::Inflector;
use serde::Serialize;

pub struct SpecModel {
    pub language: Rc<Language>,
    pub sections: Vec<SpecSection>,
    pub ebnf: EbnfModel,
}

impl SpecModel {
    pub fn build(language: Rc<Language>) -> Self {
        let mut sections = Vec::new();

        for (section_index, section) in language.sections.iter().enumerate() {
            sections.push(SpecSection::build(section_index, section));
        }

        let ebnf = EbnfModel::build(&language);

        Self {
            language,
            sections,
            ebnf,
        }
    }
}

pub struct SpecSection {
    pub title: String,
    pub slug: String,

    pub topics: Vec<SpecTopic>,
}

impl SpecSection {
    pub fn build(section_index: usize, section: &Section) -> Self {
        let title = format!(
            "{section_index}. {title}",
            section_index = section_index + 1,
            title = section.title,
        );

        let slug = format!(
            "{section_index:0>2}-{slug}",
            section_index = section_index + 1,
            slug = section.title.to_kebab_case(),
        );

        let mut topics = Vec::new();

        for (topic_index, topic) in section.topics.iter().enumerate() {
            topics.push(SpecTopic::build(section_index, topic_index, topic));
        }

        Self {
            title,
            slug,
            topics,
        }
    }
}

#[derive(Serialize)]
pub struct SpecTopic {
    pub title: String,
    pub slug: String,

    pub items: Vec<Identifier>,
}

impl SpecTopic {
    pub fn build(section_index: usize, topic_index: usize, topic: &Topic) -> Self {
        let title = format!(
            "{section_index}.{topic_index}. {title}",
            section_index = section_index + 1,
            topic_index = topic_index + 1,
            title = topic.title,
        );

        let slug = format!(
            "{topic_index:0>2}-{slug}",
            topic_index = topic_index + 1,
            slug = topic.title.to_kebab_case(),
        );

        let mut items = vec![];

        for item in &topic.items {
            items.push(item.name().to_owned());
        }

        Self { title, slug, items }
    }
}
