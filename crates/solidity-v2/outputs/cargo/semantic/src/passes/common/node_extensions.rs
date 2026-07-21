use std::ops::Range;

use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_ir::ir::{NodeIdentity, TextRange};

use crate::context::FileNodeMapper;

/// Returns the source location of a node: the file it belongs to and the
/// text range within it.
pub(crate) fn node_location(
    node: &(impl NodeIdentity + TextRange),
    file_node_mapper: &FileNodeMapper,
) -> (FileId, Range<usize>) {
    let file_id = file_node_mapper
        .file_id_from_node_id(node.node_id().expect("node has an id"))
        .to_owned();
    let range = node.calculate_text_range().expect("node has a text range");
    (file_id, range)
}
