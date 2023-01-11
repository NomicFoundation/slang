use std::path::PathBuf;

use codegen_utils::context::CodegenContext;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    manifest::{Manifest, ManifestFile, ProductionRef, TopicFile},
    validation::validate_grammar,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Grammar {
    pub manifest: Manifest,
    pub productions: IndexMap<String, Vec<ProductionRef>>,
}

impl Grammar {
    pub fn get_production(&self, name: &str) -> ProductionRef {
        // We can do this because the grammar has been validated
        for (_, v) in &self.productions {
            if let Some(production) = v.iter().find(|p| p.name == name) {
                return production.clone();
            }
        }
        panic!(
            "Cannot find {} production, should have been caught in validation pass",
            name
        )
    }
}

impl Grammar {
    pub fn from_manifest(codegen: &mut CodegenContext, manifest_path: &PathBuf) -> Grammar {
        let contents = codegen.read_file(manifest_path).unwrap();
        let manifest_path_str = &manifest_path.to_str().unwrap();
        let manifest = serde_yaml::from_str::<ManifestFile>(&contents).expect(manifest_path_str);

        let productions: IndexMap<String, Vec<ProductionRef>> = manifest
            .sections
            .iter()
            .flat_map(|section| &section.topics)
            .filter_map(|topic| match &topic.definition {
                None => None,
                Some(definition) => {
                    let topic_path = manifest_path.parent().unwrap().join(definition);
                    let topic_contents = codegen.read_file(&topic_path).unwrap();

                    let productions = serde_yaml::from_str::<TopicFile>(&topic_contents)
                        .expect(topic_path.to_str().unwrap());

                    return Some((definition.clone(), productions));
                }
            })
            .collect();

        let grammar = Grammar {
            manifest,
            productions,
        };

        validate_grammar(&grammar);

        return grammar;
    }
}
