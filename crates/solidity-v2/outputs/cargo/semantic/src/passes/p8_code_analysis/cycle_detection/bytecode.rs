use std::ops::Range;

use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    BytecodeDependencyValidatorExhausted, CyclicBytecodeDependency,
};
use slang_solidity_v2_common::nodes::NodeId;

use super::{CycleSearchResult, DependencyGraph};
use crate::binder::{Binder, Definition};
use crate::context::{ContractData, FileNodeMapper};

pub(super) fn detect_bytecode_dependency_cycles(
    binder: &Binder,
    contract_data: &ContractData,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let dependencies = contract_data.contract_dependencies();
    let edges = dependencies
        .iter()
        .map(|(contract_id, targets)| (*contract_id, targets.keys().copied().collect()))
        .collect();
    let graph = DependencyGraph::new(edges);

    let mut reported_references = Set::default();
    for (contract_id, result) in graph.find_all_cycles() {
        match result {
            CycleSearchResult::Cycle { via } => {
                let reference = &dependencies[&contract_id][&via];
                // Each referencing expression is reported once, even when
                // several contracts reach the cycle through it.
                if reported_references.insert(reference.node_id()) {
                    diagnostics.push(
                        file_node_mapper
                            .file_id_from_node_id(reference.node_id())
                            .clone(),
                        reference.range(),
                        CyclicBytecodeDependency,
                    );
                }
            }
            CycleSearchResult::DepthExceeded { node } => {
                diagnostics.push(
                    file_node_mapper.file_id_from_node_id(node).clone(),
                    contract_range(binder, node),
                    BytecodeDependencyValidatorExhausted,
                );
            }
            CycleSearchResult::None => unreachable!("cycle-free nodes are not returned"),
        }
    }
}

fn contract_range(binder: &Binder, contract_id: NodeId) -> Range<usize> {
    match binder.find_definition_by_id(contract_id) {
        Some(Definition::Contract(contract)) => contract.ir_node.range.clone(),
        Some(Definition::Interface(interface)) => interface.ir_node.range.clone(),
        Some(Definition::Library(library)) => library.ir_node.range.clone(),
        _ => panic!("graph nodes should be contract definitions"),
    }
}
