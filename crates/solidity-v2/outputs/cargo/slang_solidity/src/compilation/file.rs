use std::collections::HashMap;

use slang_solidity_v2_ir::ir::{self, NodeId};
use slang_solidity_v2_semantic::context::{FileId, SemanticFile};

#[allow(clippy::struct_field_names)]
pub struct File {
    id: String,
    file_id: FileId,
    ir_root: ir::SourceUnit,
    resolved_imports: HashMap<NodeId, FileId>,
}

impl File {
    pub(crate) fn new(id: String, file_id: FileId, ir_root: ir::SourceUnit) -> Self {
        Self {
            id,
            file_id,
            ir_root,
            resolved_imports: HashMap::new(),
        }
    }

    pub(crate) fn add_resolved_import(&mut self, node_id: NodeId, target_file_id: FileId) {
        self.resolved_imports.insert(node_id, target_file_id);
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl SemanticFile for File {
    fn file_id(&self) -> FileId {
        self.file_id
    }

    fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    fn resolved_import_by_node_id(&self, node_id: NodeId) -> Option<FileId> {
        self.resolved_imports.get(&node_id).copied()
    }
}
