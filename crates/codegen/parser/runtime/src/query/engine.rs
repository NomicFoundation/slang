use std::collections::HashMap;
use std::rc::Rc;

// This crate is copied to another crate, so all imports should be relative
use super::super::cst;
use super::super::cursor::Cursor;
use super::model::{
    AlternativesMatcher, BindingMatcher, Kind, Matcher, NodeMatcher, NodeSelector,
    OneOrMoreMatcher, OptionalMatcher, Query, SequenceMatcher,
};

impl Cursor {
    pub fn query(self, queries: Vec<Query>) -> QueryResultIterator {
        QueryResultIterator::new(self, queries)
    }

    fn irrevocably_go_to_next_sibling(&mut self) -> bool {
        if self.is_completed() {
            false
        } else {
            if !self.go_to_next_sibling() {
                self.complete();
            }
            true
        }
    }

    fn matches_node_selector(&self, node_selector: &NodeSelector) -> bool {
        match self.node() {
            | cst::Node::Rule(rule) => match node_selector {
                | NodeSelector::Anonymous => true,
                | NodeSelector::Kind { kind } => Kind::Rule(rule.kind) == *kind,
                | NodeSelector::Text { .. } => false,
                | NodeSelector::FieldName { field_name } => Some(*field_name) == self.node_name(),
                | NodeSelector::FieldNameAndKind { field_name, kind } => {
                    Some(*field_name) == self.node_name() && Kind::Rule(rule.kind) == *kind
                }
                | NodeSelector::FieldNameAndText { .. } => false,
            },

            | cst::Node::Token(token) => match node_selector {
                | NodeSelector::Anonymous => true,
                | NodeSelector::Kind { kind } => Kind::Token(token.kind) == *kind,
                | NodeSelector::Text { text } => token.text == *text,
                | NodeSelector::FieldName { field_name } => Some(*field_name) == self.node_name(),
                | NodeSelector::FieldNameAndKind { field_name, kind } => {
                    Some(*field_name) == self.node_name() && Kind::Token(token.kind) == *kind
                }
                | NodeSelector::FieldNameAndText { field_name, text } => {
                    Some(*field_name) == self.node_name() && token.text == *text
                }
            },
        }
    }
}

impl Matcher {
    // This allows for queries to pre-flight against a cursor without allocating
    fn can_match(&self, cursor: &Cursor) -> bool {
        match self {
            | Self::Binding(matcher) => matcher.child.can_match(cursor),
            | Self::Node(matcher) => cursor.matches_node_selector(&matcher.node_selector),
            | Self::Alternatives(matcher) => matcher.children.iter().any(|c| c.can_match(cursor)),
            | Self::Sequence(matcher) => matcher.children[0].can_match(cursor),
            | Self::OneOrMore(matcher) => matcher.child.can_match(cursor),
            | Self::Optional(_) => true,
            | Self::Ellipsis => true,
        }
    }

    fn create_combinator(&self, cursor: Cursor) -> CombinatorRef {
        match self {
            | Self::Binding(matcher) => Box::new(BindingCombinator::new(matcher.clone(), cursor)),
            | Self::Node(matcher) => Box::new(NodeCombinator::new(matcher.clone(), cursor)),
            | Self::Sequence(matcher) => Box::new(SequenceCombinator::new(matcher.clone(), cursor)),
            | Self::Alternatives(matcher) => {
                Box::new(AlternativesCombinator::new(matcher.clone(), cursor))
            }
            | Self::Optional(matcher) => Box::new(OptionalCombinator::new(matcher.clone(), cursor)),
            | Self::OneOrMore(matcher) => {
                Box::new(OneOrMoreCombinator::new(matcher.clone(), cursor))
            }
            | Self::Ellipsis => Box::new(EllipsisCombinator::new(cursor)),
        }
    }
}

pub struct QueryResult {
    pub query_number: usize,
    pub bindings: HashMap<String, Vec<Cursor>>,
}

