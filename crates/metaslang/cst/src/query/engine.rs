use std::collections::HashMap;
use std::rc::Rc;

// This crate is copied to another crate, so all imports should be relative
use super::super::cst::Node;
use super::super::cursor::Cursor;
use super::model::{
    AlternativesMatcher, BindingMatcher, Kind, Matcher, NodeMatcher, NodeSelector,
    OneOrMoreMatcher, OptionalMatcher, Query, SequenceMatcher,
};
use crate::KindTypes;

impl<T: KindTypes + 'static> Cursor<T> {
    pub fn query(self, queries: Vec<Query<T>>) -> QueryResultIterator<T> {
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

    fn matches_node_selector(&self, node_selector: &NodeSelector<T>) -> bool {
        match self.node() {
            Node::<T>::Rule(rule) => match node_selector {
                NodeSelector::Anonymous => true,
                NodeSelector::Kind { kind } => Kind::Rule(rule.kind) == *kind,
                NodeSelector::Text { .. } => false,
                NodeSelector::Label { label } => Some(*label) == self.label(),
                NodeSelector::LabelAndKind { label, kind } => {
                    Some(*label) == self.label() && Kind::Rule(rule.kind) == *kind
                }
                NodeSelector::LabelAndText { .. } => false,
            },

            Node::<T>::Token(token) => match node_selector {
                NodeSelector::Anonymous => true,
                NodeSelector::Kind { kind } => Kind::Token(token.kind) == *kind,
                NodeSelector::Text { text } => token.text == *text,
                NodeSelector::Label { label } => Some(*label) == self.label(),
                NodeSelector::LabelAndKind { label, kind } => {
                    Some(*label) == self.label() && Kind::Token(token.kind) == *kind
                }
                NodeSelector::LabelAndText { label, text } => {
                    Some(*label) == self.label() && token.text == *text
                }
            },
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> {
    // This allows for queries to pre-flight against a cursor without allocating
    fn can_match(&self, cursor: &Cursor<T>) -> bool {
        match self {
            Self::Binding(matcher) => matcher.child.can_match(cursor),
            Self::Node(matcher) => cursor.matches_node_selector(&matcher.node_selector),
            Self::Alternatives(matcher) => matcher.children.iter().any(|c| c.can_match(cursor)),
            Self::Sequence(matcher) => matcher.children[0].can_match(cursor),
            Self::OneOrMore(matcher) => matcher.child.can_match(cursor),
            Self::Optional(_) => true,
            Self::Ellipsis => true,
        }
    }

    fn create_combinator(&self, cursor: Cursor<T>) -> CombinatorRef<T> {
        match self {
            Self::Binding(matcher) => {
                Box::new(BindingCombinator::<T>::new(matcher.clone(), cursor))
            }
            Self::Node(matcher) => Box::new(NodeCombinator::<T>::new(matcher.clone(), cursor)),
            Self::Sequence(matcher) => {
                Box::new(SequenceCombinator::<T>::new(matcher.clone(), cursor))
            }
            Self::Alternatives(matcher) => {
                Box::new(AlternativesCombinator::<T>::new(matcher.clone(), cursor))
            }
            Self::Optional(matcher) => {
                Box::new(OptionalCombinator::<T>::new(matcher.clone(), cursor))
            }
            Self::OneOrMore(matcher) => {
                Box::new(OneOrMoreCombinator::<T>::new(matcher.clone(), cursor))
            }
            Self::Ellipsis => Box::new(EllipsisCombinator::<T>::new(cursor)),
        }
    }
}

pub struct QueryResult<T: KindTypes> {
    pub query_number: usize,
    pub bindings: HashMap<String, Vec<Cursor<T>>>,
}

pub struct QueryResultIterator<T: KindTypes> {
    cursor: Cursor<T>,
    queries: Vec<Query<T>>,
    query_number: usize,
    combinator: Option<CombinatorRef<T>>,
}

impl<T: KindTypes + 'static> QueryResultIterator<T> {
    fn new(cursor: Cursor<T>, queries: Vec<Query<T>>) -> Self {
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

impl<T: KindTypes + 'static> Iterator for QueryResultIterator<T> {
    type Item = QueryResult<T>;

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

trait Combinator<T: KindTypes> {
    // None -> failed to match, you must backtrack. DO NOT call again
    // Some(cursor) if cursor.is_complete -> matched, end of input
    // Some(cursor) if !cursor.is_complete -> matched, more input to go
    fn next(&mut self) -> Option<Cursor<T>>;
    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor<T>>>);
}
type CombinatorRef<T> = Box<dyn Combinator<T>>;

struct BindingCombinator<T: KindTypes> {
    matcher: Rc<BindingMatcher<T>>,
    cursor: Cursor<T>,
    child: CombinatorRef<T>,
}

impl<T: KindTypes + 'static> BindingCombinator<T> {
    fn new(matcher: Rc<BindingMatcher<T>>, cursor: Cursor<T>) -> Self {
        let child = matcher.child.create_combinator(cursor.clone());
        Self {
            matcher,
            cursor,
            child,
        }
    }
}

impl<T: KindTypes> Combinator<T> for BindingCombinator<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
        self.child.next()
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor<T>>>) {
        bindings
            .entry(self.matcher.name.clone())
            .or_default()
            .push(self.cursor.clone());
        self.child.accumulate_bindings(bindings);
    }
}

