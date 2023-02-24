pub struct Node<T> {
    pub cst_node: crate::yaml::cst::NodeRef,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(cst_node: &crate::yaml::cst::NodeRef, value: T) -> Self {
        return Self {
            cst_node: cst_node.to_owned(),
            value,
        };
    }
}

impl<T> std::hash::Hash for Node<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let range = self.cst_node.range();

        range.start.offset.hash(state);
        range.end.offset.hash(state);
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_range = self.cst_node.range();
        let other_range = other.cst_node.range();

        return self_range.start.offset == other_range.start.offset
            && self_range.end.offset == other_range.end.offset;
    }
}

impl<T> Eq for Node<T> {}
