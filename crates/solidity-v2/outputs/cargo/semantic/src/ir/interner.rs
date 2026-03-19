use indexmap::IndexSet;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Symbol(usize);

pub struct Interner {
    set: IndexSet<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            set: IndexSet::new(),
        }
    }

    pub fn intern(&mut self, text: &str) -> Symbol {
        let (index, _) = self.set.insert_full(text.to_owned());
        Symbol(index)
    }

    pub fn resolve(&self, symbol: Symbol) -> &str {
        self.set
            .get_index(symbol.0)
            .expect("invalid IdentifierSymbol")
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

impl Default for Interner {
    fn default() -> Self {
        Self::new()
    }
}
