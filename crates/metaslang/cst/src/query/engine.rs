use std::collections::BTreeMap;
use std::rc::Rc;

use super::super::cursor::Cursor;
use super::super::nodes::Node;
use super::model::{
    ASTNode, AlternativesASTNode, CaptureASTNode, NodeMatchASTNode, NodeSelector, OneOrMoreASTNode,
    OptionalASTNode, Query, SequenceASTNode,
};
use crate::kinds::{KindTypes, NodeKind, TerminalKindExtensions};
use crate::query::CaptureQuantifier;

impl<T: KindTypes + 'static> Cursor<T> {
    /// Returns an iterator over all matches of the given queries in the syntax tree.
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
                NodeSelector::EdgeLabel { edge_label } => *edge_label == self.label(),
                NodeSelector::EdgeLabelAndNodeKind {
                    edge_label,
                    node_kind,
                } => {
                    *edge_label == self.label()
                        && NodeKind::Nonterminal(nonterminal.kind) == *node_kind
                }
                NodeSelector::EdgeLabelAndNodeText { .. } => false,
            },

            Node::<T>::Terminal(terminal) if terminal.kind.is_trivia() => false,

            Node::<T>::Terminal(terminal) => match node_selector {
                NodeSelector::Anonymous => true,
                NodeSelector::NodeKind { node_kind } => {
                    NodeKind::Terminal(terminal.kind) == *node_kind
                }
                NodeSelector::NodeText { node_text } => terminal.text == *node_text,
                NodeSelector::EdgeLabel { edge_label } => *edge_label == self.label(),
                NodeSelector::EdgeLabelAndNodeKind {
                    edge_label,
                    node_kind,
                } => *edge_label == self.label() && NodeKind::Terminal(terminal.kind) == *node_kind,
                NodeSelector::EdgeLabelAndNodeText {
                    edge_label,
                    node_text,
                } => *edge_label == self.label() && terminal.text == *node_text,
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
            Self::Adjacency => true,
        }
    }

    // The `require_explicit_match` parameter modifies the behaviour of this and
    // later matchers. If this value is true, this and later matchers should not
    // match sibling nodes implicitly.
    // Currently this only modifies the behaviour of the ellipsis matcher, which
    // otherwise will attempt to consume any number of sibling nodes.
    // In a sequence of matchers, this value is set to true by the ellipsis
    // operator itself, to consume all available sibling nodes and prevent later
    // ellipsis matchers from doing so.
    // Conversely, it's set to false by the `NodeMatcher`, both when recursing
    // into its children and for later matchers after itself, as it handles an
    // explicit match requested by the user.
    // All other matchers should propagate the received value forward.
    //
    // The whole point of propagating this flag is to prevent a weird
    // interaction between ellipsis operators working on the same set of sibling
    // nodes. While two consecutive ellipsis operators should never happen, we
    // have the `OptionalMatcher` which will not consume any nodes in the nil
    // case. This means that `... [_]? ...` will effectively work (in one case)
    // as `... ...`. If we allow both ellipsis operators to consume any number
    // of nodes, for a sequence of N nodes we get N+1 identical query results
    // when the operators take turns matching each prefix and complementary
    // suffix of the list of nodes. By only allowing the first ellipsis operator
    // to consume an arbitrary number of nodes, we reduce the returned matches
    // to a single one.
    //
    fn create_matcher(&self, cursor: Cursor<T>, require_explicit_match: bool) -> MatcherRef<T> {
        match self {
            Self::Capture(matcher) => Box::new(CaptureMatcher::<T>::new(
                Rc::clone(matcher),
                cursor,
                require_explicit_match,
            )),
            Self::NodeMatch(matcher) => {
                // By definition this matcher matches nodes explicitly
                Box::new(NodeMatchMatcher::<T>::new(Rc::clone(matcher), cursor))
            }
            Self::Sequence(matcher) => Box::new(SequenceMatcher::<T>::new(
                Rc::clone(matcher),
                cursor,
                require_explicit_match,
            )),
            Self::Alternatives(matcher) => Box::new(AlternativesMatcher::<T>::new(
                Rc::clone(matcher),
                cursor,
                require_explicit_match,
            )),
            Self::Optional(matcher) => Box::new(OptionalMatcher::<T>::new(
                Rc::clone(matcher),
                cursor,
                require_explicit_match,
            )),
            Self::OneOrMore(matcher) => Box::new(OneOrMoreMatcher::<T>::new(
                Rc::clone(matcher),
                cursor,
                require_explicit_match,
            )),
            Self::Adjacency => Box::new(AdjacencyMatcher::<T>::new(cursor, require_explicit_match)),
        }
    }
}

