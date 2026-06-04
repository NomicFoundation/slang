use std::collections::HashMap;
use std::sync::Arc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::{SemanticContext, SemanticFile};

use crate::ast;

pub(crate) struct InternalFile {
    // TODO(v2): abstract this into a `FileId` type
    id: String,
    ir_root: ir::SourceUnit,
    resolved_imports: HashMap<NodeId, String>,
}

impl InternalFile {
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

impl SemanticFile for InternalFile {
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

pub struct FileStruct {
    file: Arc<InternalFile>,
    semantic: Arc<SemanticContext>,
}

pub type File = Arc<FileStruct>;

impl FileStruct {
    pub(crate) fn create(file: &Arc<InternalFile>, semantic: &Arc<SemanticContext>) -> File {
        Arc::new(Self {
            file: Arc::clone(file),
            semantic: Arc::clone(semantic),
        })
    }

    pub fn id(&self) -> &str {
        &self.file.id
    }

    pub fn ast(&self) -> ast::SourceUnit {
        ast::create_source_unit(&self.file.ir_root, &self.semantic)
    }
}
