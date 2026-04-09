use std::collections::VecDeque;

/// A list of elements separated by separator tokens.
///
/// - `Empty` means the list has no elements (used by `allow_empty` grammar rules).
/// - `NonEmpty` guarantees at least one element (`first`), followed by zero or more
///   `(separator, element)` pairs in `rest`.
///
/// The reason for this separation is to make the invariant of
/// having either one element and a possible list
/// of separators and elements, or no elements at all.
#[derive(Clone, Debug, PartialEq)]
pub enum SeparatedList<E, S> {
    Empty,
    NonEmpty { first: E, rest: VecDeque<(S, E)> },
}

impl<E, S> SeparatedList<E, S> {
    pub fn single(first: E) -> Self {
        Self::NonEmpty {
            first,
            rest: VecDeque::new(),
        }
    }

    /// Push an element and separator to the back of the list.
    ///
    /// `self` must not be empty.
    pub fn push(&mut self, separator: S, element: E) {
        match self {
            Self::NonEmpty { rest, .. } => rest.push_back((separator, element)),
            Self::Empty => panic!("push called on empty SeparatedList"),
        }
    }

    /// Add an element and separator to the front of the list.
    /// The separator sits between the new element and the old first element.
    ///
    /// `self` must be not empty.
    pub fn push_front(&mut self, element: E, separator: S) {
        match self {
            Self::NonEmpty { first, rest } => {
                let old_first = std::mem::replace(first, element);
                rest.push_front((separator, old_first));
            }
            Self::Empty => panic!("push_front called on empty SeparatedList"),
        }
    }

    /// Concatenate another separated list to this one, using the provided separator.
    ///
    /// Neither of the lists can be empty, since we have a separator.
    pub fn extend(&mut self, separator: S, other: Self) {
        match (&mut *self, other) {
            (
                Self::NonEmpty { rest, .. },
                Self::NonEmpty {
                    first: other_first,
                    rest: other_rest,
                },
            ) => {
                rest.push_back((separator, other_first));
                rest.extend(other_rest);
            }
            (_, Self::Empty) | (Self::Empty, _) => {
                panic!("extend called on empty SeparatedList")
            }
        }
    }

    /// Iterate over all elements (skipping separators).
    pub fn elements(&self) -> impl DoubleEndedIterator<Item = &E> + '_ {
        let (first, rest) = match self {
            Self::Empty => (None, None),
            Self::NonEmpty { first, rest } => (Some(first), Some(rest)),
        };
        first.into_iter().chain(
            rest.into_iter()
                .flat_map(|rest| rest.iter().map(|(_, element)| element)),
        )
    }

    /// Number of elements in the list.
    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::NonEmpty { rest, .. } => 1 + rest.len(),
        }
    }

    /// Whether the list has no elements.
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}
