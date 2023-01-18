use std::rc::Rc;

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
    pub fn range<'a>(&'a self) -> &'a Range {
        return match self {
            Node::Value { range, .. } | Node::Array { range, .. } | Node::Object { range, .. } => {
                range
            }
        };
    }

    pub fn pinpoint<'a>(&'a self, position: &Position) -> Option<&'a Node> {
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
