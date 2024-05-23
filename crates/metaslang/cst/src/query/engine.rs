use std::collections::BTreeMap;
use std::rc::Rc;

use super::super::cst::Node;
use super::super::cursor::Cursor;
use super::model::{
    ASTNode, AlternativesASTNode, CaptureASTNode, NodeMatchASTNode, NodeSelector, OneOrMoreASTNode,
    OptionalASTNode, Query, SequenceASTNode,
};
use crate::cst::NodeKind;
use crate::query::CaptureQuantifier;
use crate::KindTypes;

impl<T: KindTypes + 'static> Cursor<T> {
    pub fn query(self, queries: Vec<Query<T>>) -> QueryMatchIterator<T> {
        QueryMatchIterator::new(self, queries)
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

    fn matches_node_selector(&self, node_selector: &NodeSelector<T>) -> bool {
        match self.node() {
            Node::<T>::NonTerminal(nonterminal) => match node_selector {
                NodeSelector::Anonymous => true,
                NodeSelector::NodeKind { node_kind } => {
                    NodeKind::NonTerminal(nonterminal.kind) == *node_kind
                }
                NodeSelector::NodeText { .. } => false,
                NodeSelector::EdgeLabel { edge_label } => Some(*edge_label) == self.label(),
                NodeSelector::EdgeLabelAndNodeKind {
                    edge_label,
                    node_kind,
                } => {
                    Some(*edge_label) == self.label()
                        && NodeKind::NonTerminal(nonterminal.kind) == *node_kind
                }
                NodeSelector::EdgeLabelAndNodeText { .. } => false,
            },

            Node::<T>::Terminal(terminal) => match node_selector {
                NodeSelector::Anonymous => true,
                NodeSelector::NodeKind { node_kind: kind } => {
                    NodeKind::Terminal(terminal.kind) == *kind
                }
                NodeSelector::NodeText { node_text } => terminal.text == *node_text,
                NodeSelector::EdgeLabel { edge_label } => Some(*edge_label) == self.label(),
                NodeSelector::EdgeLabelAndNodeKind {
                    edge_label,
                    node_kind,
                } => {
                    Some(*edge_label) == self.label()
                        && NodeKind::Terminal(terminal.kind) == *node_kind
                }
                NodeSelector::EdgeLabelAndNodeText {
                    edge_label,
                    node_text,
                } => Some(*edge_label) == self.label() && terminal.text == *node_text,
            },
        }
    }
}

impl<T: KindTypes + 'static> ASTNode<T> {
    // This allows for queries to pre-flight against a cursor without allocating
    fn can_match(&self, cursor: &Cursor<T>) -> bool {
        match self {
            Self::Binding(matcher) => matcher.child.can_match(cursor),
            Self::NodeMatch(matcher) => cursor.matches_node_selector(&matcher.node_selector),
            Self::Alternatives(matcher) => matcher.children.iter().any(|c| c.can_match(cursor)),
            Self::Sequence(matcher) => matcher.children[0].can_match(cursor),
            Self::OneOrMore(matcher) => matcher.child.can_match(cursor),
            Self::Optional(_) => true,
            Self::Ellipsis => true,
        }
    }

    fn create_matcher(&self, cursor: Cursor<T>) -> MatcherRef<T> {
        match self {
            Self::Binding(matcher) => Box::new(CaptureMatcher::<T>::new(matcher.clone(), cursor)),
            Self::NodeMatch(matcher) => {
                Box::new(NodeMatchMatcher::<T>::new(matcher.clone(), cursor))
            }
            Self::Sequence(matcher) => Box::new(SequenceMatcher::<T>::new(matcher.clone(), cursor)),
            Self::Alternatives(matcher) => {
                Box::new(AlternativesMatcher::<T>::new(matcher.clone(), cursor))
            }
            Self::Optional(matcher) => Box::new(OptionalMatcher::<T>::new(matcher.clone(), cursor)),
            Self::OneOrMore(matcher) => {
                Box::new(OneOrMoreMatcher::<T>::new(matcher.clone(), cursor))
            }
            Self::Ellipsis => Box::new(EllipsisMatcher::<T>::new(cursor)),
        }
    }
}

pub struct QueryMatch<T: KindTypes> {
    pub queries: Rc<Vec<Query<T>>>,
    pub query_number: usize,
    pub root_cursor: Cursor<T>,
    // These correspond to the capture definitions in tne query
    pub captures: BTreeMap<String, Vec<Cursor<T>>>,
}

impl<T: KindTypes> QueryMatch<T> {
    pub fn query(&self) -> &Query<T> {
        &self.queries[self.query_number]
    }

