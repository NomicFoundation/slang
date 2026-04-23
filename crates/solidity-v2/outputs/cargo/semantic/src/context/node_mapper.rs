use slang_solidity_v2_common::nodes::NodeId;

use super::SemanticFile;

struct FileNodeInfo {
    /// File ID to map `NodeId`s to
    file_id: String,
    /// `NodeId` of the first node in the file, which should always correspond
    /// to the ID of the root `SourceUnit` (by construction)
    first_node_id: NodeId,
}

/// An ordered collection of file IDs + first `NodeId` of the files a
/// `SemanticContext` was built from. The order is ascending `first_node_id`
/// which allows us to efficiently determine which file a node belongs to. This
/// is by construction of the IR trees and the use of a monotonic
/// `NodeIdGenerator`.
pub(super) struct NodeMapper {
    files: Vec<FileNodeInfo>,
}

impl NodeMapper {
    pub(super) fn build_from(files: &[impl SemanticFile]) -> Self {
        let mut files: Vec<FileNodeInfo> = files
            .iter()
            .map(|file| {
                let file_id = file.id().to_string();
                let first_node_id = file.ir_root().id();
                FileNodeInfo {
                    file_id,
                    first_node_id,
                }
            })
            .collect();
        files.sort_by_key(|file| file.first_node_id);
        Self { files }
    }

    pub(super) fn file_id_from_node_id(&self, node_id: NodeId) -> String {
        let index = match self
            .files
            .binary_search_by_key(&node_id, |file| file.first_node_id)
        {
            Ok(index) => index,
            Err(index) => {
                // NB. this cannot possibly underflow because there should be no
                // valid node before the `SourceUnit` of the first file
                index - 1
            }
        };
        self.files[index].file_id.clone()
    }
}
