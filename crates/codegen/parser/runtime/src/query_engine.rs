use std::collections::HashMap;
use std::rc::Rc;

use super::cst;
use super::cursor::Cursor;
use super::query_model::{
    AlternativesMatch, BindingMatch, Kind, NodeId, NodeMatch, OneOrMoreMatch, OptionalMatch, Query,
    SequenceMatch,
};

impl Cursor {
    pub fn query(self, queries: Vec<Query>) -> QueryIterator {
        QueryIterator::new(self, queries)
    }

    fn consume(&mut self) -> bool {
        if self.is_completed() {
            false
        } else {
            if !self.go_to_next_sibling() {
                self.complete();
            }
            true
        }
    }

    fn matches_query_node_id(&self, node_id: &NodeId) -> bool {
        match self.node() {
            cst::Node::Rule(rule) => match node_id {
                NodeId::Anonymous => true,
                NodeId::Kind { kind } => Kind::Rule(rule.kind) == *kind,
                NodeId::String { .. } => false,
                NodeId::Field { field } => Some(*field) == self.node_name(),
                NodeId::FieldAndKind { field, kind } => {
                    Some(*field) == self.node_name() && Kind::Rule(rule.kind) == *kind
                }
                NodeId::FieldAndString { .. } => false,
            },

            cst::Node::Token(token) => match node_id {
                NodeId::Anonymous => true,
                NodeId::Kind { kind } => Kind::Token(token.kind) == *kind,
                NodeId::String { string } => token.text == *string,
                NodeId::Field { field } => Some(*field) == self.node_name(),
                NodeId::FieldAndKind { field, kind } => {
                    Some(*field) == self.node_name() && Kind::Token(token.kind) == *kind
                }
                NodeId::FieldAndString { field, string } => {
                    Some(*field) == self.node_name() && token.text == *string
                }
            },
        }
    }
}

