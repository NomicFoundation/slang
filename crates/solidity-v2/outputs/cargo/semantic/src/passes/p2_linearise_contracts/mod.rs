use std::collections::HashMap;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::common::resolve_identifier_path_in_scope;
use crate::binder::{Binder, ContractDefinition, Definition, InterfaceDefinition, ScopeId};
use crate::context::SemanticFile;

mod c3;

/// In this pass we collect all bases of contracts and interfaces and then
/// compute the linearisation for each of them.
pub fn run(files: &[impl SemanticFile], binder: &mut Binder) {
    for file in files {
        Pass::visit_file_collect_bases(file, binder);
    }
    for file in files {
        Pass::visit_file_linearise_contracts(file, binder);
    }
}

struct Pass<'a> {
    binder: &'a mut Binder,
}

impl<'a> Pass<'a> {
    fn visit_file_collect_bases(file: &impl SemanticFile, binder: &'a mut Binder) {
        let mut pass = Self { binder };
        pass.collect_bases_from(file.ir_root());
    }

    fn visit_file_linearise_contracts(file: &impl SemanticFile, binder: &'a mut Binder) {
        let mut pass = Self { binder };
        pass.linearise_contracts_from(file.ir_root());
    }

    fn collect_bases_from(&mut self, source_unit: &ir::SourceUnit) {
        let scope_id = self
            .binder
            .scope_id_for_node_id(source_unit.id())
            .expect("source unit should have a defined scope");
        for member in &source_unit.members {
            match member {
                ir::SourceUnitMember::ContractDefinition(contract_definition) => {
                    self.visit_contract_definition(contract_definition, scope_id);
                }
                ir::SourceUnitMember::InterfaceDefinition(interface_definition) => {
                    self.visit_interface_definition(interface_definition, scope_id);
                }
                _ => {}
            }
        }
    }

    fn visit_contract_definition(&mut self, node: &ir::ContractDefinition, scope_id: ScopeId) {
        let resolved_bases = self.resolve_inheritance_types(&node.inheritance_types, scope_id);

        let Definition::Contract(definition) = self.binder.get_definition_mut(node.id()) else {
            unreachable!("{node_id:?} should be a contract", node_id = node.id());
        };
        definition.bases = Some(resolved_bases);
    }

    fn visit_interface_definition(&mut self, node: &ir::InterfaceDefinition, scope_id: ScopeId) {
        let resolved_bases = if let Some(inheritance_types) = &node.inheritance {
            self.resolve_inheritance_types(inheritance_types, scope_id)
        } else {
            Vec::new()
        };

        let Definition::Interface(definition) = self.binder.get_definition_mut(node.id()) else {
            unreachable!("{node_id:?} should be an interface", node_id = node.id());
        };
        definition.bases = Some(resolved_bases);
    }

    fn resolve_inheritance_types(
        &mut self,
        types: &ir::InheritanceTypes,
        scope_id: ScopeId,
    ) -> Vec<NodeId> {
        // TODO(validation): check that we are able to resolve all bases as
        // otherwise we end up with a short list saved in the definition
        types
            .iter()
            .filter_map(|inheritance_type| {
                resolve_identifier_path_in_scope(self.binder, &inheritance_type.type_name, scope_id)
                    .as_definition_id()
                    .filter(|definition_id| {
                        // bases are either contracts or interfaces, discard anything else
                        matches!(
                            self.binder.find_definition_by_id(*definition_id).unwrap(),
                            Definition::Contract(_) | Definition::Interface(_)
                        )
                    })
            })
            .collect()
    }

    fn linearise_contracts_from(&mut self, source_unit: &ir::SourceUnit) {
        for member in &source_unit.members {
            let node_id = match member {
                ir::SourceUnitMember::ContractDefinition(contract_definition) => {
                    contract_definition.id()
                }
                ir::SourceUnitMember::InterfaceDefinition(interface_definition) => {
                    interface_definition.id()
                }
                _ => continue,
            };
            let parents = self.find_contract_bases_recursively(node_id);
            if let Some(linearisation) = c3::linearise(&node_id, &parents) {
                self.binder.insert_linearised_bases(node_id, linearisation);
            } else {
                // TODO(validation): linearisation failed, so emit an error
                self.binder.insert_linearised_bases(node_id, Vec::new());
            }
        }
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
            let (Definition::Contract(ContractDefinition { bases, .. })
            | Definition::Interface(InterfaceDefinition { bases, .. })) = definition
            else {
                unreachable!("Node {node_id:?} isn't a contract or interface");
            };
            let bases = bases.as_ref().unwrap_or_else(|| {
                unreachable!(
                    "Contract {identifier:?} hasn't got the bases resolved",
                    identifier = definition.identifier()
                )
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
