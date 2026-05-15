#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NodeId(usize);

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl serde::Serialize for NodeId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0 as u64)
    }
}