impl Query {
    fn create_qecref(&self, cursor: Cursor) -> QECRef {
        match self {
            Query::Binding(r#match) => Box::new(BindingCombinator::new(r#match.clone(), cursor)),
            Query::Node(r#match) => Box::new(NodeCombinator::new(r#match.clone(), cursor)),
            Query::Sequence(r#match) => Box::new(SequenceCombinator::new(r#match.clone(), cursor)),
            Query::Alternatives(r#match) => {
                Box::new(AlternativesCombinator::new(r#match.clone(), cursor))
            }
            Query::Optional(r#match) => Box::new(OptionalCombinator::new(r#match.clone(), cursor)),
            Query::OneOrMore(r#match) => {
                Box::new(OneOrMoreCombinator::new(r#match.clone(), cursor))
            }
            Query::Ellipsis => Box::new(EllipsisCombinator::new(cursor)),
        }
    }
}

pub struct QueryIterator {
    cursor: Cursor,
    queries: Vec<Query>,
    query_number: usize,
    combinator: QECRef,
}

impl QueryIterator {
    pub fn new(cursor: Cursor, queries: Vec<Query>) -> Self {
        let query_number = 0;
        let combinator = queries[query_number].create_qecref(cursor.clone());
        Self {
            cursor,
            queries,
            query_number,
            combinator,
        }
    }
}

impl Iterator for QueryIterator {
    type Item = (usize, HashMap<String, Vec<Cursor>>);

    fn next(&mut self) -> Option<Self::Item> {
        while self.query_number < self.queries.len() {
            if self.combinator.next().is_some() {
                let mut bindings = HashMap::new();
                self.combinator.accumulate_bindings(&mut bindings);
                return Some((self.query_number, bindings));
            }

            self.query_number += 1;
            if self.query_number < self.queries.len() {
                self.combinator =
                    self.queries[self.query_number].create_qecref(self.cursor.clone());
            }
        }

        None
    }
}

trait QueryEngineCombinator {
    // None -> failed to match, you must backtrack. DO NOT call again
    // Some(cursor) if cursor.is_complete -> matched, end of input
    // Some(cursor) if !cursor.is_complete -> matched, more input to go
    fn next(&mut self) -> Option<Cursor>;
    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>);
}
type QECRef = Box<dyn QueryEngineCombinator>;

struct BindingCombinator {
    r#match: Rc<BindingMatch>,
    cursor: Cursor,
    child: QECRef,
}

impl BindingCombinator {
    fn new(r#match: Rc<BindingMatch>, cursor: Cursor) -> Self {
        let child = r#match.child.create_qecref(cursor.clone());
        Self {
            r#match,
            cursor,
            child,
        }
    }
}

impl QueryEngineCombinator for BindingCombinator {
    fn next(&mut self) -> Option<Cursor> {
        self.child.next()
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>) {
        bindings
            .entry(self.r#match.name.clone())
            .or_default()
            .push(self.cursor.clone());
    }
}

struct NodeCombinator {
    r#match: Rc<NodeMatch>,
    child: Option<QECRef>,
    cursor: Cursor,
    is_initialised: bool,
}

impl NodeCombinator {
    fn new(r#match: Rc<NodeMatch>, cursor: Cursor) -> Self {
        Self {
            r#match,
            child: None,
            cursor,
            is_initialised: false,
        }
    }
}

impl QueryEngineCombinator for NodeCombinator {
    fn next(&mut self) -> Option<Cursor> {
        if self.cursor.is_completed() {
            return None;
        }

        if !self.is_initialised {
            self.is_initialised = true;

            if !self.cursor.matches_query_node_id(&self.r#match.id) {
                return None;
            }

            if let Some(child) = self.r#match.child.as_ref() {
                let mut child_cursor = self.cursor.clone();
                if !child_cursor.go_to_first_child() {
                    return None;
                }

                self.child = Some(child.create_qecref(child_cursor));
            } else {
                let mut return_cursor = self.cursor.clone();
                return_cursor.consume();
                return Some(return_cursor);
            }
        }

        if let Some(child) = self.child.as_mut() {
            while let Some(cursor) = child.as_mut().next() {
                if cursor.is_completed() {
                    let mut return_cursor = self.cursor.clone();
                    return_cursor.consume();
                    return Some(return_cursor);
                }
            }
            self.child = None;
        }

        None
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>) {
        if let Some(child) = self.child.as_ref() {
            child.accumulate_bindings(bindings);
        }
    }
}

struct SequenceCombinator {
    r#match: Rc<SequenceMatch>,
    children: Vec<QECRef>,
    cursor: Cursor,
    is_initialised: bool,
}

impl SequenceCombinator {
    fn new(r#match: Rc<SequenceMatch>, cursor: Cursor) -> Self {
        Self {
            r#match,
            children: vec![],
            cursor,
            is_initialised: false,
        }
    }
}

impl QueryEngineCombinator for SequenceCombinator {
    fn next(&mut self) -> Option<Cursor> {
        if !self.is_initialised {
            self.is_initialised = true;

            let child_cursor = self.cursor.clone();
            let child = self.r#match.children[0].create_qecref(child_cursor);
            self.children.push(child);
        }

        while !self.children.is_empty() {
            if let Some(child_cursor) = self.children.last_mut().unwrap().next() {
                if self.children.len() == self.r#match.children.len() {
                    return Some(child_cursor);
                }

                let child = self.r#match.children[self.children.len()].create_qecref(child_cursor);
                self.children.push(child);
            } else {
                self.children.pop();
            }
        }

        None
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>) {
        for child in &self.children {
            child.accumulate_bindings(bindings);
        }
    }
}

struct AlternativesCombinator {
    r#match: Rc<AlternativesMatch>,
    next_child_number: usize,
    child: Option<QECRef>,
    cursor: Cursor,
}

impl AlternativesCombinator {
    fn new(r#match: Rc<AlternativesMatch>, cursor: Cursor) -> Self {
        Self {
            r#match,
            next_child_number: 0,
            child: None,
            cursor,
        }
    }
}

impl QueryEngineCombinator for AlternativesCombinator {
    fn next(&mut self) -> Option<Cursor> {
        loop {
            if self.child.is_none() {
                match self.r#match.children.get(self.next_child_number) {
                    Some(child) => {
                        let child = child.create_qecref(self.cursor.clone());
                        self.child = Some(child);
                        self.next_child_number += 1;
                    }
                    None => return None,
                }
            }

            match self.child.as_mut().unwrap().next() {
                Some(cursor) => return Some(cursor),
                None => self.child = None,
            }
        }
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>) {
        self.child.as_ref().unwrap().accumulate_bindings(bindings);
    }
}

struct OptionalCombinator {
    r#match: Rc<OptionalMatch>,
    child: Option<QECRef>,
    cursor: Cursor,
    have_nonempty_match: bool,
}

impl OptionalCombinator {
    fn new(r#match: Rc<OptionalMatch>, cursor: Cursor) -> Self {
        Self {
            r#match,
            child: None,
            cursor,
            have_nonempty_match: false,
        }
    }
}

impl QueryEngineCombinator for OptionalCombinator {
    fn next(&mut self) -> Option<Cursor> {
        if let Some(child) = self.child.as_mut() {
            match child.next() {
                result @ Some(_) => {
                    self.have_nonempty_match = true;
                    result
                }
                None => {
                    self.child = None;
                    None
                }
            }
        } else {
            let child_cursor = self.cursor.clone();
            let child = self.r#match.child.create_qecref(child_cursor);
            self.child = Some(child);
            Some(self.cursor.clone())
        }
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>) {
        if self.have_nonempty_match {
            if let Some(child) = self.child.as_ref() {
                child.accumulate_bindings(bindings);
            }
        }
    }
}

struct OneOrMoreCombinator {
    r#match: Rc<OneOrMoreMatch>,
    children: Vec<QECRef>,
    cursor_for_next_repetition: Option<Cursor>,
}

impl OneOrMoreCombinator {
    fn new(r#match: Rc<OneOrMoreMatch>, cursor: Cursor) -> Self {
        let cursor_for_next_repetition = Some(cursor);
        Self {
            r#match,
            children: vec![],
            cursor_for_next_repetition,
        }
    }
}

impl QueryEngineCombinator for OneOrMoreCombinator {
    fn next(&mut self) -> Option<Cursor> {
        loop {
            if let Some(cursor_for_next_repetition) = self.cursor_for_next_repetition.take() {
                let next_child = self.r#match.child.create_qecref(cursor_for_next_repetition);
                self.children.push(next_child);
            } else {
                let tail = self.children.last_mut().unwrap();
                if let Some(cursor) = tail.next() {
                    if !cursor.is_completed() {
                        self.cursor_for_next_repetition = Some(cursor.clone());
                    }
                    return Some(cursor);
                }
                self.children.pop();
                if self.children.is_empty() {
                    return None;
                }
            }
        }
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>) {
        for child in &self.children {
            child.accumulate_bindings(bindings);
        }
    }
}

struct EllipsisCombinator {
    cursor: Cursor,
    has_returned_initial_empty_value: bool,
}

impl EllipsisCombinator {
    fn new(cursor: Cursor) -> Self {
        Self {
            cursor,
            has_returned_initial_empty_value: false,
        }
    }
}

impl QueryEngineCombinator for EllipsisCombinator {
    fn next(&mut self) -> Option<Cursor> {
        if !self.has_returned_initial_empty_value {
            self.has_returned_initial_empty_value = true;
            return Some(self.cursor.clone());
        }

        if self.cursor.consume() {
            return Some(self.cursor.clone());
        }

        None
    }

    fn accumulate_bindings(&self, _bindings: &mut HashMap<String, Vec<Cursor>>) {}
}
