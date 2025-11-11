use std::collections::HashMap;

use slang_solidity::backend::binder::Binder;
use slang_solidity::backend::l2_flat_contracts::visitor::Visitor;
use slang_solidity::backend::passes;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{Node, NodeId};

use crate::ast_api::collect_definitions::map_definitions_ids;

pub(crate) struct CompilationOutput {
    pub(crate) binder: Binder,
    pub(crate) compilation_unit: CompilationUnit,
    pub(crate) cst_map: HashMap<NodeId, Node>,
    pub(crate) ast_map: HashMap<NodeId, slang_solidity::backend::l2_flat_contracts::Node>,
}

impl CompilationOutput {
    pub(crate) fn from_passes(output: passes::p5_resolve_references::Output) -> CompilationOutput {
        let map = map_definitions_ids(&output.compilation_unit, &output.binder);
        let mut ast_visitor = collect_nodes::NodesMap::default();

        for (_, file) in output.files {
            ast_visitor.enter_source_unit(&file);
        }

        CompilationOutput {
            binder: output.binder,
            compilation_unit: output.compilation_unit,
            cst_map: map,
            ast_map: ast_visitor.map,
        }
    }
}

macro_rules! assert_eq_defs {
    ($xs:expr, $ys:expr) => {
        assert_eq!(
            $xs.iter()
                .map(|def| def.identifier().text.as_str())
                .collect::<Vec<&str>>(),
            $ys
        );
    };
}

mod collect_definitions;
mod collect_nodes;
mod find_unused_definitions;
mod follow_all_references;
// mod map_nodes;
mod pipeline;
mod test_find_unused_definitions;
mod visit_definition;
