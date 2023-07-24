use crate::legacy::char_set::CharSet;

pub struct CharFirstSet {
    pub includes_epsilon: bool,
    pub char_set: CharSet,
}

impl CharFirstSet {
    pub fn new() -> Self {
        Self {
            includes_epsilon: false,
            char_set: CharSet::empty(),
        }
    }

    pub fn from_char_set(char_set: CharSet) -> Self {
        Self {
            includes_epsilon: false,
            char_set,
        }
    }

    pub fn multiple<T: Iterator<Item = char>>(x: T) -> Self {
        Self {
            includes_epsilon: false,
            char_set: x.fold(CharSet::empty(), |acc, c| acc.union(CharSet::single(c))),
        }
    }

    pub fn epsilon() -> Self {
        Self {
            includes_epsilon: true,
            char_set: CharSet::empty(),
        }
    }

    pub fn with_epsilon(mut self) -> Self {
        self.includes_epsilon = true;
        self
    }

    pub fn follow_by(mut self, other: Self) -> Self {
        if self.includes_epsilon {
            self.includes_epsilon = other.includes_epsilon;
            self.char_set = self.char_set.union(other.char_set);
        }
        self
    }

    pub fn union_with(mut self, other: Self) -> Self {
        self.includes_epsilon |= other.includes_epsilon;
        self.char_set = self.char_set.union(other.char_set);
        self
    }
}
