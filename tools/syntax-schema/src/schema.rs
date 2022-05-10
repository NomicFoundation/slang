use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Grammar {
    pub manifest: Manifest,
    pub topics: HashMap<String, Vec<Rule>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub title: String,
    pub sections: Vec<Section>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub title: String,
    pub topics: Vec<Topic>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Topic {
    pub title: String,
    pub definition: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Rule {
    pub name: Option<String>,
    pub title: Option<String>,

    #[serde(flatten)]
    pub production: Production,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Production {
    Alternation(Alternation),
    Difference(Difference),
    Not(Not),
    OneOrMore(OneOrMore),
    Optional(Optional),
    Range(Range),
    Reference(Reference),
    Sequence(Sequence),
    Terminal(Terminal),
    Versions(Versions),
    ZeroOrMore(ZeroOrMore),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Alternation {
    pub alternation: Vec<Rule>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Difference {
    pub difference: DifferenceProperties,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DifferenceProperties {
    pub minuend: String,
    pub subtrahend: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Not {
    pub not: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OneOrMore {
    #[serde(rename = "oneOrMore")]
    pub one_or_more: Box<Rule>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Optional {
    pub optional: Box<Rule>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Range {
    pub range: RangeProperties,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RangeProperties {
    pub from: String,
    pub to: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub reference: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Sequence {
    pub sequence: Vec<Rule>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Terminal {
    pub terminal: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Versions {
    pub versions: HashMap<String, Rule>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZeroOrMore {
    #[serde(rename = "zeroOrMore")]
    pub zero_or_more: Box<Rule>,
}

pub fn load_grammar(manifest_path: &PathBuf) -> Grammar {
    let contents = std::fs::read(manifest_path).unwrap();
    let manifest_path_str = &manifest_path.to_str().unwrap();
    let manifest: Manifest = serde_yaml::from_slice(&contents).expect(manifest_path_str);

    let topics: HashMap<String, Vec<Rule>> = manifest
        .sections
        .iter()
        .flat_map(|section| &section.topics)
        .map(|topic| {
            let topic_path = manifest_path.parent().unwrap().join(&topic.definition);
            let topic_path_str = topic_path.to_str().unwrap();

            let contents = std::fs::read(&topic_path).expect(topic_path_str);
            let rules: Vec<Rule> = serde_yaml::from_slice(&contents).expect(topic_path_str);

            return (topic.definition.clone(), rules);
        })
        .collect();

    return Grammar { manifest, topics };
}
