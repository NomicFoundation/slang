use std::collections::HashMap;
use std::rc::Rc;

use crate::backend::binder::{
    Binder, ContractDefinition, Definition, ImportDefinition, InterfaceDefinition, Reference,
    Resolution, ScopeId,
};
use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::backend::semantic::SemanticAnalysis;
use crate::cst::NodeId;

mod c3;

/// In this pass we collect all bases of contracts and interfaces and then
/// compute the linearisation for each of them.
pub fn run(semantic_analysis: &mut SemanticAnalysis) {
    let mut pass = Pass::new(&mut semantic_analysis.binder);
    for semantic_file in semantic_analysis.files.values() {
        pass.visit_file_collect_bases(semantic_file.ir_root());
    }
    for semantic_file in semantic_analysis.files.values() {
        pass.visit_file_linearise_contracts(semantic_file.ir_root());
    }
}

pub struct Pass<'a> {
    pub binder: &'a mut Binder,
}

impl<'a> Pass<'a> {
    pub fn new(binder: &'a mut Binder) -> Self {
        Self { binder }
    }

    fn visit_file_collect_bases(&mut self, source_unit: &input_ir::SourceUnit) {
        let scope_id = self
            .binder
            .scope_id_for_node_id(source_unit.node_id)
            .expect("source unit should have a defined scope");
        for member in &source_unit.members {
            match member {
                input_ir::SourceUnitMember::ContractDefinition(contract_definition) => {
                    self.visit_contract_definition(contract_definition, scope_id);
                }
                input_ir::SourceUnitMember::InterfaceDefinition(interface_definition) => {
                    self.visit_interface_definition(interface_definition, scope_id);
                }
                _ => {}
            }
        }
    }

    fn visit_contract_definition(
        &mut self,
        node: &input_ir::ContractDefinition,
        scope_id: ScopeId,
    ) {
        let resolved_bases = self.resolve_inheritance_types(&node.inheritance_types, scope_id);

        let Definition::Contract(definition) = self.binder.get_definition_mut(node.node_id) else {
            unreachable!("{node_id:?} should be a contract", node_id = node.node_id);
        };
        definition.bases = Some(resolved_bases);
    }

    fn visit_interface_definition(
        &mut self,
        node: &input_ir::InterfaceDefinition,
        scope_id: ScopeId,
    ) {
        let mut resolved_bases = vec![];
        if let Some(inheritance_types) = &node.inheritance {
            resolved_bases = self.resolve_inheritance_types(inheritance_types, scope_id);
        }

        let Definition::Interface(definition) = self.binder.get_definition_mut(node.node_id) else {
            unreachable!("{node_id:?} should be an interface", node_id = node.node_id);
        };
        definition.bases = Some(resolved_bases);
    }

    fn resolve_inheritance_types(
        &mut self,
        types: &input_ir::InheritanceTypes,
        scope_id: ScopeId,
    ) -> Vec<NodeId> {
        // TODO(validation): check that we are able to resolve all bases as
        // otherwise we end up with a short list saved in the definition
        types
            .iter()
            .filter_map(|inheritance_type| {
                self.resolve_identifier_path_in_scope(&inheritance_type.type_name, scope_id)
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

    fn resolve_identifier_path_in_scope(
        &mut self,
        identifier_path: &input_ir::IdentifierPath,
        starting_scope_id: ScopeId,
    ) -> Resolution {
        let mut scope_id = Some(starting_scope_id);
        let mut use_lexical_resolution = true;
        let mut last_resolution: Resolution = Resolution::Unresolved;

        for identifier in identifier_path {
            let symbol = identifier.unparse();
            let resolution = if let Some(scope_id) = scope_id {
                if use_lexical_resolution {
                    self.binder.resolve_in_scope(scope_id, &symbol)
                } else {
                    self.binder.resolve_in_scope_as_namespace(scope_id, &symbol)
                }
            } else {
                Resolution::Unresolved
            };

            let reference = Reference::new(Rc::clone(identifier), resolution.clone());
            self.binder.insert_reference(reference);

            // Unless we used namespace resolution and in order to continue
            // resolving the identifier path, we should ensure we've followed
            // through any symbol alias (ie. import deconstruction symbol)
            let resolution = if use_lexical_resolution {
                self.binder.follow_symbol_aliases(&resolution)
            } else {
                resolution
            };

            // recurse into file scopes pointed by the resolved definition
            // to resolve the next identifier in the path
            scope_id = resolution
                .as_definition_id()
                .and_then(|node_id| self.binder.find_definition_by_id(node_id))
                .and_then(|definition| match definition {
                    Definition::Import(ImportDefinition {
                        resolved_file_id, ..
                    }) => resolved_file_id.as_ref().and_then(|resolved_file_id| {
                        self.binder.scope_id_for_file_id(resolved_file_id)
                    }),
                    Definition::Contract(_) | Definition::Interface(_) | Definition::Library(_) => {
                        use_lexical_resolution = false;
                        self.binder.scope_id_for_node_id(definition.node_id())
                    }
                    _ => None,
                });

            last_resolution = resolution;
        }
        last_resolution
    }

    fn visit_file_linearise_contracts(&mut self, source_unit: &input_ir::SourceUnit) {
        for member in &source_unit.members {
            let node_id = match member {
                input_ir::SourceUnitMember::ContractDefinition(contract_definition) => {
                    contract_definition.node_id
                }
                input_ir::SourceUnitMember::InterfaceDefinition(interface_definition) => {
                    interface_definition.node_id
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
