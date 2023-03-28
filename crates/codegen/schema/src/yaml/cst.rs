use std::{ops::Deref, rc::Rc};

use codegen_utils::errors::{Position, Range};
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
    pub fn value_at_index(&self, index: usize) -> NodeRef {
        return match self {
            Node::Value { .. } | Node::Object { .. } => {
                panic!("Cannot get indexed element from a value or object node")
            }
            Node::Array { items, .. } => items
                .get(index)
                .cloned()
                .expect(&format!("Array does not have element at index {}", index)),
        };
    }

    pub fn value_of_field(&self, key: &str) -> NodeRef {
        return match self {
            Node::Value { .. } | Node::Array { .. } => {
                panic!("Cannot get key of field from a value or array node")
            }

            Node::Object { fields, .. } => fields
                .get(key)
                .map(|field| field.value.clone())
                .expect(&format!("Object does not have field {}", key)),
        };
    }

    pub fn key_of_field(&self, key: &str) -> NodeRef {
        return match self {
            Node::Value { .. } | Node::Array { .. } => {
                panic!("Cannot get key of field from a value or array node")
            }

            Node::Object { fields, .. } => fields
                .get(key)
                .map(|field| field.key.clone())
                .expect(&format!("Object does not have field {}", key)),
        };
    }

    pub fn range(&self) -> &Range {
        return match self {
            Node::Value { range, .. } | Node::Array { range, .. } | Node::Object { range, .. } => {
                range
            }
        };
    }

    pub fn pinpoint<'a>(&self, position: &Position) -> Option<&Node> {
        if !position.inside(&self.range()) {
            return None;
        }

        return match self {
            Node::Value { .. } => None,
            Node::Array { items, .. } => items.iter().find_map(|item| item.pinpoint(position)),
            Node::Object { fields, .. } => fields.values().find_map(|field| {
                field
                    .key
                    .pinpoint(position)
                    .or_else(|| field.value.pinpoint(position))
            }),
        }
        .or_else(|| Some(self));
    }
}

impl Deref for NodeField {
    type Target = NodeRef;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
