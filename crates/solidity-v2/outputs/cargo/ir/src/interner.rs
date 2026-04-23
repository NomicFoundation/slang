use indexmap::IndexSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StringId(usize);

#[derive(Debug, Default)]
pub struct Interner {
    strings: IndexSet<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, text: &str) -> StringId {
        let (index, _) = self.strings.insert_full(text.to_owned());
        StringId(index)
    }

    pub fn resolve(&self, id: StringId) -> &str {
        &self.strings[id.0]
    }
}
