mod generated;

use std::collections::HashMap;
use std::rc::Rc;

pub use generated::*;
use transformer::Transformer;

pub struct CompilationUnit {
    pub files: HashMap<String, SourceUnit>,
}

impl CompilationUnit {
    pub fn from_l1(unit_l1: &input::CompilationUnit) -> Self {
        let mut l1_to_l2 = FromL1 {};
        let files = unit_l1
            .files
            .iter()
            .map(|(file_id, source_unit)| {
                (file_id.clone(), l1_to_l2.transform_source_unit(source_unit))
            })
            .collect();
        Self { files }
    }
}

pub struct FromL1 {}

impl transformer::Transformer for FromL1 {
    fn transform_contract_definition(
        &mut self,
        source: &input::ContractDefinition,
    ) -> ContractDefinition {
        let node_id = source.node_id;
        let abstract_keyword = source.abstract_keyword.as_ref().map(Rc::clone);
        let name = Rc::clone(&source.name);
        let members = self.transform_contract_members(&source.members);
        let inheritance_types = source
            .specifiers
            .iter()
            .find_map(|specifier| {
                if let input::ContractSpecifier::InheritanceSpecifier(inheritance) = specifier {
                    Some(self.transform_inheritance_types(&inheritance.types))
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let storage_layout = source.specifiers.iter().find_map(|specifier| {
            if let input::ContractSpecifier::StorageLayoutSpecifier(storage_layout) = specifier {
                Some(self.transform_storage_layout_specifier(storage_layout))
            } else {
                None
            }
        });

        Rc::new(ContractDefinitionStruct {
            node_id,
            abstract_keyword,
            name,
            members,
            inheritance_types,
            storage_layout,
        })
    }
}