/// Represents a match found by executing queries on a cursor.
pub struct QueryMatch<T: KindTypes> {
    /// The queries that were matched.
    pub queries: Rc<Vec<Query<T>>>,
    /// The index of the matched query within the list of queries.
    pub query_index: usize,
    /// The cursor that was used to find the match.
    pub root_cursor: Cursor<T>,
    /// The capture definitions in the query
    pub captures: BTreeMap<String, Vec<Cursor<T>>>,
}

impl<T: KindTypes> QueryMatch<T> {
    /// Returns the query that was matched.
    pub fn query(&self) -> &Query<T> {
        &self.queries[self.query_index]
    }

    /// Returns an iterator over all of the capture names matched by this query.
    pub fn capture_names(&self) -> impl Iterator<Item = &String> {
        self.query().capture_quantifiers.keys()
    }

    /// Returns an iterator over all of the captures matched by this query. The iterator item
    /// is a 3-tuple containing the capture name, a [`CaptureQuantifier`], and an iterator yielding
    /// a [`Cursor`] to the location of each capture in the parse tree.
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

    /// Try to find a single capture matched by this query. If no captures exist with the name `name`,
    /// this will return `None`. If captures do exist, then this will return a tuple containing a [`CaptureQuantifier`]
    /// and an iterator yielding a [`Cursor`] to the location of each capture in the parse tree.
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

/// Iterator over query matches in the syntax tree.
pub struct QueryMatchIterator<T: KindTypes> {
    queries: Rc<Vec<Query<T>>>,
    cursor: Cursor<T>,
    query_index: usize,
    matcher: Option<MatcherRef<T>>,
}

impl<T: KindTypes + 'static> QueryMatchIterator<T> {
    fn new(cursor: Cursor<T>, queries: Vec<Query<T>>) -> Self {
        Self {
            queries: Rc::new(queries),
            cursor,
            query_index: 0,
            matcher: None,
        }
    }

    fn advance_to_next_possible_matching_query(&mut self) {
        while !self.cursor.is_completed() {
            while self.query_index < self.queries.len() {
                let ast_node = &self.queries[self.query_index].ast_node;
                if ast_node.can_match(&self.cursor) {
                    // The first matcher in the query should allow implicit matches
                    self.matcher = Some(ast_node.create_matcher(self.cursor.clone(), false));
                    return;
                }
                self.query_index += 1;
            }
            self.cursor.go_to_next();
            self.query_index = 0;
        }
    }
}

impl<T: KindTypes + 'static> Iterator for QueryMatchIterator<T> {
    type Item = QueryMatch<T>;

    /// Returns the next match or `None` if there are no more matches.
    fn next(&mut self) -> Option<Self::Item> {
        while !self.cursor.is_completed() {
            if let Some(matcher) = self.matcher.as_mut() {
                if matcher.next().is_some() {
                    let mut captures = BTreeMap::new();
                    matcher.record_captures(&mut captures);
                    return Some(QueryMatch {
                        queries: Rc::clone(&self.queries),
                        root_cursor: self.cursor.clone(),
                        query_index: self.query_index,
                        captures,
                    });
                }
                self.query_index += 1;
            }

            self.advance_to_next_possible_matching_query();
        }

        None
    }
}

#[derive(Clone)]
struct MatcherResult<T: KindTypes> {
    // if cursor.is_completed() -> end of input
    // if !cursor.is_completed() -> there is more input to go
    cursor: Cursor<T>,