    pub fn capture_names(&self) -> impl Iterator<Item = &String> {
        self.query().capture_quantifiers.keys()
    }

    pub fn captures(
        &self,
    ) -> impl Iterator<Item = (&String, CaptureQuantifier, impl Iterator<Item = Cursor<T>>)> {
        let query = self.query();
        query.capture_quantifiers.iter().map(|(name, quantifier)| {
            let captures = self
                .captures
                .get(name)
                .unwrap_or(&vec![])
                .clone()
                .into_iter();
            (name, *quantifier, captures)
        })
    }

    pub fn capture(
        &self,
        name: &str,
    ) -> Option<(CaptureQuantifier, impl Iterator<Item = Cursor<T>>)> {
        let query = self.query();
        query.capture_quantifiers.get(name).map(|quantifier| {
            let captures = self
                .captures
                .get(name)
                .unwrap_or(&vec![])
                .clone()
                .into_iter();
            (*quantifier, captures)
        })
    }
}

pub struct QueryMatchIterator<T: KindTypes> {
    queries: Rc<Vec<Query<T>>>,
    cursor: Cursor<T>,
    query_number: usize,
    matcher: Option<MatcherRef<T>>,
}

impl<T: KindTypes + 'static> QueryMatchIterator<T> {
    fn new(cursor: Cursor<T>, queries: Vec<Query<T>>) -> Self {
        Self {
            queries: Rc::new(queries),
            cursor,
            query_number: 0,
            matcher: None,
        }
    }

    fn advance_to_next_possible_matching_query(&mut self) {
        while !self.cursor.is_completed() {
            while self.query_number < self.queries.len() {
                let ast_node = &self.queries[self.query_number].ast_node;
                if ast_node.can_match(&self.cursor) {
                    self.matcher = Some(ast_node.create_matcher(self.cursor.clone()));
                    return;
                };
                self.query_number += 1;
            }
            self.cursor.go_to_next();
            self.query_number = 0;
        }
    }
}

impl<T: KindTypes + 'static> Iterator for QueryMatchIterator<T> {
    type Item = QueryMatch<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.cursor.is_completed() {
            if let Some(matcher) = self.matcher.as_mut() {
                if matcher.next().is_some() {
                    let mut bindings = BTreeMap::new();
                    matcher.record_captures(&mut bindings);
                    return Some(QueryMatch {
                        queries: Rc::clone(&self.queries),
                        root_cursor: self.cursor.clone(),
                        query_number: self.query_number,
                        captures: bindings,
                    });
                }
                self.query_number += 1;
            }

            self.advance_to_next_possible_matching_query();
        }

        None
    }
}

trait Matcher<T: KindTypes> {
    // None -> failed to match, you must backtrack. DO NOT call again
    // Some(cursor) if cursor.is_complete -> matched, end of input
    // Some(cursor) if !cursor.is_complete -> matched, more input to go
    fn next(&mut self) -> Option<Cursor<T>>;
    fn record_captures(&self, bindings: &mut BTreeMap<String, Vec<Cursor<T>>>);
}
type MatcherRef<T> = Box<dyn Matcher<T>>;

struct CaptureMatcher<T: KindTypes> {
    matcher: Rc<CaptureASTNode<T>>,
    cursor: Cursor<T>,
    child: MatcherRef<T>,
}

impl<T: KindTypes + 'static> CaptureMatcher<T> {
    fn new(matcher: Rc<CaptureASTNode<T>>, cursor: Cursor<T>) -> Self {
        let child = matcher.child.create_matcher(cursor.clone());
        Self {
            matcher,
            cursor,
            child,
        }
    }
}

impl<T: KindTypes> Matcher<T> for CaptureMatcher<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
        self.child.next()
    }

    fn record_captures(&self, bindings: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        bindings
            .entry(self.matcher.name.clone())
            .or_default()
            .push(self.cursor.clone());
        self.child.record_captures(bindings);
    }
}

struct NodeMatchMatcher<T: KindTypes> {
    matcher: Rc<NodeMatchASTNode<T>>,
    child: Option<MatcherRef<T>>,
    cursor: Cursor<T>,
    is_initialised: bool,
}

