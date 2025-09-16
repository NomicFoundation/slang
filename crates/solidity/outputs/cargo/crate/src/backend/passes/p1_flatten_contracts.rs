use std::collections::HashMap;
use std::rc::Rc;

use super::p0_build_ast::Output as Input;
use crate::backend::l2_flat_contracts::transformer::Transformer;
use crate::backend::l2_flat_contracts::{
    input, ContractDefinition, ContractDefinitionStruct, SourceUnit,
};
use crate::compilation::CompilationUnit;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, SourceUnit>,
}

/// This pass is reserved to make ergonomic changes to the AST in order to make
/// it easier to use. For now, it will only flatten contract specifiers:
/// inheritance and storage layout specifiers. In the future, more
/// transformations will be added.
pub fn run(input: Input) -> Output {
    let mut pass = Pass {};
    let files = input
        .files
        .iter()
        .map(|(file_id, source_unit)| (file_id.clone(), pass.transform_source_unit(source_unit)))
        .collect();
    let compilation_unit = input.compilation_unit;
    Output {
        compilation_unit,
        files,
    }
}

struct Pass {}

impl Transformer for Pass {
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
