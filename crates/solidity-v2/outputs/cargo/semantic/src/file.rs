use std::collections::HashMap;
use std::ops::Range;

use crate::interner::StringId;
use crate::ir;

pub type FileId = StringId;

pub struct File {
    id: FileId,
    source: String,
    source_unit: ir::SourceUnit,
    resolved_imports: HashMap<Range<usize>, FileId>,
}

impl File {
    pub fn id(&self) -> FileId {
        self.id
    }

    pub(crate) fn source_unit(&self) -> &ir::SourceUnit {
        &self.source_unit
    }

    pub(crate) fn resolved_import(&self, range: Range<usize>) -> Option<FileId> {
        self.resolved_imports.get(&range).copied()
    }
}

impl ir::Source for File {
    fn text(&self, range: Range<usize>) -> &str {
        self.source.text(range)
    }
}
