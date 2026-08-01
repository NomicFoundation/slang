use slang_solidity_v2_common::catch_clauses::CatchClauseKind;

use super::super::nodes::CatchClauseStruct;

impl CatchClauseStruct {
    /// The kind the clause's selector name declares, or `None` for a name that
    /// is neither `Error` nor `Panic`, which the structure pass rejects.
    pub fn kind(&self) -> Option<CatchClauseKind> {
        self.ir_node.kind()
    }
}
