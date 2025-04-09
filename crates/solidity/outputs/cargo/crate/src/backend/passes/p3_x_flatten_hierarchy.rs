use std::collections::HashMap;

use super::p3_type_definitions::Output as Input;
use crate::backend::binder::{Binder, ContractDefinition, InterfaceDefinition, Typing};
use crate::backend::l2_flat_contracts::visitor::Visitor;
use crate::backend::l2_flat_contracts::{self as input_ir};
use crate::backend::passes::c3;
use crate::backend::types::{Type, TypeRegistry};
use crate::compilation::CompilationUnit;
use crate::cst::NodeId;

type Output = Input;

pub fn run(input: Input) -> Output {
    input
}

pub struct Pass {
    pub compilation_unit: CompilationUnit,
    pub linearisations: HashMap<usize, Vec<usize>>,
    pub binder: Binder,
    pub types: TypeRegistry,
}

impl Pass {
    pub fn new(compilation_unit: CompilationUnit, binder: Binder, types: TypeRegistry) -> Self {
        Self {
            compilation_unit,
            linearisations: HashMap::new(),
            binder,
            types,
        }
    }

    fn find_contract_bases_recursively(&self, node_id: NodeId) -> HashMap<NodeId, Vec<NodeId>> {
        let mut result = HashMap::new();
        let mut queue = vec![node_id];
        while let Some(node_id) = queue.pop() {
            if result.contains_key(&node_id) {
                continue;
            }
            let Some(definition) = self.binder.find_definition_by_identifier_node_id(node_id)
            else {
                panic!("Unable to resolve the definition for node {node_id:?}");
            };
            let Typing::Resolved(type_id) = self.binder.node_typing(definition.node_id()) else {
                panic!("Unable to resolve type definition for node {node_id:?}");
            };
            let Some(Type::Interface { definition_id, .. } | Type::Contract { definition_id, .. }) =
                self.types.get_type_by_id(type_id)
            else {
                panic!(
                    "Found {type_id:?} resolving parents of node {node_id:?} \
                        and it's not a contract or an interface"
                );
            };

            self.
            let Some(definition) =  self.binder.definitions.get(definition_id) else {
              panic!(
                    "Can't find definition {definition_id:?}, a contract or interface from {type_id:?} \
                      (itself a parent of node {node_id:?})"
                );
            };
            let InterfaceDefinition { } | ContractDefinition {} = definition else {
              panic!(
                    "Found {type_id:?} resolving parents of node {node_id:?}, typed as a contract or interface, \
                        but it's definition {definition_id:?} is non-existent"
                );
            };
            // Resolve parents recursively
            queue.extend(base_types.iter());

            // Solidity uses a modified version of the traditional C3 algorithm
            // where the order of parents is reversed, so we provide them in the
            // inverse order
            let mut parents = base_types.clone();
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
            self.linearisations.insert(node_id, linearisation);
        } else {
            panic!(
                "Failed to linearise contract {name}",
                name = target.name.unparse()
            );
        }
    }
}
