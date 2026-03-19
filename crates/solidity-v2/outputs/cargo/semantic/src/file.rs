use crate::ir;

pub struct File {
    pub source: String,
    pub file_id: String,
    pub source_unit: ir::SourceUnit,
}

impl File {
    pub fn id(&self) -> &str {
        &self.file_id
    }
}

impl ir::Source for File {
    fn text(&self, range: std::ops::Range<usize>) -> &str {
        self.source.text(range)
    }
}
