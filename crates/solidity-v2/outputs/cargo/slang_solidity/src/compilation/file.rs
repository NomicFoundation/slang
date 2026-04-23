use std::collections::HashMap;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::SemanticFile;

pub struct File {
    id: String,
    ir_root: ir::SourceUnit,
    resolved_imports: HashMap<NodeId, String>,
}

impl File {
    pub(crate) fn new(id: String, ir_root: ir::SourceUnit) -> Self {
        Self {
            id,
            ir_root,
            resolved_imports: HashMap::new(),
        }
    }

    pub(crate) fn add_resolved_import(&mut self, node_id: NodeId, target_file_id: String) {
        self.resolved_imports.insert(node_id, target_file_id);
    }
}

impl SemanticFile for File {
    fn id(&self) -> &str {
        &self.id
    }

    fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    fn resolved_import_by_node_id(&self, node_id: NodeId) -> Option<&String> {
        self.resolved_imports.get(&node_id)
    }
}
