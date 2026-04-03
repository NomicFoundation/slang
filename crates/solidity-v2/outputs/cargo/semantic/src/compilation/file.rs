use std::collections::HashMap;

use crate::ir::{self, NodeId};

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

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    pub(crate) fn add_resolved_import(&mut self, node_id: NodeId, target_file_id: String) {
        self.resolved_imports.insert(node_id, target_file_id);
    }

    pub(crate) fn resolved_import_by_node_id(&self, node_id: NodeId) -> Option<&String> {
        self.resolved_imports.get(&node_id)
    }
}
