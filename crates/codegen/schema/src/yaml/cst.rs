use std::rc::Rc;

use codegen_utils::errors::Range;
use indexmap::IndexMap;

pub type NodeRef = Rc<Node>;

#[derive(Debug)]
pub enum Node {
    Value {
        range: Range,
    },
    Array {
        range: Range,
        items: Vec<NodeRef>,
    },
    Object {
        range: Range,
        fields: IndexMap<String, NodeFieldRef>,
    },
}

pub type NodeFieldRef = Rc<NodeField>;

#[derive(Debug)]
pub struct NodeField {
    pub key: NodeRef,
    pub value: NodeRef,
}

impl Node {
    pub fn range(&self) -> &Range {
        return match self {
            Node::Value { range, .. } | Node::Array { range, .. } | Node::Object { range, .. } => {
                range
            }
        };
    }

    pub fn index(&self, index: usize) -> &NodeRef {
        return match self {
            Node::Array { items, .. } => items
                .get(index)
                .expect(&format!("Expected array to have index '{index}'.")),
            _ => unreachable!("Expected an array."),
        };
    }

    pub fn field(&self, field: &str) -> &NodeRef {
        return match self {
            Node::Object { fields, .. } => {
                &fields
                    .get(field)
                    .expect(&format!("Expected object to have field '{field}'."))
                    .value
            }
            _ => unreachable!("Expected an object."),
        };
    }
}
