// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::language::ParseResult::*;
use super::language::*;

#[allow(unused_macros)]
macro_rules! scan_predicate {
    ($stream:ident, $predicate:expr) => {
        if let Some(c) = $stream.next() {
            if $predicate(c) {
                true
            } else {
                $stream.undo();
                false
            }
        } else {
            $stream.undo();
            false
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_chars {
    ($stream:ident, $($char:literal),+) => {
        if $( $stream.next() == Some($char) )&&* {
            true
        } else {
            $stream.undo();
            false
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_trie {
    ($stream:ident, EMPTY, $([ $($terminal:literal)|* ])? $(,)? $($prefix:literal + $subtree:expr),*) => {
        ({
            match $stream.next() {
                $($(Some($terminal))|* => true,)?
                $(Some($prefix) => $subtree,)*
                _ => { $stream.undo(); true }
            }
        })
    };
    ($stream:ident, $([ $($terminal:literal)|* ])? $(,)? $($prefix:literal + $subtree:expr),*) => {
        match $stream.next() {
            $($(Some($terminal))|* => true,)?
            $(Some($prefix) => $subtree,)*
            _ => { $stream.undo(); false }
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_sequence {
    ($($expr:expr),*) => {
        $(($expr))&&*
    };
}

#[allow(unused_macros)]
macro_rules! scan_choice {
    ($stream:ident, $($expr:expr),*) => {
        loop {
            let save = $stream.position();
            $(
                if ($expr) { break true }
                $stream.set_position(save);
            )*
            break false
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_zero_or_more {
    ($stream:ident, $expr:expr) => {
        loop {
            let save = $stream.position();
            if !($expr) {
                $stream.set_position(save);
                break true;
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_one_or_more {
    ($stream:ident, $expr:expr) => {{
        let mut count = 0;
        loop {
            let save = $stream.position();
            if !($expr) {
                if count < 1 {
                    break false;
                } else {
                    $stream.set_position(save);
                    break true;
                }
            }
            count += 1;
        }
    }};
}

#[allow(unused_macros)]
macro_rules! scan_repeated {
    ($stream:ident, $expr:expr, $min:literal, $max:literal) => {{
        let mut count = 0;
        loop {
            let save = $stream.position();
            if !($expr) {
                if count < $min {
                    break false;
                } else {
                    $stream.set_position(save);
                    break true;
                }
            }
            count += 1;
            if count == $max {
                break true;
            }
        }
    }};
}

#[allow(unused_macros)]
macro_rules! scan_optional {
    ($stream:ident, $expr:expr) => {{
        let save = $stream.position();
        if !($expr) {
            $stream.set_position(save)
        }
        true
    }};
}

#[allow(unused_macros)]
macro_rules! scan_delimited_by {
    ($stream:ident, [$($open:literal),*], $expr:expr, [$($close:literal),*]) => {
        scan_chars!($stream, $($open),*) && ($expr) && scan_chars!($stream, $($close),*)
    };
}

#[allow(unused_macros)]
macro_rules! scan_separated_by {
    ($stream:ident, $expr:expr, [$($separator:literal),*]) => {
        loop {
            if !($expr) { break false };
            let save = $stream.position();
            if !(scan_chars!($stream, $($separator),*)) {
                $stream.set_position(save);
                break true
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! scan_difference {
    ($stream:ident, $minuend:expr, $subtrahend:expr) => {{
        let start = $stream.position();
        ($minuend)
            && ({
                let end = $stream.position();
                $stream.set_position(start);
                if ($subtrahend) && (end == $stream.position()) {
                    false
                } else {
                    $stream.set_position(end);
                    true
                }
            })
    }};
}

impl Language {
    fn optional_leading_trivia(&self, stream: &mut Stream) -> Option<Rc<cst::Node>> {
        let save = stream.position();
        match self.parse_leading_trivia(stream) {
            Fail { .. } => {
                stream.set_position(save);
                None
            }
            Pass { node, .. } => Some(node),
        }
    }
    fn optional_trailing_trivia(&self, stream: &mut Stream) -> Option<Rc<cst::Node>> {
        let save = stream.position();
        match self.parse_trailing_trivia(stream) {
            Fail { .. } => {
                stream.set_position(save);
                None
            }
            Pass { node, .. } => Some(node),
        }
    }
    // ABICoderPragma = 'abicoder' «Identifier» ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_abi_coder_pragma(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'a', 'b', 'i', 'c', 'o', 'd', 'e', 'r') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Abicoder,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'abicoder'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ABICoderPragma, node),
                error,
            },
            fail => fail,
        }
    }

    // AddSubExpression = Expression ( '+' | '-' ) Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_add_sub_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('+') => Ok(TokenKind::Plus),
                            Some('-') => Ok(TokenKind::Minus),
                            _ => Err(ParseError::new(stream.position(), "'+', or '-'")),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::AddSubExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // AddressType = 'address' [ 'payable' ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_address_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'a', 'd', 'd', 'r', 'e', 's', 's') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Address,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'address'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, 'p', 'a', 'y', 'a', 'b', 'l', 'e') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Payable,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'payable'"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::AddressType, node),
                error,
            },
            fail => fail,
        }
    }

    // AndExpression = Expression '&&' Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_and_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '&', '&') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::AmpersandAmpersand,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'&&'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::AndExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_argument_list(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '(') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::OpenParen,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'('"),
                        }
                    }
                } {
                    err @ Fail { .. } => err,
                    Pass {
                        node: open_node, ..
                    } => {
                        match {
                            let start_position = stream.position();
                            match loop {
                                let start_position = stream.position();
                                let mut furthest_error;
                                match self.parse_positional_argument_list(stream) {
                                    Fail { error } => furthest_error = error,
                                    pass => break pass,
                                }
                                stream.set_position(start_position);
                                match self.parse_named_argument_list(stream) {
                                    Fail { error } => furthest_error.merge_with(error),
                                    pass => break pass,
                                }
                                break Fail {
                                    error: furthest_error,
                                };
                            } {
                                Fail { error } => {
                                    stream.set_position(start_position);
                                    Pass {
                                        node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                        error: Some(error),
                                    }
                                }
                                pass => pass,
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ')') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseParen,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "')'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ArgumentList, node),
                error,
            },
            fail => fail,
        }
    }

    // ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_array_literal(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '[') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::OpenBracket,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'['"),
                        }
                    }
                } {
                    err @ Fail { .. } => err,
                    Pass {
                        node: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                match self.parse_expression(stream) {
                                    err @ Fail { .. } => break err,
                                    Pass { node, .. } => {
                                        result.push(node);
                                        let save = stream.position();
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if scan_chars!(stream, ',') {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::Comma,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "','"),
                                                }
                                            }
                                        } {
                                            Fail { error } => {
                                                stream.set_position(save);
                                                break Pass {
                                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                                    error: Some(error),
                                                };
                                            }
                                            Pass { node, .. } => result.push(node),
                                        }
                                    }
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ']') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseBracket,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "']'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ArrayLiteral, node),
                error,
            },
            fail => fail,
        }
    }

    // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_assembly_flags(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '(') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::OpenParen,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'('"),
                        }
                    }
                } {
                    err @ Fail { .. } => err,
                    Pass {
                        node: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if self.scan_double_quoted_ascii_string_literal(stream) {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::DoubleQuotedAsciiStringLiteral,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(
                                                start,
                                                "DoubleQuotedAsciiStringLiteral",
                                            ),
                                        }
                                    }
                                } {
                                    err @ Fail { .. } => break err,
                                    Pass { node, .. } => {
                                        result.push(node);
                                        let save = stream.position();
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if scan_chars!(stream, ',') {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::Comma,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "','"),
                                                }
                                            }
                                        } {
                                            Fail { error } => {
                                                stream.set_position(save);
                                                break Pass {
                                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                                    error: Some(error),
                                                };
                                            }
                                            Pass { node, .. } => result.push(node),
                                        }
                                    }
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ')') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseParen,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "')'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::AssemblyFlags, node),
                error,
            },
            fail => fail,
        }
    }

    // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_assembly_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'a', 's', 's', 'e', 'm', 'b', 'l', 'y') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Assembly,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'assembly'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '"', 'e', 'v', 'm', 'a', 's', 'm', '"') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::DoubleQuoteEvmasmDoubleQuote,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'\"evmasm\"'"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let start_position = stream.position();
                    match self.parse_assembly_flags(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match self.parse_yul_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::AssemblyStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_assignment_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream . next () { Some ('%') => if scan_chars ! (stream , '=') { Ok (TokenKind :: PercentEqual) } else { Err (ParseError :: new (stream . position () , "'%='")) } , Some ('&') => if scan_chars ! (stream , '=') { Ok (TokenKind :: AmpersandEqual) } else { Err (ParseError :: new (stream . position () , "'&='")) } , Some ('*') => if scan_chars ! (stream , '=') { Ok (TokenKind :: StarEqual) } else { Err (ParseError :: new (stream . position () , "'*='")) } , Some ('+') => if scan_chars ! (stream , '=') { Ok (TokenKind :: PlusEqual) } else { Err (ParseError :: new (stream . position () , "'+='")) } , Some ('-') => if scan_chars ! (stream , '=') { Ok (TokenKind :: MinusEqual) } else { Err (ParseError :: new (stream . position () , "'-='")) } , Some ('/') => if scan_chars ! (stream , '=') { Ok (TokenKind :: SlashEqual) } else { Err (ParseError :: new (stream . position () , "'/='")) } , Some ('<') => if scan_chars ! (stream , '<' , '=') { Ok (TokenKind :: LessLessEqual) } else { Err (ParseError :: new (stream . position () , "'<<='")) } , Some ('=') => Ok (TokenKind :: Equal) , Some ('>') => { if scan_chars ! (stream , '>') { match stream . next () { Some ('=') => Ok (TokenKind :: GreaterGreaterEqual) , Some ('>') => if scan_chars ! (stream , '=') { Ok (TokenKind :: GreaterGreaterGreaterEqual) } else { Err (ParseError :: new (stream . position () , "'>>>='")) } , _ => Err (ParseError :: new (stream . position () , "'>>=', or '>>>='")) } } else { Err (ParseError :: new (stream . position () , "'>>=', or '>>>='")) } } , Some ('^') => if scan_chars ! (stream , '=') { Ok (TokenKind :: CaretEqual) } else { Err (ParseError :: new (stream . position () , "'^='")) } , Some ('|') => if scan_chars ! (stream , '=') { Ok (TokenKind :: PipeEqual) } else { Err (ParseError :: new (stream . position () , "'|='")) } , _ => Err (ParseError :: new (stream . position () , "'%=', or '&=', or '*=', or '+=', or '-=', or '/=', or '<<=', or '=', or '>>=', or '>>>=', or '^=', or '|='")) }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::AssignmentExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // BitAndExpression = Expression '&' Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_bit_and_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '&') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Ampersand,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Ampersand"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::BitAndExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // BitOrExpression = Expression '|' Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_bit_or_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '|') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Pipe,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Pipe"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::BitOrExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // BitXOrExpression = Expression '^' Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_bit_x_or_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '^') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Caret,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Caret"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::BitXOrExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // Block = '{' { Statement | UncheckedBlock } '}' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_block(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '{') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::OpenBrace,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'{'"),
                        }
                    }
                } {
                    err @ Fail { .. } => err,
                    Pass {
                        node: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                let start_position = stream.position();
                                match loop {
                                    let start_position = stream.position();
                                    let mut furthest_error;
                                    match self.parse_statement(stream) {
                                        Fail { error } => furthest_error = error,
                                        pass => break pass,
                                    }
                                    stream.set_position(start_position);
                                    match self.parse_unchecked_block(stream) {
                                        Fail { error } => furthest_error.merge_with(error),
                                        pass => break pass,
                                    }
                                    break Fail {
                                        error: furthest_error,
                                    };
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        break Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, result),
                                            error: Some(error),
                                        };
                                    }
                                    Pass { node, .. } => result.push(node),
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, '}') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseBrace,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "'}'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::Block, node),
                error,
            },
            fail => fail,
        }
    }

    // BreakStatement = 'break' ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_break_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'b', 'r', 'e', 'a', 'k') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Break,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'break'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::BreakStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_catch_clause(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'c', 'a', 't', 'c', 'h') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Catch,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'catch'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let start_position = stream.position();
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if self.scan_identifier(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Identifier,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "Identifier"),
                                    }
                                }
                            } {
                                Fail { error } => {
                                    stream.set_position(start_position);
                                    Pass {
                                        node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                        error: Some(error),
                                    }
                                }
                                pass => pass,
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_parameter_list(stream) {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::CatchClause, node),
                error,
            },
            fail => fail,
        }
    }

    // ConditionalExpression = Expression '?' Expression ':' Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_conditional_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match loop {
                    let mut furthest_error = None;
                    let result_0 = match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '?') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Question,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Question"),
                            }
                        }
                    } {
                        Pass { node, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            node
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_1 = match self.parse_expression(stream) {
                        Pass { node, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            node
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_2 = match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, ':') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Colon,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Colon"),
                            }
                        }
                    } {
                        Pass { node, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            node
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_3 = match self.parse_expression(stream) {
                        Pass { node, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            node
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    break Pass {
                        node: cst::Node::rule(
                            RuleKind::_ANON,
                            vec![result_0, result_1, result_2, result_3],
                        ),
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ConditionalExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_constant_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'c', 'o', 'n', 's', 't', 'a', 'n', 't') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Constant,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'constant'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '=') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Equal,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Equal"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_5 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4, result_5],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ConstantDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_constructor_attribute(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_modifier_invocation(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('i') => {
                                if scan_chars!(stream, 'n', 't', 'e', 'r', 'n', 'a', 'l') {
                                    Ok(TokenKind::Internal)
                                } else {
                                    Err(ParseError::new(stream.position(), "'internal'"))
                                }
                            }
                            Some('p') => match stream.next() {
                                Some('a') => {
                                    if scan_chars!(stream, 'y', 'a', 'b', 'l', 'e') {
                                        Ok(TokenKind::Payable)
                                    } else {
                                        Err(ParseError::new(stream.position(), "'payable'"))
                                    }
                                }
                                Some('u') => {
                                    if scan_chars!(stream, 'b', 'l', 'i', 'c') {
                                        Ok(TokenKind::Public)
                                    } else {
                                        Err(ParseError::new(stream.position(), "'public'"))
                                    }
                                }
                                _ => Err(ParseError::new(
                                    stream.position(),
                                    "'payable', or 'public'",
                                )),
                            },
                            _ => Err(ParseError::new(
                                stream.position(),
                                "'internal', or 'payable', or 'public'",
                            )),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ConstructorAttribute, node),
                error,
            },
            fail => fail,
        }
    }

    // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_constructor_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'c', 'o', 'n', 's', 't', 'r', 'u', 'c', 't', 'o', 'r') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Constructor,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'constructor'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_parameter_list(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match self.parse_constructor_attribute(stream) {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match self.parse_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ConstructorDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // ContinueStatement = 'continue' ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_continue_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'c', 'o', 'n', 't', 'i', 'n', 'u', 'e') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Continue,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'continue'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ContinueStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_contract_body_element(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_using_directive(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_constructor_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_function_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_fallback_function_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_receive_function_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_modifier_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_struct_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_enum_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_user_defined_value_type_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_event_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_error_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_state_variable_declaration(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ContractBodyElement, node),
                error,
            },
            fail => fail,
        }
    }

    // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_contract_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, 'a', 'b', 's', 't', 'r', 'a', 'c', 't') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Abstract,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'abstract'"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'c', 'o', 'n', 't', 'r', 'a', 'c', 't') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Contract,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'contract'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let start_position = stream.position();
                    match self.parse_inheritance_specifier_list(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '{') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenBrace,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'{'"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let mut result = Vec::new();
                                loop {
                                    let start_position = stream.position();
                                    match self.parse_contract_body_element(stream) {
                                        Fail { error } => {
                                            stream.set_position(start_position);
                                            break Pass {
                                                node: cst::Node::rule(RuleKind::_ANON, result),
                                                error: Some(error),
                                            };
                                        }
                                        Pass { node, .. } => result.push(node),
                                    }
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '}') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseBrace,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "'}'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ContractDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // DataLocation = 'memory' | 'storage' | 'calldata' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_data_location(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                match {
                    match stream.next() {
                        Some('c') => {
                            if scan_chars!(stream, 'a', 'l', 'l', 'd', 'a', 't', 'a') {
                                Ok(TokenKind::Calldata)
                            } else {
                                Err(ParseError::new(stream.position(), "'calldata'"))
                            }
                        }
                        Some('m') => {
                            if scan_chars!(stream, 'e', 'm', 'o', 'r', 'y') {
                                Ok(TokenKind::Memory)
                            } else {
                                Err(ParseError::new(stream.position(), "'memory'"))
                            }
                        }
                        Some('s') => {
                            if scan_chars!(stream, 't', 'o', 'r', 'a', 'g', 'e') {
                                Ok(TokenKind::Storage)
                            } else {
                                Err(ParseError::new(stream.position(), "'storage'"))
                            }
                        }
                        _ => Err(ParseError::new(
                            stream.position(),
                            "'calldata', or 'memory', or 'storage'",
                        )),
                    }
                } {
                    Err(mut error) => {
                        error.position = start;
                        Fail { error }
                    }
                    Ok(kind) => {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                kind,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::DataLocation, node),
                error,
            },
            fail => fail,
        }
    }

    // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_contract_definition(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_interface_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_library_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_function_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_constant_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_struct_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_enum_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_user_defined_value_type_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_error_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::Definition, node),
                error,
            },
            fail => fail,
        }
    }

    // DeleteStatement = 'delete' Expression ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_delete_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'd', 'e', 'l', 'e', 't', 'e') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Delete,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'delete'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::DeleteStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // Directive = PragmaDirective | ImportDirective | UsingDirective ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_pragma_directive(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_import_directive(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_using_directive(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::Directive, node),
                error,
            },
            fail => fail,
        }
    }

    // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_do_while_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'd', 'o') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Do,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'do'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_statement(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'w', 'h', 'i', 'l', 'e') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::While,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'while'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => match self.parse_expression(stream) {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ')') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseParen,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "')'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        },
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::DoWhileStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // ElementaryType = 'bool' | 'string' | AddressType | PayableType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_elementary_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('b') => {
                                if scan_chars!(stream, 'o', 'o', 'l') {
                                    Ok(TokenKind::Bool)
                                } else {
                                    Err(ParseError::new(stream.position(), "'bool'"))
                                }
                            }
                            Some('s') => {
                                if scan_chars!(stream, 't', 'r', 'i', 'n', 'g') {
                                    Ok(TokenKind::String)
                                } else {
                                    Err(ParseError::new(stream.position(), "'string'"))
                                }
                            }
                            _ => Err(ParseError::new(stream.position(), "'bool', or 'string'")),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_address_type(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_payable_type(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_fixed_bytes_type(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::FixedBytesType,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "FixedBytesType"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_signed_integer_type(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::SignedIntegerType,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "SignedIntegerType"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_unsigned_integer_type(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::UnsignedIntegerType,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "UnsignedIntegerType"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_signed_fixed_type(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::SignedFixedType,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "SignedFixedType"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_unsigned_fixed_type(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::UnsignedFixedType,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "UnsignedFixedType"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ElementaryType, node),
                error,
            },
            fail => fail,
        }
    }

    // EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_emit_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'e', 'm', 'i', 't') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Emit,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'emit'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_identifier_path(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_argument_list(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::EmitStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // EndOfFileTrivia = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_end_of_file_trivia(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let mut result = Vec::new();
                loop {
                    let start_position = stream.position();
                    match loop {
                        let start_position = stream.position();
                        let mut furthest_error;
                        match {
                            let start = stream.position();
                            if self.scan_whitespace(stream) {
                                let end = stream.position();
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Whitespace,
                                        Range { start, end },
                                        None,
                                        None,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "Whitespace"),
                                }
                            }
                        } {
                            Fail { error } => furthest_error = error,
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match {
                            let start = stream.position();
                            if self.scan_end_of_line(stream) {
                                let end = stream.position();
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::EndOfLine,
                                        Range { start, end },
                                        None,
                                        None,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "EndOfLine"),
                                }
                            }
                        } {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match {
                            let start = stream.position();
                            if self.scan_multiline_comment(stream) {
                                let end = stream.position();
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::MultilineComment,
                                        Range { start, end },
                                        None,
                                        None,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "MultilineComment"),
                                }
                            }
                        } {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match {
                            let start = stream.position();
                            if self.scan_single_line_comment(stream) {
                                let end = stream.position();
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::SingleLineComment,
                                        Range { start, end },
                                        None,
                                        None,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "SingleLineComment"),
                                }
                            }
                        } {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        break Fail {
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            break Pass {
                                node: cst::Node::rule(RuleKind::_ANON, result),
                                error: Some(error),
                            };
                        }
                        Pass { node, .. } => result.push(node),
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::EndOfFileTrivia, node),
                error,
            },
            fail => fail,
        }
    }

    // EnumDefinition = 'enum' «Identifier» '{' [ «Identifier»  { ',' «Identifier» } ] '}' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_enum_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'e', 'n', 'u', 'm') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Enum,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'enum'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '{') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenBrace,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'{'"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let start_position = stream.position();
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if self.scan_identifier(stream) {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::Identifier,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "Identifier"),
                                                }
                                            }
                                        } {
                                            err @ Fail { .. } => break err,
                                            Pass { node, .. } => {
                                                result.push(node);
                                                let save = stream.position();
                                                match {
                                                    let leading_trivia =
                                                        self.optional_leading_trivia(stream);
                                                    let start = stream.position();
                                                    if scan_chars!(stream, ',') {
                                                        let end = stream.position();
                                                        let trailing_trivia =
                                                            self.optional_trailing_trivia(stream);
                                                        Pass {
                                                            node: cst::Node::token(
                                                                TokenKind::Comma,
                                                                Range { start, end },
                                                                leading_trivia,
                                                                trailing_trivia,
                                                            ),
                                                            error: None,
                                                        }
                                                    } else {
                                                        Fail {
                                                            error: ParseError::new(start, "','"),
                                                        }
                                                    }
                                                } {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::_ANON,
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { node, .. } => result.push(node),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '}') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseBrace,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "'}'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::EnumDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_equality_comparison_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('!') => {
                                if scan_chars!(stream, '=') {
                                    Ok(TokenKind::BangEqual)
                                } else {
                                    Err(ParseError::new(stream.position(), "'!='"))
                                }
                            }
                            Some('=') => {
                                if scan_chars!(stream, '=') {
                                    Ok(TokenKind::EqualEqual)
                                } else {
                                    Err(ParseError::new(stream.position(), "'=='"))
                                }
                            }
                            _ => Err(ParseError::new(stream.position(), "'!=', or '=='")),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::EqualityComparisonExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_error_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'e', 'r', 'r', 'o', 'r') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Error,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'error'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let start_position = stream.position();
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match self.parse_error_parameter(stream) {
                                            err @ Fail { .. } => break err,
                                            Pass { node, .. } => {
                                                result.push(node);
                                                let save = stream.position();
                                                match {
                                                    let leading_trivia =
                                                        self.optional_leading_trivia(stream);
                                                    let start = stream.position();
                                                    if scan_chars!(stream, ',') {
                                                        let end = stream.position();
                                                        let trailing_trivia =
                                                            self.optional_trailing_trivia(stream);
                                                        Pass {
                                                            node: cst::Node::token(
                                                                TokenKind::Comma,
                                                                Range { start, end },
                                                                leading_trivia,
                                                                trailing_trivia,
                                                            ),
                                                            error: None,
                                                        }
                                                    } else {
                                                        Fail {
                                                            error: ParseError::new(start, "','"),
                                                        }
                                                    }
                                                } {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::_ANON,
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { node, .. } => result.push(node),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ')') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseParen,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "')'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ErrorDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // ErrorParameter = TypeName [ «Identifier» ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_error_parameter(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_identifier(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Identifier,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Identifier"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ErrorParameter, node),
                error,
            },
            fail => fail,
        }
    }

    // EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_event_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'e', 'v', 'e', 'n', 't') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Event,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'event'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let start_position = stream.position();
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match self.parse_event_parameter(stream) {
                                            err @ Fail { .. } => break err,
                                            Pass { node, .. } => {
                                                result.push(node);
                                                let save = stream.position();
                                                match {
                                                    let leading_trivia =
                                                        self.optional_leading_trivia(stream);
                                                    let start = stream.position();
                                                    if scan_chars!(stream, ',') {
                                                        let end = stream.position();
                                                        let trailing_trivia =
                                                            self.optional_trailing_trivia(stream);
                                                        Pass {
                                                            node: cst::Node::token(
                                                                TokenKind::Comma,
                                                                Range { start, end },
                                                                leading_trivia,
                                                                trailing_trivia,
                                                            ),
                                                            error: None,
                                                        }
                                                    } else {
                                                        Fail {
                                                            error: ParseError::new(start, "','"),
                                                        }
                                                    }
                                                } {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::_ANON,
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { node, .. } => result.push(node),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ')') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseParen,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "')'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, 'a', 'n', 'o', 'n', 'y', 'm', 'o', 'u', 's') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Anonymous,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'anonymous'"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::EventDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_event_parameter(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, 'i', 'n', 'd', 'e', 'x', 'e', 'd') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Indexed,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'indexed'"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_identifier(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Identifier,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Identifier"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::EventParameter, node),
                error,
            },
            fail => fail,
        }
    }

    // ExperimentalPragma = 'experimental' «Identifier» ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_experimental_pragma(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(
                        stream, 'e', 'x', 'p', 'e', 'r', 'i', 'm', 'e', 'n', 't', 'a', 'l'
                    ) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Experimental,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'experimental'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ExperimentalPragma, node),
                error,
            },
            fail => fail,
        }
    }

    // (* 0.4.11 *) ExponentiationExpression = Expression '**' Expression ;
    // (* 0.6.0 *) ExponentiationExpression = Expression '**' Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_exponentiation_expression(&self, stream: &mut Stream) -> ParseResult {
        match if self.version_is_equal_to_or_greater_than_0_6_0 {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '*', '*') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::StarStar,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'**'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } else {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '*', '*') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::StarStar,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'**'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ExponentiationExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // (* 0.4.11 *) Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    // (* 0.6.0 *) Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_expression(&self, stream: &mut Stream) -> ParseResult {
        match if self.version_is_equal_to_or_greater_than_0_6_0 {
            {
                enum Pratt {
                    Operator {
                        kind: RuleKind,
                        node: Rc<cst::Node>,
                        left_binding_power: u8,
                        right_binding_power: u8,
                    },
                    Node(Rc<cst::Node>),
                }
                let mut elements = Vec::new();
                if let Some(error) = loop {
                    loop {
                        let start_position = stream.position();
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            match {
                                match stream.next() {
                                    Some('!') => Ok((
                                        TokenKind::Bang,
                                        RuleKind::UnaryPrefixExpression,
                                        255u8,
                                        29u8,
                                    )),
                                    Some('+') => {
                                        if scan_chars!(stream, '+') {
                                            Ok((
                                                TokenKind::PlusPlus,
                                                RuleKind::UnaryPrefixExpression,
                                                255u8,
                                                29u8,
                                            ))
                                        } else {
                                            Err(ParseError::new(stream.position(), "'++'"))
                                        }
                                    }
                                    Some('-') => {
                                        let start_position = stream.position();
                                        match stream.next() {
                                            Some('-') => Ok((
                                                TokenKind::MinusMinus,
                                                RuleKind::UnaryPrefixExpression,
                                                255u8,
                                                29u8,
                                            )),
                                            Some(_) => {
                                                stream.set_position(start_position);
                                                Ok((
                                                    TokenKind::Minus,
                                                    RuleKind::UnaryPrefixExpression,
                                                    255u8,
                                                    29u8,
                                                ))
                                            }
                                            None => Ok((
                                                TokenKind::Minus,
                                                RuleKind::UnaryPrefixExpression,
                                                255u8,
                                                29u8,
                                            )),
                                        }
                                    }
                                    Some('~') => Ok((
                                        TokenKind::Tilde,
                                        RuleKind::UnaryPrefixExpression,
                                        255u8,
                                        29u8,
                                    )),
                                    _ => Err(ParseError::new(
                                        stream.position(),
                                        "'!', or '++', or '-', or '--', or '~'",
                                    )),
                                }
                            } {
                                Ok((
                                    token_kind,
                                    rule_kind,
                                    left_binding_power,
                                    right_binding_power,
                                )) => {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Ok(Pratt::Operator {
                                        node: cst::Node::token(
                                            token_kind,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        kind: rule_kind,
                                        left_binding_power,
                                        right_binding_power,
                                    })
                                }
                                Err(error) => Err(error),
                            }
                        } {
                            Err(_) => {
                                stream.set_position(start_position);
                                break;
                            }
                            Ok(operator) => elements.push(operator),
                        }
                    }
                    match self.parse_primary_expression(stream) {
                        Fail { error } => break Some(error),
                        Pass { node, .. } => elements.push(Pratt::Node(node)),
                    }
                    loop {
                        let start_position = stream.position();
                        match loop {
                            let start_position = stream.position();
                            let mut furthest_error;
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                match {
                                    match stream.next() {
                                        Some('+') => {
                                            if scan_chars!(stream, '+') {
                                                Ok((
                                                    TokenKind::PlusPlus,
                                                    RuleKind::UnarySuffixExpression,
                                                    27u8,
                                                    255u8,
                                                ))
                                            } else {
                                                Err(ParseError::new(stream.position(), "'++'"))
                                            }
                                        }
                                        Some('-') => {
                                            if scan_chars!(stream, '-') {
                                                Ok((
                                                    TokenKind::MinusMinus,
                                                    RuleKind::UnarySuffixExpression,
                                                    27u8,
                                                    255u8,
                                                ))
                                            } else {
                                                Err(ParseError::new(stream.position(), "'--'"))
                                            }
                                        }
                                        _ => {
                                            Err(ParseError::new(stream.position(), "'++', or '--'"))
                                        }
                                    }
                                } {
                                    Ok((
                                        token_kind,
                                        rule_kind,
                                        left_binding_power,
                                        right_binding_power,
                                    )) => {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Ok(Pratt::Operator {
                                            node: cst::Node::token(
                                                token_kind,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            kind: rule_kind,
                                            left_binding_power,
                                            right_binding_power,
                                        })
                                    }
                                    Err(error) => Err(error),
                                }
                            } {
                                Err(error) => furthest_error = error,
                                ok => break ok,
                            }
                            stream.set_position(start_position);
                            match {
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '?') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Question,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "Question"),
                                            }
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match self.parse_expression(stream) {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_2 = match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ':') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Colon,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "Colon"),
                                            }
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_3 = match self.parse_expression(stream) {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![result_0, result_1, result_2, result_3],
                                        ),
                                        error: furthest_error,
                                    };
                                } {
                                    Pass { node, .. } => Ok(Pratt::Operator {
                                        node,
                                        kind: RuleKind::ConditionalExpression,
                                        left_binding_power: 3u8,
                                        right_binding_power: 255,
                                    }),
                                    Fail { error } => Err(error),
                                }
                            } {
                                Err(error) => {
                                    if furthest_error.position < error.position {
                                        furthest_error = error
                                    } else if furthest_error.position == error.position {
                                        furthest_error.expected = format!(
                                            "{}, or {}",
                                            furthest_error.expected, error.expected
                                        )
                                    }
                                }
                                ok => break ok,
                            }
                            stream.set_position(start_position);
                            match {
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match {
                                        let start_position = stream.position();
                                        match self.parse_named_argument_list(stream) {
                                            Fail { error } => {
                                                stream.set_position(start_position);
                                                Pass {
                                                    node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                                    error: Some(error),
                                                }
                                            }
                                            pass => pass,
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match self.parse_argument_list(stream) {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![result_0, result_1],
                                        ),
                                        error: furthest_error,
                                    };
                                } {
                                    Pass { node, .. } => Ok(Pratt::Operator {
                                        node,
                                        kind: RuleKind::FunctionCallExpression,
                                        left_binding_power: 31u8,
                                        right_binding_power: 255,
                                    }),
                                    Fail { error } => Err(error),
                                }
                            } {
                                Err(error) => {
                                    if furthest_error.position < error.position {
                                        furthest_error = error
                                    } else if furthest_error.position == error.position {
                                        furthest_error.expected = format!(
                                            "{}, or {}",
                                            furthest_error.expected, error.expected
                                        )
                                    }
                                }
                                ok => break ok,
                            }
                            stream.set_position(start_position);
                            match {
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '.') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Period,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "Period"),
                                            }
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match loop {
                                        let start_position = stream.position();
                                        let mut furthest_error;
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if self.scan_identifier(stream) {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::Identifier,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "Identifier"),
                                                }
                                            }
                                        } {
                                            Fail { error } => furthest_error = error,
                                            pass => break pass,
                                        }
                                        stream.set_position(start_position);
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if scan_chars!(
                                                stream, 'a', 'd', 'd', 'r', 'e', 's', 's'
                                            ) {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::Address,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "'address'"),
                                                }
                                            }
                                        } {
                                            Fail { error } => furthest_error.merge_with(error),
                                            pass => break pass,
                                        }
                                        break Fail {
                                            error: furthest_error,
                                        };
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![result_0, result_1],
                                        ),
                                        error: furthest_error,
                                    };
                                } {
                                    Pass { node, .. } => Ok(Pratt::Operator {
                                        node,
                                        kind: RuleKind::MemberAccessExpression,
                                        left_binding_power: 33u8,
                                        right_binding_power: 255,
                                    }),
                                    Fail { error } => Err(error),
                                }
                            } {
                                Err(error) => {
                                    if furthest_error.position < error.position {
                                        furthest_error = error
                                    } else if furthest_error.position == error.position {
                                        furthest_error.expected = format!(
                                            "{}, or {}",
                                            furthest_error.expected, error.expected
                                        )
                                    }
                                }
                                ok => break ok,
                            }
                            stream.set_position(start_position);
                            match {
                                match {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '[') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::OpenBracket,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "'['"),
                                            }
                                        }
                                    } {
                                        err @ Fail { .. } => err,
                                        Pass {
                                            node: open_node, ..
                                        } => {
                                            match loop {
                                                let start_position = stream.position();
                                                let mut furthest_error;
                                                match loop {
                                                    let mut furthest_error = None;
                                                    let result_0 = match self
                                                        .parse_expression(stream)
                                                    {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_1 = match {
                                                        let start_position = stream.position();
                                                        match loop {
                                                            let mut furthest_error = None;
                                                            let result_0 = match {
                                                                let leading_trivia = self
                                                                    .optional_leading_trivia(
                                                                        stream,
                                                                    );
                                                                let start = stream.position();
                                                                if scan_chars!(stream, ':') {
                                                                    let end = stream.position();
                                                                    let trailing_trivia = self
                                                                        .optional_trailing_trivia(
                                                                            stream,
                                                                        );
                                                                    Pass {
                                                                        node: cst::Node::token(
                                                                            TokenKind::Colon,
                                                                            Range { start, end },
                                                                            leading_trivia,
                                                                            trailing_trivia,
                                                                        ),
                                                                        error: None,
                                                                    }
                                                                } else {
                                                                    Fail {
                                                                        error: ParseError::new(
                                                                            start, "Colon",
                                                                        ),
                                                                    }
                                                                }
                                                            } {
                                                                Pass { node, error } => {
                                                                    furthest_error =
                                                                        error.map(|error| {
                                                                            error.maybe_merge_with(
                                                                                furthest_error,
                                                                            )
                                                                        });
                                                                    node
                                                                }
                                                                Fail { error } => {
                                                                    break Fail {
                                                                        error: error
                                                                            .maybe_merge_with(
                                                                                furthest_error,
                                                                            ),
                                                                    }
                                                                }
                                                            };
                                                            let result_1 = match {
                                                                let start_position =
                                                                    stream.position();
                                                                match self.parse_expression(stream)
                                                                {
                                                                    Fail { error } => {
                                                                        stream.set_position(
                                                                            start_position,
                                                                        );
                                                                        Pass {
                                                                            node: cst::Node::rule(
                                                                                RuleKind::_ANON,
                                                                                vec![],
                                                                            ),
                                                                            error: Some(error),
                                                                        }
                                                                    }
                                                                    pass => pass,
                                                                }
                                                            } {
                                                                Pass { node, error } => {
                                                                    furthest_error =
                                                                        error.map(|error| {
                                                                            error.maybe_merge_with(
                                                                                furthest_error,
                                                                            )
                                                                        });
                                                                    node
                                                                }
                                                                Fail { error } => {
                                                                    break Fail {
                                                                        error: error
                                                                            .maybe_merge_with(
                                                                                furthest_error,
                                                                            ),
                                                                    }
                                                                }
                                                            };
                                                            break Pass {
                                                                node: cst::Node::rule(
                                                                    RuleKind::_ANON,
                                                                    vec![result_0, result_1],
                                                                ),
                                                                error: furthest_error,
                                                            };
                                                        } {
                                                            Fail { error } => {
                                                                stream.set_position(start_position);
                                                                Pass {
                                                                    node: cst::Node::rule(
                                                                        RuleKind::_ANON,
                                                                        vec![],
                                                                    ),
                                                                    error: Some(error),
                                                                }
                                                            }
                                                            pass => pass,
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    break Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            vec![result_0, result_1],
                                                        ),
                                                        error: furthest_error,
                                                    };
                                                } {
                                                    Fail { error } => furthest_error = error,
                                                    pass => break pass,
                                                }
                                                stream.set_position(start_position);
                                                match loop {
                                                    let mut furthest_error = None;
                                                    let result_0 = match {
                                                        let leading_trivia =
                                                            self.optional_leading_trivia(stream);
                                                        let start = stream.position();
                                                        if scan_chars!(stream, ':') {
                                                            let end = stream.position();
                                                            let trailing_trivia = self
                                                                .optional_trailing_trivia(stream);
                                                            Pass {
                                                                node: cst::Node::token(
                                                                    TokenKind::Colon,
                                                                    Range { start, end },
                                                                    leading_trivia,
                                                                    trailing_trivia,
                                                                ),
                                                                error: None,
                                                            }
                                                        } else {
                                                            Fail {
                                                                error: ParseError::new(
                                                                    start, "Colon",
                                                                ),
                                                            }
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_1 = match {
                                                        let start_position = stream.position();
                                                        match self.parse_expression(stream) {
                                                            Fail { error } => {
                                                                stream.set_position(start_position);
                                                                Pass {
                                                                    node: cst::Node::rule(
                                                                        RuleKind::_ANON,
                                                                        vec![],
                                                                    ),
                                                                    error: Some(error),
                                                                }
                                                            }
                                                            pass => pass,
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    break Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            vec![result_0, result_1],
                                                        ),
                                                        error: furthest_error,
                                                    };
                                                } {
                                                    Fail { error } => {
                                                        furthest_error.merge_with(error)
                                                    }
                                                    pass => break pass,
                                                }
                                                break Fail {
                                                    error: furthest_error,
                                                };
                                            } {
                                                err @ Fail { .. } => err,
                                                Pass {
                                                    node: expr_node,
                                                    error: expr_error,
                                                } => {
                                                    match {
                                                        let leading_trivia =
                                                            self.optional_leading_trivia(stream);
                                                        let start = stream.position();
                                                        if scan_chars!(stream, ']') {
                                                            let end = stream.position();
                                                            let trailing_trivia = self
                                                                .optional_trailing_trivia(stream);
                                                            Pass {
                                                                node: cst::Node::token(
                                                                    TokenKind::CloseBracket,
                                                                    Range { start, end },
                                                                    leading_trivia,
                                                                    trailing_trivia,
                                                                ),
                                                                error: None,
                                                            }
                                                        } else {
                                                            Fail {
                                                                error: ParseError::new(
                                                                    start, "']'",
                                                                ),
                                                            }
                                                        }
                                                    } {
                                                        Fail { error } => Fail {
                                                            error: error
                                                                .maybe_merge_with(expr_error),
                                                        },
                                                        Pass {
                                                            node: close_node, ..
                                                        } => Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::_ANON,
                                                                vec![
                                                                    open_node, expr_node,
                                                                    close_node,
                                                                ],
                                                            ),
                                                            error: None,
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Pass { node, .. } => Ok(Pratt::Operator {
                                        node,
                                        kind: RuleKind::IndexAccessExpression,
                                        left_binding_power: 35u8,
                                        right_binding_power: 255,
                                    }),
                                    Fail { error } => Err(error),
                                }
                            } {
                                Err(error) => {
                                    if furthest_error.position < error.position {
                                        furthest_error = error
                                    } else if furthest_error.position == error.position {
                                        furthest_error.expected = format!(
                                            "{}, or {}",
                                            furthest_error.expected, error.expected
                                        )
                                    }
                                }
                                ok => break ok,
                            }
                            break Err(furthest_error);
                        } {
                            Err(_) => {
                                stream.set_position(start_position);
                                break;
                            }
                            Ok(operator) => elements.push(operator),
                        }
                    }
                    let start_position = stream.position();
                    match loop {
                        let start_position = stream.position();
                        let mut furthest_error;
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            match {
                                match stream . next () { Some ('!') => if scan_chars ! (stream , '=') { Ok ((TokenKind :: BangEqual , RuleKind :: EqualityComparisonExpression , 9u8 , 10u8)) } else { Err (ParseError :: new (stream . position () , "'!='")) } , Some ('%') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PercentEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Percent , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Percent , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('&') => { match stream . next () { Some ('&') => Ok ((TokenKind :: AmpersandAmpersand , RuleKind :: AndExpression , 7u8 , 8u8)) , Some ('=') => Ok ((TokenKind :: AmpersandEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , _ => Err (ParseError :: new (stream . position () , "'&&', or '&='")) } } , Some ('*') => { let start_position = stream . position () ; match stream . next () { Some ('*') => Ok ((TokenKind :: StarStar , RuleKind :: ExponentiationExpression , 26u8 , 25u8)) , Some ('=') => Ok ((TokenKind :: StarEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Star , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Star , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('+') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PlusEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Plus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } None => Ok ((TokenKind :: Plus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } } , Some ('-') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: MinusEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Minus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } None => Ok ((TokenKind :: Minus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } } , Some ('/') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: SlashEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Slash , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Slash , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('<') => { let start_position = stream . position () ; match stream . next () { Some ('<') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: LessLessEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: LessLess , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: LessLess , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some ('=') => Ok ((TokenKind :: LessEqual , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Less , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } None => Ok ((TokenKind :: Less , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } } , Some ('=') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: EqualEqual , RuleKind :: EqualityComparisonExpression , 9u8 , 10u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Equal , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } None => Ok ((TokenKind :: Equal , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } } , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterEqual , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterGreaterEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterGreaterGreaterEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: GreaterGreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: GreaterGreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: GreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: GreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Greater , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } None => Ok ((TokenKind :: Greater , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } } , Some ('^') => if scan_chars ! (stream , '=') { Ok ((TokenKind :: CaretEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } else { Err (ParseError :: new (stream . position () , "'^='")) } , Some ('|') => { match stream . next () { Some ('=') => Ok ((TokenKind :: PipeEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some ('|') => Ok ((TokenKind :: PipePipe , RuleKind :: OrExpression , 5u8 , 6u8)) , _ => Err (ParseError :: new (stream . position () , "'|=', or '||'")) } } , _ => Err (ParseError :: new (stream . position () , "'!=', or '%', or '%=', or '&&', or '&=', or '*', or '**', or '*=', or '+', or '+=', or '-', or '-=', or '/', or '/=', or '<', or '<<', or '<<=', or '<=', or '=', or '==', or '>', or '>=', or '>>', or '>>=', or '>>>', or '>>>=', or '^=', or '|=', or '||'")) }
                            } {
                                Ok((
                                    token_kind,
                                    rule_kind,
                                    left_binding_power,
                                    right_binding_power,
                                )) => {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Ok(Pratt::Operator {
                                        node: cst::Node::token(
                                            token_kind,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        kind: rule_kind,
                                        left_binding_power,
                                        right_binding_power,
                                    })
                                }
                                Err(error) => Err(error),
                            }
                        } {
                            Err(error) => furthest_error = error,
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, '|') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Pipe,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "Pipe"),
                                    }
                                }
                            } {
                                Pass { node, .. } => Ok(Pratt::Operator {
                                    node,
                                    kind: RuleKind::BitOrExpression,
                                    left_binding_power: 13u8,
                                    right_binding_power: 13u8 + 1,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => {
                                if furthest_error.position < error.position {
                                    furthest_error = error
                                } else if furthest_error.position == error.position {
                                    furthest_error.expected = format!(
                                        "{}, or {}",
                                        furthest_error.expected, error.expected
                                    )
                                }
                            }
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, '^') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Caret,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "Caret"),
                                    }
                                }
                            } {
                                Pass { node, .. } => Ok(Pratt::Operator {
                                    node,
                                    kind: RuleKind::BitXOrExpression,
                                    left_binding_power: 15u8,
                                    right_binding_power: 15u8 + 1,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => {
                                if furthest_error.position < error.position {
                                    furthest_error = error
                                } else if furthest_error.position == error.position {
                                    furthest_error.expected = format!(
                                        "{}, or {}",
                                        furthest_error.expected, error.expected
                                    )
                                }
                            }
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, '&') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Ampersand,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "Ampersand"),
                                    }
                                }
                            } {
                                Pass { node, .. } => Ok(Pratt::Operator {
                                    node,
                                    kind: RuleKind::BitAndExpression,
                                    left_binding_power: 17u8,
                                    right_binding_power: 17u8 + 1,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => {
                                if furthest_error.position < error.position {
                                    furthest_error = error
                                } else if furthest_error.position == error.position {
                                    furthest_error.expected = format!(
                                        "{}, or {}",
                                        furthest_error.expected, error.expected
                                    )
                                }
                            }
                            ok => break ok,
                        }
                        break Err(furthest_error);
                    } {
                        Err(_) => {
                            stream.set_position(start_position);
                            break None;
                        }
                        Ok(operator) => elements.push(operator),
                    }
                } {
                    Fail { error }
                } else {
                    let mut i = 0;
                    while elements.len() > 1 {
                        if let Pratt::Operator {
                            right_binding_power,
                            left_binding_power,
                            ..
                        } = &elements[i]
                        {
                            let next_left_binding_power = if elements.len() == i + 1 {
                                0
                            } else if let Pratt::Operator {
                                left_binding_power, ..
                            } = &elements[i + 1]
                            {
                                *left_binding_power
                            } else if elements.len() == i + 2 {
                                0
                            } else if let Pratt::Operator {
                                left_binding_power, ..
                            } = &elements[i + 2]
                            {
                                *left_binding_power
                            } else {
                                0
                            };
                            if *right_binding_power <= next_left_binding_power {
                                i += 1;
                                continue;
                            }
                            if *right_binding_power == 255 {
                                let left = elements.remove(i - 1);
                                let op = elements.remove(i - 1);
                                if let (Pratt::Node(left), Pratt::Operator { node: op, kind, .. }) =
                                    (left, op)
                                {
                                    let node = cst::Node::rule(kind, vec![left, op]);
                                    elements.insert(i - 1, Pratt::Node(node));
                                    i = i.saturating_sub(2);
                                } else {
                                    unreachable!()
                                }
                            } else if *left_binding_power == 255 {
                                let op = elements.remove(i);
                                let right = elements.remove(i);
                                if let (
                                    Pratt::Operator { node: op, kind, .. },
                                    Pratt::Node(right),
                                ) = (op, right)
                                {
                                    let node = cst::Node::rule(kind, vec![op, right]);
                                    elements.insert(i, Pratt::Node(node));
                                    i = i.saturating_sub(1);
                                } else {
                                    unreachable!()
                                }
                            } else {
                                let left = elements.remove(i - 1);
                                let op = elements.remove(i - 1);
                                let right = elements.remove(i - 1);
                                if let (
                                    Pratt::Node(left),
                                    Pratt::Operator { node: op, kind, .. },
                                    Pratt::Node(right),
                                ) = (left, op, right)
                                {
                                    let node = cst::Node::rule(kind, vec![left, op, right]);
                                    elements.insert(i - 1, Pratt::Node(node));
                                    i = i.saturating_sub(2);
                                } else {
                                    unreachable!()
                                }
                            }
                        } else {
                            i += 1;
                        }
                    }
                    if let Pratt::Node(node) = elements.pop().unwrap() {
                        Pass { node, error: None }
                    } else {
                        unreachable!()
                    }
                }
            }
        } else {
            {
                enum Pratt {
                    Operator {
                        kind: RuleKind,
                        node: Rc<cst::Node>,
                        left_binding_power: u8,
                        right_binding_power: u8,
                    },
                    Node(Rc<cst::Node>),
                }
                let mut elements = Vec::new();
                if let Some(error) = loop {
                    loop {
                        let start_position = stream.position();
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            match {
                                match stream.next() {
                                    Some('!') => Ok((
                                        TokenKind::Bang,
                                        RuleKind::UnaryPrefixExpression,
                                        255u8,
                                        29u8,
                                    )),
                                    Some('+') => {
                                        if scan_chars!(stream, '+') {
                                            Ok((
                                                TokenKind::PlusPlus,
                                                RuleKind::UnaryPrefixExpression,
                                                255u8,
                                                29u8,
                                            ))
                                        } else {
                                            Err(ParseError::new(stream.position(), "'++'"))
                                        }
                                    }
                                    Some('-') => {
                                        let start_position = stream.position();
                                        match stream.next() {
                                            Some('-') => Ok((
                                                TokenKind::MinusMinus,
                                                RuleKind::UnaryPrefixExpression,
                                                255u8,
                                                29u8,
                                            )),
                                            Some(_) => {
                                                stream.set_position(start_position);
                                                Ok((
                                                    TokenKind::Minus,
                                                    RuleKind::UnaryPrefixExpression,
                                                    255u8,
                                                    29u8,
                                                ))
                                            }
                                            None => Ok((
                                                TokenKind::Minus,
                                                RuleKind::UnaryPrefixExpression,
                                                255u8,
                                                29u8,
                                            )),
                                        }
                                    }
                                    Some('~') => Ok((
                                        TokenKind::Tilde,
                                        RuleKind::UnaryPrefixExpression,
                                        255u8,
                                        29u8,
                                    )),
                                    _ => Err(ParseError::new(
                                        stream.position(),
                                        "'!', or '++', or '-', or '--', or '~'",
                                    )),
                                }
                            } {
                                Ok((
                                    token_kind,
                                    rule_kind,
                                    left_binding_power,
                                    right_binding_power,
                                )) => {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Ok(Pratt::Operator {
                                        node: cst::Node::token(
                                            token_kind,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        kind: rule_kind,
                                        left_binding_power,
                                        right_binding_power,
                                    })
                                }
                                Err(error) => Err(error),
                            }
                        } {
                            Err(_) => {
                                stream.set_position(start_position);
                                break;
                            }
                            Ok(operator) => elements.push(operator),
                        }
                    }
                    match self.parse_primary_expression(stream) {
                        Fail { error } => break Some(error),
                        Pass { node, .. } => elements.push(Pratt::Node(node)),
                    }
                    loop {
                        let start_position = stream.position();
                        match loop {
                            let start_position = stream.position();
                            let mut furthest_error;
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                match {
                                    match stream.next() {
                                        Some('+') => {
                                            if scan_chars!(stream, '+') {
                                                Ok((
                                                    TokenKind::PlusPlus,
                                                    RuleKind::UnarySuffixExpression,
                                                    27u8,
                                                    255u8,
                                                ))
                                            } else {
                                                Err(ParseError::new(stream.position(), "'++'"))
                                            }
                                        }
                                        Some('-') => {
                                            if scan_chars!(stream, '-') {
                                                Ok((
                                                    TokenKind::MinusMinus,
                                                    RuleKind::UnarySuffixExpression,
                                                    27u8,
                                                    255u8,
                                                ))
                                            } else {
                                                Err(ParseError::new(stream.position(), "'--'"))
                                            }
                                        }
                                        _ => {
                                            Err(ParseError::new(stream.position(), "'++', or '--'"))
                                        }
                                    }
                                } {
                                    Ok((
                                        token_kind,
                                        rule_kind,
                                        left_binding_power,
                                        right_binding_power,
                                    )) => {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Ok(Pratt::Operator {
                                            node: cst::Node::token(
                                                token_kind,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            kind: rule_kind,
                                            left_binding_power,
                                            right_binding_power,
                                        })
                                    }
                                    Err(error) => Err(error),
                                }
                            } {
                                Err(error) => furthest_error = error,
                                ok => break ok,
                            }
                            stream.set_position(start_position);
                            match {
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '?') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Question,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "Question"),
                                            }
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match self.parse_expression(stream) {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_2 = match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ':') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Colon,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "Colon"),
                                            }
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_3 = match self.parse_expression(stream) {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![result_0, result_1, result_2, result_3],
                                        ),
                                        error: furthest_error,
                                    };
                                } {
                                    Pass { node, .. } => Ok(Pratt::Operator {
                                        node,
                                        kind: RuleKind::ConditionalExpression,
                                        left_binding_power: 3u8,
                                        right_binding_power: 255,
                                    }),
                                    Fail { error } => Err(error),
                                }
                            } {
                                Err(error) => {
                                    if furthest_error.position < error.position {
                                        furthest_error = error
                                    } else if furthest_error.position == error.position {
                                        furthest_error.expected = format!(
                                            "{}, or {}",
                                            furthest_error.expected, error.expected
                                        )
                                    }
                                }
                                ok => break ok,
                            }
                            stream.set_position(start_position);
                            match {
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match {
                                        let start_position = stream.position();
                                        match self.parse_named_argument_list(stream) {
                                            Fail { error } => {
                                                stream.set_position(start_position);
                                                Pass {
                                                    node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                                    error: Some(error),
                                                }
                                            }
                                            pass => pass,
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match self.parse_argument_list(stream) {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![result_0, result_1],
                                        ),
                                        error: furthest_error,
                                    };
                                } {
                                    Pass { node, .. } => Ok(Pratt::Operator {
                                        node,
                                        kind: RuleKind::FunctionCallExpression,
                                        left_binding_power: 31u8,
                                        right_binding_power: 255,
                                    }),
                                    Fail { error } => Err(error),
                                }
                            } {
                                Err(error) => {
                                    if furthest_error.position < error.position {
                                        furthest_error = error
                                    } else if furthest_error.position == error.position {
                                        furthest_error.expected = format!(
                                            "{}, or {}",
                                            furthest_error.expected, error.expected
                                        )
                                    }
                                }
                                ok => break ok,
                            }
                            stream.set_position(start_position);
                            match {
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '.') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Period,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "Period"),
                                            }
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match loop {
                                        let start_position = stream.position();
                                        let mut furthest_error;
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if self.scan_identifier(stream) {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::Identifier,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "Identifier"),
                                                }
                                            }
                                        } {
                                            Fail { error } => furthest_error = error,
                                            pass => break pass,
                                        }
                                        stream.set_position(start_position);
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if scan_chars!(
                                                stream, 'a', 'd', 'd', 'r', 'e', 's', 's'
                                            ) {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::Address,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "'address'"),
                                                }
                                            }
                                        } {
                                            Fail { error } => furthest_error.merge_with(error),
                                            pass => break pass,
                                        }
                                        break Fail {
                                            error: furthest_error,
                                        };
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![result_0, result_1],
                                        ),
                                        error: furthest_error,
                                    };
                                } {
                                    Pass { node, .. } => Ok(Pratt::Operator {
                                        node,
                                        kind: RuleKind::MemberAccessExpression,
                                        left_binding_power: 33u8,
                                        right_binding_power: 255,
                                    }),
                                    Fail { error } => Err(error),
                                }
                            } {
                                Err(error) => {
                                    if furthest_error.position < error.position {
                                        furthest_error = error
                                    } else if furthest_error.position == error.position {
                                        furthest_error.expected = format!(
                                            "{}, or {}",
                                            furthest_error.expected, error.expected
                                        )
                                    }
                                }
                                ok => break ok,
                            }
                            stream.set_position(start_position);
                            match {
                                match {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '[') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::OpenBracket,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "'['"),
                                            }
                                        }
                                    } {
                                        err @ Fail { .. } => err,
                                        Pass {
                                            node: open_node, ..
                                        } => {
                                            match loop {
                                                let start_position = stream.position();
                                                let mut furthest_error;
                                                match loop {
                                                    let mut furthest_error = None;
                                                    let result_0 = match self
                                                        .parse_expression(stream)
                                                    {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_1 = match {
                                                        let start_position = stream.position();
                                                        match loop {
                                                            let mut furthest_error = None;
                                                            let result_0 = match {
                                                                let leading_trivia = self
                                                                    .optional_leading_trivia(
                                                                        stream,
                                                                    );
                                                                let start = stream.position();
                                                                if scan_chars!(stream, ':') {
                                                                    let end = stream.position();
                                                                    let trailing_trivia = self
                                                                        .optional_trailing_trivia(
                                                                            stream,
                                                                        );
                                                                    Pass {
                                                                        node: cst::Node::token(
                                                                            TokenKind::Colon,
                                                                            Range { start, end },
                                                                            leading_trivia,
                                                                            trailing_trivia,
                                                                        ),
                                                                        error: None,
                                                                    }
                                                                } else {
                                                                    Fail {
                                                                        error: ParseError::new(
                                                                            start, "Colon",
                                                                        ),
                                                                    }
                                                                }
                                                            } {
                                                                Pass { node, error } => {
                                                                    furthest_error =
                                                                        error.map(|error| {
                                                                            error.maybe_merge_with(
                                                                                furthest_error,
                                                                            )
                                                                        });
                                                                    node
                                                                }
                                                                Fail { error } => {
                                                                    break Fail {
                                                                        error: error
                                                                            .maybe_merge_with(
                                                                                furthest_error,
                                                                            ),
                                                                    }
                                                                }
                                                            };
                                                            let result_1 = match {
                                                                let start_position =
                                                                    stream.position();
                                                                match self.parse_expression(stream)
                                                                {
                                                                    Fail { error } => {
                                                                        stream.set_position(
                                                                            start_position,
                                                                        );
                                                                        Pass {
                                                                            node: cst::Node::rule(
                                                                                RuleKind::_ANON,
                                                                                vec![],
                                                                            ),
                                                                            error: Some(error),
                                                                        }
                                                                    }
                                                                    pass => pass,
                                                                }
                                                            } {
                                                                Pass { node, error } => {
                                                                    furthest_error =
                                                                        error.map(|error| {
                                                                            error.maybe_merge_with(
                                                                                furthest_error,
                                                                            )
                                                                        });
                                                                    node
                                                                }
                                                                Fail { error } => {
                                                                    break Fail {
                                                                        error: error
                                                                            .maybe_merge_with(
                                                                                furthest_error,
                                                                            ),
                                                                    }
                                                                }
                                                            };
                                                            break Pass {
                                                                node: cst::Node::rule(
                                                                    RuleKind::_ANON,
                                                                    vec![result_0, result_1],
                                                                ),
                                                                error: furthest_error,
                                                            };
                                                        } {
                                                            Fail { error } => {
                                                                stream.set_position(start_position);
                                                                Pass {
                                                                    node: cst::Node::rule(
                                                                        RuleKind::_ANON,
                                                                        vec![],
                                                                    ),
                                                                    error: Some(error),
                                                                }
                                                            }
                                                            pass => pass,
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    break Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            vec![result_0, result_1],
                                                        ),
                                                        error: furthest_error,
                                                    };
                                                } {
                                                    Fail { error } => furthest_error = error,
                                                    pass => break pass,
                                                }
                                                stream.set_position(start_position);
                                                match loop {
                                                    let mut furthest_error = None;
                                                    let result_0 = match {
                                                        let leading_trivia =
                                                            self.optional_leading_trivia(stream);
                                                        let start = stream.position();
                                                        if scan_chars!(stream, ':') {
                                                            let end = stream.position();
                                                            let trailing_trivia = self
                                                                .optional_trailing_trivia(stream);
                                                            Pass {
                                                                node: cst::Node::token(
                                                                    TokenKind::Colon,
                                                                    Range { start, end },
                                                                    leading_trivia,
                                                                    trailing_trivia,
                                                                ),
                                                                error: None,
                                                            }
                                                        } else {
                                                            Fail {
                                                                error: ParseError::new(
                                                                    start, "Colon",
                                                                ),
                                                            }
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_1 = match {
                                                        let start_position = stream.position();
                                                        match self.parse_expression(stream) {
                                                            Fail { error } => {
                                                                stream.set_position(start_position);
                                                                Pass {
                                                                    node: cst::Node::rule(
                                                                        RuleKind::_ANON,
                                                                        vec![],
                                                                    ),
                                                                    error: Some(error),
                                                                }
                                                            }
                                                            pass => pass,
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    break Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            vec![result_0, result_1],
                                                        ),
                                                        error: furthest_error,
                                                    };
                                                } {
                                                    Fail { error } => {
                                                        furthest_error.merge_with(error)
                                                    }
                                                    pass => break pass,
                                                }
                                                break Fail {
                                                    error: furthest_error,
                                                };
                                            } {
                                                err @ Fail { .. } => err,
                                                Pass {
                                                    node: expr_node,
                                                    error: expr_error,
                                                } => {
                                                    match {
                                                        let leading_trivia =
                                                            self.optional_leading_trivia(stream);
                                                        let start = stream.position();
                                                        if scan_chars!(stream, ']') {
                                                            let end = stream.position();
                                                            let trailing_trivia = self
                                                                .optional_trailing_trivia(stream);
                                                            Pass {
                                                                node: cst::Node::token(
                                                                    TokenKind::CloseBracket,
                                                                    Range { start, end },
                                                                    leading_trivia,
                                                                    trailing_trivia,
                                                                ),
                                                                error: None,
                                                            }
                                                        } else {
                                                            Fail {
                                                                error: ParseError::new(
                                                                    start, "']'",
                                                                ),
                                                            }
                                                        }
                                                    } {
                                                        Fail { error } => Fail {
                                                            error: error
                                                                .maybe_merge_with(expr_error),
                                                        },
                                                        Pass {
                                                            node: close_node, ..
                                                        } => Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::_ANON,
                                                                vec![
                                                                    open_node, expr_node,
                                                                    close_node,
                                                                ],
                                                            ),
                                                            error: None,
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Pass { node, .. } => Ok(Pratt::Operator {
                                        node,
                                        kind: RuleKind::IndexAccessExpression,
                                        left_binding_power: 35u8,
                                        right_binding_power: 255,
                                    }),
                                    Fail { error } => Err(error),
                                }
                            } {
                                Err(error) => {
                                    if furthest_error.position < error.position {
                                        furthest_error = error
                                    } else if furthest_error.position == error.position {
                                        furthest_error.expected = format!(
                                            "{}, or {}",
                                            furthest_error.expected, error.expected
                                        )
                                    }
                                }
                                ok => break ok,
                            }
                            break Err(furthest_error);
                        } {
                            Err(_) => {
                                stream.set_position(start_position);
                                break;
                            }
                            Ok(operator) => elements.push(operator),
                        }
                    }
                    let start_position = stream.position();
                    match loop {
                        let start_position = stream.position();
                        let mut furthest_error;
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            match {
                                match stream . next () { Some ('!') => if scan_chars ! (stream , '=') { Ok ((TokenKind :: BangEqual , RuleKind :: EqualityComparisonExpression , 9u8 , 10u8)) } else { Err (ParseError :: new (stream . position () , "'!='")) } , Some ('%') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PercentEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Percent , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Percent , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('&') => { match stream . next () { Some ('&') => Ok ((TokenKind :: AmpersandAmpersand , RuleKind :: AndExpression , 7u8 , 8u8)) , Some ('=') => Ok ((TokenKind :: AmpersandEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , _ => Err (ParseError :: new (stream . position () , "'&&', or '&='")) } } , Some ('*') => { let start_position = stream . position () ; match stream . next () { Some ('*') => Ok ((TokenKind :: StarStar , RuleKind :: ExponentiationExpression , 25u8 , 26u8)) , Some ('=') => Ok ((TokenKind :: StarEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Star , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Star , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('+') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PlusEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Plus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } None => Ok ((TokenKind :: Plus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } } , Some ('-') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: MinusEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Minus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } None => Ok ((TokenKind :: Minus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } } , Some ('/') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: SlashEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Slash , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Slash , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('<') => { let start_position = stream . position () ; match stream . next () { Some ('<') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: LessLessEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: LessLess , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: LessLess , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some ('=') => Ok ((TokenKind :: LessEqual , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Less , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } None => Ok ((TokenKind :: Less , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } } , Some ('=') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: EqualEqual , RuleKind :: EqualityComparisonExpression , 9u8 , 10u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Equal , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } None => Ok ((TokenKind :: Equal , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } } , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterEqual , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterGreaterEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterGreaterGreaterEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: GreaterGreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: GreaterGreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: GreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: GreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Greater , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } None => Ok ((TokenKind :: Greater , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } } , Some ('^') => if scan_chars ! (stream , '=') { Ok ((TokenKind :: CaretEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } else { Err (ParseError :: new (stream . position () , "'^='")) } , Some ('|') => { match stream . next () { Some ('=') => Ok ((TokenKind :: PipeEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some ('|') => Ok ((TokenKind :: PipePipe , RuleKind :: OrExpression , 5u8 , 6u8)) , _ => Err (ParseError :: new (stream . position () , "'|=', or '||'")) } } , _ => Err (ParseError :: new (stream . position () , "'!=', or '%', or '%=', or '&&', or '&=', or '*', or '**', or '*=', or '+', or '+=', or '-', or '-=', or '/', or '/=', or '<', or '<<', or '<<=', or '<=', or '=', or '==', or '>', or '>=', or '>>', or '>>=', or '>>>', or '>>>=', or '^=', or '|=', or '||'")) }
                            } {
                                Ok((
                                    token_kind,
                                    rule_kind,
                                    left_binding_power,
                                    right_binding_power,
                                )) => {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Ok(Pratt::Operator {
                                        node: cst::Node::token(
                                            token_kind,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        kind: rule_kind,
                                        left_binding_power,
                                        right_binding_power,
                                    })
                                }
                                Err(error) => Err(error),
                            }
                        } {
                            Err(error) => furthest_error = error,
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, '|') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Pipe,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "Pipe"),
                                    }
                                }
                            } {
                                Pass { node, .. } => Ok(Pratt::Operator {
                                    node,
                                    kind: RuleKind::BitOrExpression,
                                    left_binding_power: 13u8,
                                    right_binding_power: 13u8 + 1,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => {
                                if furthest_error.position < error.position {
                                    furthest_error = error
                                } else if furthest_error.position == error.position {
                                    furthest_error.expected = format!(
                                        "{}, or {}",
                                        furthest_error.expected, error.expected
                                    )
                                }
                            }
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, '^') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Caret,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "Caret"),
                                    }
                                }
                            } {
                                Pass { node, .. } => Ok(Pratt::Operator {
                                    node,
                                    kind: RuleKind::BitXOrExpression,
                                    left_binding_power: 15u8,
                                    right_binding_power: 15u8 + 1,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => {
                                if furthest_error.position < error.position {
                                    furthest_error = error
                                } else if furthest_error.position == error.position {
                                    furthest_error.expected = format!(
                                        "{}, or {}",
                                        furthest_error.expected, error.expected
                                    )
                                }
                            }
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, '&') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Ampersand,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "Ampersand"),
                                    }
                                }
                            } {
                                Pass { node, .. } => Ok(Pratt::Operator {
                                    node,
                                    kind: RuleKind::BitAndExpression,
                                    left_binding_power: 17u8,
                                    right_binding_power: 17u8 + 1,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => {
                                if furthest_error.position < error.position {
                                    furthest_error = error
                                } else if furthest_error.position == error.position {
                                    furthest_error.expected = format!(
                                        "{}, or {}",
                                        furthest_error.expected, error.expected
                                    )
                                }
                            }
                            ok => break ok,
                        }
                        break Err(furthest_error);
                    } {
                        Err(_) => {
                            stream.set_position(start_position);
                            break None;
                        }
                        Ok(operator) => elements.push(operator),
                    }
                } {
                    Fail { error }
                } else {
                    let mut i = 0;
                    while elements.len() > 1 {
                        if let Pratt::Operator {
                            right_binding_power,
                            left_binding_power,
                            ..
                        } = &elements[i]
                        {
                            let next_left_binding_power = if elements.len() == i + 1 {
                                0
                            } else if let Pratt::Operator {
                                left_binding_power, ..
                            } = &elements[i + 1]
                            {
                                *left_binding_power
                            } else if elements.len() == i + 2 {
                                0
                            } else if let Pratt::Operator {
                                left_binding_power, ..
                            } = &elements[i + 2]
                            {
                                *left_binding_power
                            } else {
                                0
                            };
                            if *right_binding_power <= next_left_binding_power {
                                i += 1;
                                continue;
                            }
                            if *right_binding_power == 255 {
                                let left = elements.remove(i - 1);
                                let op = elements.remove(i - 1);
                                if let (Pratt::Node(left), Pratt::Operator { node: op, kind, .. }) =
                                    (left, op)
                                {
                                    let node = cst::Node::rule(kind, vec![left, op]);
                                    elements.insert(i - 1, Pratt::Node(node));
                                    i = i.saturating_sub(2);
                                } else {
                                    unreachable!()
                                }
                            } else if *left_binding_power == 255 {
                                let op = elements.remove(i);
                                let right = elements.remove(i);
                                if let (
                                    Pratt::Operator { node: op, kind, .. },
                                    Pratt::Node(right),
                                ) = (op, right)
                                {
                                    let node = cst::Node::rule(kind, vec![op, right]);
                                    elements.insert(i, Pratt::Node(node));
                                    i = i.saturating_sub(1);
                                } else {
                                    unreachable!()
                                }
                            } else {
                                let left = elements.remove(i - 1);
                                let op = elements.remove(i - 1);
                                let right = elements.remove(i - 1);
                                if let (
                                    Pratt::Node(left),
                                    Pratt::Operator { node: op, kind, .. },
                                    Pratt::Node(right),
                                ) = (left, op, right)
                                {
                                    let node = cst::Node::rule(kind, vec![left, op, right]);
                                    elements.insert(i - 1, Pratt::Node(node));
                                    i = i.saturating_sub(2);
                                } else {
                                    unreachable!()
                                }
                            }
                        } else {
                            i += 1;
                        }
                    }
                    if let Pratt::Node(node) = elements.pop().unwrap() {
                        Pass { node, error: None }
                    } else {
                        unreachable!()
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::Expression, node),
                error,
            },
            fail => fail,
        }
    }

    // ExpressionStatement = Expression ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_expression_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ExpressionStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_fallback_function_attribute(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_modifier_invocation(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_override_specifier(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('e') => {
                                if scan_chars!(stream, 'x', 't', 'e', 'r', 'n', 'a', 'l') {
                                    Ok(TokenKind::External)
                                } else {
                                    Err(ParseError::new(stream.position(), "'external'"))
                                }
                            }
                            Some('p') => match stream.next() {
                                Some('a') => {
                                    if scan_chars!(stream, 'y', 'a', 'b', 'l', 'e') {
                                        Ok(TokenKind::Payable)
                                    } else {
                                        Err(ParseError::new(stream.position(), "'payable'"))
                                    }
                                }
                                Some('u') => {
                                    if scan_chars!(stream, 'r', 'e') {
                                        Ok(TokenKind::Pure)
                                    } else {
                                        Err(ParseError::new(stream.position(), "'pure'"))
                                    }
                                }
                                _ => {
                                    Err(ParseError::new(stream.position(), "'payable', or 'pure'"))
                                }
                            },
                            Some('v') => {
                                if scan_chars!(stream, 'i') {
                                    match stream.next() {
                                        Some('e') => {
                                            if scan_chars!(stream, 'w') {
                                                Ok(TokenKind::View)
                                            } else {
                                                Err(ParseError::new(stream.position(), "'view'"))
                                            }
                                        }
                                        Some('r') => {
                                            if scan_chars!(stream, 't', 'u', 'a', 'l') {
                                                Ok(TokenKind::Virtual)
                                            } else {
                                                Err(ParseError::new(stream.position(), "'virtual'"))
                                            }
                                        }
                                        _ => Err(ParseError::new(
                                            stream.position(),
                                            "'view', or 'virtual'",
                                        )),
                                    }
                                } else {
                                    Err(ParseError::new(stream.position(), "'view', or 'virtual'"))
                                }
                            }
                            _ => Err(ParseError::new(
                                stream.position(),
                                "'external', or 'payable', or 'pure', or 'view', or 'virtual'",
                            )),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::FallbackFunctionAttribute, node),
                error,
            },
            fail => fail,
        }
    }

    // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_fallback_function_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'f', 'a', 'l', 'l', 'b', 'a', 'c', 'k') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Fallback,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'fallback'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_parameter_list(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match self.parse_fallback_function_attribute(stream) {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, 'r', 'e', 't', 'u', 'r', 'n', 's') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Returns,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'returns'"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_parameter_list(stream) {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, ';') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Semicolon,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "';'"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_block(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::FallbackFunctionDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_for_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'f', 'o', 'r') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::For,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'for'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match loop {
                                let mut furthest_error = None;
                                let result_0 = match loop {
                                    let start_position = stream.position();
                                    let mut furthest_error;
                                    match self.parse_simple_statement(stream) {
                                        Fail { error } => furthest_error = error,
                                        pass => break pass,
                                    }
                                    stream.set_position(start_position);
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ';') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Semicolon,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "';'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => furthest_error.merge_with(error),
                                        pass => break pass,
                                    }
                                    break Fail {
                                        error: furthest_error,
                                    };
                                } {
                                    Pass { node, error } => {
                                        furthest_error = error
                                            .map(|error| error.maybe_merge_with(furthest_error));
                                        node
                                    }
                                    Fail { error } => {
                                        break Fail {
                                            error: error.maybe_merge_with(furthest_error),
                                        }
                                    }
                                };
                                let result_1 = match loop {
                                    let start_position = stream.position();
                                    let mut furthest_error;
                                    match self.parse_expression_statement(stream) {
                                        Fail { error } => furthest_error = error,
                                        pass => break pass,
                                    }
                                    stream.set_position(start_position);
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ';') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Semicolon,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "';'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => furthest_error.merge_with(error),
                                        pass => break pass,
                                    }
                                    break Fail {
                                        error: furthest_error,
                                    };
                                } {
                                    Pass { node, error } => {
                                        furthest_error = error
                                            .map(|error| error.maybe_merge_with(furthest_error));
                                        node
                                    }
                                    Fail { error } => {
                                        break Fail {
                                            error: error.maybe_merge_with(furthest_error),
                                        }
                                    }
                                };
                                let result_2 = match {
                                    let start_position = stream.position();
                                    match self.parse_expression(stream) {
                                        Fail { error } => {
                                            stream.set_position(start_position);
                                            Pass {
                                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                                error: Some(error),
                                            }
                                        }
                                        pass => pass,
                                    }
                                } {
                                    Pass { node, error } => {
                                        furthest_error = error
                                            .map(|error| error.maybe_merge_with(furthest_error));
                                        node
                                    }
                                    Fail { error } => {
                                        break Fail {
                                            error: error.maybe_merge_with(furthest_error),
                                        }
                                    }
                                };
                                break Pass {
                                    node: cst::Node::rule(
                                        RuleKind::_ANON,
                                        vec![result_0, result_1, result_2],
                                    ),
                                    error: furthest_error,
                                };
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ')') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseParen,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "')'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_statement(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ForStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_function_attribute(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_modifier_invocation(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_override_specifier(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream . next () { Some ('e') => if scan_chars ! (stream , 'x' , 't' , 'e' , 'r' , 'n' , 'a' , 'l') { Ok (TokenKind :: External) } else { Err (ParseError :: new (stream . position () , "'external'")) } , Some ('i') => if scan_chars ! (stream , 'n' , 't' , 'e' , 'r' , 'n' , 'a' , 'l') { Ok (TokenKind :: Internal) } else { Err (ParseError :: new (stream . position () , "'internal'")) } , Some ('p') => { match stream . next () { Some ('a') => if scan_chars ! (stream , 'y' , 'a' , 'b' , 'l' , 'e') { Ok (TokenKind :: Payable) } else { Err (ParseError :: new (stream . position () , "'payable'")) } , Some ('r') => if scan_chars ! (stream , 'i' , 'v' , 'a' , 't' , 'e') { Ok (TokenKind :: Private) } else { Err (ParseError :: new (stream . position () , "'private'")) } , Some ('u') => { match stream . next () { Some ('b') => if scan_chars ! (stream , 'l' , 'i' , 'c') { Ok (TokenKind :: Public) } else { Err (ParseError :: new (stream . position () , "'public'")) } , Some ('r') => if scan_chars ! (stream , 'e') { Ok (TokenKind :: Pure) } else { Err (ParseError :: new (stream . position () , "'pure'")) } , _ => Err (ParseError :: new (stream . position () , "'public', or 'pure'")) } } , _ => Err (ParseError :: new (stream . position () , "'payable', or 'private', or 'public', or 'pure'")) } } , Some ('v') => { if scan_chars ! (stream , 'i') { match stream . next () { Some ('e') => if scan_chars ! (stream , 'w') { Ok (TokenKind :: View) } else { Err (ParseError :: new (stream . position () , "'view'")) } , Some ('r') => if scan_chars ! (stream , 't' , 'u' , 'a' , 'l') { Ok (TokenKind :: Virtual) } else { Err (ParseError :: new (stream . position () , "'virtual'")) } , _ => Err (ParseError :: new (stream . position () , "'view', or 'virtual'")) } } else { Err (ParseError :: new (stream . position () , "'view', or 'virtual'")) } } , _ => Err (ParseError :: new (stream . position () , "'external', or 'internal', or 'payable', or 'private', or 'public', or 'pure', or 'view', or 'virtual'")) }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::FunctionAttribute, node),
                error,
            },
            fail => fail,
        }
    }

    // FunctionCallExpression = Expression [ NamedArgumentList ] ArgumentList ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_function_call_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match self.parse_named_argument_list(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_argument_list(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::FunctionCallExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_function_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'f', 'u', 'n', 'c', 't', 'i', 'o', 'n') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Function,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'function'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_identifier(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Identifier,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Identifier"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        match {
                            match stream.next() {
                                Some('f') => {
                                    if scan_chars!(stream, 'a', 'l', 'l', 'b', 'a', 'c', 'k') {
                                        Ok(TokenKind::Fallback)
                                    } else {
                                        Err(ParseError::new(stream.position(), "'fallback'"))
                                    }
                                }
                                Some('r') => {
                                    if scan_chars!(stream, 'e', 'c', 'e', 'i', 'v', 'e') {
                                        Ok(TokenKind::Receive)
                                    } else {
                                        Err(ParseError::new(stream.position(), "'receive'"))
                                    }
                                }
                                _ => Err(ParseError::new(
                                    stream.position(),
                                    "'fallback', or 'receive'",
                                )),
                            }
                        } {
                            Err(mut error) => {
                                error.position = start;
                                Fail { error }
                            }
                            Ok(kind) => {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        kind,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            }
                        }
                    } {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_parameter_list(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match self.parse_function_attribute(stream) {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, 'r', 'e', 't', 'u', 'r', 'n', 's') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Returns,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'returns'"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_parameter_list(stream) {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_5 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, ';') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Semicolon,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "';'"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_block(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4, result_5],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::FunctionDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_function_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'f', 'u', 'n', 'c', 't', 'i', 'o', 'n') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Function,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'function'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_parameter_list(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            match {
                                match stream . next () { Some ('e') => if scan_chars ! (stream , 'x' , 't' , 'e' , 'r' , 'n' , 'a' , 'l') { Ok (TokenKind :: External) } else { Err (ParseError :: new (stream . position () , "'external'")) } , Some ('i') => if scan_chars ! (stream , 'n' , 't' , 'e' , 'r' , 'n' , 'a' , 'l') { Ok (TokenKind :: Internal) } else { Err (ParseError :: new (stream . position () , "'internal'")) } , Some ('p') => { match stream . next () { Some ('a') => if scan_chars ! (stream , 'y' , 'a' , 'b' , 'l' , 'e') { Ok (TokenKind :: Payable) } else { Err (ParseError :: new (stream . position () , "'payable'")) } , Some ('r') => if scan_chars ! (stream , 'i' , 'v' , 'a' , 't' , 'e') { Ok (TokenKind :: Private) } else { Err (ParseError :: new (stream . position () , "'private'")) } , Some ('u') => { match stream . next () { Some ('b') => if scan_chars ! (stream , 'l' , 'i' , 'c') { Ok (TokenKind :: Public) } else { Err (ParseError :: new (stream . position () , "'public'")) } , Some ('r') => if scan_chars ! (stream , 'e') { Ok (TokenKind :: Pure) } else { Err (ParseError :: new (stream . position () , "'pure'")) } , _ => Err (ParseError :: new (stream . position () , "'public', or 'pure'")) } } , _ => Err (ParseError :: new (stream . position () , "'payable', or 'private', or 'public', or 'pure'")) } } , Some ('v') => if scan_chars ! (stream , 'i' , 'e' , 'w') { Ok (TokenKind :: View) } else { Err (ParseError :: new (stream . position () , "'view'")) } , _ => Err (ParseError :: new (stream . position () , "'external', or 'internal', or 'payable', or 'private', or 'public', or 'pure', or 'view'")) }
                            } {
                                Err(mut error) => {
                                    error.position = start;
                                    Fail { error }
                                }
                                Ok(kind) => {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            kind,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                }
                            }
                        } {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, 'r', 'e', 't', 'u', 'r', 'n', 's') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Returns,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'returns'"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_parameter_list(stream) {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::FunctionType, node),
                error,
            },
            fail => fail,
        }
    }

    // IdentifierPath = «Identifier»  { '.' «Identifier» } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_identifier_path(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let mut result = Vec::new();
                loop {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_identifier(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Identifier,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Identifier"),
                            }
                        }
                    } {
                        err @ Fail { .. } => break err,
                        Pass { node, .. } => {
                            result.push(node);
                            let save = stream.position();
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, '.') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Period,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "'.'"),
                                    }
                                }
                            } {
                                Fail { error } => {
                                    stream.set_position(save);
                                    break Pass {
                                        node: cst::Node::rule(RuleKind::_ANON, result),
                                        error: Some(error),
                                    };
                                }
                                Pass { node, .. } => result.push(node),
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::IdentifierPath, node),
                error,
            },
            fail => fail,
        }
    }

    // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_if_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'i', 'f') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::If,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'if'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => match self.parse_expression(stream) {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ')') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseParen,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "')'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        },
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_statement(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, 'e', 'l', 's', 'e') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Else,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'else'"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_statement(stream) {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::IfStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_import_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'i', 'm', 'p', 'o', 'r', 't') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Import,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'import'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_simple_import_directive(stream) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_star_import_directive(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_selecting_import_directive(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ImportDirective, node),
                error,
            },
            fail => fail,
        }
    }

    // ImportPath = «AsciiStringLiteral» ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_import_path(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if self.scan_ascii_string_literal(stream) {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass {
                        node: cst::Node::token(
                            TokenKind::AsciiStringLiteral,
                            Range { start, end },
                            leading_trivia,
                            trailing_trivia,
                        ),
                        error: None,
                    }
                } else {
                    Fail {
                        error: ParseError::new(start, "AsciiStringLiteral"),
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ImportPath, node),
                error,
            },
            fail => fail,
        }
    }

    // IndexAccessExpression = Expression '[' ( Expression [ ':' [ Expression ] ] | ':' [ Expression ] ) ']' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_index_access_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '[') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenBracket,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'['"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match loop {
                                let start_position = stream.position();
                                let mut furthest_error;
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match self.parse_expression(stream) {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match {
                                        let start_position = stream.position();
                                        match loop {
                                            let mut furthest_error = None;
                                            let result_0 = match {
                                                let leading_trivia =
                                                    self.optional_leading_trivia(stream);
                                                let start = stream.position();
                                                if scan_chars!(stream, ':') {
                                                    let end = stream.position();
                                                    let trailing_trivia =
                                                        self.optional_trailing_trivia(stream);
                                                    Pass {
                                                        node: cst::Node::token(
                                                            TokenKind::Colon,
                                                            Range { start, end },
                                                            leading_trivia,
                                                            trailing_trivia,
                                                        ),
                                                        error: None,
                                                    }
                                                } else {
                                                    Fail {
                                                        error: ParseError::new(start, "Colon"),
                                                    }
                                                }
                                            } {
                                                Pass { node, error } => {
                                                    furthest_error = error.map(|error| {
                                                        error.maybe_merge_with(furthest_error)
                                                    });
                                                    node
                                                }
                                                Fail { error } => {
                                                    break Fail {
                                                        error: error
                                                            .maybe_merge_with(furthest_error),
                                                    }
                                                }
                                            };
                                            let result_1 = match {
                                                let start_position = stream.position();
                                                match self.parse_expression(stream) {
                                                    Fail { error } => {
                                                        stream.set_position(start_position);
                                                        Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::_ANON,
                                                                vec![],
                                                            ),
                                                            error: Some(error),
                                                        }
                                                    }
                                                    pass => pass,
                                                }
                                            } {
                                                Pass { node, error } => {
                                                    furthest_error = error.map(|error| {
                                                        error.maybe_merge_with(furthest_error)
                                                    });
                                                    node
                                                }
                                                Fail { error } => {
                                                    break Fail {
                                                        error: error
                                                            .maybe_merge_with(furthest_error),
                                                    }
                                                }
                                            };
                                            break Pass {
                                                node: cst::Node::rule(
                                                    RuleKind::_ANON,
                                                    vec![result_0, result_1],
                                                ),
                                                error: furthest_error,
                                            };
                                        } {
                                            Fail { error } => {
                                                stream.set_position(start_position);
                                                Pass {
                                                    node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                                    error: Some(error),
                                                }
                                            }
                                            pass => pass,
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![result_0, result_1],
                                        ),
                                        error: furthest_error,
                                    };
                                } {
                                    Fail { error } => furthest_error = error,
                                    pass => break pass,
                                }
                                stream.set_position(start_position);
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ':') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Colon,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "Colon"),
                                            }
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match {
                                        let start_position = stream.position();
                                        match self.parse_expression(stream) {
                                            Fail { error } => {
                                                stream.set_position(start_position);
                                                Pass {
                                                    node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                                    error: Some(error),
                                                }
                                            }
                                            pass => pass,
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![result_0, result_1],
                                        ),
                                        error: furthest_error,
                                    };
                                } {
                                    Fail { error } => furthest_error.merge_with(error),
                                    pass => break pass,
                                }
                                break Fail {
                                    error: furthest_error,
                                };
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ']') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseBracket,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "']'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::IndexAccessExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_inheritance_specifier(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_identifier_path(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match self.parse_argument_list(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::InheritanceSpecifier, node),
                error,
            },
            fail => fail,
        }
    }

    // InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_inheritance_specifier_list(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'i', 's') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Is,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'is'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let mut result = Vec::new();
                    loop {
                        match self.parse_inheritance_specifier(stream) {
                            err @ Fail { .. } => break err,
                            Pass { node, .. } => {
                                result.push(node);
                                let save = stream.position();
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ',') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::Comma,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "','"),
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(save);
                                        break Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, result),
                                            error: Some(error),
                                        };
                                    }
                                    Pass { node, .. } => result.push(node),
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::InheritanceSpecifierList, node),
                error,
            },
            fail => fail,
        }
    }

    // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_interface_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'i', 'n', 't', 'e', 'r', 'f', 'a', 'c', 'e') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Interface,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'interface'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let start_position = stream.position();
                    match self.parse_inheritance_specifier_list(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '{') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenBrace,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'{'"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let mut result = Vec::new();
                                loop {
                                    let start_position = stream.position();
                                    match self.parse_contract_body_element(stream) {
                                        Fail { error } => {
                                            stream.set_position(start_position);
                                            break Pass {
                                                node: cst::Node::rule(RuleKind::_ANON, result),
                                                error: Some(error),
                                            };
                                        }
                                        Pass { node, .. } => result.push(node),
                                    }
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '}') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseBrace,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "'}'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::InterfaceDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // LeadingTrivia = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_leading_trivia(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let mut result = Vec::new();
                loop {
                    let start_position = stream.position();
                    match loop {
                        let start_position = stream.position();
                        let mut furthest_error;
                        match {
                            let start = stream.position();
                            if self.scan_whitespace(stream) {
                                let end = stream.position();
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Whitespace,
                                        Range { start, end },
                                        None,
                                        None,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "Whitespace"),
                                }
                            }
                        } {
                            Fail { error } => furthest_error = error,
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match {
                            let start = stream.position();
                            if self.scan_end_of_line(stream) {
                                let end = stream.position();
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::EndOfLine,
                                        Range { start, end },
                                        None,
                                        None,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "EndOfLine"),
                                }
                            }
                        } {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match {
                            let start = stream.position();
                            if self.scan_multiline_comment(stream) {
                                let end = stream.position();
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::MultilineComment,
                                        Range { start, end },
                                        None,
                                        None,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "MultilineComment"),
                                }
                            }
                        } {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match {
                            let start = stream.position();
                            if self.scan_single_line_comment(stream) {
                                let end = stream.position();
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::SingleLineComment,
                                        Range { start, end },
                                        None,
                                        None,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "SingleLineComment"),
                                }
                            }
                        } {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        break Fail {
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            break Pass {
                                node: cst::Node::rule(RuleKind::_ANON, result),
                                error: Some(error),
                            };
                        }
                        Pass { node, .. } => result.push(node),
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::LeadingTrivia, node),
                error,
            },
            fail => fail,
        }
    }

    // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_library_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'l', 'i', 'b', 'r', 'a', 'r', 'y') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Library,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'library'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '{') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenBrace,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'{'"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let mut result = Vec::new();
                                loop {
                                    let start_position = stream.position();
                                    match self.parse_contract_body_element(stream) {
                                        Fail { error } => {
                                            stream.set_position(start_position);
                                            break Pass {
                                                node: cst::Node::rule(RuleKind::_ANON, result),
                                                error: Some(error),
                                            };
                                        }
                                        Pass { node, .. } => result.push(node),
                                    }
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '}') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseBrace,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "'}'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::LibraryDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // (* 0.4.11 *) MappingKeyType = ( ElementaryType | IdentifierPath ) ;
    // (* 0.8.18 *) MappingKeyType = ( ElementaryType | IdentifierPath ) [ «Identifier» ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_mapping_key_type(&self, stream: &mut Stream) -> ParseResult {
        match if self.version_is_equal_to_or_greater_than_0_8_18 {
            loop {
                let mut furthest_error = None;
                let result_0 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_elementary_type(stream) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_identifier_path(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_identifier(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Identifier,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Identifier"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } else {
            loop {
                let mut furthest_error = None;
                let result_0 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_elementary_type(stream) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_identifier_path(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::MappingKeyType, node),
                error,
            },
            fail => fail,
        }
    }

    // MappingType = 'mapping' '(' MappingKeyType '=>' MappingValueType ')' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_mapping_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'm', 'a', 'p', 'p', 'i', 'n', 'g') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Mapping,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'mapping'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match loop {
                                let mut furthest_error = None;
                                let result_0 = match self.parse_mapping_key_type(stream) {
                                    Pass { node, error } => {
                                        furthest_error = error
                                            .map(|error| error.maybe_merge_with(furthest_error));
                                        node
                                    }
                                    Fail { error } => {
                                        break Fail {
                                            error: error.maybe_merge_with(furthest_error),
                                        }
                                    }
                                };
                                let result_1 = match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, '=', '>') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::EqualGreater,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "'=>'"),
                                        }
                                    }
                                } {
                                    Pass { node, error } => {
                                        furthest_error = error
                                            .map(|error| error.maybe_merge_with(furthest_error));
                                        node
                                    }
                                    Fail { error } => {
                                        break Fail {
                                            error: error.maybe_merge_with(furthest_error),
                                        }
                                    }
                                };
                                let result_2 = match self.parse_mapping_value_type(stream) {
                                    Pass { node, error } => {
                                        furthest_error = error
                                            .map(|error| error.maybe_merge_with(furthest_error));
                                        node
                                    }
                                    Fail { error } => {
                                        break Fail {
                                            error: error.maybe_merge_with(furthest_error),
                                        }
                                    }
                                };
                                break Pass {
                                    node: cst::Node::rule(
                                        RuleKind::_ANON,
                                        vec![result_0, result_1, result_2],
                                    ),
                                    error: furthest_error,
                                };
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ')') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseParen,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "')'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::MappingType, node),
                error,
            },
            fail => fail,
        }
    }

    // (* 0.4.11 *) MappingValueType = TypeName ;
    // (* 0.8.18 *) MappingValueType = TypeName [ «Identifier» ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_mapping_value_type(&self, stream: &mut Stream) -> ParseResult {
        match if self.version_is_equal_to_or_greater_than_0_8_18 {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_identifier(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Identifier,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Identifier"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } else {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::MappingValueType, node),
                error,
            },
            fail => fail,
        }
    }

    // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_member_access_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '.') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Period,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Period"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_identifier(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Identifier,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Identifier"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, 'a', 'd', 'd', 'r', 'e', 's', 's') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Address,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'address'"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::MemberAccessExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // ModifierAttribute = OverrideSpecifier | 'virtual' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_modifier_attribute(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_override_specifier(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'v', 'i', 'r', 't', 'u', 'a', 'l') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Virtual,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'virtual'"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ModifierAttribute, node),
                error,
            },
            fail => fail,
        }
    }

    // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_modifier_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'm', 'o', 'd', 'i', 'f', 'i', 'e', 'r') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Modifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'modifier'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let start_position = stream.position();
                    match self.parse_parameter_list(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match self.parse_modifier_attribute(stream) {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, ';') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Semicolon,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "';'"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_block(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ModifierDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_modifier_invocation(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_identifier_path(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match self.parse_argument_list(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ModifierInvocation, node),
                error,
            },
            fail => fail,
        }
    }

    // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_mul_div_mod_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('%') => Ok(TokenKind::Percent),
                            Some('*') => Ok(TokenKind::Star),
                            Some('/') => Ok(TokenKind::Slash),
                            _ => Err(ParseError::new(stream.position(), "'%', or '*', or '/'")),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::MulDivModExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // NamedArgument = «Identifier» ':' Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_named_argument(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ':') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Colon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Colon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::NamedArgument, node),
                error,
            },
            fail => fail,
        }
    }

    // NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_named_argument_list(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '{') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::OpenBrace,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'{'"),
                        }
                    }
                } {
                    err @ Fail { .. } => err,
                    Pass {
                        node: open_node, ..
                    } => {
                        match {
                            let start_position = stream.position();
                            match {
                                let mut result = Vec::new();
                                loop {
                                    match self.parse_named_argument(stream) {
                                        err @ Fail { .. } => break err,
                                        Pass { node, .. } => {
                                            result.push(node);
                                            let save = stream.position();
                                            match {
                                                let leading_trivia =
                                                    self.optional_leading_trivia(stream);
                                                let start = stream.position();
                                                if scan_chars!(stream, ',') {
                                                    let end = stream.position();
                                                    let trailing_trivia =
                                                        self.optional_trailing_trivia(stream);
                                                    Pass {
                                                        node: cst::Node::token(
                                                            TokenKind::Comma,
                                                            Range { start, end },
                                                            leading_trivia,
                                                            trailing_trivia,
                                                        ),
                                                        error: None,
                                                    }
                                                } else {
                                                    Fail {
                                                        error: ParseError::new(start, "','"),
                                                    }
                                                }
                                            } {
                                                Fail { error } => {
                                                    stream.set_position(save);
                                                    break Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            result,
                                                        ),
                                                        error: Some(error),
                                                    };
                                                }
                                                Pass { node, .. } => result.push(node),
                                            }
                                        }
                                    }
                                }
                            } {
                                Fail { error } => {
                                    stream.set_position(start_position);
                                    Pass {
                                        node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                        error: Some(error),
                                    }
                                }
                                pass => pass,
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, '}') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseBrace,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "'}'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::NamedArgumentList, node),
                error,
            },
            fail => fail,
        }
    }

    // NewExpression = 'new' TypeName ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_new_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'n', 'e', 'w') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::New,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'new'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::NewExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // NumericLiteral = ( «HexLiteral» | «DecimalLiteral» ) [ «NumberUnit» ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_numeric_literal(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_hex_literal(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::HexLiteral,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "HexLiteral"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_decimal_literal(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::DecimalLiteral,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "DecimalLiteral"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_number_unit(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::NumberUnit,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "NumberUnit"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::NumericLiteral, node),
                error,
            },
            fail => fail,
        }
    }

    // OrExpression = Expression '||' Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_or_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '|', '|') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PipePipe,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'||'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::OrExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_order_comparison_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('<') => {
                                let start_position = stream.position();
                                match stream.next() {
                                    Some('=') => Ok(TokenKind::LessEqual),
                                    Some(_) => {
                                        stream.set_position(start_position);
                                        Ok(TokenKind::Less)
                                    }
                                    None => Ok(TokenKind::Less),
                                }
                            }
                            Some('>') => {
                                let start_position = stream.position();
                                match stream.next() {
                                    Some('=') => Ok(TokenKind::GreaterEqual),
                                    Some(_) => {
                                        stream.set_position(start_position);
                                        Ok(TokenKind::Greater)
                                    }
                                    None => Ok(TokenKind::Greater),
                                }
                            }
                            _ => Err(ParseError::new(
                                stream.position(),
                                "'<', or '<=', or '>', or '>='",
                            )),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::OrderComparisonExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_override_specifier(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'o', 'v', 'e', 'r', 'r', 'i', 'd', 'e') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Override,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'override'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, '(') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::OpenParen,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'('"),
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: open_node, ..
                            } => {
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match self.parse_identifier_path(stream) {
                                            err @ Fail { .. } => break err,
                                            Pass { node, .. } => {
                                                result.push(node);
                                                let save = stream.position();
                                                match {
                                                    let leading_trivia =
                                                        self.optional_leading_trivia(stream);
                                                    let start = stream.position();
                                                    if scan_chars!(stream, ',') {
                                                        let end = stream.position();
                                                        let trailing_trivia =
                                                            self.optional_trailing_trivia(stream);
                                                        Pass {
                                                            node: cst::Node::token(
                                                                TokenKind::Comma,
                                                                Range { start, end },
                                                                leading_trivia,
                                                                trailing_trivia,
                                                            ),
                                                            error: None,
                                                        }
                                                    } else {
                                                        Fail {
                                                            error: ParseError::new(start, "','"),
                                                        }
                                                    }
                                                } {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::_ANON,
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { node, .. } => result.push(node),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    err @ Fail { .. } => err,
                                    Pass {
                                        node: expr_node,
                                        error: expr_error,
                                    } => {
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if scan_chars!(stream, ')') {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::CloseParen,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "')'"),
                                                }
                                            }
                                        } {
                                            Fail { error } => Fail {
                                                error: error.maybe_merge_with(expr_error),
                                            },
                                            Pass {
                                                node: close_node, ..
                                            } => Pass {
                                                node: cst::Node::rule(
                                                    RuleKind::_ANON,
                                                    vec![open_node, expr_node, close_node],
                                                ),
                                                error: None,
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::OverrideSpecifier, node),
                error,
            },
            fail => fail,
        }
    }

    // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_parameter_declaration(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match self.parse_data_location(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_identifier(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Identifier,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "Identifier"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ParameterDeclaration, node),
                error,
            },
            fail => fail,
        }
    }

    // ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_parameter_list(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '(') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::OpenParen,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'('"),
                        }
                    }
                } {
                    err @ Fail { .. } => err,
                    Pass {
                        node: open_node, ..
                    } => {
                        match {
                            let start_position = stream.position();
                            match {
                                let mut result = Vec::new();
                                loop {
                                    match self.parse_parameter_declaration(stream) {
                                        err @ Fail { .. } => break err,
                                        Pass { node, .. } => {
                                            result.push(node);
                                            let save = stream.position();
                                            match {
                                                let leading_trivia =
                                                    self.optional_leading_trivia(stream);
                                                let start = stream.position();
                                                if scan_chars!(stream, ',') {
                                                    let end = stream.position();
                                                    let trailing_trivia =
                                                        self.optional_trailing_trivia(stream);
                                                    Pass {
                                                        node: cst::Node::token(
                                                            TokenKind::Comma,
                                                            Range { start, end },
                                                            leading_trivia,
                                                            trailing_trivia,
                                                        ),
                                                        error: None,
                                                    }
                                                } else {
                                                    Fail {
                                                        error: ParseError::new(start, "','"),
                                                    }
                                                }
                                            } {
                                                Fail { error } => {
                                                    stream.set_position(save);
                                                    break Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            result,
                                                        ),
                                                        error: Some(error),
                                                    };
                                                }
                                                Pass { node, .. } => result.push(node),
                                            }
                                        }
                                    }
                                }
                            } {
                                Fail { error } => {
                                    stream.set_position(start_position);
                                    Pass {
                                        node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                        error: Some(error),
                                    }
                                }
                                pass => pass,
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ')') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseParen,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "')'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ParameterList, node),
                error,
            },
            fail => fail,
        }
    }

    // PayableType = 'payable' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_payable_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if scan_chars!(stream, 'p', 'a', 'y', 'a', 'b', 'l', 'e') {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass {
                        node: cst::Node::token(
                            TokenKind::Payable,
                            Range { start, end },
                            leading_trivia,
                            trailing_trivia,
                        ),
                        error: None,
                    }
                } else {
                    Fail {
                        error: ParseError::new(start, "'payable'"),
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::PayableType, node),
                error,
            },
            fail => fail,
        }
    }

    // PositionalArgumentList = Expression  { ',' Expression } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_positional_argument_list(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let mut result = Vec::new();
                loop {
                    match self.parse_expression(stream) {
                        err @ Fail { .. } => break err,
                        Pass { node, .. } => {
                            result.push(node);
                            let save = stream.position();
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, ',') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Comma,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "','"),
                                    }
                                }
                            } {
                                Fail { error } => {
                                    stream.set_position(save);
                                    break Pass {
                                        node: cst::Node::rule(RuleKind::_ANON, result),
                                        error: Some(error),
                                    };
                                }
                                Pass { node, .. } => result.push(node),
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::PositionalArgumentList, node),
                error,
            },
            fail => fail,
        }
    }

    // PragmaDirective = 'pragma' ( VersionPragma | ABICoderPragma | ExperimentalPragma ) ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_pragma_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'p', 'r', 'a', 'g', 'm', 'a') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Pragma,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'pragma'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_version_pragma(stream) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_abi_coder_pragma(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_experimental_pragma(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::PragmaDirective, node),
                error,
            },
            fail => fail,
        }
    }

    // PrimaryExpression = «Identifier» | TupleExpression | ArrayLiteral | StringExpression | NumericLiteral | «BooleanLiteral» | NewExpression | TypeExpression | ElementaryType ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_primary_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_tuple_expression(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_array_literal(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_string_expression(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_numeric_literal(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_boolean_literal(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::BooleanLiteral,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "BooleanLiteral"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_new_expression(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_type_expression(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_elementary_type(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::PrimaryExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_receive_function_attribute(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_modifier_invocation(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_override_specifier(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('e') => {
                                if scan_chars!(stream, 'x', 't', 'e', 'r', 'n', 'a', 'l') {
                                    Ok(TokenKind::External)
                                } else {
                                    Err(ParseError::new(stream.position(), "'external'"))
                                }
                            }
                            Some('p') => {
                                if scan_chars!(stream, 'a', 'y', 'a', 'b', 'l', 'e') {
                                    Ok(TokenKind::Payable)
                                } else {
                                    Err(ParseError::new(stream.position(), "'payable'"))
                                }
                            }
                            Some('v') => {
                                if scan_chars!(stream, 'i', 'r', 't', 'u', 'a', 'l') {
                                    Ok(TokenKind::Virtual)
                                } else {
                                    Err(ParseError::new(stream.position(), "'virtual'"))
                                }
                            }
                            _ => Err(ParseError::new(
                                stream.position(),
                                "'external', or 'payable', or 'virtual'",
                            )),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ReceiveFunctionAttribute, node),
                error,
            },
            fail => fail,
        }
    }

    // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_receive_function_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'r', 'e', 'c', 'e', 'i', 'v', 'e') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Receive,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'receive'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_parameter_list(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match self.parse_receive_function_attribute(stream) {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, ';') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Semicolon,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "';'"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_block(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ReceiveFunctionDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // ReturnStatement = 'return' [ Expression ] ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_return_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'r', 'e', 't', 'u', 'r', 'n') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Return,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'return'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match self.parse_expression(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ReturnStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_revert_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'r', 'e', 'v', 'e', 'r', 't') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Revert,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'revert'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match self.parse_identifier_path(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_argument_list(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::RevertStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_selected_import(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, 'a', 's') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::As,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'as'"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if self.scan_identifier(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Identifier,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "Identifier"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::SelectedImport, node),
                error,
            },
            fail => fail,
        }
    }

    // SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_selecting_import_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '{') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenBrace,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'{'"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let mut result = Vec::new();
                                loop {
                                    match self.parse_selected_import(stream) {
                                        err @ Fail { .. } => break err,
                                        Pass { node, .. } => {
                                            result.push(node);
                                            let save = stream.position();
                                            match {
                                                let leading_trivia =
                                                    self.optional_leading_trivia(stream);
                                                let start = stream.position();
                                                if scan_chars!(stream, ',') {
                                                    let end = stream.position();
                                                    let trailing_trivia =
                                                        self.optional_trailing_trivia(stream);
                                                    Pass {
                                                        node: cst::Node::token(
                                                            TokenKind::Comma,
                                                            Range { start, end },
                                                            leading_trivia,
                                                            trailing_trivia,
                                                        ),
                                                        error: None,
                                                    }
                                                } else {
                                                    Fail {
                                                        error: ParseError::new(start, "','"),
                                                    }
                                                }
                                            } {
                                                Fail { error } => {
                                                    stream.set_position(save);
                                                    break Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            result,
                                                        ),
                                                        error: Some(error),
                                                    };
                                                }
                                                Pass { node, .. } => result.push(node),
                                            }
                                        }
                                    }
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '}') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseBrace,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "'}'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'f', 'r', 'o', 'm') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::From,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'from'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_import_path(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::SelectingImportDirective, node),
                error,
            },
            fail => fail,
        }
    }

    // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_shift_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('<') => {
                                if scan_chars!(stream, '<') {
                                    Ok(TokenKind::LessLess)
                                } else {
                                    Err(ParseError::new(stream.position(), "'<<'"))
                                }
                            }
                            Some('>') => {
                                if scan_chars!(stream, '>') {
                                    let start_position = stream.position();
                                    match stream.next() {
                                        Some('>') => Ok(TokenKind::GreaterGreaterGreater),
                                        Some(_) => {
                                            stream.set_position(start_position);
                                            Ok(TokenKind::GreaterGreater)
                                        }
                                        None => Ok(TokenKind::GreaterGreater),
                                    }
                                } else {
                                    Err(ParseError::new(stream.position(), "'>>', or '>>>'"))
                                }
                            }
                            _ => Err(ParseError::new(
                                stream.position(),
                                "'<<', or '>>', or '>>>'",
                            )),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::ShiftExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_simple_import_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_import_path(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match loop {
                            let mut furthest_error = None;
                            let result_0 = match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, 'a', 's') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::As,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "'as'"),
                                    }
                                }
                            } {
                                Pass { node, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    node
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            let result_1 = match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if self.scan_identifier(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Identifier,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "Identifier"),
                                    }
                                }
                            } {
                                Pass { node, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    node
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            break Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                                error: furthest_error,
                            };
                        } {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::SimpleImportDirective, node),
                error,
            },
            fail => fail,
        }
    }

    // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_simple_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_tuple_deconstruction_statement(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_variable_declaration_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_expression_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::SimpleStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // SourceUnit = LeadingTrivia { Directive | Definition } EndOfFileTrivia ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_source_unit(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_leading_trivia(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match loop {
                            let start_position = stream.position();
                            let mut furthest_error;
                            match self.parse_directive(stream) {
                                Fail { error } => furthest_error = error,
                                pass => break pass,
                            }
                            stream.set_position(start_position);
                            match self.parse_definition(stream) {
                                Fail { error } => furthest_error.merge_with(error),
                                pass => break pass,
                            }
                            break Fail {
                                error: furthest_error,
                            };
                        } {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_end_of_file_trivia(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::SourceUnit, node),
                error,
            },
            fail => fail,
        }
    }

    // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_star_import_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '*') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Star,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Star"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'a', 's') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::As,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'as'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'f', 'r', 'o', 'm') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::From,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'from'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match self.parse_import_path(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::StarImportDirective, node),
                error,
            },
            fail => fail,
        }
    }

    // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_state_variable_attribute(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_override_specifier(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream . next () { Some ('c') => if scan_chars ! (stream , 'o' , 'n' , 's' , 't' , 'a' , 'n' , 't') { Ok (TokenKind :: Constant) } else { Err (ParseError :: new (stream . position () , "'constant'")) } , Some ('i') => { match stream . next () { Some ('m') => if scan_chars ! (stream , 'm' , 'u' , 't' , 'a' , 'b' , 'l' , 'e') { Ok (TokenKind :: Immutable) } else { Err (ParseError :: new (stream . position () , "'immutable'")) } , Some ('n') => if scan_chars ! (stream , 't' , 'e' , 'r' , 'n' , 'a' , 'l') { Ok (TokenKind :: Internal) } else { Err (ParseError :: new (stream . position () , "'internal'")) } , _ => Err (ParseError :: new (stream . position () , "'immutable', or 'internal'")) } } , Some ('p') => { match stream . next () { Some ('r') => if scan_chars ! (stream , 'i' , 'v' , 'a' , 't' , 'e') { Ok (TokenKind :: Private) } else { Err (ParseError :: new (stream . position () , "'private'")) } , Some ('u') => if scan_chars ! (stream , 'b' , 'l' , 'i' , 'c') { Ok (TokenKind :: Public) } else { Err (ParseError :: new (stream . position () , "'public'")) } , _ => Err (ParseError :: new (stream . position () , "'private', or 'public'")) } } , _ => Err (ParseError :: new (stream . position () , "'constant', or 'immutable', or 'internal', or 'private', or 'public'")) }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::StateVariableAttribute, node),
                error,
            },
            fail => fail,
        }
    }

    // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_state_variable_declaration(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match self.parse_state_variable_attribute(stream) {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, '=') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Equal,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "Equal"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_expression(stream) {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::StateVariableDeclaration, node),
                error,
            },
            fail => fail,
        }
    }

    // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_block(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_simple_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_if_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_for_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_while_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_do_while_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_continue_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_break_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_try_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_return_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_emit_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_revert_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_delete_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_assembly_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::Statement, node),
                error,
            },
            fail => fail,
        }
    }

    // StringExpression = 1…*{ «HexStringLiteral» } | 1…*{ «AsciiStringLiteral» } | 1…*{ «UnicodeStringLiteral» } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_string_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if self.scan_hex_string_literal(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::HexStringLiteral,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "HexStringLiteral"),
                                }
                            }
                        } {
                            Fail { error } => {
                                if result.is_empty() {
                                    break Fail { error };
                                }
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if self.scan_ascii_string_literal(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::AsciiStringLiteral,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "AsciiStringLiteral"),
                                }
                            }
                        } {
                            Fail { error } => {
                                if result.is_empty() {
                                    break Fail { error };
                                }
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if self.scan_unicode_string_literal(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::UnicodeStringLiteral,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "UnicodeStringLiteral"),
                                }
                            }
                        } {
                            Fail { error } => {
                                if result.is_empty() {
                                    break Fail { error };
                                }
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::StringExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_struct_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 's', 't', 'r', 'u', 'c', 't') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Struct,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'struct'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '{') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenBrace,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'{'"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let mut result = Vec::new();
                                loop {
                                    let start_position = stream.position();
                                    match self.parse_struct_member(stream) {
                                        Fail { error } => {
                                            if result.is_empty() {
                                                break Fail { error };
                                            }
                                            stream.set_position(start_position);
                                            break Pass {
                                                node: cst::Node::rule(RuleKind::_ANON, result),
                                                error: Some(error),
                                            };
                                        }
                                        Pass { node, .. } => result.push(node),
                                    }
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, '}') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseBrace,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "'}'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::StructDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // StructMember = TypeName «Identifier» ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_struct_member(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::StructMember, node),
                error,
            },
            fail => fail,
        }
    }

    // TrailingTrivia = { «Whitespace» } [ «SingleLineComment» ] [ «EndOfLine» ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_trailing_trivia(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match {
                            let start = stream.position();
                            if self.scan_whitespace(stream) {
                                let end = stream.position();
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Whitespace,
                                        Range { start, end },
                                        None,
                                        None,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "Whitespace"),
                                }
                            }
                        } {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        let start = stream.position();
                        if self.scan_single_line_comment(stream) {
                            let end = stream.position();
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::SingleLineComment,
                                    Range { start, end },
                                    None,
                                    None,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "SingleLineComment"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let start_position = stream.position();
                    match {
                        let start = stream.position();
                        if self.scan_end_of_line(stream) {
                            let end = stream.position();
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::EndOfLine,
                                    Range { start, end },
                                    None,
                                    None,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "EndOfLine"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::TrailingTrivia, node),
                error,
            },
            fail => fail,
        }
    }

    // TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_try_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 't', 'r', 'y') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Try,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'try'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, 'r', 'e', 't', 'u', 'r', 'n', 's') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Returns,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'returns'"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_parameter_list(stream) {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match self.parse_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match self.parse_catch_clause(stream) {
                            Fail { error } => {
                                if result.is_empty() {
                                    break Fail { error };
                                }
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::TryStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // TupleDeconstructionStatement = '(' [ [ TypeName [ DataLocation ] «Identifier» | [ DataLocation ] «Identifier» ]  { ',' [ TypeName [ DataLocation ] «Identifier» | [ DataLocation ] «Identifier» ] } ] ')' '=' Expression ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_tuple_deconstruction_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let start_position = stream.position();
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match {
                                            let start_position = stream.position();
                                            match loop {
                                                let start_position = stream.position();
                                                let mut furthest_error;
                                                match loop {
                                                    let mut furthest_error = None;
                                                    let result_0 = match self
                                                        .parse_type_name(stream)
                                                    {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_1 = match {
                                                        let start_position = stream.position();
                                                        match self.parse_data_location(stream) {
                                                            Fail { error } => {
                                                                stream.set_position(start_position);
                                                                Pass {
                                                                    node: cst::Node::rule(
                                                                        RuleKind::_ANON,
                                                                        vec![],
                                                                    ),
                                                                    error: Some(error),
                                                                }
                                                            }
                                                            pass => pass,
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_2 = match {
                                                        let leading_trivia =
                                                            self.optional_leading_trivia(stream);
                                                        let start = stream.position();
                                                        if self.scan_identifier(stream) {
                                                            let end = stream.position();
                                                            let trailing_trivia = self
                                                                .optional_trailing_trivia(stream);
                                                            Pass {
                                                                node: cst::Node::token(
                                                                    TokenKind::Identifier,
                                                                    Range { start, end },
                                                                    leading_trivia,
                                                                    trailing_trivia,
                                                                ),
                                                                error: None,
                                                            }
                                                        } else {
                                                            Fail {
                                                                error: ParseError::new(
                                                                    start,
                                                                    "Identifier",
                                                                ),
                                                            }
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    break Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            vec![result_0, result_1, result_2],
                                                        ),
                                                        error: furthest_error,
                                                    };
                                                } {
                                                    Fail { error } => furthest_error = error,
                                                    pass => break pass,
                                                }
                                                stream.set_position(start_position);
                                                match loop {
                                                    let mut furthest_error = None;
                                                    let result_0 = match {
                                                        let start_position = stream.position();
                                                        match self.parse_data_location(stream) {
                                                            Fail { error } => {
                                                                stream.set_position(start_position);
                                                                Pass {
                                                                    node: cst::Node::rule(
                                                                        RuleKind::_ANON,
                                                                        vec![],
                                                                    ),
                                                                    error: Some(error),
                                                                }
                                                            }
                                                            pass => pass,
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_1 = match {
                                                        let leading_trivia =
                                                            self.optional_leading_trivia(stream);
                                                        let start = stream.position();
                                                        if self.scan_identifier(stream) {
                                                            let end = stream.position();
                                                            let trailing_trivia = self
                                                                .optional_trailing_trivia(stream);
                                                            Pass {
                                                                node: cst::Node::token(
                                                                    TokenKind::Identifier,
                                                                    Range { start, end },
                                                                    leading_trivia,
                                                                    trailing_trivia,
                                                                ),
                                                                error: None,
                                                            }
                                                        } else {
                                                            Fail {
                                                                error: ParseError::new(
                                                                    start,
                                                                    "Identifier",
                                                                ),
                                                            }
                                                        }
                                                    } {
                                                        Pass { node, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            node
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    break Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            vec![result_0, result_1],
                                                        ),
                                                        error: furthest_error,
                                                    };
                                                } {
                                                    Fail { error } => {
                                                        furthest_error.merge_with(error)
                                                    }
                                                    pass => break pass,
                                                }
                                                break Fail {
                                                    error: furthest_error,
                                                };
                                            } {
                                                Fail { error } => {
                                                    stream.set_position(start_position);
                                                    Pass {
                                                        node: cst::Node::rule(
                                                            RuleKind::_ANON,
                                                            vec![],
                                                        ),
                                                        error: Some(error),
                                                    }
                                                }
                                                pass => pass,
                                            }
                                        } {
                                            err @ Fail { .. } => break err,
                                            Pass { node, .. } => {
                                                result.push(node);
                                                let save = stream.position();
                                                match {
                                                    let leading_trivia =
                                                        self.optional_leading_trivia(stream);
                                                    let start = stream.position();
                                                    if scan_chars!(stream, ',') {
                                                        let end = stream.position();
                                                        let trailing_trivia =
                                                            self.optional_trailing_trivia(stream);
                                                        Pass {
                                                            node: cst::Node::token(
                                                                TokenKind::Comma,
                                                                Range { start, end },
                                                                leading_trivia,
                                                                trailing_trivia,
                                                            ),
                                                            error: None,
                                                        }
                                                    } else {
                                                        Fail {
                                                            error: ParseError::new(start, "','"),
                                                        }
                                                    }
                                                } {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::_ANON,
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { node, .. } => result.push(node),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ')') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseParen,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "')'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '=') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Equal,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Equal"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::TupleDeconstructionStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // TupleExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_tuple_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '(') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::OpenParen,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'('"),
                        }
                    }
                } {
                    err @ Fail { .. } => err,
                    Pass {
                        node: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                match {
                                    let start_position = stream.position();
                                    match self.parse_expression(stream) {
                                        Fail { error } => {
                                            stream.set_position(start_position);
                                            Pass {
                                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                                error: Some(error),
                                            }
                                        }
                                        pass => pass,
                                    }
                                } {
                                    err @ Fail { .. } => break err,
                                    Pass { node, .. } => {
                                        result.push(node);
                                        let save = stream.position();
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if scan_chars!(stream, ',') {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::Comma,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "','"),
                                                }
                                            }
                                        } {
                                            Fail { error } => {
                                                stream.set_position(save);
                                                break Pass {
                                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                                    error: Some(error),
                                                };
                                            }
                                            Pass { node, .. } => result.push(node),
                                        }
                                    }
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ')') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseParen,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "')'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::TupleExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // TypeExpression = 'type' '(' TypeName ')' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_type_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 't', 'y', 'p', 'e') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Type,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'type'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => match self.parse_type_name(stream) {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ')') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseParen,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "')'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        },
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::TypeExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_type_name(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_elementary_type(stream) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_function_type(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_mapping_type(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_identifier_path(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match {
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, '[') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::OpenBracket,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "'['"),
                                    }
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: open_node, ..
                                } => {
                                    match {
                                        let start_position = stream.position();
                                        match self.parse_expression(stream) {
                                            Fail { error } => {
                                                stream.set_position(start_position);
                                                Pass {
                                                    node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                                    error: Some(error),
                                                }
                                            }
                                            pass => pass,
                                        }
                                    } {
                                        err @ Fail { .. } => err,
                                        Pass {
                                            node: expr_node,
                                            error: expr_error,
                                        } => {
                                            match {
                                                let leading_trivia =
                                                    self.optional_leading_trivia(stream);
                                                let start = stream.position();
                                                if scan_chars!(stream, ']') {
                                                    let end = stream.position();
                                                    let trailing_trivia =
                                                        self.optional_trailing_trivia(stream);
                                                    Pass {
                                                        node: cst::Node::token(
                                                            TokenKind::CloseBracket,
                                                            Range { start, end },
                                                            leading_trivia,
                                                            trailing_trivia,
                                                        ),
                                                        error: None,
                                                    }
                                                } else {
                                                    Fail {
                                                        error: ParseError::new(start, "']'"),
                                                    }
                                                }
                                            } {
                                                Fail { error } => Fail {
                                                    error: error.maybe_merge_with(expr_error),
                                                },
                                                Pass {
                                                    node: close_node, ..
                                                } => Pass {
                                                    node: cst::Node::rule(
                                                        RuleKind::_ANON,
                                                        vec![open_node, expr_node, close_node],
                                                    ),
                                                    error: None,
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        } {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::TypeName, node),
                error,
            },
            fail => fail,
        }
    }

    // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_unary_prefix_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('!') => Ok(TokenKind::Bang),
                            Some('+') => {
                                if scan_chars!(stream, '+') {
                                    Ok(TokenKind::PlusPlus)
                                } else {
                                    Err(ParseError::new(stream.position(), "'++'"))
                                }
                            }
                            Some('-') => {
                                let start_position = stream.position();
                                match stream.next() {
                                    Some('-') => Ok(TokenKind::MinusMinus),
                                    Some(_) => {
                                        stream.set_position(start_position);
                                        Ok(TokenKind::Minus)
                                    }
                                    None => Ok(TokenKind::Minus),
                                }
                            }
                            Some('~') => Ok(TokenKind::Tilde),
                            _ => Err(ParseError::new(
                                stream.position(),
                                "'!', or '++', or '-', or '--', or '~'",
                            )),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::UnaryPrefixExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // UnarySuffixExpression = Expression ( '++' | '--' ) ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_unary_suffix_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match {
                        match stream.next() {
                            Some('+') => {
                                if scan_chars!(stream, '+') {
                                    Ok(TokenKind::PlusPlus)
                                } else {
                                    Err(ParseError::new(stream.position(), "'++'"))
                                }
                            }
                            Some('-') => {
                                if scan_chars!(stream, '-') {
                                    Ok(TokenKind::MinusMinus)
                                } else {
                                    Err(ParseError::new(stream.position(), "'--'"))
                                }
                            }
                            _ => Err(ParseError::new(stream.position(), "'++', or '--'")),
                        }
                    } {
                        Err(mut error) => {
                            error.position = start;
                            Fail { error }
                        }
                        Ok(kind) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::UnarySuffixExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // UncheckedBlock = 'unchecked' Block ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_unchecked_block(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'u', 'n', 'c', 'h', 'e', 'c', 'k', 'e', 'd') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Unchecked,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'unchecked'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::UncheckedBlock, node),
                error,
            },
            fail => fail,
        }
    }

    // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_user_defined_value_type_definition(
        &self,
        stream: &mut Stream,
    ) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 't', 'y', 'p', 'e') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Type,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'type'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'i', 's') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Is,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'is'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match self.parse_elementary_type(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::UserDefinedValueTypeDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_using_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'u', 's', 'i', 'n', 'g') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Using,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'using'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_identifier_path(stream) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match {
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, '{') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::OpenBrace,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'{'"),
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: open_node, ..
                            } => {
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match self.parse_identifier_path(stream) {
                                            err @ Fail { .. } => break err,
                                            Pass { node, .. } => {
                                                result.push(node);
                                                let save = stream.position();
                                                match {
                                                    let leading_trivia =
                                                        self.optional_leading_trivia(stream);
                                                    let start = stream.position();
                                                    if scan_chars!(stream, ',') {
                                                        let end = stream.position();
                                                        let trailing_trivia =
                                                            self.optional_trailing_trivia(stream);
                                                        Pass {
                                                            node: cst::Node::token(
                                                                TokenKind::Comma,
                                                                Range { start, end },
                                                                leading_trivia,
                                                                trailing_trivia,
                                                            ),
                                                            error: None,
                                                        }
                                                    } else {
                                                        Fail {
                                                            error: ParseError::new(start, "','"),
                                                        }
                                                    }
                                                } {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::_ANON,
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { node, .. } => result.push(node),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    err @ Fail { .. } => err,
                                    Pass {
                                        node: expr_node,
                                        error: expr_error,
                                    } => {
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if scan_chars!(stream, '}') {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::CloseBrace,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "'}'"),
                                                }
                                            }
                                        } {
                                            Fail { error } => Fail {
                                                error: error.maybe_merge_with(expr_error),
                                            },
                                            Pass {
                                                node: close_node, ..
                                            } => Pass {
                                                node: cst::Node::rule(
                                                    RuleKind::_ANON,
                                                    vec![open_node, expr_node, close_node],
                                                ),
                                                error: None,
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    } {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'f', 'o', 'r') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::For,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'for'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '*') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Star,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'*'"),
                            }
                        }
                    } {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_type_name(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, 'g', 'l', 'o', 'b', 'a', 'l') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::Global,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'global'"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_5 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4, result_5],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::UsingDirective, node),
                error,
            },
            fail => fail,
        }
    }

    // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_variable_declaration_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match self.parse_data_location(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Identifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Identifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, '=') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::Equal,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "Equal"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_expression(stream) {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ';') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Semicolon,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "Semicolon"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::VariableDeclarationStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // VersionPragma = 'solidity' 1…*{ VersionPragmaSpecifier } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_version_pragma(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 's', 'o', 'l', 'i', 'd', 'i', 't', 'y') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Solidity,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'solidity'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match self.parse_version_pragma_specifier(stream) {
                            Fail { error } => {
                                if result.is_empty() {
                                    break Fail { error };
                                }
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::VersionPragma, node),
                error,
            },
            fail => fail,
        }
    }

    // VersionPragmaSpecifier = [ «VersionPragmaOperator» ] «VersionPragmaValue»  { '.' «VersionPragmaValue» } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_version_pragma_specifier(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let start_position = stream.position();
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_version_pragma_operator(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::VersionPragmaOperator,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "VersionPragmaOperator"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let mut result = Vec::new();
                    loop {
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if self.scan_version_pragma_value(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::VersionPragmaValue,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "VersionPragmaValue"),
                                }
                            }
                        } {
                            err @ Fail { .. } => break err,
                            Pass { node, .. } => {
                                result.push(node);
                                let save = stream.position();
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, '.') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::Period,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "'.'"),
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(save);
                                        break Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, result),
                                            error: Some(error),
                                        };
                                    }
                                    Pass { node, .. } => result.push(node),
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::VersionPragmaSpecifier, node),
                error,
            },
            fail => fail,
        }
    }

    // WhileStatement = 'while' '(' Expression ')' Statement ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_while_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'w', 'h', 'i', 'l', 'e') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::While,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'while'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => match self.parse_expression(stream) {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ')') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseParen,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "')'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        },
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_statement(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::WhileStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_assignment_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let mut result = Vec::new();
                    loop {
                        match self.parse_yul_identifier_path(stream) {
                            err @ Fail { .. } => break err,
                            Pass { node, .. } => {
                                result.push(node);
                                let save = stream.position();
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ',') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::Comma,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "','"),
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(save);
                                        break Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, result),
                                            error: Some(error),
                                        };
                                    }
                                    Pass { node, .. } => result.push(node),
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, ':', '=') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ColonEqual,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "':='"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_yul_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulAssignmentStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // YulBlock = '{' { YulStatement } '}' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_block(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, '{') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::OpenBrace,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'{'"),
                        }
                    }
                } {
                    err @ Fail { .. } => err,
                    Pass {
                        node: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                let start_position = stream.position();
                                match self.parse_yul_statement(stream) {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        break Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, result),
                                            error: Some(error),
                                        };
                                    }
                                    Pass { node, .. } => result.push(node),
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: expr_node,
                                error: expr_error,
                            } => {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, '}') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::CloseBrace,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "'}'"),
                                        }
                                    }
                                } {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        node: close_node, ..
                                    } => Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![open_node, expr_node, close_node],
                                        ),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulBlock, node),
                error,
            },
            fail => fail,
        }
    }

    // YulBreakStatement = 'break' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_break_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if scan_chars!(stream, 'b', 'r', 'e', 'a', 'k') {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass {
                        node: cst::Node::token(
                            TokenKind::Break,
                            Range { start, end },
                            leading_trivia,
                            trailing_trivia,
                        ),
                        error: None,
                    }
                } else {
                    Fail {
                        error: ParseError::new(start, "'break'"),
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulBreakStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // YulContinueStatement = 'continue' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_continue_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if scan_chars!(stream, 'c', 'o', 'n', 't', 'i', 'n', 'u', 'e') {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass {
                        node: cst::Node::token(
                            TokenKind::Continue,
                            Range { start, end },
                            leading_trivia,
                            trailing_trivia,
                        ),
                        error: None,
                    }
                } else {
                    Fail {
                        error: ParseError::new(start, "'continue'"),
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulContinueStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // YulExpression = YulFunctionCall | YulLiteral ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_yul_function_call(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_literal(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulExpression, node),
                error,
            },
            fail => fail,
        }
    }

    // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_for_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'f', 'o', 'r') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::For,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'for'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_yul_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_yul_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match self.parse_yul_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match self.parse_yul_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulForStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // YulFunctionCall = YulIdentifierPath [ '(' [ YulExpression  { ',' YulExpression } ] ')' ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_function_call(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_yul_identifier_path(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let start_position = stream.position();
                    match {
                        match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, '(') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::OpenParen,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'('"),
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                node: open_node, ..
                            } => {
                                match {
                                    let start_position = stream.position();
                                    match {
                                        let mut result = Vec::new();
                                        loop {
                                            match self.parse_yul_expression(stream) {
                                                err @ Fail { .. } => break err,
                                                Pass { node, .. } => {
                                                    result.push(node);
                                                    let save = stream.position();
                                                    match {
                                                        let leading_trivia =
                                                            self.optional_leading_trivia(stream);
                                                        let start = stream.position();
                                                        if scan_chars!(stream, ',') {
                                                            let end = stream.position();
                                                            let trailing_trivia = self
                                                                .optional_trailing_trivia(stream);
                                                            Pass {
                                                                node: cst::Node::token(
                                                                    TokenKind::Comma,
                                                                    Range { start, end },
                                                                    leading_trivia,
                                                                    trailing_trivia,
                                                                ),
                                                                error: None,
                                                            }
                                                        } else {
                                                            Fail {
                                                                error: ParseError::new(
                                                                    start, "','",
                                                                ),
                                                            }
                                                        }
                                                    } {
                                                        Fail { error } => {
                                                            stream.set_position(save);
                                                            break Pass {
                                                                node: cst::Node::rule(
                                                                    RuleKind::_ANON,
                                                                    result,
                                                                ),
                                                                error: Some(error),
                                                            };
                                                        }
                                                        Pass { node, .. } => result.push(node),
                                                    }
                                                }
                                            }
                                        }
                                    } {
                                        Fail { error } => {
                                            stream.set_position(start_position);
                                            Pass {
                                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                                error: Some(error),
                                            }
                                        }
                                        pass => pass,
                                    }
                                } {
                                    err @ Fail { .. } => err,
                                    Pass {
                                        node: expr_node,
                                        error: expr_error,
                                    } => {
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if scan_chars!(stream, ')') {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::CloseParen,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "')'"),
                                                }
                                            }
                                        } {
                                            Fail { error } => Fail {
                                                error: error.maybe_merge_with(expr_error),
                                            },
                                            Pass {
                                                node: close_node, ..
                                            } => Pass {
                                                node: cst::Node::rule(
                                                    RuleKind::_ANON,
                                                    vec![open_node, expr_node, close_node],
                                                ),
                                                error: None,
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulFunctionCall, node),
                error,
            },
            fail => fail,
        }
    }

    // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_function_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'f', 'u', 'n', 'c', 't', 'i', 'o', 'n') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Function,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'function'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_yul_identifier(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::YulIdentifier,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "YulIdentifier"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if scan_chars!(stream, '(') {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::OpenParen,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "'('"),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            node: open_node, ..
                        } => {
                            match {
                                let start_position = stream.position();
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if self.scan_yul_identifier(stream) {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::YulIdentifier,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "YulIdentifier"),
                                                }
                                            }
                                        } {
                                            err @ Fail { .. } => break err,
                                            Pass { node, .. } => {
                                                result.push(node);
                                                let save = stream.position();
                                                match {
                                                    let leading_trivia =
                                                        self.optional_leading_trivia(stream);
                                                    let start = stream.position();
                                                    if scan_chars!(stream, ',') {
                                                        let end = stream.position();
                                                        let trailing_trivia =
                                                            self.optional_trailing_trivia(stream);
                                                        Pass {
                                                            node: cst::Node::token(
                                                                TokenKind::Comma,
                                                                Range { start, end },
                                                                leading_trivia,
                                                                trailing_trivia,
                                                            ),
                                                            error: None,
                                                        }
                                                    } else {
                                                        Fail {
                                                            error: ParseError::new(start, "','"),
                                                        }
                                                    }
                                                } {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            node: cst::Node::rule(
                                                                RuleKind::Arguments,
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { node, .. } => result.push(node),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    node: expr_node,
                                    error: expr_error,
                                } => {
                                    match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, ')') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CloseParen,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "')'"),
                                            }
                                        }
                                    } {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            node: close_node, ..
                                        } => Pass {
                                            node: cst::Node::rule(
                                                RuleKind::_ANON,
                                                vec![open_node, expr_node, close_node],
                                            ),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, '-', '>') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::MinusGreater,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "'->'"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match {
                            let mut result = Vec::new();
                            loop {
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if self.scan_yul_identifier(stream) {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::YulIdentifier,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "YulIdentifier"),
                                        }
                                    }
                                } {
                                    err @ Fail { .. } => break err,
                                    Pass { node, .. } => {
                                        result.push(node);
                                        let save = stream.position();
                                        match {
                                            let leading_trivia =
                                                self.optional_leading_trivia(stream);
                                            let start = stream.position();
                                            if scan_chars!(stream, ',') {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::Comma,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "','"),
                                                }
                                            }
                                        } {
                                            Fail { error } => {
                                                stream.set_position(save);
                                                break Pass {
                                                    node: cst::Node::rule(
                                                        RuleKind::Results,
                                                        result,
                                                    ),
                                                    error: Some(error),
                                                };
                                            }
                                            Pass { node, .. } => result.push(node),
                                        }
                                    }
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match self.parse_yul_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(
                        RuleKind::_ANON,
                        vec![result_0, result_1, result_2, result_3, result_4],
                    ),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulFunctionDefinition, node),
                error,
            },
            fail => fail,
        }
    }

    // YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_identifier_path(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let mut result = Vec::new();
                loop {
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        if self.scan_yul_identifier(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::YulIdentifier,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "YulIdentifier"),
                            }
                        }
                    } {
                        err @ Fail { .. } => break err,
                        Pass { node, .. } => {
                            result.push(node);
                            let save = stream.position();
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if scan_chars!(stream, '.') {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::Period,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "'.'"),
                                    }
                                }
                            } {
                                Fail { error } => {
                                    stream.set_position(save);
                                    break Pass {
                                        node: cst::Node::rule(RuleKind::_ANON, result),
                                        error: Some(error),
                                    };
                                }
                                Pass { node, .. } => result.push(node),
                            }
                        }
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulIdentifierPath, node),
                error,
            },
            fail => fail,
        }
    }

    // YulIfStatement = 'if' YulExpression YulBlock ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_if_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'i', 'f') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::If,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'if'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_yul_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_yul_block(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulIfStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // YulLeaveStatement = 'leave' ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_leave_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if scan_chars!(stream, 'l', 'e', 'a', 'v', 'e') {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass {
                        node: cst::Node::token(
                            TokenKind::Leave,
                            Range { start, end },
                            leading_trivia,
                            trailing_trivia,
                        ),
                        error: None,
                    }
                } else {
                    Fail {
                        error: ParseError::new(start, "'leave'"),
                    }
                }
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulLeaveStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // YulLiteral = «BooleanLiteral» | «YulHexLiteral» | «YulDecimalLiteral» | «HexStringLiteral» | «AsciiStringLiteral» ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_literal(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_boolean_literal(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::BooleanLiteral,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "BooleanLiteral"),
                        }
                    }
                } {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_yul_hex_literal(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::YulHexLiteral,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "YulHexLiteral"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_yul_decimal_literal(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::YulDecimalLiteral,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "YulDecimalLiteral"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_hex_string_literal(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::HexStringLiteral,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "HexStringLiteral"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_ascii_string_literal(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::AsciiStringLiteral,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "AsciiStringLiteral"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulLiteral, node),
                error,
            },
            fail => fail,
        }
    }

    // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_yul_block(stream) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_variable_declaration(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_function_definition(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_assignment_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_function_call(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_if_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_for_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_switch_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_leave_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_break_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_yul_continue_statement(stream) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_switch_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 's', 'w', 'i', 't', 'c', 'h') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Switch,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'switch'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_yul_expression(stream) {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let mut result = Vec::new();
                    loop {
                        let start_position = stream.position();
                        match loop {
                            let mut furthest_error = None;
                            let result_0 = match loop {
                                let start_position = stream.position();
                                let mut furthest_error;
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match {
                                        let leading_trivia = self.optional_leading_trivia(stream);
                                        let start = stream.position();
                                        if scan_chars!(stream, 'c', 'a', 's', 'e') {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::Case,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "'case'"),
                                            }
                                        }
                                    } {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match self.parse_yul_literal(stream) {
                                        Pass { node, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            node
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        node: cst::Node::rule(
                                            RuleKind::_ANON,
                                            vec![result_0, result_1],
                                        ),
                                        error: furthest_error,
                                    };
                                } {
                                    Fail { error } => furthest_error = error,
                                    pass => break pass,
                                }
                                stream.set_position(start_position);
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, 'd', 'e', 'f', 'a', 'u', 'l', 't') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::Default,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "'default'"),
                                        }
                                    }
                                } {
                                    Fail { error } => furthest_error.merge_with(error),
                                    pass => break pass,
                                }
                                break Fail {
                                    error: furthest_error,
                                };
                            } {
                                Pass { node, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    node
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            let result_1 = match self.parse_yul_block(stream) {
                                Pass { node, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    node
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            break Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                                error: furthest_error,
                            };
                        } {
                            Fail { error } => {
                                if result.is_empty() {
                                    break Fail { error };
                                }
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_ANON, result),
                                    error: Some(error),
                                };
                            }
                            Pass { node, .. } => result.push(node),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulSwitchStatement, node),
                error,
            },
            fail => fail,
        }
    }

    // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_variable_declaration(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if scan_chars!(stream, 'l', 'e', 't') {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::Let,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "'let'"),
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match {
                    let mut result = Vec::new();
                    loop {
                        match self.parse_yul_identifier_path(stream) {
                            err @ Fail { .. } => break err,
                            Pass { node, .. } => {
                                result.push(node);
                                let save = stream.position();
                                match {
                                    let leading_trivia = self.optional_leading_trivia(stream);
                                    let start = stream.position();
                                    if scan_chars!(stream, ',') {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::Comma,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "','"),
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(save);
                                        break Pass {
                                            node: cst::Node::rule(RuleKind::_ANON, result),
                                            error: Some(error),
                                        };
                                    }
                                    Pass { node, .. } => result.push(node),
                                }
                            }
                        }
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    let start_position = stream.position();
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match {
                            let leading_trivia = self.optional_leading_trivia(stream);
                            let start = stream.position();
                            if scan_chars!(stream, ':', '=') {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::ColonEqual,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "':='"),
                                }
                            }
                        } {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_yul_expression(stream) {
                            Pass { node, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                node
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_ANON, vec![]),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { node, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        node
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    node: cst::Node::rule(RuleKind::_ANON, vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            }
        } {
            Pass { node, error } => Pass {
                node: cst::Node::top_level_rule(RuleKind::YulVariableDeclaration, node),
                error,
            },
            fail => fail,
        }
    }
}