impl<T: KindTypes> NodeMatchMatcher<T> {
    fn new(matcher: Rc<NodeMatchASTNode<T>>, cursor: Cursor<T>) -> Self {
        Self {
            matcher,
            child: None,
            cursor,
            is_initialised: false,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for NodeMatchMatcher<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
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

                self.child = Some(child.create_matcher(child_cursor));
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

    fn record_captures(&self, bindings: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        if let Some(child) = self.child.as_ref() {
            child.record_captures(bindings);
        }
    }
}

struct SequenceMatcher<T: KindTypes> {
    matcher: Rc<SequenceASTNode<T>>,
    children: Vec<MatcherRef<T>>,
    cursor: Cursor<T>,
    is_initialised: bool,
}

impl<T: KindTypes> SequenceMatcher<T> {
    fn new(matcher: Rc<SequenceASTNode<T>>, cursor: Cursor<T>) -> Self {
        Self {
            matcher,
            children: vec![],
            cursor,
            is_initialised: false,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for SequenceMatcher<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
        if !self.is_initialised {
            self.is_initialised = true;

            let child_cursor = self.cursor.clone();
            let child = self.matcher.children[0].create_matcher(child_cursor);
            self.children.push(child);
        }

        while !self.children.is_empty() {
            if let Some(child_cursor) = self.children.last_mut().unwrap().next() {
                if self.children.len() == self.matcher.children.len() {
                    return Some(child_cursor);
                }

                let child = self.matcher.children[self.children.len()].create_matcher(child_cursor);
                self.children.push(child);
            } else {
                self.children.pop();
            }
        }

        None
    }

    fn record_captures(&self, bindings: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        for child in &self.children {
            child.record_captures(bindings);
        }
    }
}

struct AlternativesMatcher<T: KindTypes> {
    matcher: Rc<AlternativesASTNode<T>>,
    next_child_number: usize,
    child: Option<MatcherRef<T>>,
    cursor: Cursor<T>,
}

impl<T: KindTypes> AlternativesMatcher<T> {
    fn new(matcher: Rc<AlternativesASTNode<T>>, cursor: Cursor<T>) -> Self {
        Self {
            matcher,
            next_child_number: 0,
            child: None,
            cursor,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for AlternativesMatcher<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
        loop {
            if self.child.is_none() {
                match self.matcher.children.get(self.next_child_number) {
                    Some(child) => {
                        let child = child.create_matcher(self.cursor.clone());
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

    fn record_captures(&self, bindings: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        self.child.as_ref().unwrap().record_captures(bindings);
    }
}

struct OptionalMatcher<T: KindTypes> {
    matcher: Rc<OptionalASTNode<T>>,
    child: Option<MatcherRef<T>>,
    cursor: Cursor<T>,
    have_nonempty_match: bool,
}

impl<T: KindTypes> OptionalMatcher<T> {
    fn new(matcher: Rc<OptionalASTNode<T>>, cursor: Cursor<T>) -> Self {
        Self {
            matcher,
            child: None,
            cursor,
            have_nonempty_match: false,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for OptionalMatcher<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
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
            let child = self.matcher.child.create_matcher(child_cursor);
            self.child = Some(child);
            Some(self.cursor.clone())
        }
    }

    fn record_captures(&self, bindings: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        if self.have_nonempty_match {
            if let Some(child) = self.child.as_ref() {
                child.record_captures(bindings);
            }
        }
    }
}

struct OneOrMoreMatcher<T: KindTypes> {
    matcher: Rc<OneOrMoreASTNode<T>>,
    children: Vec<MatcherRef<T>>,
    cursor_for_next_repetition: Option<Cursor<T>>,
}

impl<T: KindTypes> OneOrMoreMatcher<T> {
    fn new(matcher: Rc<OneOrMoreASTNode<T>>, cursor: Cursor<T>) -> Self {
        let cursor_for_next_repetition = Some(cursor);
        Self {
            matcher,
            children: vec![],
            cursor_for_next_repetition,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for OneOrMoreMatcher<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
        loop {
            if let Some(cursor_for_next_repetition) = self.cursor_for_next_repetition.take() {
                let next_child = self
                    .matcher
                    .child
                    .create_matcher(cursor_for_next_repetition);
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

    fn record_captures(&self, bindings: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        for child in &self.children {
            child.record_captures(bindings);
        }
    }
}

struct EllipsisMatcher<T: KindTypes> {
    cursor: Cursor<T>,
    has_returned_initial_empty_value: bool,
}

impl<T: KindTypes> EllipsisMatcher<T> {
    fn new(cursor: Cursor<T>) -> Self {
        Self {
            cursor,
            has_returned_initial_empty_value: false,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for EllipsisMatcher<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
        if !self.has_returned_initial_empty_value {
            self.has_returned_initial_empty_value = true;
            return Some(self.cursor.clone());
        }

        if self.cursor.irrevocably_go_to_next_sibling() {
            return Some(self.cursor.clone());
        }

        None
    }

    fn record_captures(&self, _bindings: &mut BTreeMap<String, Vec<Cursor<T>>>) {}
}