    // Controls whether next matchers can match nodes implicitly. For matchers
    // applied on a sequence of sibling nodes, this will be:
    // - initially false, allowing the first found ellipsis matcher to consume
    //   an arbitrary number of nodes
    // - true after the execution of an ellipsis, thus preventing later ellipsis
    //   from consuming nodes
    // - propagated forward by other matchers, until
    // - an actual `NodeMatcher` successfully matches a node, which then flips
    //   this value back to false
    require_explicit_match: bool,
}

trait Matcher<T: KindTypes> {
    // None -> failed to match, you must backtrack. DO NOT call again
    // Some(result) -> matched, check result.cursor and pass require_explicit_match forward
    fn next(&mut self) -> Option<MatcherResult<T>>;
    fn record_captures(&self, captures: &mut BTreeMap<String, Vec<Cursor<T>>>);
}
type MatcherRef<T> = Box<dyn Matcher<T>>;

struct CaptureMatcher<T: KindTypes> {
    matcher: Rc<CaptureASTNode<T>>,
    cursor: Cursor<T>,
    child: MatcherRef<T>,
}

impl<T: KindTypes + 'static> CaptureMatcher<T> {
    fn new(
        matcher: Rc<CaptureASTNode<T>>,
        cursor: Cursor<T>,
        require_explicit_match: bool,
    ) -> Self {
        let child = matcher
            .child
            .create_matcher(cursor.clone(), require_explicit_match);
        Self {
            matcher,
            cursor,
            child,
        }
    }
}

impl<T: KindTypes> Matcher<T> for CaptureMatcher<T> {
    fn next(&mut self) -> Option<MatcherResult<T>> {
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
    fn next(&mut self) -> Option<MatcherResult<T>> {
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
                    // We have child matchers, but no children.
                    return None;
                }

                // Start traversing the children nodes allowing an ellipsis
                // operator to match implicitly.
                self.child = Some(child.create_matcher(child_cursor, false));
            } else {
                // We have no child matchers, we can return the result now.
                let mut return_cursor = self.cursor.clone();
                return_cursor.irrevocably_go_to_next_sibling();
                return Some(MatcherResult {
                    cursor: return_cursor,
                    require_explicit_match: false,
                });
            }
        }

        if let Some(child) = self.child.as_mut() {
            // Match our children with the child matcher repeatedly.
            while let Some(MatcherResult { cursor, .. }) = child.as_mut().next() {
                if cursor.is_completed() {
                    // If match found and exhausted our children list, return
                    // the match *from our own cursor*
                    let mut return_cursor = self.cursor.clone();
                    return_cursor.irrevocably_go_to_next_sibling();
                    return Some(MatcherResult {
                        cursor: return_cursor,
                        require_explicit_match: false,
                    });
                }
            }
            // No more matches from the child matcher, we will backtrack at this point.
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
    require_explicit_match: bool,
}

