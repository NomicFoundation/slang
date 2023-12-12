pub type BindingName = Option<String>;

pub struct Query {
    pub binding: BindingName,
    pub children: Vec<Node>,
}

pub struct Node {
    pub id: NodeId,
    pub children: Vec<NodeChild>,
}

pub enum NodeId {
    Anonymous,
    Kind { kind: String },
    String { string: String },
    Field { field: String },
    FieldAndKind { field: String, kind: String },
    FieldAndString { field: String, string: String },
}

pub enum NodeChild {
    Node {
        binding: BindingName,
        node: Node,
    },
    Group {
        binding: BindingName,
        children: Vec<NodeChild>,
        quantifier: Quantifier,
    },
    Ellipsis {
        binding: BindingName,
    },
}

#[derive(Clone, Copy)]
pub enum Quantifier {
    OneOf,      // i.e. choice or unquantified if children.len() == 1
    ZeroOrOne,  // i.e. ?
    ZeroOrMore, // i.e. *
    OneOrMore,  // i.e. +
}
