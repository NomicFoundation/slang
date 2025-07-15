use std::collections::HashMap;

use super::p3_type_definitions::Output as Input;
use crate::backend::binder::{Binder, ContractDefinition, Definition, InterfaceDefinition};
use crate::backend::c3;
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::types::TypeRegistry;
use crate::compilation::CompilationUnit;
use crate::cst::NodeId;

pub struct Output {
    pub compilation_unit: CompilationUnit,
    pub files: HashMap<String, input_ir::SourceUnit>,
    pub binder: Binder,
    pub types: TypeRegistry,
}

pub fn run(input: Input) -> Output {
    let files = input.files;
    let mut pass = Pass::new(input.compilation_unit, input.binder, input.types);
    for source_unit in files.values() {
        pass.visit_file(source_unit);
    }

    let compilation_unit = pass.compilation_unit;
    let binder = pass.binder;
    let types = pass.types;
    Output {
        compilation_unit,
        files,
        binder,
        types,
    }
}

pub struct Pass {
    pub compilation_unit: CompilationUnit,
    pub binder: Binder,
    pub types: TypeRegistry,
}

impl Pass {
    pub fn new(compilation_unit: CompilationUnit, binder: Binder, types: TypeRegistry) -> Self {
        Self {
            compilation_unit,
            binder,
            types,
        }
    }

    fn visit_file(&mut self, source_unit: &input_ir::SourceUnit) {
        input_ir::visitor::accept_source_unit(source_unit, self);
    }

    fn find_contract_bases_recursively(&self, node_id: NodeId) -> HashMap<NodeId, Vec<NodeId>> {
        let mut result = HashMap::new();
        let mut queue = vec![node_id];
        while let Some(node_id) = queue.pop() {
            if result.contains_key(&node_id) {
                continue;
            }
            let Some(definition) = self.binder.find_definition_by_id(node_id) else {
                unreachable!("Unable to resolve the definition for node {node_id:?}");
            };
            let (Definition::Contract(ContractDefinition {
                identifier, bases, ..
            })
            | Definition::Interface(InterfaceDefinition {
                identifier, bases, ..
            })) = definition
            else {
                unreachable!("Node {node_id:?} isn't the identifier of a contract");
            };
            let bases = bases.as_ref().unwrap_or_else(|| {
                unreachable!("Contract {identifier:?} hasn't got the bases resolved")
            });

            queue.extend(bases);

            // Solidity uses a modified version of the traditional C3 algorithm
            // where the order of parents is reversed, so we provide them in the
            // inverse order
            let mut parents = bases.clone();
            parents.reverse();
            result.insert(node_id, parents);
        }
        result
    }
}

impl Visitor for Pass {
    fn leave_contract_definition(&mut self, target: &input_ir::ContractDefinition) {
        let node_id = target.node_id;
        let parents = self.find_contract_bases_recursively(node_id);
        if let Some(linearisation) = c3::linearise(&node_id, &parents) {
            self.binder.insert_linearised_bases(node_id, linearisation);
        }
    }

    fn leave_interface_definition(&mut self, target: &input_ir::InterfaceDefinition) {
        let node_id = target.node_id;
        let parents = self.find_contract_bases_recursively(node_id);
        if let Some(linearisation) = c3::linearise(&node_id, &parents) {
            self.binder.insert_linearised_bases(node_id, linearisation);
        }
    }
}
