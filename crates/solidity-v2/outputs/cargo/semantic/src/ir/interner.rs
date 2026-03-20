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
        if let Some(index) = self.set.get_index_of(text) {
            return Symbol(index);
        }
        let (index, _) = self.set.insert_full(text.to_owned());
        Symbol(index)
    }

    pub fn resolve(&self, symbol: Symbol) -> &str {
        self.set
            .get_index(symbol.0)
            .expect("invalid IdentifierSymbol")
    }
}

impl Default for Interner {
    fn default() -> Self {
        Self::new()
    }
}
