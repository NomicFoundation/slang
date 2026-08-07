use crate::ir;

impl ir::CatchClauseStruct {
    pub fn parameters(&self) -> Option<&ir::Parameters> {
        match &self.kind {
            ir::CatchClauseKind::ClauseErrorKind(clause_error_kind) => {
                Some(&clause_error_kind.parameters)
            }
            ir::CatchClauseKind::ClausePanicKind(clause_panic_kind) => {
                Some(&clause_panic_kind.parameters)
            }
            ir::CatchClauseKind::ClauseLowLevelKind(clause_low_level_kind) => {
                clause_low_level_kind.parameters.as_ref()
            }
        }
    }
}