struct NodeCombinator<T: KindTypes> {
    matcher: Rc<NodeMatcher<T>>,
    child: Option<CombinatorRef<T>>,
    cursor: Cursor<T>,
    is_initialised: bool,
}

impl<T: KindTypes> NodeCombinator<T> {
    fn new(matcher: Rc<NodeMatcher<T>>, cursor: Cursor<T>) -> Self {
        Self {
            matcher,
            child: None,
            cursor,
            is_initialised: false,
        }
    }
}

impl<T: KindTypes + 'static> Combinator<T> for NodeCombinator<T> {
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

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor<T>>>) {
        if let Some(child) = self.child.as_ref() {
            child.accumulate_bindings(bindings);
        }
    }
}

struct SequenceCombinator<T: KindTypes> {
    matcher: Rc<SequenceMatcher<T>>,
    children: Vec<CombinatorRef<T>>,
    cursor: Cursor<T>,
    is_initialised: bool,
}

impl<T: KindTypes> SequenceCombinator<T> {
    fn new(matcher: Rc<SequenceMatcher<T>>, cursor: Cursor<T>) -> Self {
        Self {
            matcher,
            children: vec![],
            cursor,
            is_initialised: false,
        }
    }
}

impl<T: KindTypes + 'static> Combinator<T> for SequenceCombinator<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
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

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor<T>>>) {
        for child in &self.children {
            child.accumulate_bindings(bindings);
        }
    }
}

struct AlternativesCombinator<T: KindTypes> {
    matcher: Rc<AlternativesMatcher<T>>,
    next_child_number: usize,
    child: Option<CombinatorRef<T>>,
    cursor: Cursor<T>,
}

impl<T: KindTypes> AlternativesCombinator<T> {
    fn new(matcher: Rc<AlternativesMatcher<T>>, cursor: Cursor<T>) -> Self {
        Self {
            matcher,
            next_child_number: 0,
            child: None,
            cursor,
        }
    }
}

impl<T: KindTypes + 'static> Combinator<T> for AlternativesCombinator<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
        loop {
            if self.child.is_none() {
                match self.matcher.children.get(self.next_child_number) {
                    Some(child) => {
                        let child = child.create_combinator(self.cursor.clone());
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

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor<T>>>) {
        self.child.as_ref().unwrap().accumulate_bindings(bindings);
    }
}

struct OptionalCombinator<T: KindTypes> {
    matcher: Rc<OptionalMatcher<T>>,
    child: Option<CombinatorRef<T>>,
    cursor: Cursor<T>,
    have_nonempty_match: bool,
}

impl<T: KindTypes> OptionalCombinator<T> {
    fn new(matcher: Rc<OptionalMatcher<T>>, cursor: Cursor<T>) -> Self {
        Self {
            matcher,
            child: None,
            cursor,
            have_nonempty_match: false,
        }
    }
}

impl<T: KindTypes + 'static> Combinator<T> for OptionalCombinator<T> {
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
            let child = self.matcher.child.create_combinator(child_cursor);
            self.child = Some(child);
            Some(self.cursor.clone())
        }
    }

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor<T>>>) {
        if self.have_nonempty_match {
            if let Some(child) = self.child.as_ref() {
                child.accumulate_bindings(bindings);
            }
        }
    }
}

struct OneOrMoreCombinator<T: KindTypes> {
    matcher: Rc<OneOrMoreMatcher<T>>,
    children: Vec<CombinatorRef<T>>,
    cursor_for_next_repetition: Option<Cursor<T>>,
}

impl<T: KindTypes> OneOrMoreCombinator<T> {
    fn new(matcher: Rc<OneOrMoreMatcher<T>>, cursor: Cursor<T>) -> Self {
        let cursor_for_next_repetition = Some(cursor);
        Self {
            matcher,
            children: vec![],
            cursor_for_next_repetition,
        }
    }
}

impl<T: KindTypes + 'static> Combinator<T> for OneOrMoreCombinator<T> {
    fn next(&mut self) -> Option<Cursor<T>> {
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

    fn accumulate_bindings(&self, bindings: &mut HashMap<String, Vec<Cursor<T>>>) {
        for child in &self.children {
            child.accumulate_bindings(bindings);
        }
    }
}

struct EllipsisCombinator<T: KindTypes> {
    cursor: Cursor<T>,
    has_returned_initial_empty_value: bool,
}

impl<T: KindTypes> EllipsisCombinator<T> {
    fn new(cursor: Cursor<T>) -> Self {
        Self {
            cursor,
            has_returned_initial_empty_value: false,
        }
    }
}

impl<T: KindTypes + 'static> Combinator<T> for EllipsisCombinator<T> {
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

    fn accumulate_bindings(&self, _bindings: &mut HashMap<String, Vec<Cursor<T>>>) {}
}
