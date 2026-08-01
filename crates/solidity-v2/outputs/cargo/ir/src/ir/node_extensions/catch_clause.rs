use slang_solidity_v2_common::catch_clauses::CatchClauseKind;

use crate::ir;

impl ir::CatchClauseStruct {
    /// The kind the clause's selector name declares, or `None` for a name that
    /// is neither `Error` nor `Panic`. Whether `Panic` is available at the
    /// language version in use is a validity question, decided by the caller.
    pub fn kind(&self) -> Option<CatchClauseKind> {
        let Some(name) = self.error.as_ref().and_then(|error| error.name.as_ref()) else {
            return Some(CatchClauseKind::LowLevel);
        };
        match name.text.as_str() {
            "Error" => Some(CatchClauseKind::Error),
            "Panic" => Some(CatchClauseKind::Panic),
            _ => None,
        }
    }
}