pub struct QueryResultIterator {
    cursor: Cursor,
    queries: Vec<Query>,
    query_number: usize,
    combinator: Option<CombinatorRef>,
}

impl QueryResultIterator {
    fn new(cursor: Cursor, queries: Vec<Query>) -> Self {
        Self {
            cursor,
            queries,
            query_number: 0,
            combinator: None,
        }
    }

    fn advance_to_next_possible_matching_query(&mut self) {
        while !self.cursor.is_completed() {
            while self.query_number < self.queries.len() {
                let matcher = &self.queries[self.query_number].0;
                if matcher.can_match(&self.cursor) {
                    self.combinator = Some(matcher.create_combinator(self.cursor.clone()));
                    return;
                };
                self.query_number += 1;
            }
            self.cursor.go_to_next();
            self.query_number = 0;
        }
    }
}

impl Iterator for QueryResultIterator {
    type Item = QueryResult;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.cursor.is_completed() {
            if let Some(combinator) = self.combinator.as_mut() {
                if combinator.next().is_some() {
                    let mut bindings = HashMap::new();
                    combinator.accumulate_bindings(&mut bindings);
                    return Some(QueryResult {
                        query_number: self.query_number,
                        bindings,
                    });
                }
                self.query_number += 1;
            }

            self.advance_to_next_possible_matching_query();
        }

        None
    }
}

trait Combinator {
    // None -> failed to match, you must backtrack. DO NOT call again
    // Some(cursor) if cursor.is_complete -> matched, end of input
    // Some(cursor) if !cursor.is_complete -> matched, more input to go
    fn next(&mut self) -> Option<Cursor>;
    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>);
}
type CombinatorRef = Box<dyn Combinator>;

struct BindingCombinator {
    matcher: Rc<BindingMatcher>,
    cursor: Cursor,
    child: CombinatorRef,
}

impl BindingCombinator {
    fn new(matcher: Rc<BindingMatcher>, cursor: Cursor) -> Self {
        let child = matcher.child.create_combinator(cursor.clone());
        Self {
            matcher,
            cursor,
            child,
        }
    }
}

impl Combinator for BindingCombinator {
    fn next(&mut self) -> Option<Cursor> {
        self.child.next()
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>) {
        bindings
            .entry(self.matcher.name.clone())
            .or_default()
            .push(self.cursor.clone());
    }
}

struct NodeCombinator {
    matcher: Rc<NodeMatcher>,
    child: Option<CombinatorRef>,
    cursor: Cursor,
    is_initialised: bool,
}

impl NodeCombinator {
    fn new(matcher: Rc<NodeMatcher>, cursor: Cursor) -> Self {
        Self {
            matcher,
            child: None,
            cursor,
            is_initialised: false,
        }
    }
}

