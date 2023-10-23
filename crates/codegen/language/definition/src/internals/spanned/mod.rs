use proc_macro2::Span;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    span: Span,
    value: T,
}

impl<T> Spanned<T> {
    pub fn new(span: Span, value: T) -> Self {
        return Self { span, value };
    }

    pub fn span(&self) -> Span {
        return self.span;
    }
}

impl<T> std::ops::Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

impl<T> std::fmt::Display for Spanned<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.value.fmt(f);
    }
}

impl<T: std::cmp::Eq> std::cmp::Eq for Spanned<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        return self.value.hash(state);
    }
}

impl<T: std::cmp::Ord> std::cmp::Ord for Spanned<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.value.cmp(&other.value);
    }
}

impl<T: std::cmp::PartialEq> std::cmp::PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd for Spanned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.value.partial_cmp(&other.value);
    }
}

impl<T: Serialize> Serialize for Spanned<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return self.value.serialize(serializer);
    }
}
