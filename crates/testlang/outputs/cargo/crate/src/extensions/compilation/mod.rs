use crate::cst::Cursor;

pub struct ImportPathsExtractor;

impl ImportPathsExtractor {
    pub fn new() -> Self {
        Self
    }

    #[allow(clippy::unused_self)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn extract(&self, _: Cursor) -> Vec<Cursor> {
        unreachable!("Import paths are Solidity-specific")
    }
}
