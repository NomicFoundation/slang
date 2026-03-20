use std::collections::HashMap;
use std::ops::Range;

use crate::ir;

pub struct File {
    source: String,
    id: String,
    source_unit: ir::SourceUnit,
    resolved_imports: HashMap<Range<usize>, String>,
}

impl File {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn source_unit(&self) -> &ir::SourceUnit {
        &self.source_unit
    }

    pub(crate) fn resolved_import(&self, range: Range<usize>) -> Option<String> {
        self.resolved_imports.get(&range).cloned()
    }
}

impl ir::Source for File {
    fn text(&self, range: Range<usize>) -> &str {
        self.source.text(range)
    }
}
