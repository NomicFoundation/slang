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
            Node::<T>::Nonterminal(nonterminal) => match node_selector {
                NodeSelector::Anonymous => true,
                NodeSelector::NodeKind { node_kind } => {
                    NodeKind::Nonterminal(nonterminal.kind) == *node_kind
                }
                NodeSelector::NodeText { .. } => false,
                NodeSelector::EdgeLabel { edge_label } => Some(*edge_label) == self.label(),
                NodeSelector::EdgeLabelAndNodeKind {
                    edge_label,
                    node_kind,
                } => {
                    Some(*edge_label) == self.label()
                        && NodeKind::Nonterminal(nonterminal.kind) == *node_kind
                }
                NodeSelector::EdgeLabelAndNodeText { .. } => false,
            },

            Node::<T>::Terminal(terminal) => match node_selector {
                NodeSelector::Anonymous => true,
                NodeSelector::NodeKind { node_kind } => {
                    NodeKind::Terminal(terminal.kind) == *node_kind
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
            Self::Capture(matcher) => matcher.child.can_match(cursor),
            Self::NodeMatch(matcher) => cursor.matches_node_selector(&matcher.node_selector),
            Self::Alternatives(matcher) => matcher.children.iter().any(|c| c.can_match(cursor)),
            Self::Sequence(matcher) => matcher.children[0].can_match(cursor),
            Self::OneOrMore(matcher) => matcher.child.can_match(cursor),
            Self::Optional(_) => true,
            Self::Anchor => true,
        }
    }

    fn create_matcher(&self, cursor: Cursor<T>, needs_actual_match: bool) -> MatcherRef<T> {
        match self {
            Self::Capture(matcher) => {
                Box::new(CaptureMatcher::<T>::new(Rc::clone(matcher), cursor, needs_actual_match))
            }
            Self::NodeMatch(matcher) => {
                Box::new(NodeMatchMatcher::<T>::new(Rc::clone(matcher), cursor, needs_actual_match))
            }
            Self::Sequence(matcher) => {
                Box::new(SequenceMatcher::<T>::new(Rc::clone(matcher), cursor, needs_actual_match))
            }
            Self::Alternatives(matcher) => {
                Box::new(AlternativesMatcher::<T>::new(Rc::clone(matcher), cursor, needs_actual_match))
            }
            Self::Optional(matcher) => {
                Box::new(OptionalMatcher::<T>::new(Rc::clone(matcher), cursor, needs_actual_match))
            }
            Self::OneOrMore(matcher) => {
                Box::new(OneOrMoreMatcher::<T>::new(Rc::clone(matcher), cursor, needs_actual_match))
            }
            Self::Anchor => Box::new(AnchorMatcher::<T>::new(cursor, needs_actual_match)),
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
                    self.matcher = Some(ast_node.create_matcher(self.cursor.clone(), false));
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
                    let mut captures = BTreeMap::new();
                    matcher.record_captures(&mut captures);
                    return Some(QueryMatch {
                        queries: Rc::clone(&self.queries),
                        root_cursor: self.cursor.clone(),
                        query_number: self.query_number,
                        captures,
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
    // Some(cursor, needs_actual_match) if cursor.is_complete -> matched, end of input
    // Some(cursor, needs_actual_match) if !cursor.is_complete -> matched, more input to go
    //
    // needs_actual_match indicates if we can skip further nodes (eg. with an
    // ellipsis). If true, this is used to indicate that there already is an
    // adjacent ellipsis operator that can skip over nodes. Further matchers
    // need to either nil-match, ie. match without consuming any nodes and
    // forwarding this flag, or match an actual node with a NodeMatcher. This is
    // to resolve the interaction between two adjacent (although maybe not
    // strictly consecutive) ellipsis matchers.
    fn next(&mut self) -> Option<(Cursor<T>, bool)>;
    fn record_captures(&self, captures: &mut BTreeMap<String, Vec<Cursor<T>>>);
}
type MatcherRef<T> = Box<dyn Matcher<T>>;

struct CaptureMatcher<T: KindTypes> {
    matcher: Rc<CaptureASTNode<T>>,
    cursor: Cursor<T>,
    child: MatcherRef<T>,
}

impl<T: KindTypes + 'static> CaptureMatcher<T> {
    fn new(matcher: Rc<CaptureASTNode<T>>, cursor: Cursor<T>, needs_actual_match: bool) -> Self {
        let child = matcher.child.create_matcher(cursor.clone(), needs_actual_match);
        Self {
            matcher,
            cursor,
            child,
        }
    }
}

impl<T: KindTypes> Matcher<T> for CaptureMatcher<T> {
    fn next(&mut self) -> Option<(Cursor<T>, bool)> {
        self.child.next()
    }

    fn record_captures(&self, captures: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        captures
            .entry(self.matcher.name.clone())
            .or_default()
            .push(self.cursor.clone());
        self.child.record_captures(captures);
    }
}

struct NodeMatchMatcher<T: KindTypes> {
    matcher: Rc<NodeMatchASTNode<T>>,
    child: Option<MatcherRef<T>>,
    cursor: Cursor<T>,
    is_initialised: bool,
}

impl<T: KindTypes> NodeMatchMatcher<T> {
    fn new(matcher: Rc<NodeMatchASTNode<T>>, cursor: Cursor<T>, _needs_actual_match: bool) -> Self {
        // is this matcher succeeds it will always return an actual match, so we
        // can ignore the argument
        Self {
            matcher,
            child: None,
            cursor,
            is_initialised: false,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for NodeMatchMatcher<T> {
    fn next(&mut self) -> Option<(Cursor<T>, bool)> {
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
                    // have child matchers, but no children
                    return None;
                }

                self.child = Some(child.create_matcher(child_cursor, false));
            } else {
                // no child matchers
                let mut return_cursor = self.cursor.clone();
                return_cursor.irrevocably_go_to_next_sibling();
                return Some((return_cursor, false));
            }
        }

        if let Some(child) = self.child.as_mut() {
            // get matches from the child matcher
            while let Some((cursor, _)) = child.as_mut().next() {
                if cursor.is_completed() {
                    // and if found and complete, return the match *from our own cursor*
                    let mut return_cursor = self.cursor.clone();
                    return_cursor.irrevocably_go_to_next_sibling();
                    return Some((return_cursor, false));
                }
            }
            // reset the child matcher if it cannot find more matches
            self.child = None;
        }

        None
    }

    fn record_captures(&self, captures: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        if let Some(child) = self.child.as_ref() {
            child.record_captures(captures);
        }
    }
}

enum SequenceItem {
    ChildMatcher(usize),
    Ellipsis,
}

struct SequenceMatcher<T: KindTypes> {
    matcher: Rc<SequenceASTNode<T>>,
    children: Vec<MatcherRef<T>>,
    cursor: Cursor<T>,
    is_initialised: bool,
    template: Vec<SequenceItem>,
    needs_actual_match: bool,
}

impl<T: KindTypes + 'static> SequenceMatcher<T> {
    fn new(matcher: Rc<SequenceASTNode<T>>, cursor: Cursor<T>, needs_actual_match: bool) -> Self {
        // Produce a template of instructions to create the matchers for the
        // sequence by inserting ellipsis matchers at the start, end, and in
        // between each of the child matchers, unless it's explicitly disabled
        // by an anchor token.
        // If the sequence is anchored (eg. option in alt or quantified
        // group sequence) then the starting and ending anchors are implicit.
        let (mut template, last_anchor) = matcher.children.iter().enumerate().fold(
            (Vec::new(), matcher.anchored),
            |(mut acc, last_anchor), (index, child)| {
                if matches!(child, ASTNode::Anchor) {
                    if last_anchor {
                        unreachable!("Found two consecutive anchors")
                    }
                    acc.push(SequenceItem::ChildMatcher(index));
                    (acc, true)
                } else {
                    if !last_anchor {
                        acc.push(SequenceItem::Ellipsis);
                    }
                    acc.push(SequenceItem::ChildMatcher(index));
                    (acc, false)
                }
            },
        );
        if !last_anchor && !matcher.anchored {
            template.push(SequenceItem::Ellipsis);
        }
        Self {
            matcher,
            children: vec![],
            cursor,
            is_initialised: false,
            template,
            needs_actual_match,
        }
    }

    fn create_matcher(&self, index: usize, cursor: Cursor<T>, needs_actual_match: bool) -> MatcherRef<T> {
        let item = &self.template[index];
        match item {
            SequenceItem::Ellipsis => Box::new(EllipsisMatcher::new(cursor, needs_actual_match)),
            SequenceItem::ChildMatcher(index) => {
                self.matcher.children[*index].create_matcher(cursor, needs_actual_match)
            }
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for SequenceMatcher<T> {
    fn next(&mut self) -> Option<(Cursor<T>, bool)> {
        if !self.is_initialised {
            self.is_initialised = true;

            let child_cursor = self.cursor.clone();
            let child = self.create_matcher(0, child_cursor, self.needs_actual_match);
            self.children.push(child);
        }

        while !self.children.is_empty() {
            if let Some((child_cursor, child_needs_match)) = self.children.last_mut().unwrap().next() {
                if self.children.len() == self.template.len() {
                    return Some((child_cursor, child_needs_match));
                }
                let child = self.create_matcher(self.children.len(), child_cursor, child_needs_match);
                self.children.push(child);
            } else {
                self.children.pop();
            }
        }

        None
    }

    fn record_captures(&self, captures: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        for child in &self.children {
            child.record_captures(captures);
        }
    }
}

struct AlternativesMatcher<T: KindTypes> {
    matcher: Rc<AlternativesASTNode<T>>,
    next_child_number: usize,
    child: Option<MatcherRef<T>>,
    cursor: Cursor<T>,
    needs_actual_match: bool,
}

impl<T: KindTypes> AlternativesMatcher<T> {
    fn new(matcher: Rc<AlternativesASTNode<T>>, cursor: Cursor<T>, needs_actual_match: bool) -> Self {
        Self {
            matcher,
            next_child_number: 0,
            child: None,
            cursor,
            needs_actual_match,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for AlternativesMatcher<T> {
    fn next(&mut self) -> Option<(Cursor<T>, bool)> {
        loop {
            if self.child.is_none() {
                match self.matcher.children.get(self.next_child_number) {
                    Some(child) => {
                        let child = child.create_matcher(self.cursor.clone(), self.needs_actual_match);
                        self.child = Some(child);
                        self.next_child_number += 1;
                    }
                    None => return None,
                }
            }

            match self.child.as_mut().unwrap().next() {
                Some(cursor_and_needs_match) => return Some(cursor_and_needs_match),
                None => self.child = None,
            }
        }
    }

    fn record_captures(&self, captures: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        self.child.as_ref().unwrap().record_captures(captures);
    }
}

struct OptionalMatcher<T: KindTypes> {
    matcher: Rc<OptionalASTNode<T>>,
    child: Option<MatcherRef<T>>,
    cursor: Cursor<T>,
    have_nonempty_match: bool,
    needs_actual_match: bool,
}

impl<T: KindTypes> OptionalMatcher<T> {
    fn new(matcher: Rc<OptionalASTNode<T>>, cursor: Cursor<T>, needs_actual_match: bool) -> Self {
        Self {
            matcher,
            child: None,
            cursor,
            have_nonempty_match: false,
            needs_actual_match,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for OptionalMatcher<T> {
    fn next(&mut self) -> Option<(Cursor<T>, bool)> {
        if let Some(child) = self.child.as_mut() {
            // this is for the second entry, when we have a child matcher created
            match child.next() {
                Some((cursor, child_needs_match)) => {
                    self.have_nonempty_match = true;
                    Some((cursor, child_needs_match))
                }
                None => {
                    self.child = None;
                    None
                }
            }
        } else {
            // we don't have a child matcher yet, so create it forwarding our needs_actual_match
            let child_cursor = self.cursor.clone();
            let child = self.matcher.child.create_matcher(child_cursor, self.needs_actual_match);
            self.child = Some(child);
            // this is the return for the empty case, so we forward needs_actual_match
            Some((self.cursor.clone(), self.needs_actual_match))
        }
    }

    fn record_captures(&self, captures: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        if self.have_nonempty_match {
            if let Some(child) = self.child.as_ref() {
                child.record_captures(captures);
            }
        }
    }
}

struct OneOrMoreMatcher<T: KindTypes> {
    matcher: Rc<OneOrMoreASTNode<T>>,
    children: Vec<MatcherRef<T>>,
    cursor_for_next_repetition: Option<(Cursor<T>, bool)>,
}

impl<T: KindTypes> OneOrMoreMatcher<T> {
    fn new(matcher: Rc<OneOrMoreASTNode<T>>, cursor: Cursor<T>, needs_actual_match: bool) -> Self {
        let cursor_for_next_repetition = Some((cursor, needs_actual_match));
        Self {
            matcher,
            children: vec![],
            cursor_for_next_repetition,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for OneOrMoreMatcher<T> {
    fn next(&mut self) -> Option<(Cursor<T>, bool)> {
        loop {
            if let Some((cursor_for_next_repetition, needs_actual_match)) = self.cursor_for_next_repetition.take() {
                let next_child = self
                    .matcher
                    .child
                    .create_matcher(cursor_for_next_repetition, needs_actual_match);
                self.children.push(next_child);
            } else {
                let tail = self.children.last_mut().unwrap();
                if let Some((cursor, child_needs_match)) = tail.next() {
                    if !cursor.is_completed() {
                        self.cursor_for_next_repetition = Some((cursor.clone(), child_needs_match));
                    }
                    return Some((cursor, child_needs_match));
                }
                self.children.pop();
                if self.children.is_empty() {
                    return None;
                }
            }
        }
    }

    fn record_captures(&self, captures: &mut BTreeMap<String, Vec<Cursor<T>>>) {
        for child in &self.children {
            child.record_captures(captures);
        }
    }
}

/// Matches any number of sibling nodes and is used in between other matchers
/// when matching sequences, unless an explicit anchor is added.
struct EllipsisMatcher<T: KindTypes> {
    cursor: Cursor<T>,
    has_returned_initial_empty_value: bool,
    needs_actual_match: bool,
}

impl<T: KindTypes> EllipsisMatcher<T> {
    fn new(cursor: Cursor<T>, needs_actual_match: bool) -> Self {
        Self {
            cursor,
            has_returned_initial_empty_value: false,
            needs_actual_match,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for EllipsisMatcher<T> {
    fn next(&mut self) -> Option<(Cursor<T>, bool)> {
        if !self.has_returned_initial_empty_value {
            self.has_returned_initial_empty_value = true;
            // we will be consuming nodes, so we need a later matcher to avoid that and return an actual match
            return Some((self.cursor.clone(), true));
        }

        // we only consume nodes we don't need an actual match, ie. if this is
        // the *first* adjacent ellipsis operator; if there is another one
        // before us, it will be consuming an arbitrary sequence of nodes
        if !self.needs_actual_match && self.cursor.irrevocably_go_to_next_sibling() {
            // we are consuming nodes, so we need a later matcher to avoid that and return an actual match
            return Some((self.cursor.clone(), true));
        }

        None
    }

    fn record_captures(&self, _: &mut BTreeMap<String, Vec<Cursor<T>>>) {}
}

/// Greedily consumes all available trivia nodes
struct AnchorMatcher<T: KindTypes> {
    cursor: Option<Cursor<T>>,
    needs_actual_match: bool,
}

impl<T: KindTypes + 'static> AnchorMatcher<T> {
    fn new(cursor: Cursor<T>, needs_actual_match: bool) -> Self {
        Self {
            cursor: Some(cursor),
            needs_actual_match,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for AnchorMatcher<T> {
    fn next(&mut self) -> Option<(Cursor<T>, bool)> {
        if let Some(mut cursor) = self.cursor.take() {
            while !cursor.is_completed() && cursor.node().is_trivia() {
                cursor.irrevocably_go_to_next_sibling();
            }
            Some((cursor, self.needs_actual_match))
        } else {
            None
        }
    }

    fn record_captures(&self, _: &mut BTreeMap<String, Vec<Cursor<T>>>) {}
}
