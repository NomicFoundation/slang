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

#[allow(unused_macros)]
macro_rules! scan_not_followed_by {
    ($stream:ident, $expression:expr, $not_followed_by:expr) => {
        ($expression)
            && ({
                let end = $stream.position();
                let following = $not_followed_by;
                $stream.set_position(end);
                !following
            })
    };
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
    // ABICoderPragma = «AbicoderKeyword» «Identifier» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_abi_coder_pragma(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_abicoder_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::AbicoderKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "AbicoderKeyword"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // AddressType = «AddressKeyword» [ «PayableKeyword» ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_address_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_address_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::AddressKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "AddressKeyword"),
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
                        if self.scan_payable_keyword(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::PayableKeyword,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "PayableKeyword"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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
                                        node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                            RuleKind::_DELIMITEDBY,
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

    // ArrayLiteral = '[' Expression { ',' Expression } ']' ;

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
                                                    node: cst::Node::rule(
                                                        RuleKind::_SEPARATEDBY,
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
                                            RuleKind::_DELIMITEDBY,
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

    // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral» { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;

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
                                                    node: cst::Node::rule(
                                                        RuleKind::_SEPARATEDBY,
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
                                            RuleKind::_DELIMITEDBY,
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

    // AssemblyStatement = «AssemblyKeyword» [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_assembly_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_assembly_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::AssemblyKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "AssemblyKeyword"),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                        RuleKind::_SEQUENCE,
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
                                            node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                                            RuleKind::_DELIMITEDBY,
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

    // BooleanLiteral = «TrueKeyword» | «FalseKeyword» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_boolean_literal(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_true_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::TrueKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "TrueKeyword"),
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
                    if self.scan_false_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::FalseKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "FalseKeyword"),
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
                node: cst::Node::top_level_rule(RuleKind::BooleanLiteral, node),
                error,
            },
            fail => fail,
        }
    }

    // BreakStatement = «BreakKeyword» ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_break_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_break_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::BreakKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "BreakKeyword"),
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
                            error: ParseError::new(start, "';'"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // CatchClause = «CatchKeyword» [ [ «Identifier» ] ParameterList ] Block ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_catch_clause(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_catch_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::CatchKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "CatchKeyword"),
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
                                        node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // ConstantDefinition = TypeName «ConstantKeyword» «Identifier» '=' Expression ';' ;

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
                    if self.scan_constant_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ConstantKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ConstantKeyword"),
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
                            error: ParseError::new(start, "'='"),
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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

    // ConstructorAttribute = ModifierInvocation | «InternalKeyword» | «PayableKeyword» | «PublicKeyword» ;

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
                    if self.scan_internal_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::InternalKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "InternalKeyword"),
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
                    if self.scan_payable_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PayableKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PayableKeyword"),
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
                    if self.scan_public_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PublicKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PublicKeyword"),
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

    // ConstructorDefinition = «ConstructorKeyword» ParameterList { ConstructorAttribute } Block ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_constructor_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_constructor_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ConstructorKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ConstructorKeyword"),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                        RuleKind::_SEQUENCE,
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

    // ContinueStatement = «ContinueKeyword» ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_continue_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_continue_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ContinueKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ContinueKeyword"),
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
                            error: ParseError::new(start, "';'"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // ContractDefinition = [ «AbstractKeyword» ] «ContractKeyword» «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;

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
                        if self.scan_abstract_keyword(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::AbstractKeyword,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "AbstractKeyword"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    if self.scan_contract_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ContractKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ContractKeyword"),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                                node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                                                RuleKind::_DELIMITEDBY,
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
                        RuleKind::_SEQUENCE,
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

    // DataLocation = «MemoryKeyword» | «StorageKeyword» | «CalldataKeyword» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_data_location(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_memory_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::MemoryKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "MemoryKeyword"),
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
                    if self.scan_storage_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::StorageKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "StorageKeyword"),
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
                    if self.scan_calldata_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::CalldataKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "CalldataKeyword"),
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

    // DeleteStatement = «DeleteKeyword» Expression ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_delete_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_delete_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::DeleteKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "DeleteKeyword"),
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
                            error: ParseError::new(start, "';'"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // DoWhileStatement = «DoKeyword» Statement «WhileKeyword» '(' Expression ')' ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_do_while_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_do_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::DoKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "DoKeyword"),
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
                    if self.scan_while_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::WhileKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "WhileKeyword"),
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
                                            RuleKind::_DELIMITEDBY,
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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

    // ElementaryType = «BoolKeyword» | «StringKeyword» | AddressType | PayableType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_elementary_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_bool_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::BoolKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "BoolKeyword"),
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
                    if self.scan_string_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::StringKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "StringKeyword"),
                        }
                    }
                } {
                    Fail { error } => furthest_error.merge_with(error),
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

    // EmitStatement = «EmitKeyword» IdentifierPath ArgumentList ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_emit_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_emit_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::EmitKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "EmitKeyword"),
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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

    // EndOfFileTrivia = 1…{ «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;

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
                            if result.is_empty() {
                                break Fail { error };
                            }
                            stream.set_position(start_position);
                            break Pass {
                                node: cst::Node::rule(RuleKind::_REPEATED, result),
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

    // EnumDefinition = «EnumKeyword» «Identifier» '{' [ «Identifier» { ',' «Identifier» } ] '}' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_enum_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_enum_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::EnumKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "EnumKeyword"),
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
                                                                RuleKind::_SEPARATEDBY,
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
                                            node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                                RuleKind::_DELIMITEDBY,
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // ErrorDefinition = «ErrorKeyword» «Identifier» '(' [ ErrorParameter { ',' ErrorParameter } ] ')' ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_error_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_error_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ErrorKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ErrorKeyword"),
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
                                                                RuleKind::_SEPARATEDBY,
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
                                            node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                                RuleKind::_DELIMITEDBY,
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // EventDefinition = «EventKeyword» «Identifier» '(' [ EventParameter { ',' EventParameter } ] ')' [ «AnonymousKeyword» ] ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_event_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_event_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::EventKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "EventKeyword"),
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
                                                                RuleKind::_SEPARATEDBY,
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
                                            node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                                RuleKind::_DELIMITEDBY,
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
                        if self.scan_anonymous_keyword(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::AnonymousKeyword,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "AnonymousKeyword"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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

    // EventParameter = TypeName [ «IndexedKeyword» ] [ «Identifier» ] ;

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
                        if self.scan_indexed_keyword(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::IndexedKeyword,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "IndexedKeyword"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // ExperimentalPragma = «ExperimentalKeyword» «Identifier» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_experimental_pragma(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_experimental_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ExperimentalKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ExperimentalKeyword"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // (* v0.4.11 *) Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    // AssignmentExpression = Expression ( '=' | "|=" | "^=" | "&=" | "<<=" | ">>=" | ">>>=" | "+=" | "-=" | "*=" | "/=" | "%=" ) Expression ;
    // ConditionalExpression = Expression ( '?' Expression ':' Expression ) ;
    // OrExpression = Expression ( "||" ) Expression ;
    // AndExpression = Expression ( "&&" ) Expression ;
    // EqualityComparisonExpression = Expression ( "==" | "!=" ) Expression ;
    // OrderComparisonExpression = Expression ( '<' | '>' | "<=" | ">=" ) Expression ;
    // BitOrExpression = Expression ( '|' ) Expression ;
    // BitXOrExpression = Expression ( '^' ) Expression ;
    // BitAndExpression = Expression ( '&' ) Expression ;
    // ShiftExpression = Expression ( "<<" | ">>" | ">>>" ) Expression ;
    // AddSubExpression = Expression ( '+' | '-' ) Expression ;
    // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
    // ExponentiationExpression = Expression ( "**" ) Expression ;
    // UnarySuffixExpression = Expression ( "++" | "--" ) ;
    // UnaryPrefixExpression = ( "++" | "--" | '!' | '~' | '-' ) Expression ;
    // FunctionCallExpression = Expression ( [ NamedArgumentList ] ArgumentList ) ;
    // MemberAccessExpression = Expression ( '.' ( «Identifier» | «AddressKeyword» ) ) ;
    // IndexAccessExpression = Expression ( '[' Expression [ ':' [ Expression ] ] | ':' [ Expression ] ']' ) ;
    // (* v0.6.0 *) Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    // AssignmentExpression = Expression ( '=' | "|=" | "^=" | "&=" | "<<=" | ">>=" | ">>>=" | "+=" | "-=" | "*=" | "/=" | "%=" ) Expression ;
    // ConditionalExpression = Expression ( '?' Expression ':' Expression ) ;
    // OrExpression = Expression ( "||" ) Expression ;
    // AndExpression = Expression ( "&&" ) Expression ;
    // EqualityComparisonExpression = Expression ( "==" | "!=" ) Expression ;
    // OrderComparisonExpression = Expression ( '<' | '>' | "<=" | ">=" ) Expression ;
    // BitOrExpression = Expression ( '|' ) Expression ;
    // BitXOrExpression = Expression ( '^' ) Expression ;
    // BitAndExpression = Expression ( '&' ) Expression ;
    // ShiftExpression = Expression ( "<<" | ">>" | ">>>" ) Expression ;
    // AddSubExpression = Expression ( '+' | '-' ) Expression ;
    // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
    // ExponentiationExpression = Expression ( "**" ) Expression ;
    // UnarySuffixExpression = Expression ( "++" | "--" ) ;
    // UnaryPrefixExpression = ( "++" | "--" | '!' | '~' | '-' ) Expression ;
    // FunctionCallExpression = Expression ( [ NamedArgumentList ] ArgumentList ) ;
    // MemberAccessExpression = Expression ( '.' ( «Identifier» | «AddressKeyword» ) ) ;
    // IndexAccessExpression = Expression ( '[' Expression [ ':' [ Expression ] ] | ':' [ Expression ] ']' ) ;

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
                                                error: ParseError::new(start, "'?'"),
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
                                                error: ParseError::new(start, "':'"),
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
                                            RuleKind::_SEQUENCE,
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
                                                    node: cst::Node::rule(
                                                        RuleKind::_OPTIONAL,
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
                                            RuleKind::_SEQUENCE,
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
                                                error: ParseError::new(start, "'.'"),
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
                                            if self.scan_address_keyword(stream) {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::AddressKeyword,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "AddressKeyword"),
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
                                            RuleKind::_SEQUENCE,
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
                                                                            start, "':'",
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
                                                                                RuleKind::_OPTIONAL,
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
                                                                    RuleKind::_SEQUENCE,
                                                                    vec![result_0, result_1],
                                                                ),
                                                                error: furthest_error,
                                                            };
                                                        } {
                                                            Fail { error } => {
                                                                stream.set_position(start_position);
                                                                Pass {
                                                                    node: cst::Node::rule(
                                                                        RuleKind::_OPTIONAL,
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
                                                            RuleKind::_SEQUENCE,
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
                                                                    start, "':'",
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
                                                                        RuleKind::_OPTIONAL,
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
                                                            RuleKind::_SEQUENCE,
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
                                                                RuleKind::_DELIMITEDBY,
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
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        match {
                            match stream . next () { Some ('!') => if scan_chars ! (stream , '=') { Ok ((TokenKind :: BangEqual , RuleKind :: EqualityComparisonExpression , 9u8 , 10u8)) } else { Err (ParseError :: new (stream . position () , "'!='")) } , Some ('%') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PercentEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Percent , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Percent , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('&') => { let start_position = stream . position () ; match stream . next () { Some ('&') => Ok ((TokenKind :: AmpersandAmpersand , RuleKind :: AndExpression , 7u8 , 8u8)) , Some ('=') => Ok ((TokenKind :: AmpersandEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Ampersand , RuleKind :: BitAndExpression , 17u8 , 18u8)) } None => Ok ((TokenKind :: Ampersand , RuleKind :: BitAndExpression , 17u8 , 18u8)) } } , Some ('*') => { let start_position = stream . position () ; match stream . next () { Some ('*') => Ok ((TokenKind :: StarStar , RuleKind :: ExponentiationExpression , 26u8 , 25u8)) , Some ('=') => Ok ((TokenKind :: StarEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Star , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Star , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('+') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PlusEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Plus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } None => Ok ((TokenKind :: Plus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } } , Some ('-') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: MinusEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Minus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } None => Ok ((TokenKind :: Minus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } } , Some ('/') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: SlashEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Slash , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Slash , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('<') => { let start_position = stream . position () ; match stream . next () { Some ('<') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: LessLessEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: LessLess , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: LessLess , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some ('=') => Ok ((TokenKind :: LessEqual , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Less , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } None => Ok ((TokenKind :: Less , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } } , Some ('=') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: EqualEqual , RuleKind :: EqualityComparisonExpression , 9u8 , 10u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Equal , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } None => Ok ((TokenKind :: Equal , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } } , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterEqual , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterGreaterEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterGreaterGreaterEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: GreaterGreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: GreaterGreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: GreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: GreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Greater , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } None => Ok ((TokenKind :: Greater , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } } , Some ('^') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: CaretEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Caret , RuleKind :: BitXOrExpression , 15u8 , 16u8)) } None => Ok ((TokenKind :: Caret , RuleKind :: BitXOrExpression , 15u8 , 16u8)) } } , Some ('|') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PipeEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some ('|') => Ok ((TokenKind :: PipePipe , RuleKind :: OrExpression , 5u8 , 6u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Pipe , RuleKind :: BitOrExpression , 13u8 , 14u8)) } None => Ok ((TokenKind :: Pipe , RuleKind :: BitOrExpression , 13u8 , 14u8)) } } , _ => Err (ParseError :: new (stream . position () , "'!=', or '%', or '%=', or '&', or '&&', or '&=', or '*', or '**', or '*=', or '+', or '+=', or '-', or '-=', or '/', or '/=', or '<', or '<<', or '<<=', or '<=', or '=', or '==', or '>', or '>=', or '>>', or '>>=', or '>>>', or '>>>=', or '^', or '^=', or '|', or '|=', or '||'")) }
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
                                                error: ParseError::new(start, "'?'"),
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
                                                error: ParseError::new(start, "':'"),
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
                                            RuleKind::_SEQUENCE,
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
                                                    node: cst::Node::rule(
                                                        RuleKind::_OPTIONAL,
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
                                            RuleKind::_SEQUENCE,
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
                                                error: ParseError::new(start, "'.'"),
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
                                            if self.scan_address_keyword(stream) {
                                                let end = stream.position();
                                                let trailing_trivia =
                                                    self.optional_trailing_trivia(stream);
                                                Pass {
                                                    node: cst::Node::token(
                                                        TokenKind::AddressKeyword,
                                                        Range { start, end },
                                                        leading_trivia,
                                                        trailing_trivia,
                                                    ),
                                                    error: None,
                                                }
                                            } else {
                                                Fail {
                                                    error: ParseError::new(start, "AddressKeyword"),
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
                                            RuleKind::_SEQUENCE,
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
                                                                            start, "':'",
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
                                                                                RuleKind::_OPTIONAL,
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
                                                                    RuleKind::_SEQUENCE,
                                                                    vec![result_0, result_1],
                                                                ),
                                                                error: furthest_error,
                                                            };
                                                        } {
                                                            Fail { error } => {
                                                                stream.set_position(start_position);
                                                                Pass {
                                                                    node: cst::Node::rule(
                                                                        RuleKind::_OPTIONAL,
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
                                                            RuleKind::_SEQUENCE,
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
                                                                    start, "':'",
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
                                                                        RuleKind::_OPTIONAL,
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
                                                            RuleKind::_SEQUENCE,
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
                                                                RuleKind::_DELIMITEDBY,
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
                    match {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        match {
                            match stream . next () { Some ('!') => if scan_chars ! (stream , '=') { Ok ((TokenKind :: BangEqual , RuleKind :: EqualityComparisonExpression , 9u8 , 10u8)) } else { Err (ParseError :: new (stream . position () , "'!='")) } , Some ('%') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PercentEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Percent , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Percent , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('&') => { let start_position = stream . position () ; match stream . next () { Some ('&') => Ok ((TokenKind :: AmpersandAmpersand , RuleKind :: AndExpression , 7u8 , 8u8)) , Some ('=') => Ok ((TokenKind :: AmpersandEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Ampersand , RuleKind :: BitAndExpression , 17u8 , 18u8)) } None => Ok ((TokenKind :: Ampersand , RuleKind :: BitAndExpression , 17u8 , 18u8)) } } , Some ('*') => { let start_position = stream . position () ; match stream . next () { Some ('*') => Ok ((TokenKind :: StarStar , RuleKind :: ExponentiationExpression , 25u8 , 26u8)) , Some ('=') => Ok ((TokenKind :: StarEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Star , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Star , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('+') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PlusEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Plus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } None => Ok ((TokenKind :: Plus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } } , Some ('-') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: MinusEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Minus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } None => Ok ((TokenKind :: Minus , RuleKind :: AddSubExpression , 21u8 , 22u8)) } } , Some ('/') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: SlashEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Slash , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } None => Ok ((TokenKind :: Slash , RuleKind :: MulDivModExpression , 23u8 , 24u8)) } } , Some ('<') => { let start_position = stream . position () ; match stream . next () { Some ('<') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: LessLessEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: LessLess , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: LessLess , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some ('=') => Ok ((TokenKind :: LessEqual , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Less , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } None => Ok ((TokenKind :: Less , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } } , Some ('=') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: EqualEqual , RuleKind :: EqualityComparisonExpression , 9u8 , 10u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Equal , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } None => Ok ((TokenKind :: Equal , RuleKind :: AssignmentExpression , 1u8 , 2u8)) } } , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterEqual , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterGreaterEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some ('>') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: GreaterGreaterGreaterEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: GreaterGreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: GreaterGreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: GreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } None => Ok ((TokenKind :: GreaterGreater , RuleKind :: ShiftExpression , 19u8 , 20u8)) } } , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Greater , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } None => Ok ((TokenKind :: Greater , RuleKind :: OrderComparisonExpression , 11u8 , 12u8)) } } , Some ('^') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: CaretEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Caret , RuleKind :: BitXOrExpression , 15u8 , 16u8)) } None => Ok ((TokenKind :: Caret , RuleKind :: BitXOrExpression , 15u8 , 16u8)) } } , Some ('|') => { let start_position = stream . position () ; match stream . next () { Some ('=') => Ok ((TokenKind :: PipeEqual , RuleKind :: AssignmentExpression , 1u8 , 2u8)) , Some ('|') => Ok ((TokenKind :: PipePipe , RuleKind :: OrExpression , 5u8 , 6u8)) , Some (_) => { stream . set_position (start_position) ; Ok ((TokenKind :: Pipe , RuleKind :: BitOrExpression , 13u8 , 14u8)) } None => Ok ((TokenKind :: Pipe , RuleKind :: BitOrExpression , 13u8 , 14u8)) } } , _ => Err (ParseError :: new (stream . position () , "'!=', or '%', or '%=', or '&', or '&&', or '&=', or '*', or '**', or '*=', or '+', or '+=', or '-', or '-=', or '/', or '/=', or '<', or '<<', or '<<=', or '<=', or '=', or '==', or '>', or '>=', or '>>', or '>>=', or '>>>', or '>>>=', or '^', or '^=', or '|', or '|=', or '||'")) }
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
                            error: ParseError::new(start, "';'"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | «ExternalKeyword» | «PayableKeyword» | «PureKeyword» | «ViewKeyword» | «VirtualKeyword» ;

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
                    if self.scan_external_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ExternalKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ExternalKeyword"),
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
                    if self.scan_payable_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PayableKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PayableKeyword"),
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
                    if self.scan_pure_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PureKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PureKeyword"),
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
                    if self.scan_view_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ViewKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ViewKeyword"),
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
                    if self.scan_virtual_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::VirtualKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "VirtualKeyword"),
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

    // FallbackFunctionDefinition = «FallbackKeyword» ParameterList { FallbackFunctionAttribute } [ «ReturnsKeyword» ParameterList ] ( ';' | Block ) ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_fallback_function_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_fallback_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::FallbackKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "FallbackKeyword"),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                            if self.scan_returns_keyword(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::ReturnsKeyword,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "ReturnsKeyword"),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                        RuleKind::_SEQUENCE,
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

    // ForStatement = «ForKeyword» '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_for_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_for_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ForKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ForKeyword"),
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
                                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                        RuleKind::_SEQUENCE,
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
                                                RuleKind::_DELIMITEDBY,
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // FunctionAttribute = ModifierInvocation | OverrideSpecifier | «ExternalKeyword» | «InternalKeyword» | «PayableKeyword» | «PrivateKeyword» | «PublicKeyword» | «PureKeyword» | «ViewKeyword» | «VirtualKeyword» ;

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
                    if self.scan_external_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ExternalKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ExternalKeyword"),
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
                    if self.scan_internal_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::InternalKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "InternalKeyword"),
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
                    if self.scan_payable_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PayableKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PayableKeyword"),
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
                    if self.scan_private_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PrivateKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PrivateKeyword"),
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
                    if self.scan_public_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PublicKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PublicKeyword"),
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
                    if self.scan_pure_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PureKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PureKeyword"),
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
                    if self.scan_view_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ViewKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ViewKeyword"),
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
                    if self.scan_virtual_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::VirtualKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "VirtualKeyword"),
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

    // FunctionDefinition = «FunctionKeyword» ( «Identifier» | «FallbackKeyword» | «ReceiveKeyword» ) ParameterList { FunctionAttribute } [ «ReturnsKeyword» ParameterList ] ( ';' | Block ) ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_function_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_function_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::FunctionKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "FunctionKeyword"),
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
                        if self.scan_fallback_keyword(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::FallbackKeyword,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "FallbackKeyword"),
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
                        if self.scan_receive_keyword(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::ReceiveKeyword,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "ReceiveKeyword"),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                            if self.scan_returns_keyword(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::ReturnsKeyword,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "ReturnsKeyword"),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                        RuleKind::_SEQUENCE,
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

    // FunctionType = «FunctionKeyword» ParameterList { «InternalKeyword» | «ExternalKeyword» | «PrivateKeyword» | «PublicKeyword» | «PureKeyword» | «ViewKeyword» | «PayableKeyword» } [ «ReturnsKeyword» ParameterList ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_function_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_function_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::FunctionKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "FunctionKeyword"),
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
                        match loop {
                            let start_position = stream.position();
                            let mut furthest_error;
                            match {
                                let leading_trivia = self.optional_leading_trivia(stream);
                                let start = stream.position();
                                if self.scan_internal_keyword(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::InternalKeyword,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "InternalKeyword"),
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
                                if self.scan_external_keyword(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::ExternalKeyword,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "ExternalKeyword"),
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
                                if self.scan_private_keyword(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::PrivateKeyword,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "PrivateKeyword"),
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
                                if self.scan_public_keyword(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::PublicKeyword,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "PublicKeyword"),
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
                                if self.scan_pure_keyword(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::PureKeyword,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "PureKeyword"),
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
                                if self.scan_view_keyword(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::ViewKeyword,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "ViewKeyword"),
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
                                if self.scan_payable_keyword(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::PayableKeyword,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "PayableKeyword"),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                            if self.scan_returns_keyword(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::ReturnsKeyword,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "ReturnsKeyword"),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                        RuleKind::_SEQUENCE,
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

    // IdentifierPath = «Identifier» { '.' «Identifier» } ;

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
                                        node: cst::Node::rule(RuleKind::_SEPARATEDBY, result),
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

    // IfStatement = «IfKeyword» '(' Expression ')' Statement [ «ElseKeyword» Statement ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_if_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_if_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::IfKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "IfKeyword"),
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
                                            RuleKind::_DELIMITEDBY,
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
                            if self.scan_else_keyword(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::ElseKeyword,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "ElseKeyword"),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                        RuleKind::_SEQUENCE,
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

    // ImportDirective = «ImportKeyword» ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_import_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_import_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ImportKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ImportKeyword"),
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
                            error: ParseError::new(start, "';'"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // InheritanceSpecifierList = «IsKeyword» InheritanceSpecifier { ',' InheritanceSpecifier } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_inheritance_specifier_list(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_is_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::IsKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "IsKeyword"),
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
                                            node: cst::Node::rule(RuleKind::_SEPARATEDBY, result),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // InterfaceDefinition = «InterfaceKeyword» «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_interface_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_interface_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::InterfaceKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "InterfaceKeyword"),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                                node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                                                RuleKind::_DELIMITEDBY,
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
                        RuleKind::_SEQUENCE,
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

    // LeadingTrivia = 1…{ «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;

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
                            if result.is_empty() {
                                break Fail { error };
                            }
                            stream.set_position(start_position);
                            break Pass {
                                node: cst::Node::rule(RuleKind::_REPEATED, result),
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

    // LibraryDefinition = «LibraryKeyword» «Identifier» '{' { ContractBodyElement } '}' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_library_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_library_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::LibraryKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "LibraryKeyword"),
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
                                                node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                                                RuleKind::_DELIMITEDBY,
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // (* v0.4.11 *) MappingKeyType = ( ElementaryType | IdentifierPath ) ;
    // (* v0.8.18 *) MappingKeyType = ( ElementaryType | IdentifierPath ) [ «Identifier» ] ;

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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0]),
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

    // MappingType = «MappingKeyword» '(' MappingKeyType "=>" MappingValueType ')' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_mapping_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_mapping_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::MappingKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "MappingKeyword"),
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
                                        RuleKind::_SEQUENCE,
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
                                                RuleKind::_DELIMITEDBY,
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // (* v0.4.11 *) MappingValueType = TypeName ;
    // (* v0.8.18 *) MappingValueType = TypeName [ «Identifier» ] ;

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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0]),
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

    // ModifierAttribute = OverrideSpecifier | «VirtualKeyword» ;

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
                    if self.scan_virtual_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::VirtualKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "VirtualKeyword"),
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

    // ModifierDefinition = «ModifierKeyword» «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_modifier_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_modifier_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ModifierKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ModifierKeyword"),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                        RuleKind::_SEQUENCE,
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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
                            error: ParseError::new(start, "':'"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // NamedArgumentList = '{' [ NamedArgument { ',' NamedArgument } ] '}' ;

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
                                                            RuleKind::_SEPARATEDBY,
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
                                        node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                            RuleKind::_DELIMITEDBY,
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

    // NewExpression = «NewKeyword» TypeName ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_new_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_new_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::NewKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "NewKeyword"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // NumberUnit = «DaysKeyword» | «EtherKeyword» | «FinneyKeyword» | «GweiKeyword» | «HoursKeyword» | «MinutesKeyword» | «SecondsKeyword» | «SzaboKeyword» | «WeeksKeyword» | «WeiKeyword» | «YearsKeyword» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_number_unit(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_days_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::DaysKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "DaysKeyword"),
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
                    if self.scan_ether_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::EtherKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "EtherKeyword"),
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
                    if self.scan_finney_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::FinneyKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "FinneyKeyword"),
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
                    if self.scan_gwei_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::GweiKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "GweiKeyword"),
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
                    if self.scan_hours_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::HoursKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "HoursKeyword"),
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
                    if self.scan_minutes_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::MinutesKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "MinutesKeyword"),
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
                    if self.scan_seconds_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::SecondsKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "SecondsKeyword"),
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
                    if self.scan_szabo_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::SzaboKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "SzaboKeyword"),
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
                    if self.scan_weeks_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::WeeksKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "WeeksKeyword"),
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
                    if self.scan_wei_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::WeiKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "WeiKeyword"),
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
                    if self.scan_years_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::YearsKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "YearsKeyword"),
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
                node: cst::Node::top_level_rule(RuleKind::NumberUnit, node),
                error,
            },
            fail => fail,
        }
    }

    // NumericLiteral = ( «HexLiteral» | «DecimalLiteral» ) [ NumberUnit ] ;

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
                    match self.parse_number_unit(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // OverrideSpecifier = «OverrideKeyword» [ '(' IdentifierPath { ',' IdentifierPath } ')' ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_override_specifier(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_override_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::OverrideKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "OverrideKeyword"),
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
                                                                RuleKind::_SEPARATEDBY,
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
                                                    RuleKind::_DELIMITEDBY,
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // ParameterList = '(' [ ParameterDeclaration { ',' ParameterDeclaration } ] ')' ;

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
                                                            RuleKind::_SEPARATEDBY,
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
                                        node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                            RuleKind::_DELIMITEDBY,
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

    // PayableType = «PayableKeyword» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_payable_type(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if self.scan_payable_keyword(stream) {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass {
                        node: cst::Node::token(
                            TokenKind::PayableKeyword,
                            Range { start, end },
                            leading_trivia,
                            trailing_trivia,
                        ),
                        error: None,
                    }
                } else {
                    Fail {
                        error: ParseError::new(start, "PayableKeyword"),
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

    // PositionalArgumentList = Expression { ',' Expression } ;

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
                                        node: cst::Node::rule(RuleKind::_SEPARATEDBY, result),
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

    // PragmaDirective = «PragmaKeyword» ( VersionPragma | ABICoderPragma | ExperimentalPragma ) ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_pragma_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_pragma_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PragmaKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PragmaKeyword"),
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
                            error: ParseError::new(start, "';'"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // PrimaryExpression = «Identifier» | TupleExpression | ArrayLiteral | StringExpression | NumericLiteral | BooleanLiteral | NewExpression | TypeExpression | ElementaryType ;

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
                match self.parse_boolean_literal(stream) {
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

    // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | «ExternalKeyword» | «PayableKeyword» | «VirtualKeyword» ;

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
                    if self.scan_external_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ExternalKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ExternalKeyword"),
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
                    if self.scan_payable_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PayableKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PayableKeyword"),
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
                    if self.scan_virtual_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::VirtualKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "VirtualKeyword"),
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

    // ReceiveFunctionDefinition = «ReceiveKeyword» ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_receive_function_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_receive_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ReceiveKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ReceiveKeyword"),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                        RuleKind::_SEQUENCE,
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

    // ReturnStatement = «ReturnKeyword» [ Expression ] ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_return_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_return_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ReturnKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ReturnKeyword"),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                            error: ParseError::new(start, "';'"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // RevertStatement = «RevertKeyword» [ IdentifierPath ] ArgumentList ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_revert_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_revert_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::RevertKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "RevertKeyword"),
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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

    // SelectedImport = «Identifier» [ «AsKeyword» «Identifier» ] ;

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
                            if self.scan_as_keyword(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::AsKeyword,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "AsKeyword"),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // SelectingImportDirective = '{' SelectedImport { ',' SelectedImport } '}' «FromKeyword» ImportPath ;

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
                                                            RuleKind::_SEPARATEDBY,
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
                                                RuleKind::_DELIMITEDBY,
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
                    if self.scan_from_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::FromKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "FromKeyword"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // SimpleImportDirective = ImportPath { «AsKeyword» «Identifier» } ;

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
                                if self.scan_as_keyword(stream) {
                                    let end = stream.position();
                                    let trailing_trivia = self.optional_trailing_trivia(stream);
                                    Pass {
                                        node: cst::Node::token(
                                            TokenKind::AsKeyword,
                                            Range { start, end },
                                            leading_trivia,
                                            trailing_trivia,
                                        ),
                                        error: None,
                                    }
                                } else {
                                    Fail {
                                        error: ParseError::new(start, "AsKeyword"),
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
                                node: cst::Node::rule(
                                    RuleKind::_SEQUENCE,
                                    vec![result_0, result_1],
                                ),
                                error: furthest_error,
                            };
                        } {
                            Fail { error } => {
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // SourceUnit = 1…{ Directive | Definition } [ EndOfFileTrivia ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_source_unit(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
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
                                if result.is_empty() {
                                    break Fail { error };
                                }
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                    match self.parse_end_of_file_trivia(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // StarImportDirective = '*' «AsKeyword» «Identifier» «FromKeyword» ImportPath ;

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
                            error: ParseError::new(start, "'*'"),
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
                    if self.scan_as_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::AsKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "AsKeyword"),
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
                    if self.scan_from_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::FromKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "FromKeyword"),
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
                        RuleKind::_SEQUENCE,
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

    // StateVariableAttribute = OverrideSpecifier | «ConstantKeyword» | «ImmutableKeyword» | «InternalKeyword» | «PrivateKeyword» | «PublicKeyword» ;

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
                    if self.scan_constant_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ConstantKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ConstantKeyword"),
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
                    if self.scan_immutable_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ImmutableKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ImmutableKeyword"),
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
                    if self.scan_internal_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::InternalKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "InternalKeyword"),
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
                    if self.scan_private_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PrivateKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PrivateKeyword"),
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
                    if self.scan_public_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::PublicKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "PublicKeyword"),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                                    error: ParseError::new(start, "'='"),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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

    // StringExpression = 1…{ «HexStringLiteral» } | 1…{ «AsciiStringLiteral» } | 1…{ «UnicodeStringLiteral» } ;

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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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

    // StructDefinition = «StructKeyword» «Identifier» '{' 1…{ StructMember } '}' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_struct_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_struct_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::StructKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "StructKeyword"),
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
                                                node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                                                RuleKind::_DELIMITEDBY,
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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
                            error: ParseError::new(start, "';'"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // TrailingTrivia = { «Whitespace» | «SingleLineComment» } «EndOfLine» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_trailing_trivia(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                    Pass { node, error } => {
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // TryStatement = «TryKeyword» Expression [ «ReturnsKeyword» ParameterList ] Block 1…{ CatchClause } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_try_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_try_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::TryKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "TryKeyword"),
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
                            if self.scan_returns_keyword(stream) {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass {
                                    node: cst::Node::token(
                                        TokenKind::ReturnsKeyword,
                                        Range { start, end },
                                        leading_trivia,
                                        trailing_trivia,
                                    ),
                                    error: None,
                                }
                            } else {
                                Fail {
                                    error: ParseError::new(start, "ReturnsKeyword"),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                        RuleKind::_SEQUENCE,
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

    // TupleDeconstructionStatement = '(' [ [ TypeName [ DataLocation ] «Identifier» | [ DataLocation ] «Identifier» ] { ',' [ TypeName [ DataLocation ] «Identifier» | [ DataLocation ] «Identifier» ] } ] ')' '=' Expression ';' ;

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
                                                                        RuleKind::_OPTIONAL,
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
                                                            RuleKind::_SEQUENCE,
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
                                                                        RuleKind::_OPTIONAL,
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
                                                            RuleKind::_SEQUENCE,
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
                                                            RuleKind::_OPTIONAL,
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
                                                                RuleKind::_SEPARATEDBY,
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
                                            node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                                RuleKind::_DELIMITEDBY,
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
                            error: ParseError::new(start, "'='"),
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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

    // TupleExpression = '(' [ Expression ] { ',' [ Expression ] } ')' ;

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
                                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                                        RuleKind::_SEPARATEDBY,
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
                                            RuleKind::_DELIMITEDBY,
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

    // TypeExpression = «TypeKeyword» '(' TypeName ')' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_type_expression(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_type_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::TypeKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "TypeKeyword"),
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
                                            RuleKind::_DELIMITEDBY,
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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
                                                    node: cst::Node::rule(
                                                        RuleKind::_OPTIONAL,
                                                        vec![],
                                                    ),
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
                                                        RuleKind::_DELIMITEDBY,
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // UncheckedBlock = «UncheckedKeyword» Block ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_unchecked_block(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_unchecked_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::UncheckedKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "UncheckedKeyword"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // UserDefinedValueTypeDefinition = «TypeKeyword» «Identifier» «IsKeyword» ElementaryType ';' ;

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
                    if self.scan_type_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::TypeKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "TypeKeyword"),
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
                    if self.scan_is_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::IsKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "IsKeyword"),
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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

    // UsingDirective = «UsingKeyword» ( IdentifierPath | '{' IdentifierPath { ',' IdentifierPath } '}' ) «ForKeyword» ( '*' | TypeName ) [ «GlobalKeyword» ] ';' ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_using_directive(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_using_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::UsingKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "UsingKeyword"),
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
                                                                RuleKind::_SEPARATEDBY,
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
                                                    RuleKind::_DELIMITEDBY,
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
                    if self.scan_for_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ForKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ForKeyword"),
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
                        if self.scan_global_keyword(stream) {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Pass {
                                node: cst::Node::token(
                                    TokenKind::GlobalKeyword,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia,
                                ),
                                error: None,
                            }
                        } else {
                            Fail {
                                error: ParseError::new(start, "GlobalKeyword"),
                            }
                        }
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                    error: ParseError::new(start, "'='"),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                            error: ParseError::new(start, "';'"),
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
                        RuleKind::_SEQUENCE,
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

    // VersionPragma = «SolidityKeyword» 1…{ VersionPragmaSpecifier } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_version_pragma(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_solidity_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::SolidityKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "SolidityKeyword"),
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
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // VersionPragmaSpecifier = [ «VersionPragmaOperator» ] «VersionPragmaValue» { '.' «VersionPragmaValue» } ;

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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                            node: cst::Node::rule(RuleKind::_SEPARATEDBY, result),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // WhileStatement = «WhileKeyword» '(' Expression ')' Statement ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_while_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_while_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::WhileKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "WhileKeyword"),
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
                                            RuleKind::_DELIMITEDBY,
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // YulAssignmentStatement = YulIdentifierPath { ',' YulIdentifierPath } ":=" YulExpression ;

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
                                            node: cst::Node::rule(RuleKind::_SEPARATEDBY, result),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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
                                            node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                                            RuleKind::_DELIMITEDBY,
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

    // YulBreakStatement = «BreakKeyword» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_break_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if self.scan_break_keyword(stream) {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass {
                        node: cst::Node::token(
                            TokenKind::BreakKeyword,
                            Range { start, end },
                            leading_trivia,
                            trailing_trivia,
                        ),
                        error: None,
                    }
                } else {
                    Fail {
                        error: ParseError::new(start, "BreakKeyword"),
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

    // YulContinueStatement = «ContinueKeyword» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_continue_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if self.scan_continue_keyword(stream) {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass {
                        node: cst::Node::token(
                            TokenKind::ContinueKeyword,
                            Range { start, end },
                            leading_trivia,
                            trailing_trivia,
                        ),
                        error: None,
                    }
                } else {
                    Fail {
                        error: ParseError::new(start, "ContinueKeyword"),
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

    // YulForStatement = «ForKeyword» YulBlock YulExpression YulBlock YulBlock ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_for_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_for_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::ForKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "ForKeyword"),
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
                        RuleKind::_SEQUENCE,
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

    // YulFunctionCall = YulIdentifierPath [ '(' [ YulExpression { ',' YulExpression } ] ')' ] ;

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
                                                                    RuleKind::_SEPARATEDBY,
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
                                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                                    RuleKind::_DELIMITEDBY,
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
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
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

    // YulFunctionDefinition = «FunctionKeyword» «YulIdentifier» '(' [ «YulIdentifier» { ',' «YulIdentifier» } ] ')' [ "->" «YulIdentifier» { ',' «YulIdentifier» } ] YulBlock ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_function_definition(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_function_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::FunctionKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "FunctionKeyword"),
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
                                            node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                                                RuleKind::_DELIMITEDBY,
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                        RuleKind::_SEQUENCE,
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

    // YulIdentifierPath = «YulIdentifier» { '.' «YulIdentifier» } ;

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
                                        node: cst::Node::rule(RuleKind::_SEPARATEDBY, result),
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

    // YulIfStatement = «IfKeyword» YulExpression YulBlock ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_if_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_if_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::IfKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "IfKeyword"),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // YulLeaveStatement = «LeaveKeyword» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_leave_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if self.scan_leave_keyword(stream) {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass {
                        node: cst::Node::token(
                            TokenKind::LeaveKeyword,
                            Range { start, end },
                            leading_trivia,
                            trailing_trivia,
                        ),
                        error: None,
                    }
                } else {
                    Fail {
                        error: ParseError::new(start, "LeaveKeyword"),
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

    // YulLiteral = BooleanLiteral | «YulHexLiteral» | «YulDecimalLiteral» | «HexStringLiteral» | «AsciiStringLiteral» ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_literal(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_boolean_literal(stream) {
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

    // YulSwitchStatement = «SwitchKeyword» YulExpression 1…{ ( «CaseKeyword» YulLiteral | «DefaultKeyword» ) YulBlock } ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_switch_statement(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_switch_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::SwitchKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "SwitchKeyword"),
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
                                        if self.scan_case_keyword(stream) {
                                            let end = stream.position();
                                            let trailing_trivia =
                                                self.optional_trailing_trivia(stream);
                                            Pass {
                                                node: cst::Node::token(
                                                    TokenKind::CaseKeyword,
                                                    Range { start, end },
                                                    leading_trivia,
                                                    trailing_trivia,
                                                ),
                                                error: None,
                                            }
                                        } else {
                                            Fail {
                                                error: ParseError::new(start, "CaseKeyword"),
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
                                            RuleKind::_SEQUENCE,
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
                                    if self.scan_default_keyword(stream) {
                                        let end = stream.position();
                                        let trailing_trivia = self.optional_trailing_trivia(stream);
                                        Pass {
                                            node: cst::Node::token(
                                                TokenKind::DefaultKeyword,
                                                Range { start, end },
                                                leading_trivia,
                                                trailing_trivia,
                                            ),
                                            error: None,
                                        }
                                    } else {
                                        Fail {
                                            error: ParseError::new(start, "DefaultKeyword"),
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
                                node: cst::Node::rule(
                                    RuleKind::_SEQUENCE,
                                    vec![result_0, result_1],
                                ),
                                error: furthest_error,
                            };
                        } {
                            Fail { error } => {
                                if result.is_empty() {
                                    break Fail { error };
                                }
                                stream.set_position(start_position);
                                break Pass {
                                    node: cst::Node::rule(RuleKind::_REPEATED, result),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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

    // YulVariableDeclaration = «LetKeyword» YulIdentifierPath { ',' YulIdentifierPath } [ ":=" YulExpression ] ;

    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parse_yul_variable_declaration(&self, stream: &mut Stream) -> ParseResult {
        match {
            loop {
                let mut furthest_error = None;
                let result_0 = match {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    if self.scan_let_keyword(stream) {
                        let end = stream.position();
                        let trailing_trivia = self.optional_trailing_trivia(stream);
                        Pass {
                            node: cst::Node::token(
                                TokenKind::LetKeyword,
                                Range { start, end },
                                leading_trivia,
                                trailing_trivia,
                            ),
                            error: None,
                        }
                    } else {
                        Fail {
                            error: ParseError::new(start, "LetKeyword"),
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
                                            node: cst::Node::rule(RuleKind::_SEPARATEDBY, result),
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
                            node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                node: cst::Node::rule(RuleKind::_OPTIONAL, vec![]),
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
                    node: cst::Node::rule(RuleKind::_SEQUENCE, vec![result_0, result_1, result_2]),
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