impl Combinator for NodeCombinator {
    fn next(&mut self) -> Option<Cursor> {
        if self.cursor.is_completed() {
            return None;
        }

        if !self.is_initialised {
            self.is_initialised = true;

            if !self
                .cursor
                .matches_node_selector(&self.matcher.node_selector)
            {
                return None;
            }

            if let Some(child) = self.matcher.child.as_ref() {
                let mut child_cursor = self.cursor.clone();
                if !child_cursor.go_to_first_child() {
                    return None;
                }

                self.child = Some(child.create_combinator(child_cursor));
            } else {
                let mut return_cursor = self.cursor.clone();
                return_cursor.irrevocably_go_to_next_sibling();
                return Some(return_cursor);
            }
        }

        if let Some(child) = self.child.as_mut() {
            while let Some(cursor) = child.as_mut().next() {
                if cursor.is_completed() {
                    let mut return_cursor = self.cursor.clone();
                    return_cursor.irrevocably_go_to_next_sibling();
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
    matcher: Rc<SequenceMatcher>,
    children: Vec<CombinatorRef>,
    cursor: Cursor,
    is_initialised: bool,
}

impl SequenceCombinator {
    fn new(matcher: Rc<SequenceMatcher>, cursor: Cursor) -> Self {
        Self {
            matcher,
            children: vec![],
            cursor,
            is_initialised: false,
        }
    }
}

impl Combinator for SequenceCombinator {
    fn next(&mut self) -> Option<Cursor> {
        if !self.is_initialised {
            self.is_initialised = true;

            let child_cursor = self.cursor.clone();
            let child = self.matcher.children[0].create_combinator(child_cursor);
            self.children.push(child);
        }

        while !self.children.is_empty() {
            if let Some(child_cursor) = self.children.last_mut().unwrap().next() {
                if self.children.len() == self.matcher.children.len() {
                    return Some(child_cursor);
                }

                let child =
                    self.matcher.children[self.children.len()].create_combinator(child_cursor);
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
    matcher: Rc<AlternativesMatcher>,
    next_child_number: usize,
    child: Option<CombinatorRef>,
    cursor: Cursor,
}

impl AlternativesCombinator {
    fn new(matcher: Rc<AlternativesMatcher>, cursor: Cursor) -> Self {
        Self {
            matcher,
            next_child_number: 0,
            child: None,
            cursor,
        }
    }
}

impl Combinator for AlternativesCombinator {
    fn next(&mut self) -> Option<Cursor> {
        loop {
            if self.child.is_none() {
                match self.matcher.children.get(self.next_child_number) {
                    | Some(child) => {
                        let child = child.create_combinator(self.cursor.clone());
                        self.child = Some(child);
                        self.next_child_number += 1;
                    }
                    | None => return None,
                }
            }

            match self.child.as_mut().unwrap().next() {
                | Some(cursor) => return Some(cursor),
                | None => self.child = None,
            }
        }
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor>>) {
        self.child.as_ref().unwrap().accumulate_bindings(bindings);
    }
}

struct OptionalCombinator {
    matcher: Rc<OptionalMatcher>,
    child: Option<CombinatorRef>,
    cursor: Cursor,
    have_nonempty_match: bool,
}

impl OptionalCombinator {
    fn new(matcher: Rc<OptionalMatcher>, cursor: Cursor) -> Self {
        Self {
            matcher,
            child: None,
            cursor,
            have_nonempty_match: false,
        }
    }
}

impl Combinator for OptionalCombinator {
    fn next(&mut self) -> Option<Cursor> {
        if let Some(child) = self.child.as_mut() {
            match child.next() {
                | result @ Some(_) => {
                    self.have_nonempty_match = true;
                    result
                }
                | None => {
                    self.child = None;
                    None
                }
            }
        } else {
            let child_cursor = self.cursor.clone();
            let child = self.matcher.child.create_combinator(child_cursor);
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
    matcher: Rc<OneOrMoreMatcher>,
    children: Vec<CombinatorRef>,
    cursor_for_next_repetition: Option<Cursor>,
}

impl OneOrMoreCombinator {
    fn new(matcher: Rc<OneOrMoreMatcher>, cursor: Cursor) -> Self {
        let cursor_for_next_repetition = Some(cursor);
        Self {
            matcher,
            children: vec![],
            cursor_for_next_repetition,
        }
    }
}

impl Combinator for OneOrMoreCombinator {
    fn next(&mut self) -> Option<Cursor> {
        loop {
            if let Some(cursor_for_next_repetition) = self.cursor_for_next_repetition.take() {
                let next_child = self
                    .matcher
                    .child
                    .create_combinator(cursor_for_next_repetition);
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

impl Combinator for EllipsisCombinator {
    fn next(&mut self) -> Option<Cursor> {
        if !self.has_returned_initial_empty_value {
            self.has_returned_initial_empty_value = true;
            return Some(self.cursor.clone());
        }

        if self.cursor.irrevocably_go_to_next_sibling() {
            return Some(self.cursor.clone());
        }

        None
    }

    fn accumulate_bindings(&self, _bindings: &mut HashMap<String, Vec<Cursor>>) {}
}
