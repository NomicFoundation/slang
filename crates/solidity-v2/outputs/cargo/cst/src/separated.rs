use std::collections::VecDeque;

/// A list of elements separated by separator tokens, stored as two parallel
/// `VecDeque`s.
///
/// Invariant: `separators.len() == elements.len().saturating_sub(1)`.
/// In particular, `elements` is empty iff the list is empty, and otherwise
/// the i-th separator (i ≥ 0) sits between `elements[i]` and `elements[i + 1]`.
///
/// The fields are private so the invariant can only be broken via the API.
/// Callers that need to walk separators alongside elements should use
/// [`SeparatedList::split_first`] or [`SeparatedList::into_split_first`]
/// rather than reaching for the inner collections.
#[derive(Clone, Debug, PartialEq)]
pub struct SeparatedList<E, S> {
    elements: VecDeque<E>,
    separators: VecDeque<S>,
}

// Manual `Default` impl: the derived one would add `E: Default, S: Default`
// bounds we don't want — the empty list doesn't need them.
impl<E, S> Default for SeparatedList<E, S> {
    fn default() -> Self {
        Self {
            elements: VecDeque::new(),
            separators: VecDeque::new(),
        }
    }
}

impl<E, S> SeparatedList<E, S> {
    /// Creates a list with a single element and no separators.
    pub fn single(first: E) -> Self {
        let mut elements = VecDeque::new();
        elements.push_back(first);
        Self {
            elements,
            separators: VecDeque::new(),
        }
    }

    /// Push an element and separator to the back of the list.
    ///
    /// `self` must not be empty.
    pub fn push(&mut self, separator: S, element: E) {
        debug_assert!(
            !self.elements.is_empty(),
            "push called on empty SeparatedList"
        );
        self.separators.push_back(separator);
        self.elements.push_back(element);
    }

    /// Add an element and separator to the front of the list.
    /// The separator sits between the new element and the old first element.
    ///
    /// `self` must not be empty.
    pub fn push_front(&mut self, element: E, separator: S) {
        debug_assert!(
            !self.elements.is_empty(),
            "push_front called on empty SeparatedList"
        );
        self.separators.push_front(separator);
        self.elements.push_front(element);
    }

    /// Concatenate another separated list to this one, using the provided separator.
    ///
    /// Neither of the lists can be empty, since we have a separator.
    pub fn extend(&mut self, separator: S, other: Self) {
        debug_assert!(
            !self.elements.is_empty() && !other.elements.is_empty(),
            "extend called on empty SeparatedList"
        );
        let Self {
            elements: other_elements,
            separators: other_separators,
        } = other;
        self.separators.push_back(separator);
        self.elements.extend(other_elements);
        self.separators.extend(other_separators);
    }

    /// Iterate over all elements (skipping separators).
    pub fn elements(
        &self,
    ) -> impl DoubleEndedIterator<Item = &E> + ExactSizeIterator<Item = &E> + '_ {
        self.elements.iter()
    }

    /// Returns the first element together with an iterator over the trailing
    /// `(separator, element)` pairs, or `None` if the list is empty.
    ///
    /// Use this when you need to walk both elements and separators without
    /// touching the parallel-collection layout directly.
    pub fn split_first(&self) -> Option<(&E, impl Iterator<Item = (&S, &E)> + '_)> {
        let mut elements = self.elements.iter();
        let first = elements.next()?;
        Some((first, self.separators.iter().zip(elements)))
    }

    /// Owning version of [`Self::split_first`], for code that consumes the list.
    pub fn into_split_first(self) -> Option<(E, impl Iterator<Item = (S, E)>)> {
        let mut elements = self.elements.into_iter();
        let first = elements.next()?;
        Some((first, self.separators.into_iter().zip(elements)))
    }

    /// Number of elements in the list.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Whether the list has no elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