impl<T: KindTypes + 'static> SequenceMatcher<T> {
    fn new(
        matcher: Rc<SequenceASTNode<T>>,
        cursor: Cursor<T>,
        require_explicit_match: bool,
    ) -> Self {
        // Produce a template of instructions to create the matchers for the
        // sequence by inserting ellipsis matchers at the start, end, and in
        // between each of the child matchers, unless we find an adjacency
        // operator. If the sequence is adjacent (eg. option in alt or
        // quantified group sequence) then we should not add matchers at the
        // edges.
        let (mut template, last_adjacent) = matcher.children.iter().enumerate().fold(
            (Vec::new(), matcher.adjacent),
            |(mut acc, last_adjacent), (index, child)| {
                if matches!(child, ASTNode::Adjacency) {
                    if last_adjacent {
                        unreachable!("Found two consecutive adjacency operators")
                    }
                    acc.push(SequenceItem::ChildMatcher(index));
                    (acc, true)
                } else {
                    if !last_adjacent {
                        acc.push(SequenceItem::Ellipsis);
                    }
                    acc.push(SequenceItem::ChildMatcher(index));
                    (acc, false)
                }
            },
        );
        if !last_adjacent && !matcher.adjacent {
            template.push(SequenceItem::Ellipsis);
        }
        Self {
            matcher,
            children: vec![],
            cursor,
            is_initialised: false,
            template,
            require_explicit_match,
        }
    }

    fn create_matcher(
        &self,
        index: usize,
        cursor: Cursor<T>,
        require_explicit_match: bool,
    ) -> MatcherRef<T> {
        let item = &self.template[index];
        match item {
            SequenceItem::Ellipsis => {
                Box::new(EllipsisMatcher::new(cursor, require_explicit_match))
            }
            SequenceItem::ChildMatcher(index) => {
                self.matcher.children[*index].create_matcher(cursor, require_explicit_match)
            }
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for SequenceMatcher<T> {
    fn next(&mut self) -> Option<MatcherResult<T>> {
        if !self.is_initialised {
            self.is_initialised = true;

            let child_cursor = self.cursor.clone();
            let child = self.create_matcher(0, child_cursor, self.require_explicit_match);
            self.children.push(child);
        }

        while !self.children.is_empty() {
            if let Some(child_matcher_result) = self.children.last_mut().unwrap().next() {
                if self.children.len() == self.template.len() {
                    // Last child, return its result as our own
                    return Some(child_matcher_result);
                }
                // Create the next child matcher propagating the
                // `require_explicit_match` flag forward.
                let child = self.create_matcher(
                    self.children.len(),
                    child_matcher_result.cursor,
                    child_matcher_result.require_explicit_match,
                );
                self.children.push(child);
            } else {
                // Backtrack
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
    require_explicit_match: bool,
}

impl<T: KindTypes> AlternativesMatcher<T> {
    fn new(
        matcher: Rc<AlternativesASTNode<T>>,
        cursor: Cursor<T>,
        require_explicit_match: bool,
    ) -> Self {
        Self {
            matcher,
            next_child_number: 0,
            child: None,
            cursor,
            require_explicit_match,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for AlternativesMatcher<T> {
    fn next(&mut self) -> Option<MatcherResult<T>> {
        loop {
            if self.child.is_none() {
                // Create the next available child matcher forwarding the
                // `require_explicit_match` flag, or give up if we have no more
                match self.matcher.children.get(self.next_child_number) {
                    Some(child) => {
                        let child =
                            child.create_matcher(self.cursor.clone(), self.require_explicit_match);
                        self.child = Some(child);
                        self.next_child_number += 1;
                    }
                    None => return None,
                }
            }

            match self.child.as_mut().unwrap().next() {
                Some(child_matcher_result) => return Some(child_matcher_result),
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
    require_explicit_match: bool,
}

impl<T: KindTypes> OptionalMatcher<T> {
    fn new(
        matcher: Rc<OptionalASTNode<T>>,
        cursor: Cursor<T>,
        require_explicit_match: bool,
    ) -> Self {
        Self {
            matcher,
            child: None,
            cursor,
            have_nonempty_match: false,
            require_explicit_match,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for OptionalMatcher<T> {
    fn next(&mut self) -> Option<MatcherResult<T>> {
        if let Some(child) = self.child.as_mut() {
            // Second visit, we have a child matcher created
            if let Some(child_matcher_result) = child.next() {
                self.have_nonempty_match = true;
                Some(child_matcher_result)
            } else {
                self.child = None;
                None
            }
        } else {
            // First visit, we don't have a child matcher yet, so create it
            // forwarding our `require_explicit_match` flag
            let child_cursor = self.cursor.clone();
            let child = self
                .matcher
                .child
                .create_matcher(child_cursor, self.require_explicit_match);
            self.child = Some(child);

            // Return a match result for the empty case, forwarding the
            // `require_explicit_match` flag.
            Some(MatcherResult {
                cursor: self.cursor.clone(),
                require_explicit_match: self.require_explicit_match,
            })
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
    result_for_next_repetition: Option<MatcherResult<T>>,
}

impl<T: KindTypes> OneOrMoreMatcher<T> {
    fn new(
        matcher: Rc<OneOrMoreASTNode<T>>,
        cursor: Cursor<T>,
        require_explicit_match: bool,
    ) -> Self {
        let result_for_next_repetition = Some(MatcherResult {
            cursor,
            require_explicit_match,
        });
        Self {
            matcher,
            children: vec![],
            result_for_next_repetition,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for OneOrMoreMatcher<T> {
    fn next(&mut self) -> Option<MatcherResult<T>> {
        loop {
            if let Some(last_result) = self.result_for_next_repetition.take() {
                let next_child = self
                    .matcher
                    .child
                    .create_matcher(last_result.cursor, last_result.require_explicit_match);
                self.children.push(next_child);
            } else {
                let tail = self.children.last_mut().unwrap();
                if let Some(child_matcher_result) = tail.next() {
                    // Skip over trivia before saving the result for next repetition
                    let mut cursor = child_matcher_result.cursor.clone();
                    while !cursor.is_completed() && cursor.node().is_trivia() {
                        cursor.irrevocably_go_to_next_sibling();
                    }
                    if !cursor.is_completed() {
                        self.result_for_next_repetition = Some(MatcherResult {
                            cursor,
                            ..child_matcher_result
                        });
                    }
                    return Some(child_matcher_result);
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
/// when matching sequences, unless an explicit adjacency operator is found.
/// If `require_explicit_match` is true, then this matcher can only return a
/// result for the empty case. This usually means that in the same sequence of
/// siblings we found a previous ellipsis matcher which will be able to consume
/// an arbitrary number of nodes. Then, the value is false if this is the first
/// `EllipsisMatcher` in a sibling list, or there was an explicit match (by a
/// `NodeMatcher`) in a previous matcher of the sequence.
struct EllipsisMatcher<T: KindTypes> {
    cursor: Cursor<T>,
    has_returned_initial_empty_value: bool,
    require_explicit_match: bool,
}

impl<T: KindTypes> EllipsisMatcher<T> {
    fn new(cursor: Cursor<T>, require_explicit_match: bool) -> Self {
        Self {
            cursor,
            has_returned_initial_empty_value: false,
            require_explicit_match,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for EllipsisMatcher<T> {
    fn next(&mut self) -> Option<MatcherResult<T>> {
        // First visit, we always return a match for empty case
        if !self.has_returned_initial_empty_value {
            self.has_returned_initial_empty_value = true;
            // We need later matchers to avoid consuming nodes
            return Some(MatcherResult {
                cursor: self.cursor.clone(),
                require_explicit_match: true,
            });
        }

        // Subsequent visits: we only consume nodes if an explicit match is not
        // required, ie. if this is the *first* ellipsis operator in a sibling
        // sequence or there was an explicit match before us.
        if !self.require_explicit_match && self.cursor.irrevocably_go_to_next_sibling() {
            return Some(MatcherResult {
                cursor: self.cursor.clone(),
                require_explicit_match: true,
            });
        }

        None
    }

    fn record_captures(&self, _: &mut BTreeMap<String, Vec<Cursor<T>>>) {}
}

/// Greedily consumes available trivia nodes only
struct AdjacencyMatcher<T: KindTypes> {
    cursor: Option<Cursor<T>>,
    require_explicit_match: bool,
}

impl<T: KindTypes + 'static> AdjacencyMatcher<T> {
    fn new(cursor: Cursor<T>, require_explicit_match: bool) -> Self {
        Self {
            cursor: Some(cursor),
            require_explicit_match,
        }
    }
}

impl<T: KindTypes + 'static> Matcher<T> for AdjacencyMatcher<T> {
    fn next(&mut self) -> Option<MatcherResult<T>> {
        if let Some(mut cursor) = self.cursor.take() {
            while !cursor.is_completed() && cursor.node().is_trivia() {
                cursor.irrevocably_go_to_next_sibling();
            }
            Some(MatcherResult {
                cursor,
                require_explicit_match: self.require_explicit_match,
            })
        } else {
            None
        }
    }

    fn record_captures(&self, _: &mut BTreeMap<String, Vec<Cursor<T>>>) {}
}
