#[path = "node_checker.generated.rs"]
mod node_checker;

#[cfg(feature = "__private_testing_utils")]
pub mod testing;
use slang_solidity::cst::{Cursor, TextRange};
use slang_solidity_v2_common::versions::LanguageVersion;
#[cfg(feature = "__private_testing_utils")]
pub use testing::V2TesterConstructor;

use crate::parser::{Parser, SourceUnitParser};
use crate::temp_testing::node_checker::{NodeChecker, NodeCheckerError};

pub fn compare_with_v1_cursor(source: &str, root_cursor: &Cursor) -> Vec<NodeCheckerError> {
    let v2_output = SourceUnitParser::parse(source, LanguageVersion::V0_8_30);

    match v2_output {
        Ok(v2_tree) => v2_tree.check_node(&root_cursor.node()),
        Err(error_message) => vec![NodeCheckerError::new(error_message, TextRange::default())],
    }
}
