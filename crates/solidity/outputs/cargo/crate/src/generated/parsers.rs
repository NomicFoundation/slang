// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::language::ParserResult::*;
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
    #[inline]
    fn parse_token_with_trivia<F>(
        &self,
        stream: &mut Stream,
        scanner: F,
        kind: TokenKind,
    ) -> ParserResult
    where
        F: Fn(&Self, &mut Stream) -> bool,
    {
        let leading_trivia = {
            let save = stream.position();
            match self.parse_leading_trivia(stream) {
                Fail { .. } => {
                    stream.set_position(save);
                    None
                }
                Pass { builder, .. } => Some(builder.build()),
            }
        };
        let start = stream.position();
        if !scanner(self, stream) {
            return Fail {
                error: ParseError::new(start, kind.as_ref()),
            };
        }
        let end = stream.position();
        let trailing_trivia = {
            let save = stream.position();
            match self.parse_trailing_trivia(stream) {
                Fail { .. } => {
                    stream.set_position(save);
                    None
                }
                Pass { builder, .. } => Some(builder.build()),
            }
        };
        let token = cst::Node::token(kind, Range { start, end }, leading_trivia, trailing_trivia);
        return Pass {
            builder: cst::NodeBuilder::single(token),
            error: None,
        };
    }
    #[inline]
    fn parse_token<F>(&self, stream: &mut Stream, scanner: F, kind: TokenKind) -> ParserResult
    where
        F: Fn(&Self, &mut Stream) -> bool,
    {
        let start = stream.position();
        if !scanner(self, stream) {
            return Fail {
                error: ParseError::new(start, kind.as_ref()),
            };
        }
        let end = stream.position();
        let token = cst::Node::token(kind, Range { start, end }, None, None);
        return Pass {
            builder: cst::NodeBuilder::single(token),
            error: None,
        };
    }
    // ABICoderPragma = «AbicoderKeyword» «Identifier»;

    #[allow(dead_code)]
    fn parse_abi_coder_pragma_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_abicoder_keyword,
                TokenKind::AbicoderKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_abi_coder_pragma(&self, stream: &mut Stream) -> ParserResult {
        self.parse_abi_coder_pragma_0_4_11(stream)
            .with_kind(RuleKind::ABICoderPragma)
    }

    // AddSubOperator = «Plus» | «Minus»;

    #[allow(dead_code)]
    fn parse_add_sub_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(stream, Self::scan_plus, TokenKind::Plus) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_minus, TokenKind::Minus) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_add_sub_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_add_sub_operator_0_4_11(stream)
            .with_kind(RuleKind::AddSubOperator)
    }

    // AddressType = «AddressKeyword» «PayableKeyword»?;

    #[allow(dead_code)]
    fn parse_address_type_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_address_keyword,
                TokenKind::AddressKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                let start_position = stream.position();
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_payable_keyword,
                    TokenKind::PayableKeyword,
                ) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_address_type(&self, stream: &mut Stream) -> ParserResult {
        self.parse_address_type_0_4_11(stream)
            .with_kind(RuleKind::AddressType)
    }

    // AndOperator = «AmpersandAmpersand»;

    #[allow(dead_code)]
    fn parse_and_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            Self::scan_ampersand_ampersand,
            TokenKind::AmpersandAmpersand,
        )
    }

    #[inline]
    pub(crate) fn parse_and_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_and_operator_0_4_11(stream)
            .with_kind(RuleKind::AndOperator)
    }

    // ArgumentList = «OpenParen» (PositionalArgumentList | NamedArgumentList)? «CloseParen»;

    #[allow(dead_code)]
    fn parse_argument_list_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(stream, Self::scan_open_paren, TokenKind::OpenParen)
            {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
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
                                    builder: cst::NodeBuilder::empty(start_position),
                                    error: Some(error),
                                }
                            }
                            pass => pass,
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_paren,
                                TokenKind::CloseParen,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_argument_list(&self, stream: &mut Stream) -> ParserResult {
        self.parse_argument_list_0_4_11(stream)
            .with_kind(RuleKind::ArgumentList)
    }

    // ArrayLiteral = «OpenBracket» Expression («Comma» Expression)* «CloseBracket»;

    #[allow(dead_code)]
    fn parse_array_literal_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(
                stream,
                Self::scan_open_bracket,
                TokenKind::OpenBracket,
            ) {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
                } => {
                    match {
                        let mut result = Vec::new();
                        loop {
                            match self.parse_expression(stream) {
                                err @ Fail { .. } => break err,
                                Pass { builder, .. } => {
                                    result.push(builder);
                                    let save = stream.position();
                                    match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_comma,
                                        TokenKind::Comma,
                                    ) {
                                        Fail { error } => {
                                            stream.set_position(save);
                                            break Pass {
                                                builder: cst::NodeBuilder::multiple(result),
                                                error: Some(error),
                                            };
                                        }
                                        Pass { builder, .. } => result.push(builder),
                                    }
                                }
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_bracket,
                                TokenKind::CloseBracket,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_array_literal(&self, stream: &mut Stream) -> ParserResult {
        self.parse_array_literal_0_4_11(stream)
            .with_kind(RuleKind::ArrayLiteral)
    }

    // AssemblyFlags = «OpenParen» «DoubleQuotedAsciiStringLiteral» («Comma» «DoubleQuotedAsciiStringLiteral»)* «CloseParen»;

    #[allow(dead_code)]
    fn parse_assembly_flags_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(stream, Self::scan_open_paren, TokenKind::OpenParen)
            {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
                } => {
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match self.parse_token_with_trivia(
                            stream,
                            Self::scan_double_quoted_ascii_string_literal,
                            TokenKind::DoubleQuotedAsciiStringLiteral,
                        ) {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
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
                                    let result_0 = match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_comma,
                                        TokenKind::Comma,
                                    ) {
                                        Pass { builder, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            builder
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_double_quoted_ascii_string_literal,
                                        TokenKind::DoubleQuotedAsciiStringLiteral,
                                    ) {
                                        Pass { builder, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            builder
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            result_0, result_1,
                                        ]),
                                        error: furthest_error,
                                    };
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        break Pass {
                                            builder: if result.is_empty() {
                                                cst::NodeBuilder::empty(start_position)
                                            } else {
                                                cst::NodeBuilder::multiple(result)
                                            },
                                            error: Some(error),
                                        };
                                    }
                                    Pass { builder, .. } => result.push(builder),
                                }
                            }
                        } {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_paren,
                                TokenKind::CloseParen,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_assembly_flags(&self, stream: &mut Stream) -> ParserResult {
        self.parse_assembly_flags_0_4_11(stream)
            .with_kind(RuleKind::AssemblyFlags)
    }

    // AssemblyStatement = «AssemblyKeyword» «Evmasm»? AssemblyFlags? YulBlock;

    #[allow(dead_code)]
    fn parse_assembly_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_assembly_keyword,
                TokenKind::AssemblyKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                let start_position = stream.position();
                match self.parse_token_with_trivia(stream, Self::scan_evmasm, TokenKind::Evmasm) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_3 = match self.parse_yul_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_assembly_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_assembly_statement_0_4_11(stream)
            .with_kind(RuleKind::AssemblyStatement)
    }

    // AssignmentOperator = «Equal»
    //                    | «BarEqual»
    //                    | «CaretEqual»
    //                    | «AmpersandEqual»
    //                    | «LessThanLessThanEqual»
    //                    | «GreaterThanGreaterThanEqual»
    //                    | «GreaterThanGreaterThanGreaterThanEqual»
    //                    | «PlusEqual»
    //                    | «MinusEqual»
    //                    | «AsteriskEqual»
    //                    | «SlashEqual»
    //                    | «PercentEqual»;

    #[allow(dead_code)]
    fn parse_assignment_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(stream, Self::scan_equal, TokenKind::Equal) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_bar_equal, TokenKind::BarEqual) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_caret_equal,
                TokenKind::CaretEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_ampersand_equal,
                TokenKind::AmpersandEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_less_than_less_than_equal,
                TokenKind::LessThanLessThanEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_greater_than_greater_than_equal,
                TokenKind::GreaterThanGreaterThanEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_greater_than_greater_than_greater_than_equal,
                TokenKind::GreaterThanGreaterThanGreaterThanEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_plus_equal, TokenKind::PlusEqual)
            {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_minus_equal,
                TokenKind::MinusEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_asterisk_equal,
                TokenKind::AsteriskEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_slash_equal,
                TokenKind::SlashEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_percent_equal,
                TokenKind::PercentEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_assignment_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_assignment_operator_0_4_11(stream)
            .with_kind(RuleKind::AssignmentOperator)
    }

    // AsteriskImport = «Asterisk» ImportAlias «FromKeyword» ImportPath;

    #[allow(dead_code)]
    fn parse_asterisk_import_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_asterisk,
                TokenKind::Asterisk,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_import_alias(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_token_with_trivia(
                stream,
                Self::scan_from_keyword,
                TokenKind::FromKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_3 = match self.parse_import_path(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_asterisk_import(&self, stream: &mut Stream) -> ParserResult {
        self.parse_asterisk_import_0_4_11(stream)
            .with_kind(RuleKind::AsteriskImport)
    }

    // BitAndOperator = «Ampersand»;

    #[allow(dead_code)]
    fn parse_bit_and_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, Self::scan_ampersand, TokenKind::Ampersand)
    }

    #[inline]
    pub(crate) fn parse_bit_and_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_bit_and_operator_0_4_11(stream)
            .with_kind(RuleKind::BitAndOperator)
    }

    // BitOrOperator = «Bar»;

    #[allow(dead_code)]
    fn parse_bit_or_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, Self::scan_bar, TokenKind::Bar)
    }

    #[inline]
    pub(crate) fn parse_bit_or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_bit_or_operator_0_4_11(stream)
            .with_kind(RuleKind::BitOrOperator)
    }

    // BitXOrOperator = «Caret»;

    #[allow(dead_code)]
    fn parse_bit_x_or_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, Self::scan_caret, TokenKind::Caret)
    }

    #[inline]
    pub(crate) fn parse_bit_x_or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_bit_x_or_operator_0_4_11(stream)
            .with_kind(RuleKind::BitXOrOperator)
    }

    // (* v0.4.11 *)
    // Block = «OpenBrace» Statement* «CloseBrace»;

    #[allow(dead_code)]
    fn parse_block_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(stream, Self::scan_open_brace, TokenKind::OpenBrace)
            {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
                } => {
                    match {
                        let mut result = Vec::new();
                        loop {
                            let start_position = stream.position();
                            match self.parse_statement(stream) {
                                Fail { error } => {
                                    stream.set_position(start_position);
                                    break Pass {
                                        builder: if result.is_empty() {
                                            cst::NodeBuilder::empty(start_position)
                                        } else {
                                            cst::NodeBuilder::multiple(result)
                                        },
                                        error: Some(error),
                                    };
                                }
                                Pass { builder, .. } => result.push(builder),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_brace,
                                TokenKind::CloseBrace,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    // (* v0.8.0 *)
    // Block = «OpenBrace» (Statement | UncheckedBlock)* «CloseBrace»;

    #[allow(dead_code)]
    fn parse_block_0_8_0(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(stream, Self::scan_open_brace, TokenKind::OpenBrace)
            {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
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
                                        builder: if result.is_empty() {
                                            cst::NodeBuilder::empty(start_position)
                                        } else {
                                            cst::NodeBuilder::multiple(result)
                                        },
                                        error: Some(error),
                                    };
                                }
                                Pass { builder, .. } => result.push(builder),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_brace,
                                TokenKind::CloseBrace,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    fn dispatch_parse_block(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            self.parse_block_0_8_0(stream)
        } else {
            self.parse_block_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_block(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_block(stream).with_kind(RuleKind::Block)
    }

    // BooleanLiteral = «TrueKeyword» | «FalseKeyword»;

    #[allow(dead_code)]
    fn parse_boolean_literal_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_true_keyword,
                TokenKind::TrueKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_false_keyword,
                TokenKind::FalseKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_boolean_literal(&self, stream: &mut Stream) -> ParserResult {
        self.parse_boolean_literal_0_4_11(stream)
            .with_kind(RuleKind::BooleanLiteral)
    }

    // BreakStatement = «BreakKeyword» «Semicolon»;

    #[allow(dead_code)]
    fn parse_break_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(
                stream,
                Self::scan_break_keyword,
                TokenKind::BreakKeyword,
            ) {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_break_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_break_statement_0_4_11(stream)
            .with_kind(RuleKind::BreakStatement)
    }

    // (* v0.6.0 *)
    // CatchClause = «CatchKeyword» («Identifier»? ParameterList)? Block;

    #[allow(dead_code)]
    fn parse_catch_clause_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_catch_keyword,
                TokenKind::CatchKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_identifier,
                            TokenKind::Identifier,
                        ) {
                            Fail { error } => {
                                stream.set_position(start_position);
                                Pass {
                                    builder: cst::NodeBuilder::empty(start_position),
                                    error: Some(error),
                                }
                            }
                            pass => pass,
                        }
                    } {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_1 = match self.parse_parameter_list(stream) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    break Pass {
                        builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                        error: furthest_error,
                    };
                } {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_catch_clause(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.parse_catch_clause_0_6_0(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_catch_clause(&self, stream: &mut Stream) -> Option<ParserResult> {
        self.dispatch_parse_catch_clause(stream)
            .map(|body| body.with_kind(RuleKind::CatchClause))
    }

    #[inline]
    pub(crate) fn parse_catch_clause(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_catch_clause(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // ConditionalOperator = «QuestionMark» Expression «Colon» Expression;

    #[allow(dead_code)]
    fn parse_conditional_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_question_mark,
                TokenKind::QuestionMark,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_expression(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 =
                match self.parse_token_with_trivia(stream, Self::scan_colon, TokenKind::Colon) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
            let result_3 = match self.parse_expression(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_conditional_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_conditional_operator_0_4_11(stream)
            .with_kind(RuleKind::ConditionalOperator)
    }

    // ConstantDefinition = TypeName «ConstantKeyword» «Identifier» «Equal» Expression «Semicolon»;

    #[allow(dead_code)]
    fn parse_constant_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_constant_keyword,
                    TokenKind::ConstantKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_equal,
                    TokenKind::Equal,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match self.parse_expression(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![
                        result_0, result_1, result_2, result_3, result_4,
                    ]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_constant_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_constant_definition_0_4_11(stream)
            .with_kind(RuleKind::ConstantDefinition)
    }

    // (* v0.4.22 *)
    // ConstructorAttribute = ModifierInvocation | «InternalKeyword» | «PayableKeyword» | «PublicKeyword»;

    #[allow(dead_code)]
    fn parse_constructor_attribute_0_4_22(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_modifier_invocation(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_internal_keyword,
                TokenKind::InternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_payable_keyword,
                TokenKind::PayableKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_public_keyword,
                TokenKind::PublicKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_constructor_attribute(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_4_22 {
            Some(self.parse_constructor_attribute_0_4_22(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_constructor_attribute(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_constructor_attribute(stream)
            .map(|body| body.with_kind(RuleKind::ConstructorAttribute))
    }

    #[inline]
    pub(crate) fn parse_constructor_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_constructor_attribute(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.22 *)
    // ConstructorDefinition = «ConstructorKeyword» ParameterList ConstructorAttribute* Block;

    #[allow(dead_code)]
    fn parse_constructor_definition_0_4_22(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_constructor_keyword,
                TokenKind::ConstructorKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_parameter_list(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                                builder: if result.is_empty() {
                                    cst::NodeBuilder::empty(start_position)
                                } else {
                                    cst::NodeBuilder::multiple(result)
                                },
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_3 = match self.parse_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_constructor_definition(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_4_22 {
            Some(self.parse_constructor_definition_0_4_22(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_constructor_definition(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_constructor_definition(stream)
            .map(|body| body.with_kind(RuleKind::ConstructorDefinition))
    }

    #[inline]
    pub(crate) fn parse_constructor_definition(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_constructor_definition(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // ContinueStatement = «ContinueKeyword» «Semicolon»;

    #[allow(dead_code)]
    fn parse_continue_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(
                stream,
                Self::scan_continue_keyword,
                TokenKind::ContinueKeyword,
            ) {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_continue_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_continue_statement_0_4_11(stream)
            .with_kind(RuleKind::ContinueStatement)
    }

    // (* v0.4.11 *)
    // ContractBodyElement = UsingDirective
    //                     | FunctionDefinition
    //                     | UnnamedFunctionDefinition
    //                     | ModifierDefinition
    //                     | StructDefinition
    //                     | EnumDefinition
    //                     | EventDefinition
    //                     | ErrorDefinition
    //                     | StateVariableDeclaration;

    #[allow(dead_code)]
    fn parse_contract_body_element_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_using_directive(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_function_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_unnamed_function_definition(stream) {
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
    }

    // (* v0.4.22 *)
    // ContractBodyElement = UsingDirective
    //                     | ConstructorDefinition
    //                     | FunctionDefinition
    //                     | UnnamedFunctionDefinition
    //                     | ModifierDefinition
    //                     | StructDefinition
    //                     | EnumDefinition
    //                     | EventDefinition
    //                     | ErrorDefinition
    //                     | StateVariableDeclaration;

    #[allow(dead_code)]
    fn parse_contract_body_element_0_4_22(&self, stream: &mut Stream) -> ParserResult {
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
            match self.parse_unnamed_function_definition(stream) {
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
    }

    // (* v0.6.0 *)
    // ContractBodyElement = UsingDirective
    //                     | ConstructorDefinition
    //                     | FunctionDefinition
    //                     | FallbackFunctionDefinition
    //                     | ReceiveFunctionDefinition
    //                     | ModifierDefinition
    //                     | StructDefinition
    //                     | EnumDefinition
    //                     | EventDefinition
    //                     | ErrorDefinition
    //                     | StateVariableDeclaration;

    #[allow(dead_code)]
    fn parse_contract_body_element_0_6_0(&self, stream: &mut Stream) -> ParserResult {
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
    }

    // (* v0.8.8 *)
    // ContractBodyElement = UsingDirective
    //                     | ConstructorDefinition
    //                     | FunctionDefinition
    //                     | FallbackFunctionDefinition
    //                     | ReceiveFunctionDefinition
    //                     | ModifierDefinition
    //                     | StructDefinition
    //                     | EnumDefinition
    //                     | UserDefinedValueTypeDefinition
    //                     | EventDefinition
    //                     | ErrorDefinition
    //                     | StateVariableDeclaration;

    #[allow(dead_code)]
    fn parse_contract_body_element_0_8_8(&self, stream: &mut Stream) -> ParserResult {
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
    }

    fn dispatch_parse_contract_body_element(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_8 {
            self.parse_contract_body_element_0_8_8(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.parse_contract_body_element_0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_4_22 {
            self.parse_contract_body_element_0_4_22(stream)
        } else {
            self.parse_contract_body_element_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_contract_body_element(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_contract_body_element(stream)
    }

    // (* v0.4.11 *)
    // ContractDefinition = «ContractKeyword» «Identifier» InheritanceSpecifierList? «OpenBrace» ContractBodyElement* «CloseBrace»;

    #[allow(dead_code)]
    fn parse_contract_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_contract_keyword,
                TokenKind::ContractKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_3 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_brace,
                    TokenKind::OpenBrace,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                let start_position = stream.position();
                                match self.parse_contract_body_element(stream) {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        break Pass {
                                            builder: if result.is_empty() {
                                                cst::NodeBuilder::empty(start_position)
                                            } else {
                                                cst::NodeBuilder::multiple(result)
                                            },
                                            error: Some(error),
                                        };
                                    }
                                    Pass { builder, .. } => result.push(builder),
                                }
                            }
                        }
                        .with_kind(RuleKind::ContractBodyElements)
                        {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_brace,
                                    TokenKind::CloseBrace,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    // (* v0.6.0 *)
    // ContractDefinition = «AbstractKeyword»? «ContractKeyword» «Identifier» InheritanceSpecifierList? «OpenBrace» ContractBodyElement* «CloseBrace»;

    #[allow(dead_code)]
    fn parse_contract_definition_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match {
                let start_position = stream.position();
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_abstract_keyword,
                    TokenKind::AbstractKeyword,
                ) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_contract_keyword,
                TokenKind::ContractKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_4 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_brace,
                    TokenKind::OpenBrace,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                let start_position = stream.position();
                                match self.parse_contract_body_element(stream) {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        break Pass {
                                            builder: if result.is_empty() {
                                                cst::NodeBuilder::empty(start_position)
                                            } else {
                                                cst::NodeBuilder::multiple(result)
                                            },
                                            error: Some(error),
                                        };
                                    }
                                    Pass { builder, .. } => result.push(builder),
                                }
                            }
                        }
                        .with_kind(RuleKind::ContractBodyElements)
                        {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_brace,
                                    TokenKind::CloseBrace,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![
                    result_0, result_1, result_2, result_3, result_4,
                ]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_contract_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.parse_contract_definition_0_6_0(stream)
        } else {
            self.parse_contract_definition_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_contract_definition(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_contract_definition(stream)
            .with_kind(RuleKind::ContractDefinition)
    }

    // (* v0.4.11 *)
    // DataLocation = «MemoryKeyword» | «StorageKeyword»;

    #[allow(dead_code)]
    fn parse_data_location_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_memory_keyword,
                TokenKind::MemoryKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_storage_keyword,
                TokenKind::StorageKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.5.0 *)
    // DataLocation = «MemoryKeyword» | «StorageKeyword» | «CalldataKeyword»;

    #[allow(dead_code)]
    fn parse_data_location_0_5_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_memory_keyword,
                TokenKind::MemoryKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_storage_keyword,
                TokenKind::StorageKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_calldata_keyword,
                TokenKind::CalldataKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_data_location(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.parse_data_location_0_5_0(stream)
        } else {
            self.parse_data_location_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_data_location(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_data_location(stream)
            .with_kind(RuleKind::DataLocation)
    }

    // (* v0.4.11 *)
    // Definition = ConstantDefinition
    //            | ContractDefinition
    //            | EnumDefinition
    //            | ErrorDefinition
    //            | FunctionDefinition
    //            | InterfaceDefinition
    //            | LibraryDefinition
    //            | StructDefinition;

    #[allow(dead_code)]
    fn parse_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_constant_definition(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_contract_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_enum_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_error_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_function_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
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
            match self.parse_struct_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.8.8 *)
    // Definition = ConstantDefinition
    //            | ContractDefinition
    //            | EnumDefinition
    //            | ErrorDefinition
    //            | FunctionDefinition
    //            | InterfaceDefinition
    //            | LibraryDefinition
    //            | StructDefinition
    //            | UserDefinedValueTypeDefinition;

    #[allow(dead_code)]
    fn parse_definition_0_8_8(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_constant_definition(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_contract_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_enum_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_error_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_function_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
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
            match self.parse_struct_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_user_defined_value_type_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_8 {
            self.parse_definition_0_8_8(stream)
        } else {
            self.parse_definition_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_definition(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_definition(stream)
            .with_kind(RuleKind::Definition)
    }

    // DeleteStatement = «DeleteKeyword» Expression «Semicolon»;

    #[allow(dead_code)]
    fn parse_delete_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_delete_keyword,
                    TokenKind::DeleteKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_expression(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_delete_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_delete_statement_0_4_11(stream)
            .with_kind(RuleKind::DeleteStatement)
    }

    // Directive = PragmaDirective | ImportDirective | UsingDirective;

    #[allow(dead_code)]
    fn parse_directive_0_4_11(&self, stream: &mut Stream) -> ParserResult {
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
    }

    #[inline]
    pub(crate) fn parse_directive(&self, stream: &mut Stream) -> ParserResult {
        self.parse_directive_0_4_11(stream)
            .with_kind(RuleKind::Directive)
    }

    // DoWhileStatement = «DoKeyword» Statement «WhileKeyword» «OpenParen» Expression «CloseParen» «Semicolon»;

    #[allow(dead_code)]
    fn parse_do_while_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_do_keyword,
                    TokenKind::DoKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_statement(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_while_keyword,
                    TokenKind::WhileKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_open_paren,
                        TokenKind::OpenParen,
                    ) {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: open_node, ..
                        } => match self.parse_expression(stream) {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_paren,
                                    TokenKind::CloseParen,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        },
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![
                        result_0, result_1, result_2, result_3,
                    ]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_do_while_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_do_while_statement_0_4_11(stream)
            .with_kind(RuleKind::DoWhileStatement)
    }

    // (* v0.4.11 *)
    // ElementaryType = «BoolKeyword»
    //                | «StringKeyword»
    //                | AddressType
    //                | PayableType
    //                | «ByteType»
    //                | «FixedBytesType»
    //                | «SignedIntegerType»
    //                | «UnsignedIntegerType»
    //                | «SignedFixedType»
    //                | «UnsignedFixedType»;

    #[allow(dead_code)]
    fn parse_elementary_type_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_bool_keyword,
                TokenKind::BoolKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_string_keyword,
                TokenKind::StringKeyword,
            ) {
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
            match self.parse_token_with_trivia(stream, Self::scan_byte_type, TokenKind::ByteType) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_fixed_bytes_type,
                TokenKind::FixedBytesType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_signed_integer_type,
                TokenKind::SignedIntegerType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_unsigned_integer_type,
                TokenKind::UnsignedIntegerType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_signed_fixed_type,
                TokenKind::SignedFixedType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_unsigned_fixed_type,
                TokenKind::UnsignedFixedType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.8.0 *)
    // ElementaryType = «BoolKeyword»
    //                | «StringKeyword»
    //                | AddressType
    //                | PayableType
    //                | «FixedBytesType»
    //                | «SignedIntegerType»
    //                | «UnsignedIntegerType»
    //                | «SignedFixedType»
    //                | «UnsignedFixedType»;

    #[allow(dead_code)]
    fn parse_elementary_type_0_8_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_bool_keyword,
                TokenKind::BoolKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_string_keyword,
                TokenKind::StringKeyword,
            ) {
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
            match self.parse_token_with_trivia(
                stream,
                Self::scan_fixed_bytes_type,
                TokenKind::FixedBytesType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_signed_integer_type,
                TokenKind::SignedIntegerType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_unsigned_integer_type,
                TokenKind::UnsignedIntegerType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_signed_fixed_type,
                TokenKind::SignedFixedType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_unsigned_fixed_type,
                TokenKind::UnsignedFixedType,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_elementary_type(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            self.parse_elementary_type_0_8_0(stream)
        } else {
            self.parse_elementary_type_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_elementary_type(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_elementary_type(stream)
            .with_kind(RuleKind::ElementaryType)
    }

    // (* v0.4.21 *)
    // EmitStatement = «EmitKeyword» IdentifierPath ArgumentList «Semicolon»;

    #[allow(dead_code)]
    fn parse_emit_statement_0_4_21(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_emit_keyword,
                    TokenKind::EmitKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_identifier_path(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_argument_list(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    fn dispatch_parse_emit_statement(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_4_21 {
            Some(self.parse_emit_statement_0_4_21(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_emit_statement(&self, stream: &mut Stream) -> Option<ParserResult> {
        self.dispatch_parse_emit_statement(stream)
            .map(|body| body.with_kind(RuleKind::EmitStatement))
    }

    #[inline]
    pub(crate) fn parse_emit_statement(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_emit_statement(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // EndOfFileTrivia = («Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment»)+;

    #[allow(dead_code)]
    fn parse_end_of_file_trivia_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut result = Vec::new();
            loop {
                let start_position = stream.position();
                match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_token(stream, Self::scan_whitespace, TokenKind::Whitespace) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_token(stream, Self::scan_end_of_line, TokenKind::EndOfLine) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_token(
                        stream,
                        Self::scan_multiline_comment,
                        TokenKind::MultilineComment,
                    ) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_token(
                        stream,
                        Self::scan_single_line_comment,
                        TokenKind::SingleLineComment,
                    ) {
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
                            builder: cst::NodeBuilder::multiple(result),
                            error: Some(error),
                        };
                    }
                    Pass { builder, .. } => result.push(builder),
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_end_of_file_trivia(&self, stream: &mut Stream) -> ParserResult {
        self.parse_end_of_file_trivia_0_4_11(stream)
            .with_kind(RuleKind::EndOfFileTrivia)
    }

    // EnumDefinition = «EnumKeyword» «Identifier» «OpenBrace» («Identifier» («Comma» «Identifier»)*)? «CloseBrace»;

    #[allow(dead_code)]
    fn parse_enum_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_enum_keyword,
                TokenKind::EnumKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_brace,
                    TokenKind::OpenBrace,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => {
                        match {
                            let start_position = stream.position();
                            match {
                                let mut result = Vec::new();
                                loop {
                                    match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_identifier,
                                        TokenKind::Identifier,
                                    ) {
                                        err @ Fail { .. } => break err,
                                        Pass { builder, .. } => {
                                            result.push(builder);
                                            let save = stream.position();
                                            match self.parse_token_with_trivia(
                                                stream,
                                                Self::scan_comma,
                                                TokenKind::Comma,
                                            ) {
                                                Fail { error } => {
                                                    stream.set_position(save);
                                                    break Pass {
                                                        builder: cst::NodeBuilder::multiple(result),
                                                        error: Some(error),
                                                    };
                                                }
                                                Pass { builder, .. } => result.push(builder),
                                            }
                                        }
                                    }
                                }
                            } {
                                Fail { error } => {
                                    stream.set_position(start_position);
                                    Pass {
                                        builder: cst::NodeBuilder::empty(start_position),
                                        error: Some(error),
                                    }
                                }
                                pass => pass,
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_brace,
                                    TokenKind::CloseBrace,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_enum_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_enum_definition_0_4_11(stream)
            .with_kind(RuleKind::EnumDefinition)
    }

    // EqualityComparisonOperator = «EqualEqual» | «BangEqual»;

    #[allow(dead_code)]
    fn parse_equality_comparison_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_equal_equal,
                TokenKind::EqualEqual,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_bang_equal, TokenKind::BangEqual)
            {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_equality_comparison_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_equality_comparison_operator_0_4_11(stream)
            .with_kind(RuleKind::EqualityComparisonOperator)
    }

    // ErrorDefinition = «ErrorKeyword» «Identifier» «OpenParen» (ErrorParameter («Comma» ErrorParameter)*)? «CloseParen» «Semicolon»;

    #[allow(dead_code)]
    fn parse_error_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_error_keyword,
                    TokenKind::ErrorKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_open_paren,
                        TokenKind::OpenParen,
                    ) {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: open_node, ..
                        } => {
                            match {
                                let start_position = stream.position();
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match self.parse_error_parameter(stream) {
                                            err @ Fail { .. } => break err,
                                            Pass { builder, .. } => {
                                                result.push(builder);
                                                let save = stream.position();
                                                match self.parse_token_with_trivia(
                                                    stream,
                                                    Self::scan_comma,
                                                    TokenKind::Comma,
                                                ) {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            builder: cst::NodeBuilder::multiple(
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { builder, .. } => result.push(builder),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        Pass {
                                            builder: cst::NodeBuilder::empty(start_position),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    builder: expr_node,
                                    error: expr_error,
                                } => {
                                    match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_close_paren,
                                        TokenKind::CloseParen,
                                    ) {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            builder: close_node,
                                            ..
                                        } => Pass {
                                            builder: cst::NodeBuilder::multiple(vec![
                                                open_node, expr_node, close_node,
                                            ]),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_error_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_error_definition_0_4_11(stream)
            .with_kind(RuleKind::ErrorDefinition)
    }

    // ErrorParameter = TypeName «Identifier»?;

    #[allow(dead_code)]
    fn parse_error_parameter_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_type_name(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                let start_position = stream.position();
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_error_parameter(&self, stream: &mut Stream) -> ParserResult {
        self.parse_error_parameter_0_4_11(stream)
            .with_kind(RuleKind::ErrorParameter)
    }

    // EventDefinition = «EventKeyword» «Identifier» «OpenParen» (EventParameter («Comma» EventParameter)*)? «CloseParen» «AnonymousKeyword»? «Semicolon»;

    #[allow(dead_code)]
    fn parse_event_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_event_keyword,
                    TokenKind::EventKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_open_paren,
                        TokenKind::OpenParen,
                    ) {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: open_node, ..
                        } => {
                            match {
                                let start_position = stream.position();
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match self.parse_event_parameter(stream) {
                                            err @ Fail { .. } => break err,
                                            Pass { builder, .. } => {
                                                result.push(builder);
                                                let save = stream.position();
                                                match self.parse_token_with_trivia(
                                                    stream,
                                                    Self::scan_comma,
                                                    TokenKind::Comma,
                                                ) {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            builder: cst::NodeBuilder::multiple(
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { builder, .. } => result.push(builder),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        Pass {
                                            builder: cst::NodeBuilder::empty(start_position),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    builder: expr_node,
                                    error: expr_error,
                                } => {
                                    match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_close_paren,
                                        TokenKind::CloseParen,
                                    ) {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            builder: close_node,
                                            ..
                                        } => Pass {
                                            builder: cst::NodeBuilder::multiple(vec![
                                                open_node, expr_node, close_node,
                                            ]),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match {
                    let start_position = stream.position();
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_anonymous_keyword,
                        TokenKind::AnonymousKeyword,
                    ) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![
                        result_0, result_1, result_2, result_3,
                    ]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_event_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_event_definition_0_4_11(stream)
            .with_kind(RuleKind::EventDefinition)
    }

    // EventParameter = TypeName «IndexedKeyword»? «Identifier»?;

    #[allow(dead_code)]
    fn parse_event_parameter_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_type_name(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                let start_position = stream.position();
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_indexed_keyword,
                    TokenKind::IndexedKeyword,
                ) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match {
                let start_position = stream.position();
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_event_parameter(&self, stream: &mut Stream) -> ParserResult {
        self.parse_event_parameter_0_4_11(stream)
            .with_kind(RuleKind::EventParameter)
    }

    // ExperimentalPragma = «ExperimentalKeyword» «Identifier»;

    #[allow(dead_code)]
    fn parse_experimental_pragma_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_experimental_keyword,
                TokenKind::ExperimentalKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_experimental_pragma(&self, stream: &mut Stream) -> ParserResult {
        self.parse_experimental_pragma_0_4_11(stream)
            .with_kind(RuleKind::ExperimentalPragma)
    }

    // ExponentiationOperator = «AsteriskAsterisk»;

    #[allow(dead_code)]
    fn parse_exponentiation_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            Self::scan_asterisk_asterisk,
            TokenKind::AsteriskAsterisk,
        )
    }

    #[inline]
    pub(crate) fn parse_exponentiation_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_exponentiation_operator_0_4_11(stream)
            .with_kind(RuleKind::ExponentiationOperator)
    }

    // (* v0.4.11 *)
    // Expression = AssignmentExpression
    //            | ConditionalExpression
    //            | OrExpression
    //            | AndExpression
    //            | EqualityComparisonExpression
    //            | OrderComparisonExpression
    //            | BitOrExpression
    //            | BitXOrExpression
    //            | BitAndExpression
    //            | ShiftExpression
    //            | AddSubExpression
    //            | MulDivModExpression
    //            | ExponentiationExpression
    //            | UnaryPostfixExpression
    //            | UnaryPrefixExpression
    //            | FunctionCallExpression
    //            | MemberAccessExpression
    //            | IndexAccessExpression
    //            | PrimaryExpression;
    // AssignmentExpression = Expression AssignmentOperator Expression;
    // ConditionalExpression = Expression ConditionalOperator;
    // OrExpression = Expression OrOperator Expression;
    // AndExpression = Expression AndOperator Expression;
    // EqualityComparisonExpression = Expression EqualityComparisonOperator Expression;
    // OrderComparisonExpression = Expression OrderComparisonOperator Expression;
    // BitOrExpression = Expression BitOrOperator Expression;
    // BitXOrExpression = Expression BitXOrOperator Expression;
    // BitAndExpression = Expression BitAndOperator Expression;
    // ShiftExpression = Expression ShiftOperator Expression;
    // AddSubExpression = Expression AddSubOperator Expression;
    // MulDivModExpression = Expression MulDivModOperator Expression;
    // ExponentiationExpression = Expression ExponentiationOperator Expression;
    // UnaryPostfixExpression = Expression UnaryPostfixOperator;
    // UnaryPrefixExpression = UnaryPrefixOperator Expression;
    // FunctionCallExpression = Expression FunctionCallOperator;
    // MemberAccessExpression = Expression MemberAccessOperator;
    // IndexAccessExpression = Expression IndexAccessOperator;

    #[allow(dead_code)]
    fn parse_expression_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            enum Pratt {
                Operator {
                    kind: RuleKind,
                    builder: cst::NodeBuilder,
                    left_binding_power: u8,
                    right_binding_power: u8,
                },
                Builder(cst::NodeBuilder),
            }
            let mut elements = Vec::new();
            if let Some(error) = loop {
                loop {
                    let start_position = stream.position();
                    match match self.parse_unary_prefix_operator(stream) {
                        Pass { builder, .. } => Ok(Pratt::Operator {
                            builder,
                            kind: RuleKind::UnaryPrefixExpression,
                            left_binding_power: 255,
                            right_binding_power: 29u8,
                        }),
                        Fail { error } => Err(error),
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
                    Pass { builder, .. } => elements.push(Pratt::Builder(builder)),
                }
                loop {
                    let start_position = stream.position();
                    match loop {
                        let start_position = stream.position();
                        let mut furthest_error;
                        match match self.parse_conditional_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::ConditionalExpression,
                                left_binding_power: 3u8,
                                right_binding_power: 255,
                            }),
                            Fail { error } => Err(error),
                        } {
                            Err(error) => furthest_error = error,
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match self.parse_unary_postfix_operator(stream) {
                                Pass { builder, .. } => Ok(Pratt::Operator {
                                    builder,
                                    kind: RuleKind::UnaryPostfixExpression,
                                    left_binding_power: 27u8,
                                    right_binding_power: 255,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => furthest_error.merge_with(error),
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match self.parse_function_call_operator(stream) {
                                Pass { builder, .. } => Ok(Pratt::Operator {
                                    builder,
                                    kind: RuleKind::FunctionCallExpression,
                                    left_binding_power: 31u8,
                                    right_binding_power: 255,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => furthest_error.merge_with(error),
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match self.parse_member_access_operator(stream) {
                                Pass { builder, .. } => Ok(Pratt::Operator {
                                    builder,
                                    kind: RuleKind::MemberAccessExpression,
                                    left_binding_power: 33u8,
                                    right_binding_power: 255,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => furthest_error.merge_with(error),
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match self.parse_index_access_operator(stream) {
                                Pass { builder, .. } => Ok(Pratt::Operator {
                                    builder,
                                    kind: RuleKind::IndexAccessExpression,
                                    left_binding_power: 35u8,
                                    right_binding_power: 255,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => furthest_error.merge_with(error),
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
                    match match self.parse_assignment_operator(stream) {
                        Pass { builder, .. } => Ok(Pratt::Operator {
                            builder,
                            kind: RuleKind::AssignmentExpression,
                            left_binding_power: 1u8,
                            right_binding_power: 1u8 + 1,
                        }),
                        Fail { error } => Err(error),
                    } {
                        Err(error) => furthest_error = error,
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_or_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::OrExpression,
                                left_binding_power: 5u8,
                                right_binding_power: 5u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_and_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::AndExpression,
                                left_binding_power: 7u8,
                                right_binding_power: 7u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_equality_comparison_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::EqualityComparisonExpression,
                                left_binding_power: 9u8,
                                right_binding_power: 9u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_order_comparison_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::OrderComparisonExpression,
                                left_binding_power: 11u8,
                                right_binding_power: 11u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_bit_or_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::BitOrExpression,
                                left_binding_power: 13u8,
                                right_binding_power: 13u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_bit_x_or_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::BitXOrExpression,
                                left_binding_power: 15u8,
                                right_binding_power: 15u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_bit_and_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::BitAndExpression,
                                left_binding_power: 17u8,
                                right_binding_power: 17u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_shift_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::ShiftExpression,
                                left_binding_power: 19u8,
                                right_binding_power: 19u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_add_sub_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::AddSubExpression,
                                left_binding_power: 21u8,
                                right_binding_power: 21u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_mul_div_mod_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::MulDivModExpression,
                                left_binding_power: 23u8,
                                right_binding_power: 23u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_exponentiation_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::ExponentiationExpression,
                                left_binding_power: 25u8,
                                right_binding_power: 25u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
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
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                            ) = (left, op)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![left, op]).with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        } else if *left_binding_power == 255 {
                            let op = elements.remove(i);
                            let right = elements.remove(i);
                            if let (
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (op, right)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![op, right]).with_kind(kind);
                                elements.insert(i, Pratt::Builder(builder));
                                i = i.saturating_sub(1);
                            } else {
                                unreachable!()
                            }
                        } else {
                            let left = elements.remove(i - 1);
                            let op = elements.remove(i - 1);
                            let right = elements.remove(i - 1);
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (left, op, right)
                            {
                                let builder = cst::NodeBuilder::multiple(vec![left, op, right])
                                    .with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        }
                    } else {
                        i += 1;
                    }
                }
                if let Pratt::Builder(builder) = elements.pop().unwrap() {
                    Pass {
                        builder,
                        error: None,
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }

    // (* v0.6.0 *)
    // Expression = AssignmentExpression
    //            | ConditionalExpression
    //            | OrExpression
    //            | AndExpression
    //            | EqualityComparisonExpression
    //            | OrderComparisonExpression
    //            | BitOrExpression
    //            | BitXOrExpression
    //            | BitAndExpression
    //            | ShiftExpression
    //            | AddSubExpression
    //            | MulDivModExpression
    //            | ExponentiationExpression
    //            | UnaryPostfixExpression
    //            | UnaryPrefixExpression
    //            | FunctionCallExpression
    //            | MemberAccessExpression
    //            | IndexAccessExpression
    //            | PrimaryExpression;
    // AssignmentExpression = Expression AssignmentOperator Expression;
    // ConditionalExpression = Expression ConditionalOperator;
    // OrExpression = Expression OrOperator Expression;
    // AndExpression = Expression AndOperator Expression;
    // EqualityComparisonExpression = Expression EqualityComparisonOperator Expression;
    // OrderComparisonExpression = Expression OrderComparisonOperator Expression;
    // BitOrExpression = Expression BitOrOperator Expression;
    // BitXOrExpression = Expression BitXOrOperator Expression;
    // BitAndExpression = Expression BitAndOperator Expression;
    // ShiftExpression = Expression ShiftOperator Expression;
    // AddSubExpression = Expression AddSubOperator Expression;
    // MulDivModExpression = Expression MulDivModOperator Expression;
    // ExponentiationExpression = Expression ExponentiationOperator Expression; (* Right Associative *)
    // UnaryPostfixExpression = Expression UnaryPostfixOperator;
    // UnaryPrefixExpression = UnaryPrefixOperator Expression;
    // FunctionCallExpression = Expression FunctionCallOperator;
    // MemberAccessExpression = Expression MemberAccessOperator;
    // IndexAccessExpression = Expression IndexAccessOperator;

    #[allow(dead_code)]
    fn parse_expression_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            enum Pratt {
                Operator {
                    kind: RuleKind,
                    builder: cst::NodeBuilder,
                    left_binding_power: u8,
                    right_binding_power: u8,
                },
                Builder(cst::NodeBuilder),
            }
            let mut elements = Vec::new();
            if let Some(error) = loop {
                loop {
                    let start_position = stream.position();
                    match match self.parse_unary_prefix_operator(stream) {
                        Pass { builder, .. } => Ok(Pratt::Operator {
                            builder,
                            kind: RuleKind::UnaryPrefixExpression,
                            left_binding_power: 255,
                            right_binding_power: 29u8,
                        }),
                        Fail { error } => Err(error),
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
                    Pass { builder, .. } => elements.push(Pratt::Builder(builder)),
                }
                loop {
                    let start_position = stream.position();
                    match loop {
                        let start_position = stream.position();
                        let mut furthest_error;
                        match match self.parse_conditional_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::ConditionalExpression,
                                left_binding_power: 3u8,
                                right_binding_power: 255,
                            }),
                            Fail { error } => Err(error),
                        } {
                            Err(error) => furthest_error = error,
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match self.parse_unary_postfix_operator(stream) {
                                Pass { builder, .. } => Ok(Pratt::Operator {
                                    builder,
                                    kind: RuleKind::UnaryPostfixExpression,
                                    left_binding_power: 27u8,
                                    right_binding_power: 255,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => furthest_error.merge_with(error),
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match self.parse_function_call_operator(stream) {
                                Pass { builder, .. } => Ok(Pratt::Operator {
                                    builder,
                                    kind: RuleKind::FunctionCallExpression,
                                    left_binding_power: 31u8,
                                    right_binding_power: 255,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => furthest_error.merge_with(error),
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match self.parse_member_access_operator(stream) {
                                Pass { builder, .. } => Ok(Pratt::Operator {
                                    builder,
                                    kind: RuleKind::MemberAccessExpression,
                                    left_binding_power: 33u8,
                                    right_binding_power: 255,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => furthest_error.merge_with(error),
                            ok => break ok,
                        }
                        stream.set_position(start_position);
                        match {
                            match self.parse_index_access_operator(stream) {
                                Pass { builder, .. } => Ok(Pratt::Operator {
                                    builder,
                                    kind: RuleKind::IndexAccessExpression,
                                    left_binding_power: 35u8,
                                    right_binding_power: 255,
                                }),
                                Fail { error } => Err(error),
                            }
                        } {
                            Err(error) => furthest_error.merge_with(error),
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
                    match match self.parse_assignment_operator(stream) {
                        Pass { builder, .. } => Ok(Pratt::Operator {
                            builder,
                            kind: RuleKind::AssignmentExpression,
                            left_binding_power: 1u8,
                            right_binding_power: 1u8 + 1,
                        }),
                        Fail { error } => Err(error),
                    } {
                        Err(error) => furthest_error = error,
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_or_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::OrExpression,
                                left_binding_power: 5u8,
                                right_binding_power: 5u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_and_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::AndExpression,
                                left_binding_power: 7u8,
                                right_binding_power: 7u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_equality_comparison_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::EqualityComparisonExpression,
                                left_binding_power: 9u8,
                                right_binding_power: 9u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_order_comparison_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::OrderComparisonExpression,
                                left_binding_power: 11u8,
                                right_binding_power: 11u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_bit_or_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::BitOrExpression,
                                left_binding_power: 13u8,
                                right_binding_power: 13u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_bit_x_or_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::BitXOrExpression,
                                left_binding_power: 15u8,
                                right_binding_power: 15u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_bit_and_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::BitAndExpression,
                                left_binding_power: 17u8,
                                right_binding_power: 17u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_shift_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::ShiftExpression,
                                left_binding_power: 19u8,
                                right_binding_power: 19u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_add_sub_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::AddSubExpression,
                                left_binding_power: 21u8,
                                right_binding_power: 21u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_mul_div_mod_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::MulDivModExpression,
                                left_binding_power: 23u8,
                                right_binding_power: 23u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_exponentiation_operator(stream) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::ExponentiationExpression,
                                left_binding_power: 25u8 + 1,
                                right_binding_power: 25u8,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
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
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                            ) = (left, op)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![left, op]).with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        } else if *left_binding_power == 255 {
                            let op = elements.remove(i);
                            let right = elements.remove(i);
                            if let (
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (op, right)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![op, right]).with_kind(kind);
                                elements.insert(i, Pratt::Builder(builder));
                                i = i.saturating_sub(1);
                            } else {
                                unreachable!()
                            }
                        } else {
                            let left = elements.remove(i - 1);
                            let op = elements.remove(i - 1);
                            let right = elements.remove(i - 1);
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (left, op, right)
                            {
                                let builder = cst::NodeBuilder::multiple(vec![left, op, right])
                                    .with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        }
                    } else {
                        i += 1;
                    }
                }
                if let Pratt::Builder(builder) = elements.pop().unwrap() {
                    Pass {
                        builder,
                        error: None,
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn dispatch_parse_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.parse_expression_0_6_0(stream)
        } else {
            self.parse_expression_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_expression(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_expression(stream)
            .with_kind(RuleKind::Expression)
    }

    // ExpressionStatement = Expression «Semicolon»;

    #[allow(dead_code)]
    fn parse_expression_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_expression(stream) {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_expression_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_expression_statement_0_4_11(stream)
            .with_kind(RuleKind::ExpressionStatement)
    }

    // (* v0.6.0 *)
    // FallbackFunctionAttribute = ModifierInvocation
    //                           | OverrideSpecifier
    //                           | «ExternalKeyword»
    //                           | «PayableKeyword»
    //                           | «PureKeyword»
    //                           | «ViewKeyword»
    //                           | «VirtualKeyword»;

    #[allow(dead_code)]
    fn parse_fallback_function_attribute_0_6_0(&self, stream: &mut Stream) -> ParserResult {
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
            match self.parse_token_with_trivia(
                stream,
                Self::scan_external_keyword,
                TokenKind::ExternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_payable_keyword,
                TokenKind::PayableKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_pure_keyword,
                TokenKind::PureKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_view_keyword,
                TokenKind::ViewKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_virtual_keyword,
                TokenKind::VirtualKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_fallback_function_attribute(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.parse_fallback_function_attribute_0_6_0(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_fallback_function_attribute(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_fallback_function_attribute(stream)
            .map(|body| body.with_kind(RuleKind::FallbackFunctionAttribute))
    }

    #[inline]
    pub(crate) fn parse_fallback_function_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_fallback_function_attribute(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.6.0 *)
    // FallbackFunctionDefinition = «FallbackKeyword» ParameterList FallbackFunctionAttribute* («ReturnsKeyword» ParameterList)? («Semicolon» | Block);

    #[allow(dead_code)]
    fn parse_fallback_function_definition_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_fallback_keyword,
                TokenKind::FallbackKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_parameter_list(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                                builder: if result.is_empty() {
                                    cst::NodeBuilder::empty(start_position)
                                } else {
                                    cst::NodeBuilder::multiple(result)
                                },
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    let result_0 = match self.parse_token_with_trivia(
                        stream,
                        Self::scan_returns_keyword,
                        TokenKind::ReturnsKeyword,
                    ) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_1 = match self.parse_parameter_list(stream) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    break Pass {
                        builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                        error: furthest_error,
                    };
                } {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_semicolon,
                    TokenKind::Semicolon,
                ) {
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
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![
                    result_0, result_1, result_2, result_3, result_4,
                ]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_fallback_function_definition(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.parse_fallback_function_definition_0_6_0(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_fallback_function_definition(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_fallback_function_definition(stream)
            .map(|body| body.with_kind(RuleKind::FallbackFunctionDefinition))
    }

    #[inline]
    pub(crate) fn parse_fallback_function_definition(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_fallback_function_definition(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // ForStatement = «ForKeyword» «OpenParen» (SimpleStatement | «Semicolon») (ExpressionStatement | «Semicolon») Expression? «CloseParen» Statement;

    #[allow(dead_code)]
    fn parse_for_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_for_keyword,
                TokenKind::ForKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_paren,
                    TokenKind::OpenParen,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
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
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_semicolon,
                                    TokenKind::Semicolon,
                                ) {
                                    Fail { error } => furthest_error.merge_with(error),
                                    pass => break pass,
                                }
                                break Fail {
                                    error: furthest_error,
                                };
                            } {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
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
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_semicolon,
                                    TokenKind::Semicolon,
                                ) {
                                    Fail { error } => furthest_error.merge_with(error),
                                    pass => break pass,
                                }
                                break Fail {
                                    error: furthest_error,
                                };
                            } {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
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
                                            builder: cst::NodeBuilder::empty(start_position),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            break Pass {
                                builder: cst::NodeBuilder::multiple(vec![
                                    result_0, result_1, result_2,
                                ]),
                                error: furthest_error,
                            };
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_paren,
                                    TokenKind::CloseParen,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_statement(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_for_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_for_statement_0_4_11(stream)
            .with_kind(RuleKind::ForStatement)
    }

    // (* v0.4.11 *)
    // FunctionAttribute = «ConstantKeyword»
    //                   | «ExternalKeyword»
    //                   | «InternalKeyword»
    //                   | ModifierInvocation
    //                   | OverrideSpecifier
    //                   | «PayableKeyword»
    //                   | «PrivateKeyword»
    //                   | «PublicKeyword»
    //                   | «PureKeyword»
    //                   | «ViewKeyword»;

    #[allow(dead_code)]
    fn parse_function_attribute_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_constant_keyword,
                TokenKind::ConstantKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_external_keyword,
                TokenKind::ExternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_internal_keyword,
                TokenKind::InternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_modifier_invocation(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_override_specifier(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_payable_keyword,
                TokenKind::PayableKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_private_keyword,
                TokenKind::PrivateKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_public_keyword,
                TokenKind::PublicKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_pure_keyword,
                TokenKind::PureKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_view_keyword,
                TokenKind::ViewKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.5.0 *)
    // FunctionAttribute = «ExternalKeyword»
    //                   | «InternalKeyword»
    //                   | ModifierInvocation
    //                   | OverrideSpecifier
    //                   | «PayableKeyword»
    //                   | «PrivateKeyword»
    //                   | «PublicKeyword»
    //                   | «PureKeyword»
    //                   | «ViewKeyword»;

    #[allow(dead_code)]
    fn parse_function_attribute_0_5_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_external_keyword,
                TokenKind::ExternalKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_internal_keyword,
                TokenKind::InternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_modifier_invocation(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_override_specifier(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_payable_keyword,
                TokenKind::PayableKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_private_keyword,
                TokenKind::PrivateKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_public_keyword,
                TokenKind::PublicKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_pure_keyword,
                TokenKind::PureKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_view_keyword,
                TokenKind::ViewKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.6.0 *)
    // FunctionAttribute = «ExternalKeyword»
    //                   | «InternalKeyword»
    //                   | ModifierInvocation
    //                   | OverrideSpecifier
    //                   | «PayableKeyword»
    //                   | «PrivateKeyword»
    //                   | «PublicKeyword»
    //                   | «PureKeyword»
    //                   | «ViewKeyword»
    //                   | «VirtualKeyword»;

    #[allow(dead_code)]
    fn parse_function_attribute_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_external_keyword,
                TokenKind::ExternalKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_internal_keyword,
                TokenKind::InternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_modifier_invocation(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_override_specifier(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_payable_keyword,
                TokenKind::PayableKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_private_keyword,
                TokenKind::PrivateKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_public_keyword,
                TokenKind::PublicKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_pure_keyword,
                TokenKind::PureKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_view_keyword,
                TokenKind::ViewKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_virtual_keyword,
                TokenKind::VirtualKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_function_attribute(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.parse_function_attribute_0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.parse_function_attribute_0_5_0(stream)
        } else {
            self.parse_function_attribute_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_function_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_function_attribute(stream)
            .with_kind(RuleKind::FunctionAttribute)
    }

    // (* v0.4.11 *)
    // FunctionCallOperator = ArgumentList;

    #[allow(dead_code)]
    fn parse_function_call_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_argument_list(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0]),
                error: furthest_error,
            };
        }
    }

    // (* v0.6.2 *)
    // FunctionCallOperator = FunctionCallOptions* ArgumentList;

    #[allow(dead_code)]
    fn parse_function_call_operator_0_6_2(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match {
                let mut result = Vec::new();
                loop {
                    let start_position = stream.position();
                    match self.parse_function_call_options(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            break Pass {
                                builder: if result.is_empty() {
                                    cst::NodeBuilder::empty(start_position)
                                } else {
                                    cst::NodeBuilder::multiple(result)
                                },
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_argument_list(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    // (* v0.8.0 *)
    // FunctionCallOperator = FunctionCallOptions? ArgumentList;

    #[allow(dead_code)]
    fn parse_function_call_operator_0_8_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match {
                let start_position = stream.position();
                match self.parse_function_call_options(stream) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_argument_list(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_function_call_operator(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            self.parse_function_call_operator_0_8_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_2 {
            self.parse_function_call_operator_0_6_2(stream)
        } else {
            self.parse_function_call_operator_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_function_call_operator(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_function_call_operator(stream)
            .with_kind(RuleKind::FunctionCallOperator)
    }

    // (* v0.6.2 *)
    // FunctionCallOptions = «OpenBrace» (NamedArgument («Comma» NamedArgument)*)? «CloseBrace»;

    #[allow(dead_code)]
    fn parse_function_call_options_0_6_2(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(stream, Self::scan_open_brace, TokenKind::OpenBrace)
            {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
                } => {
                    match {
                        let start_position = stream.position();
                        match {
                            let mut result = Vec::new();
                            loop {
                                match self.parse_named_argument(stream) {
                                    err @ Fail { .. } => break err,
                                    Pass { builder, .. } => {
                                        result.push(builder);
                                        let save = stream.position();
                                        match self.parse_token_with_trivia(
                                            stream,
                                            Self::scan_comma,
                                            TokenKind::Comma,
                                        ) {
                                            Fail { error } => {
                                                stream.set_position(save);
                                                break Pass {
                                                    builder: cst::NodeBuilder::multiple(result),
                                                    error: Some(error),
                                                };
                                            }
                                            Pass { builder, .. } => result.push(builder),
                                        }
                                    }
                                }
                            }
                        } {
                            Fail { error } => {
                                stream.set_position(start_position);
                                Pass {
                                    builder: cst::NodeBuilder::empty(start_position),
                                    error: Some(error),
                                }
                            }
                            pass => pass,
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_brace,
                                TokenKind::CloseBrace,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    fn dispatch_parse_function_call_options(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_2 {
            Some(self.parse_function_call_options_0_6_2(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_function_call_options(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_function_call_options(stream)
            .map(|body| body.with_kind(RuleKind::FunctionCallOptions))
    }

    #[inline]
    pub(crate) fn parse_function_call_options(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_function_call_options(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // FunctionDefinition = «FunctionKeyword» («Identifier» | «FallbackKeyword» | «ReceiveKeyword») ParameterList FunctionAttribute* («ReturnsKeyword» ParameterList)? («Semicolon» | Block);

    #[allow(dead_code)]
    fn parse_function_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_function_keyword,
                TokenKind::FunctionKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_fallback_keyword,
                    TokenKind::FallbackKeyword,
                ) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_receive_keyword,
                    TokenKind::ReceiveKeyword,
                ) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_parameter_list(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                                builder: if result.is_empty() {
                                    cst::NodeBuilder::empty(start_position)
                                } else {
                                    cst::NodeBuilder::multiple(result)
                                },
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    let result_0 = match self.parse_token_with_trivia(
                        stream,
                        Self::scan_returns_keyword,
                        TokenKind::ReturnsKeyword,
                    ) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_1 = match self.parse_parameter_list(stream) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    break Pass {
                        builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                        error: furthest_error,
                    };
                } {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_semicolon,
                    TokenKind::Semicolon,
                ) {
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
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![
                    result_0, result_1, result_2, result_3, result_4, result_5,
                ]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_function_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_function_definition_0_4_11(stream)
            .with_kind(RuleKind::FunctionDefinition)
    }

    // FunctionType = «FunctionKeyword» ParameterList («InternalKeyword» | «ExternalKeyword» | «PrivateKeyword» | «PublicKeyword» | «PureKeyword» | «ViewKeyword» | «PayableKeyword»)* («ReturnsKeyword» ParameterList)?;

    #[allow(dead_code)]
    fn parse_function_type_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_function_keyword,
                TokenKind::FunctionKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_parameter_list(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_internal_keyword,
                            TokenKind::InternalKeyword,
                        ) {
                            Fail { error } => furthest_error = error,
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_external_keyword,
                            TokenKind::ExternalKeyword,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_private_keyword,
                            TokenKind::PrivateKeyword,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_public_keyword,
                            TokenKind::PublicKeyword,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_pure_keyword,
                            TokenKind::PureKeyword,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_view_keyword,
                            TokenKind::ViewKeyword,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_payable_keyword,
                            TokenKind::PayableKeyword,
                        ) {
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
                                builder: if result.is_empty() {
                                    cst::NodeBuilder::empty(start_position)
                                } else {
                                    cst::NodeBuilder::multiple(result)
                                },
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    let result_0 = match self.parse_token_with_trivia(
                        stream,
                        Self::scan_returns_keyword,
                        TokenKind::ReturnsKeyword,
                    ) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_1 = match self.parse_parameter_list(stream) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    break Pass {
                        builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                        error: furthest_error,
                    };
                } {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_function_type(&self, stream: &mut Stream) -> ParserResult {
        self.parse_function_type_0_4_11(stream)
            .with_kind(RuleKind::FunctionType)
    }

    // IdentifierPath = «Identifier» («Period» «Identifier»)*;

    #[allow(dead_code)]
    fn parse_identifier_path_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut result = Vec::new();
            loop {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    err @ Fail { .. } => break err,
                    Pass { builder, .. } => {
                        result.push(builder);
                        let save = stream.position();
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_period,
                            TokenKind::Period,
                        ) {
                            Fail { error } => {
                                stream.set_position(save);
                                break Pass {
                                    builder: cst::NodeBuilder::multiple(result),
                                    error: Some(error),
                                };
                            }
                            Pass { builder, .. } => result.push(builder),
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_identifier_path(&self, stream: &mut Stream) -> ParserResult {
        self.parse_identifier_path_0_4_11(stream)
            .with_kind(RuleKind::IdentifierPath)
    }

    // IfStatement = «IfKeyword» «OpenParen» Expression «CloseParen» Statement («ElseKeyword» Statement)?;

    #[allow(dead_code)]
    fn parse_if_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_if_keyword,
                TokenKind::IfKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_paren,
                    TokenKind::OpenParen,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => match self.parse_expression(stream) {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_paren,
                                TokenKind::CloseParen,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    },
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_statement(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    let result_0 = match self.parse_token_with_trivia(
                        stream,
                        Self::scan_else_keyword,
                        TokenKind::ElseKeyword,
                    ) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_1 = match self.parse_statement(stream) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    break Pass {
                        builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                        error: furthest_error,
                    };
                } {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_if_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_if_statement_0_4_11(stream)
            .with_kind(RuleKind::IfStatement)
    }

    // ImportAlias = «AsKeyword» «Identifier»;

    #[allow(dead_code)]
    fn parse_import_alias_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_as_keyword,
                TokenKind::AsKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_import_alias(&self, stream: &mut Stream) -> ParserResult {
        self.parse_import_alias_0_4_11(stream)
            .with_kind(RuleKind::ImportAlias)
    }

    // ImportDirective = «ImportKeyword» (SimpleImport | AsteriskImport | SelectiveImport) «Semicolon»;

    #[allow(dead_code)]
    fn parse_import_directive_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_import_keyword,
                    TokenKind::ImportKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                    match self.parse_simple_import(stream) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_asterisk_import(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_selective_import(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_import_directive(&self, stream: &mut Stream) -> ParserResult {
        self.parse_import_directive_0_4_11(stream)
            .with_kind(RuleKind::ImportDirective)
    }

    // ImportPath = «AsciiStringLiteral»;

    #[allow(dead_code)]
    fn parse_import_path_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            Self::scan_ascii_string_literal,
            TokenKind::AsciiStringLiteral,
        )
    }

    #[inline]
    pub(crate) fn parse_import_path(&self, stream: &mut Stream) -> ParserResult {
        self.parse_import_path_0_4_11(stream)
            .with_kind(RuleKind::ImportPath)
    }

    // IndexAccessOperator = «OpenBracket» ((Expression («Colon» Expression?)?) | («Colon» Expression?)) «CloseBracket»;

    #[allow(dead_code)]
    fn parse_index_access_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(
                stream,
                Self::scan_open_bracket,
                TokenKind::OpenBracket,
            ) {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
                } => {
                    match loop {
                        let start_position = stream.position();
                        let mut furthest_error;
                        match loop {
                            let mut furthest_error = None;
                            let result_0 = match self.parse_expression(stream) {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
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
                                    let result_0 = match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_colon,
                                        TokenKind::Colon,
                                    ) {
                                        Pass { builder, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            builder
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
                                                    builder: cst::NodeBuilder::empty(
                                                        start_position,
                                                    ),
                                                    error: Some(error),
                                                }
                                            }
                                            pass => pass,
                                        }
                                    } {
                                        Pass { builder, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            builder
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            result_0, result_1,
                                        ]),
                                        error: furthest_error,
                                    };
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        Pass {
                                            builder: cst::NodeBuilder::empty(start_position),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            break Pass {
                                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                                error: furthest_error,
                            };
                        } {
                            Fail { error } => furthest_error = error,
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match loop {
                            let mut furthest_error = None;
                            let result_0 = match self.parse_token_with_trivia(
                                stream,
                                Self::scan_colon,
                                TokenKind::Colon,
                            ) {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
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
                                            builder: cst::NodeBuilder::empty(start_position),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            break Pass {
                                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
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
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_bracket,
                                TokenKind::CloseBracket,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_index_access_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_index_access_operator_0_4_11(stream)
            .with_kind(RuleKind::IndexAccessOperator)
    }

    // InheritanceSpecifier = IdentifierPath ArgumentList?;

    #[allow(dead_code)]
    fn parse_inheritance_specifier_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_identifier_path(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_inheritance_specifier(&self, stream: &mut Stream) -> ParserResult {
        self.parse_inheritance_specifier_0_4_11(stream)
            .with_kind(RuleKind::InheritanceSpecifier)
    }

    // InheritanceSpecifierList = «IsKeyword» InheritanceSpecifier («Comma» InheritanceSpecifier)*;

    #[allow(dead_code)]
    fn parse_inheritance_specifier_list_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_is_keyword,
                TokenKind::IsKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                        Pass { builder, .. } => {
                            result.push(builder);
                            let save = stream.position();
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_comma,
                                TokenKind::Comma,
                            ) {
                                Fail { error } => {
                                    stream.set_position(save);
                                    break Pass {
                                        builder: cst::NodeBuilder::multiple(result),
                                        error: Some(error),
                                    };
                                }
                                Pass { builder, .. } => result.push(builder),
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_inheritance_specifier_list(&self, stream: &mut Stream) -> ParserResult {
        self.parse_inheritance_specifier_list_0_4_11(stream)
            .with_kind(RuleKind::InheritanceSpecifierList)
    }

    // InterfaceDefinition = «InterfaceKeyword» «Identifier» InheritanceSpecifierList? «OpenBrace» ContractBodyElement* «CloseBrace»;

    #[allow(dead_code)]
    fn parse_interface_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_interface_keyword,
                TokenKind::InterfaceKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_3 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_brace,
                    TokenKind::OpenBrace,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                let start_position = stream.position();
                                match self.parse_contract_body_element(stream) {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        break Pass {
                                            builder: if result.is_empty() {
                                                cst::NodeBuilder::empty(start_position)
                                            } else {
                                                cst::NodeBuilder::multiple(result)
                                            },
                                            error: Some(error),
                                        };
                                    }
                                    Pass { builder, .. } => result.push(builder),
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_brace,
                                    TokenKind::CloseBrace,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_interface_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_interface_definition_0_4_11(stream)
            .with_kind(RuleKind::InterfaceDefinition)
    }

    // LeadingTrivia = («Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment»)+;

    #[allow(dead_code)]
    fn parse_leading_trivia_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut result = Vec::new();
            loop {
                let start_position = stream.position();
                match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_token(stream, Self::scan_whitespace, TokenKind::Whitespace) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_token(stream, Self::scan_end_of_line, TokenKind::EndOfLine) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_token(
                        stream,
                        Self::scan_multiline_comment,
                        TokenKind::MultilineComment,
                    ) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_token(
                        stream,
                        Self::scan_single_line_comment,
                        TokenKind::SingleLineComment,
                    ) {
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
                            builder: cst::NodeBuilder::multiple(result),
                            error: Some(error),
                        };
                    }
                    Pass { builder, .. } => result.push(builder),
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_leading_trivia(&self, stream: &mut Stream) -> ParserResult {
        self.parse_leading_trivia_0_4_11(stream)
            .with_kind(RuleKind::LeadingTrivia)
    }

    // LibraryDefinition = «LibraryKeyword» «Identifier» «OpenBrace» ContractBodyElement* «CloseBrace»;

    #[allow(dead_code)]
    fn parse_library_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_library_keyword,
                TokenKind::LibraryKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_brace,
                    TokenKind::OpenBrace,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                let start_position = stream.position();
                                match self.parse_contract_body_element(stream) {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        break Pass {
                                            builder: if result.is_empty() {
                                                cst::NodeBuilder::empty(start_position)
                                            } else {
                                                cst::NodeBuilder::multiple(result)
                                            },
                                            error: Some(error),
                                        };
                                    }
                                    Pass { builder, .. } => result.push(builder),
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_brace,
                                    TokenKind::CloseBrace,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_library_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_library_definition_0_4_11(stream)
            .with_kind(RuleKind::LibraryDefinition)
    }

    // (* v0.4.11 *)
    // MappingKeyType = (ElementaryType | IdentifierPath);

    #[allow(dead_code)]
    fn parse_mapping_key_type_0_4_11(&self, stream: &mut Stream) -> ParserResult {
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
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0]),
                error: furthest_error,
            };
        }
    }

    // (* v0.8.18 *)
    // MappingKeyType = (ElementaryType | IdentifierPath) «Identifier»?;

    #[allow(dead_code)]
    fn parse_mapping_key_type_0_8_18(&self, stream: &mut Stream) -> ParserResult {
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
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                let start_position = stream.position();
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_mapping_key_type(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_18 {
            self.parse_mapping_key_type_0_8_18(stream)
        } else {
            self.parse_mapping_key_type_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_mapping_key_type(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_mapping_key_type(stream)
            .with_kind(RuleKind::MappingKeyType)
    }

    // MappingType = «MappingKeyword» «OpenParen» MappingKeyType «EqualGreaterThan» MappingValueType «CloseParen»;

    #[allow(dead_code)]
    fn parse_mapping_type_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_mapping_keyword,
                TokenKind::MappingKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_paren,
                    TokenKind::OpenParen,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => {
                        match loop {
                            let mut furthest_error = None;
                            let result_0 = match self.parse_mapping_key_type(stream) {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            let result_1 = match self.parse_token_with_trivia(
                                stream,
                                Self::scan_equal_greater_than,
                                TokenKind::EqualGreaterThan,
                            ) {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            let result_2 = match self.parse_mapping_value_type(stream) {
                                Pass { builder, error } => {
                                    furthest_error =
                                        error.map(|error| error.maybe_merge_with(furthest_error));
                                    builder
                                }
                                Fail { error } => {
                                    break Fail {
                                        error: error.maybe_merge_with(furthest_error),
                                    }
                                }
                            };
                            break Pass {
                                builder: cst::NodeBuilder::multiple(vec![
                                    result_0, result_1, result_2,
                                ]),
                                error: furthest_error,
                            };
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_paren,
                                    TokenKind::CloseParen,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_mapping_type(&self, stream: &mut Stream) -> ParserResult {
        self.parse_mapping_type_0_4_11(stream)
            .with_kind(RuleKind::MappingType)
    }

    // (* v0.4.11 *)
    // MappingValueType = TypeName;

    #[allow(dead_code)]
    fn parse_mapping_value_type_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_type_name(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0]),
                error: furthest_error,
            };
        }
    }

    // (* v0.8.18 *)
    // MappingValueType = TypeName «Identifier»?;

    #[allow(dead_code)]
    fn parse_mapping_value_type_0_8_18(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_type_name(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                let start_position = stream.position();
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_mapping_value_type(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_18 {
            self.parse_mapping_value_type_0_8_18(stream)
        } else {
            self.parse_mapping_value_type_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_mapping_value_type(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_mapping_value_type(stream)
            .with_kind(RuleKind::MappingValueType)
    }

    // MemberAccessOperator = «Period» («Identifier» | «AddressKeyword»);

    #[allow(dead_code)]
    fn parse_member_access_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 =
                match self.parse_token_with_trivia(stream, Self::scan_period, TokenKind::Period) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_address_keyword,
                    TokenKind::AddressKeyword,
                ) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_member_access_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_member_access_operator_0_4_11(stream)
            .with_kind(RuleKind::MemberAccessOperator)
    }

    // (* v0.4.11 *)
    // ModifierAttribute = OverrideSpecifier;

    #[allow(dead_code)]
    fn parse_modifier_attribute_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_override_specifier(stream)
    }

    // (* v0.6.0 *)
    // ModifierAttribute = OverrideSpecifier | «VirtualKeyword»;

    #[allow(dead_code)]
    fn parse_modifier_attribute_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_override_specifier(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_virtual_keyword,
                TokenKind::VirtualKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_modifier_attribute(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.parse_modifier_attribute_0_6_0(stream)
        } else {
            self.parse_modifier_attribute_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_modifier_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_modifier_attribute(stream)
            .with_kind(RuleKind::ModifierAttribute)
    }

    // ModifierDefinition = «ModifierKeyword» «Identifier» ParameterList? ModifierAttribute* («Semicolon» | Block);

    #[allow(dead_code)]
    fn parse_modifier_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_modifier_keyword,
                TokenKind::ModifierKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                                builder: if result.is_empty() {
                                    cst::NodeBuilder::empty(start_position)
                                } else {
                                    cst::NodeBuilder::multiple(result)
                                },
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_semicolon,
                    TokenKind::Semicolon,
                ) {
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
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![
                    result_0, result_1, result_2, result_3, result_4,
                ]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_modifier_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_modifier_definition_0_4_11(stream)
            .with_kind(RuleKind::ModifierDefinition)
    }

    // ModifierInvocation = IdentifierPath ArgumentList?;

    #[allow(dead_code)]
    fn parse_modifier_invocation_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_identifier_path(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_modifier_invocation(&self, stream: &mut Stream) -> ParserResult {
        self.parse_modifier_invocation_0_4_11(stream)
            .with_kind(RuleKind::ModifierInvocation)
    }

    // MulDivModOperator = «Asterisk» | «Slash» | «Percent»;

    #[allow(dead_code)]
    fn parse_mul_div_mod_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(stream, Self::scan_asterisk, TokenKind::Asterisk) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_slash, TokenKind::Slash) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_percent, TokenKind::Percent) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_mul_div_mod_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_mul_div_mod_operator_0_4_11(stream)
            .with_kind(RuleKind::MulDivModOperator)
    }

    // NamedArgument = «Identifier» «Colon» Expression;

    #[allow(dead_code)]
    fn parse_named_argument_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 =
                match self.parse_token_with_trivia(stream, Self::scan_colon, TokenKind::Colon) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
            let result_2 = match self.parse_expression(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_named_argument(&self, stream: &mut Stream) -> ParserResult {
        self.parse_named_argument_0_4_11(stream)
            .with_kind(RuleKind::NamedArgument)
    }

    // NamedArgumentList = «OpenBrace» (NamedArgument («Comma» NamedArgument)*)? «CloseBrace»;

    #[allow(dead_code)]
    fn parse_named_argument_list_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(stream, Self::scan_open_brace, TokenKind::OpenBrace)
            {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
                } => {
                    match {
                        let start_position = stream.position();
                        match {
                            let mut result = Vec::new();
                            loop {
                                match self.parse_named_argument(stream) {
                                    err @ Fail { .. } => break err,
                                    Pass { builder, .. } => {
                                        result.push(builder);
                                        let save = stream.position();
                                        match self.parse_token_with_trivia(
                                            stream,
                                            Self::scan_comma,
                                            TokenKind::Comma,
                                        ) {
                                            Fail { error } => {
                                                stream.set_position(save);
                                                break Pass {
                                                    builder: cst::NodeBuilder::multiple(result),
                                                    error: Some(error),
                                                };
                                            }
                                            Pass { builder, .. } => result.push(builder),
                                        }
                                    }
                                }
                            }
                        } {
                            Fail { error } => {
                                stream.set_position(start_position);
                                Pass {
                                    builder: cst::NodeBuilder::empty(start_position),
                                    error: Some(error),
                                }
                            }
                            pass => pass,
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_brace,
                                TokenKind::CloseBrace,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_named_argument_list(&self, stream: &mut Stream) -> ParserResult {
        self.parse_named_argument_list_0_4_11(stream)
            .with_kind(RuleKind::NamedArgumentList)
    }

    // NewExpression = «NewKeyword» TypeName;

    #[allow(dead_code)]
    fn parse_new_expression_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_new_keyword,
                TokenKind::NewKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_type_name(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_new_expression(&self, stream: &mut Stream) -> ParserResult {
        self.parse_new_expression_0_4_11(stream)
            .with_kind(RuleKind::NewExpression)
    }

    // (* v0.4.11 *)
    // NumberUnit = «DaysKeyword»
    //            | «EtherKeyword»
    //            | «FinneyKeyword»
    //            | «HoursKeyword»
    //            | «MinutesKeyword»
    //            | «SecondsKeyword»
    //            | «SzaboKeyword»
    //            | «WeeksKeyword»
    //            | «WeiKeyword»
    //            | «YearsKeyword»;

    #[allow(dead_code)]
    fn parse_number_unit_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_days_keyword,
                TokenKind::DaysKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_ether_keyword,
                TokenKind::EtherKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_finney_keyword,
                TokenKind::FinneyKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_hours_keyword,
                TokenKind::HoursKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_minutes_keyword,
                TokenKind::MinutesKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_seconds_keyword,
                TokenKind::SecondsKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_szabo_keyword,
                TokenKind::SzaboKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_weeks_keyword,
                TokenKind::WeeksKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_wei_keyword,
                TokenKind::WeiKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_years_keyword,
                TokenKind::YearsKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.5.0 *)
    // NumberUnit = «DaysKeyword»
    //            | «EtherKeyword»
    //            | «FinneyKeyword»
    //            | «HoursKeyword»
    //            | «MinutesKeyword»
    //            | «SecondsKeyword»
    //            | «SzaboKeyword»
    //            | «WeeksKeyword»
    //            | «WeiKeyword»;

    #[allow(dead_code)]
    fn parse_number_unit_0_5_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_days_keyword,
                TokenKind::DaysKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_ether_keyword,
                TokenKind::EtherKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_finney_keyword,
                TokenKind::FinneyKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_hours_keyword,
                TokenKind::HoursKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_minutes_keyword,
                TokenKind::MinutesKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_seconds_keyword,
                TokenKind::SecondsKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_szabo_keyword,
                TokenKind::SzaboKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_weeks_keyword,
                TokenKind::WeeksKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_wei_keyword,
                TokenKind::WeiKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.6.11 *)
    // NumberUnit = «DaysKeyword»
    //            | «EtherKeyword»
    //            | «FinneyKeyword»
    //            | «GweiKeyword»
    //            | «HoursKeyword»
    //            | «MinutesKeyword»
    //            | «SecondsKeyword»
    //            | «SzaboKeyword»
    //            | «WeeksKeyword»
    //            | «WeiKeyword»;

    #[allow(dead_code)]
    fn parse_number_unit_0_6_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_days_keyword,
                TokenKind::DaysKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_ether_keyword,
                TokenKind::EtherKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_finney_keyword,
                TokenKind::FinneyKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_gwei_keyword,
                TokenKind::GweiKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_hours_keyword,
                TokenKind::HoursKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_minutes_keyword,
                TokenKind::MinutesKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_seconds_keyword,
                TokenKind::SecondsKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_szabo_keyword,
                TokenKind::SzaboKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_weeks_keyword,
                TokenKind::WeeksKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_wei_keyword,
                TokenKind::WeiKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.7.0 *)
    // NumberUnit = «DaysKeyword»
    //            | «EtherKeyword»
    //            | «GweiKeyword»
    //            | «HoursKeyword»
    //            | «MinutesKeyword»
    //            | «SecondsKeyword»
    //            | «WeeksKeyword»
    //            | «WeiKeyword»;

    #[allow(dead_code)]
    fn parse_number_unit_0_7_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_days_keyword,
                TokenKind::DaysKeyword,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_ether_keyword,
                TokenKind::EtherKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_gwei_keyword,
                TokenKind::GweiKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_hours_keyword,
                TokenKind::HoursKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_minutes_keyword,
                TokenKind::MinutesKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_seconds_keyword,
                TokenKind::SecondsKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_weeks_keyword,
                TokenKind::WeeksKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_wei_keyword,
                TokenKind::WeiKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_number_unit(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            self.parse_number_unit_0_7_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_11 {
            self.parse_number_unit_0_6_11(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.parse_number_unit_0_5_0(stream)
        } else {
            self.parse_number_unit_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_number_unit(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_number_unit(stream)
            .with_kind(RuleKind::NumberUnit)
    }

    // (* v0.4.11 *)
    // NumericExpression = («HexLiteral» | «DecimalLiteral») NumberUnit?;

    #[allow(dead_code)]
    fn parse_numeric_expression_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match loop {
                let start_position = stream.position();
                let mut furthest_error;
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_hex_literal,
                    TokenKind::HexLiteral,
                ) {
                    Fail { error } => furthest_error = error,
                    pass => break pass,
                }
                stream.set_position(start_position);
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_decimal_literal,
                    TokenKind::DecimalLiteral,
                ) {
                    Fail { error } => furthest_error.merge_with(error),
                    pass => break pass,
                }
                break Fail {
                    error: furthest_error,
                };
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    // (* v0.5.0 *)
    // NumericExpression = «HexLiteral» | («DecimalLiteral» NumberUnit?);

    #[allow(dead_code)]
    fn parse_numeric_expression_0_5_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_hex_literal,
                TokenKind::HexLiteral,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_decimal_literal,
                    TokenKind::DecimalLiteral,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                    error: furthest_error,
                };
            } {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_numeric_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.parse_numeric_expression_0_5_0(stream)
        } else {
            self.parse_numeric_expression_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_numeric_expression(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_numeric_expression(stream)
            .with_kind(RuleKind::NumericExpression)
    }

    // OrOperator = «BarBar»;

    #[allow(dead_code)]
    fn parse_or_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, Self::scan_bar_bar, TokenKind::BarBar)
    }

    #[inline]
    pub(crate) fn parse_or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_or_operator_0_4_11(stream)
            .with_kind(RuleKind::OrOperator)
    }

    // OrderComparisonOperator = «LessThan» | «GreaterThan» | «LessThanEqual» | «GreaterThanEqual»;

    #[allow(dead_code)]
    fn parse_order_comparison_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(stream, Self::scan_less_than, TokenKind::LessThan) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_greater_than,
                TokenKind::GreaterThan,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_less_than_equal,
                TokenKind::LessThanEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_greater_than_equal,
                TokenKind::GreaterThanEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_order_comparison_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_order_comparison_operator_0_4_11(stream)
            .with_kind(RuleKind::OrderComparisonOperator)
    }

    // OverrideSpecifier = «OverrideKeyword» («OpenParen» IdentifierPath («Comma» IdentifierPath)* «CloseParen»)?;

    #[allow(dead_code)]
    fn parse_override_specifier_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_override_keyword,
                TokenKind::OverrideKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_open_paren,
                        TokenKind::OpenParen,
                    ) {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: open_node, ..
                        } => {
                            match {
                                let mut result = Vec::new();
                                loop {
                                    match self.parse_identifier_path(stream) {
                                        err @ Fail { .. } => break err,
                                        Pass { builder, .. } => {
                                            result.push(builder);
                                            let save = stream.position();
                                            match self.parse_token_with_trivia(
                                                stream,
                                                Self::scan_comma,
                                                TokenKind::Comma,
                                            ) {
                                                Fail { error } => {
                                                    stream.set_position(save);
                                                    break Pass {
                                                        builder: cst::NodeBuilder::multiple(result),
                                                        error: Some(error),
                                                    };
                                                }
                                                Pass { builder, .. } => result.push(builder),
                                            }
                                        }
                                    }
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    builder: expr_node,
                                    error: expr_error,
                                } => {
                                    match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_close_paren,
                                        TokenKind::CloseParen,
                                    ) {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            builder: close_node,
                                            ..
                                        } => Pass {
                                            builder: cst::NodeBuilder::multiple(vec![
                                                open_node, expr_node, close_node,
                                            ]),
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_override_specifier(&self, stream: &mut Stream) -> ParserResult {
        self.parse_override_specifier_0_4_11(stream)
            .with_kind(RuleKind::OverrideSpecifier)
    }

    // ParameterDeclaration = TypeName DataLocation? «Identifier»?;

    #[allow(dead_code)]
    fn parse_parameter_declaration_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_type_name(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match {
                let start_position = stream.position();
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_parameter_declaration(&self, stream: &mut Stream) -> ParserResult {
        self.parse_parameter_declaration_0_4_11(stream)
            .with_kind(RuleKind::ParameterDeclaration)
    }

    // ParameterList = «OpenParen» (ParameterDeclaration («Comma» ParameterDeclaration)*)? «CloseParen»;

    #[allow(dead_code)]
    fn parse_parameter_list_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(stream, Self::scan_open_paren, TokenKind::OpenParen)
            {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
                } => {
                    match {
                        let start_position = stream.position();
                        match {
                            let mut result = Vec::new();
                            loop {
                                match self.parse_parameter_declaration(stream) {
                                    err @ Fail { .. } => break err,
                                    Pass { builder, .. } => {
                                        result.push(builder);
                                        let save = stream.position();
                                        match self.parse_token_with_trivia(
                                            stream,
                                            Self::scan_comma,
                                            TokenKind::Comma,
                                        ) {
                                            Fail { error } => {
                                                stream.set_position(save);
                                                break Pass {
                                                    builder: cst::NodeBuilder::multiple(result),
                                                    error: Some(error),
                                                };
                                            }
                                            Pass { builder, .. } => result.push(builder),
                                        }
                                    }
                                }
                            }
                        } {
                            Fail { error } => {
                                stream.set_position(start_position);
                                Pass {
                                    builder: cst::NodeBuilder::empty(start_position),
                                    error: Some(error),
                                }
                            }
                            pass => pass,
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_paren,
                                TokenKind::CloseParen,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_parameter_list(&self, stream: &mut Stream) -> ParserResult {
        self.parse_parameter_list_0_4_11(stream)
            .with_kind(RuleKind::ParameterList)
    }

    // PayableType = «PayableKeyword»;

    #[allow(dead_code)]
    fn parse_payable_type_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            Self::scan_payable_keyword,
            TokenKind::PayableKeyword,
        )
    }

    #[inline]
    pub(crate) fn parse_payable_type(&self, stream: &mut Stream) -> ParserResult {
        self.parse_payable_type_0_4_11(stream)
            .with_kind(RuleKind::PayableType)
    }

    // PositionalArgumentList = Expression («Comma» Expression)*;

    #[allow(dead_code)]
    fn parse_positional_argument_list_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut result = Vec::new();
            loop {
                match self.parse_expression(stream) {
                    err @ Fail { .. } => break err,
                    Pass { builder, .. } => {
                        result.push(builder);
                        let save = stream.position();
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_comma,
                            TokenKind::Comma,
                        ) {
                            Fail { error } => {
                                stream.set_position(save);
                                break Pass {
                                    builder: cst::NodeBuilder::multiple(result),
                                    error: Some(error),
                                };
                            }
                            Pass { builder, .. } => result.push(builder),
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_positional_argument_list(&self, stream: &mut Stream) -> ParserResult {
        self.parse_positional_argument_list_0_4_11(stream)
            .with_kind(RuleKind::PositionalArgumentList)
    }

    // PragmaDirective = «PragmaKeyword» (VersionPragma | ABICoderPragma | ExperimentalPragma) «Semicolon»;

    #[allow(dead_code)]
    fn parse_pragma_directive_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_pragma_keyword,
                    TokenKind::PragmaKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_pragma_directive(&self, stream: &mut Stream) -> ParserResult {
        self.parse_pragma_directive_0_4_11(stream)
            .with_kind(RuleKind::PragmaDirective)
    }

    // (* v0.4.11 *)
    // PrimaryExpression = NewExpression
    //                   | TupleExpression
    //                   | ArrayLiteral
    //                   | BooleanLiteral
    //                   | NumericExpression
    //                   | StringExpression
    //                   | ElementaryType
    //                   | «Identifier»;

    #[allow(dead_code)]
    fn parse_primary_expression_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_new_expression(stream) {
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
            match self.parse_boolean_literal(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_numeric_expression(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_string_expression(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_elementary_type(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_identifier, TokenKind::Identifier)
            {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.5.3 *)
    // PrimaryExpression = NewExpression
    //                   | TupleExpression
    //                   | TypeExpression
    //                   | ArrayLiteral
    //                   | BooleanLiteral
    //                   | NumericExpression
    //                   | StringExpression
    //                   | ElementaryType
    //                   | «Identifier»;

    #[allow(dead_code)]
    fn parse_primary_expression_0_5_3(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_new_expression(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_tuple_expression(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_type_expression(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_array_literal(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_boolean_literal(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_numeric_expression(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_string_expression(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_elementary_type(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_identifier, TokenKind::Identifier)
            {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_primary_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_3 {
            self.parse_primary_expression_0_5_3(stream)
        } else {
            self.parse_primary_expression_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_primary_expression(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_primary_expression(stream)
            .with_kind(RuleKind::PrimaryExpression)
    }

    // (* v0.6.0 *)
    // ReceiveFunctionAttribute = ModifierInvocation
    //                          | OverrideSpecifier
    //                          | «ExternalKeyword»
    //                          | «PayableKeyword»
    //                          | «VirtualKeyword»;

    #[allow(dead_code)]
    fn parse_receive_function_attribute_0_6_0(&self, stream: &mut Stream) -> ParserResult {
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
            match self.parse_token_with_trivia(
                stream,
                Self::scan_external_keyword,
                TokenKind::ExternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_payable_keyword,
                TokenKind::PayableKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_virtual_keyword,
                TokenKind::VirtualKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_receive_function_attribute(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.parse_receive_function_attribute_0_6_0(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_receive_function_attribute(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_receive_function_attribute(stream)
            .map(|body| body.with_kind(RuleKind::ReceiveFunctionAttribute))
    }

    #[inline]
    pub(crate) fn parse_receive_function_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_receive_function_attribute(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.6.0 *)
    // ReceiveFunctionDefinition = «ReceiveKeyword» ParameterList ReceiveFunctionAttribute* («Semicolon» | Block);

    #[allow(dead_code)]
    fn parse_receive_function_definition_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_receive_keyword,
                TokenKind::ReceiveKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_parameter_list(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                                builder: if result.is_empty() {
                                    cst::NodeBuilder::empty(start_position)
                                } else {
                                    cst::NodeBuilder::multiple(result)
                                },
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_semicolon,
                    TokenKind::Semicolon,
                ) {
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
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_receive_function_definition(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.parse_receive_function_definition_0_6_0(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_receive_function_definition(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_receive_function_definition(stream)
            .map(|body| body.with_kind(RuleKind::ReceiveFunctionDefinition))
    }

    #[inline]
    pub(crate) fn parse_receive_function_definition(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_receive_function_definition(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // ReturnStatement = «ReturnKeyword» Expression? «Semicolon»;

    #[allow(dead_code)]
    fn parse_return_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_return_keyword,
                    TokenKind::ReturnKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_return_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_return_statement_0_4_11(stream)
            .with_kind(RuleKind::ReturnStatement)
    }

    // RevertStatement = «RevertKeyword» IdentifierPath? ArgumentList «Semicolon»;

    #[allow(dead_code)]
    fn parse_revert_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_revert_keyword,
                    TokenKind::RevertKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_argument_list(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_revert_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_revert_statement_0_4_11(stream)
            .with_kind(RuleKind::RevertStatement)
    }

    // SelectiveImport = «OpenBrace» «Identifier» ImportAlias? («Comma» «Identifier» ImportAlias?)* «CloseBrace» «FromKeyword» ImportPath;

    #[allow(dead_code)]
    fn parse_selective_import_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_brace,
                    TokenKind::OpenBrace,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => {
                        match {
                            let mut result = Vec::new();
                            loop {
                                match loop {
                                    let mut furthest_error = None;
                                    let result_0 = match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_identifier,
                                        TokenKind::Identifier,
                                    ) {
                                        Pass { builder, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            builder
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    let result_1 = match {
                                        let start_position = stream.position();
                                        match self.parse_import_alias(stream) {
                                            Fail { error } => {
                                                stream.set_position(start_position);
                                                Pass {
                                                    builder: cst::NodeBuilder::empty(
                                                        start_position,
                                                    ),
                                                    error: Some(error),
                                                }
                                            }
                                            pass => pass,
                                        }
                                    } {
                                        Pass { builder, error } => {
                                            furthest_error = error.map(|error| {
                                                error.maybe_merge_with(furthest_error)
                                            });
                                            builder
                                        }
                                        Fail { error } => {
                                            break Fail {
                                                error: error.maybe_merge_with(furthest_error),
                                            }
                                        }
                                    };
                                    break Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            result_0, result_1,
                                        ]),
                                        error: furthest_error,
                                    };
                                } {
                                    err @ Fail { .. } => break err,
                                    Pass { builder, .. } => {
                                        result.push(builder);
                                        let save = stream.position();
                                        match self.parse_token_with_trivia(
                                            stream,
                                            Self::scan_comma,
                                            TokenKind::Comma,
                                        ) {
                                            Fail { error } => {
                                                stream.set_position(save);
                                                break Pass {
                                                    builder: cst::NodeBuilder::multiple(result),
                                                    error: Some(error),
                                                };
                                            }
                                            Pass { builder, .. } => result.push(builder),
                                        }
                                    }
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_brace,
                                    TokenKind::CloseBrace,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_from_keyword,
                TokenKind::FromKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_import_path(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_selective_import(&self, stream: &mut Stream) -> ParserResult {
        self.parse_selective_import_0_4_11(stream)
            .with_kind(RuleKind::SelectiveImport)
    }

    // ShiftOperator = «LessThanLessThan» | «GreaterThanGreaterThan» | «GreaterThanGreaterThanGreaterThan»;

    #[allow(dead_code)]
    fn parse_shift_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(
                stream,
                Self::scan_less_than_less_than,
                TokenKind::LessThanLessThan,
            ) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_greater_than_greater_than,
                TokenKind::GreaterThanGreaterThan,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_greater_than_greater_than_greater_than,
                TokenKind::GreaterThanGreaterThanGreaterThan,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_shift_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_shift_operator_0_4_11(stream)
            .with_kind(RuleKind::ShiftOperator)
    }

    // SimpleImport = ImportPath ImportAlias?;

    #[allow(dead_code)]
    fn parse_simple_import_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_import_path(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                let start_position = stream.position();
                match self.parse_import_alias(stream) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_simple_import(&self, stream: &mut Stream) -> ParserResult {
        self.parse_simple_import_0_4_11(stream)
            .with_kind(RuleKind::SimpleImport)
    }

    // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement;

    #[allow(dead_code)]
    fn parse_simple_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
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
    }

    #[inline]
    pub(crate) fn parse_simple_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_simple_statement_0_4_11(stream)
            .with_kind(RuleKind::SimpleStatement)
    }

    // SourceUnit = (Directive | Definition)* EndOfFileTrivia?;

    #[allow(dead_code)]
    fn parse_source_unit_0_4_11(&self, stream: &mut Stream) -> ParserResult {
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
                            stream.set_position(start_position);
                            break Pass {
                                builder: if result.is_empty() {
                                    cst::NodeBuilder::empty(start_position)
                                } else {
                                    cst::NodeBuilder::multiple(result)
                                },
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_source_unit(&self, stream: &mut Stream) -> ParserResult {
        self.parse_source_unit_0_4_11(stream)
            .with_kind(RuleKind::SourceUnit)
    }

    // (* v0.4.11 *)
    // StateVariableAttribute = OverrideSpecifier
    //                        | «ConstantKeyword»
    //                        | «InternalKeyword»
    //                        | «PrivateKeyword»
    //                        | «PublicKeyword»;

    #[allow(dead_code)]
    fn parse_state_variable_attribute_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_override_specifier(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_constant_keyword,
                TokenKind::ConstantKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_internal_keyword,
                TokenKind::InternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_private_keyword,
                TokenKind::PrivateKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_public_keyword,
                TokenKind::PublicKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.6.5 *)
    // StateVariableAttribute = OverrideSpecifier
    //                        | «ConstantKeyword»
    //                        | «ImmutableKeyword»
    //                        | «InternalKeyword»
    //                        | «PrivateKeyword»
    //                        | «PublicKeyword»;

    #[allow(dead_code)]
    fn parse_state_variable_attribute_0_6_5(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_override_specifier(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_constant_keyword,
                TokenKind::ConstantKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_immutable_keyword,
                TokenKind::ImmutableKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_internal_keyword,
                TokenKind::InternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_private_keyword,
                TokenKind::PrivateKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_public_keyword,
                TokenKind::PublicKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_state_variable_attribute(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_5 {
            self.parse_state_variable_attribute_0_6_5(stream)
        } else {
            self.parse_state_variable_attribute_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_state_variable_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_state_variable_attribute(stream)
            .with_kind(RuleKind::StateVariableAttribute)
    }

    // StateVariableDeclaration = TypeName StateVariableAttribute* «Identifier» («Equal» Expression)? «Semicolon»;

    #[allow(dead_code)]
    fn parse_state_variable_declaration_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                                    builder: if result.is_empty() {
                                        cst::NodeBuilder::empty(start_position)
                                    } else {
                                        cst::NodeBuilder::multiple(result)
                                    },
                                    error: Some(error),
                                };
                            }
                            Pass { builder, .. } => result.push(builder),
                        }
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                        let result_0 = match self.parse_token_with_trivia(
                            stream,
                            Self::scan_equal,
                            TokenKind::Equal,
                        ) {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_expression(stream) {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![
                        result_0, result_1, result_2, result_3,
                    ]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_state_variable_declaration(&self, stream: &mut Stream) -> ParserResult {
        self.parse_state_variable_declaration_0_4_11(stream)
            .with_kind(RuleKind::StateVariableDeclaration)
    }

    // (* v0.4.11 *)
    // Statement = Block
    //           | SimpleStatement
    //           | IfStatement
    //           | ForStatement
    //           | WhileStatement
    //           | DoWhileStatement
    //           | ContinueStatement
    //           | BreakStatement
    //           | ReturnStatement
    //           | ThrowStatement
    //           | RevertStatement
    //           | DeleteStatement
    //           | AssemblyStatement;

    #[allow(dead_code)]
    fn parse_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
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
            match self.parse_return_statement(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_throw_statement(stream) {
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
    }

    // (* v0.4.21 *)
    // Statement = Block
    //           | SimpleStatement
    //           | IfStatement
    //           | ForStatement
    //           | WhileStatement
    //           | DoWhileStatement
    //           | ContinueStatement
    //           | BreakStatement
    //           | ReturnStatement
    //           | EmitStatement
    //           | ThrowStatement
    //           | RevertStatement
    //           | DeleteStatement
    //           | AssemblyStatement;

    #[allow(dead_code)]
    fn parse_statement_0_4_21(&self, stream: &mut Stream) -> ParserResult {
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
            match self.parse_throw_statement(stream) {
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
    }

    // (* v0.5.0 *)
    // Statement = Block
    //           | SimpleStatement
    //           | IfStatement
    //           | ForStatement
    //           | WhileStatement
    //           | DoWhileStatement
    //           | ContinueStatement
    //           | BreakStatement
    //           | ReturnStatement
    //           | EmitStatement
    //           | RevertStatement
    //           | DeleteStatement
    //           | AssemblyStatement;

    #[allow(dead_code)]
    fn parse_statement_0_5_0(&self, stream: &mut Stream) -> ParserResult {
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
    }

    // (* v0.6.0 *)
    // Statement = Block
    //           | SimpleStatement
    //           | IfStatement
    //           | ForStatement
    //           | WhileStatement
    //           | DoWhileStatement
    //           | ContinueStatement
    //           | BreakStatement
    //           | TryStatement
    //           | ReturnStatement
    //           | EmitStatement
    //           | RevertStatement
    //           | DeleteStatement
    //           | AssemblyStatement;

    #[allow(dead_code)]
    fn parse_statement_0_6_0(&self, stream: &mut Stream) -> ParserResult {
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
    }

    fn dispatch_parse_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.parse_statement_0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.parse_statement_0_5_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_4_21 {
            self.parse_statement_0_4_21(stream)
        } else {
            self.parse_statement_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_statement(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_statement(stream)
            .with_kind(RuleKind::Statement)
    }

    // (* v0.4.11 *)
    // StringExpression = «HexStringLiteral»+ | «AsciiStringLiteral»+;

    #[allow(dead_code)]
    fn parse_string_expression_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match {
                let mut result = Vec::new();
                loop {
                    let start_position = stream.position();
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_hex_string_literal,
                        TokenKind::HexStringLiteral,
                    ) {
                        Fail { error } => {
                            if result.is_empty() {
                                break Fail { error };
                            }
                            stream.set_position(start_position);
                            break Pass {
                                builder: cst::NodeBuilder::multiple(result),
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
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
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_ascii_string_literal,
                        TokenKind::AsciiStringLiteral,
                    ) {
                        Fail { error } => {
                            if result.is_empty() {
                                break Fail { error };
                            }
                            stream.set_position(start_position);
                            break Pass {
                                builder: cst::NodeBuilder::multiple(result),
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
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
    }

    // (* v0.7.0 *)
    // StringExpression = «HexStringLiteral»+ | «AsciiStringLiteral»+ | «UnicodeStringLiteral»+;

    #[allow(dead_code)]
    fn parse_string_expression_0_7_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match {
                let mut result = Vec::new();
                loop {
                    let start_position = stream.position();
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_hex_string_literal,
                        TokenKind::HexStringLiteral,
                    ) {
                        Fail { error } => {
                            if result.is_empty() {
                                break Fail { error };
                            }
                            stream.set_position(start_position);
                            break Pass {
                                builder: cst::NodeBuilder::multiple(result),
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
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
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_ascii_string_literal,
                        TokenKind::AsciiStringLiteral,
                    ) {
                        Fail { error } => {
                            if result.is_empty() {
                                break Fail { error };
                            }
                            stream.set_position(start_position);
                            break Pass {
                                builder: cst::NodeBuilder::multiple(result),
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
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
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_unicode_string_literal,
                        TokenKind::UnicodeStringLiteral,
                    ) {
                        Fail { error } => {
                            if result.is_empty() {
                                break Fail { error };
                            }
                            stream.set_position(start_position);
                            break Pass {
                                builder: cst::NodeBuilder::multiple(result),
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
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
    }

    fn dispatch_parse_string_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            self.parse_string_expression_0_7_0(stream)
        } else {
            self.parse_string_expression_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_string_expression(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_string_expression(stream)
            .with_kind(RuleKind::StringExpression)
    }

    // StructDefinition = «StructKeyword» «Identifier» «OpenBrace» StructMember+ «CloseBrace»;

    #[allow(dead_code)]
    fn parse_struct_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_struct_keyword,
                TokenKind::StructKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_identifier,
                TokenKind::Identifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_brace,
                    TokenKind::OpenBrace,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
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
                                            builder: cst::NodeBuilder::multiple(result),
                                            error: Some(error),
                                        };
                                    }
                                    Pass { builder, .. } => result.push(builder),
                                }
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_brace,
                                    TokenKind::CloseBrace,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_struct_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_struct_definition_0_4_11(stream)
            .with_kind(RuleKind::StructDefinition)
    }

    // StructMember = TypeName «Identifier» «Semicolon»;

    #[allow(dead_code)]
    fn parse_struct_member_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_struct_member(&self, stream: &mut Stream) -> ParserResult {
        self.parse_struct_member_0_4_11(stream)
            .with_kind(RuleKind::StructMember)
    }

    // (* v0.4.11 *)
    // ThrowStatement = «ThrowKeyword» «Semicolon»;

    #[allow(dead_code)]
    fn parse_throw_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(
                stream,
                Self::scan_throw_keyword,
                TokenKind::ThrowKeyword,
            ) {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    fn dispatch_parse_throw_statement(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            None
        } else {
            Some(self.parse_throw_statement_0_4_11(stream))
        }
    }

    pub(crate) fn maybe_parse_throw_statement(&self, stream: &mut Stream) -> Option<ParserResult> {
        self.dispatch_parse_throw_statement(stream)
            .map(|body| body.with_kind(RuleKind::ThrowStatement))
    }

    #[inline]
    pub(crate) fn parse_throw_statement(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_throw_statement(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // TrailingTrivia = «Whitespace»? «SingleLineComment»? «EndOfLine»;

    #[allow(dead_code)]
    fn parse_trailing_trivia_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match {
                let start_position = stream.position();
                match self.parse_token(stream, Self::scan_whitespace, TokenKind::Whitespace) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                let start_position = stream.position();
                match self.parse_token(
                    stream,
                    Self::scan_single_line_comment,
                    TokenKind::SingleLineComment,
                ) {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 =
                match self.parse_token(stream, Self::scan_end_of_line, TokenKind::EndOfLine) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_trailing_trivia(&self, stream: &mut Stream) -> ParserResult {
        self.parse_trailing_trivia_0_4_11(stream)
            .with_kind(RuleKind::TrailingTrivia)
    }

    // (* v0.6.0 *)
    // TryStatement = «TryKeyword» Expression («ReturnsKeyword» ParameterList)? Block CatchClause+;

    #[allow(dead_code)]
    fn parse_try_statement_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_try_keyword,
                TokenKind::TryKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_expression(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    let result_0 = match self.parse_token_with_trivia(
                        stream,
                        Self::scan_returns_keyword,
                        TokenKind::ReturnsKeyword,
                    ) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_1 = match self.parse_parameter_list(stream) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    break Pass {
                        builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                        error: furthest_error,
                    };
                } {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_3 = match self.parse_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                                builder: cst::NodeBuilder::multiple(result),
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![
                    result_0, result_1, result_2, result_3, result_4,
                ]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_try_statement(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.parse_try_statement_0_6_0(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_try_statement(&self, stream: &mut Stream) -> Option<ParserResult> {
        self.dispatch_parse_try_statement(stream)
            .map(|body| body.with_kind(RuleKind::TryStatement))
    }

    #[inline]
    pub(crate) fn parse_try_statement(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_try_statement(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // TupleDeconstructionStatement = «OpenParen» (((TypeName DataLocation? «Identifier») | (DataLocation? «Identifier»))? («Comma» ((TypeName DataLocation? «Identifier») | (DataLocation? «Identifier»))?)*)? «CloseParen» «Equal» Expression «Semicolon»;

    #[allow(dead_code)]
    fn parse_tuple_deconstruction_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_open_paren,
                        TokenKind::OpenParen,
                    ) {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: open_node, ..
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
                                                        Pass { builder, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            builder
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
                                                                    builder:
                                                                        cst::NodeBuilder::empty(
                                                                            start_position,
                                                                        ),
                                                                    error: Some(error),
                                                                }
                                                            }
                                                            pass => pass,
                                                        }
                                                    } {
                                                        Pass { builder, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            builder
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_2 = match self
                                                        .parse_token_with_trivia(
                                                            stream,
                                                            Self::scan_identifier,
                                                            TokenKind::Identifier,
                                                        ) {
                                                        Pass { builder, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            builder
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
                                                        builder: cst::NodeBuilder::multiple(vec![
                                                            result_0, result_1, result_2,
                                                        ]),
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
                                                                    builder:
                                                                        cst::NodeBuilder::empty(
                                                                            start_position,
                                                                        ),
                                                                    error: Some(error),
                                                                }
                                                            }
                                                            pass => pass,
                                                        }
                                                    } {
                                                        Pass { builder, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            builder
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_1 = match self
                                                        .parse_token_with_trivia(
                                                            stream,
                                                            Self::scan_identifier,
                                                            TokenKind::Identifier,
                                                        ) {
                                                        Pass { builder, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            builder
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
                                                        builder: cst::NodeBuilder::multiple(vec![
                                                            result_0, result_1,
                                                        ]),
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
                                                        builder: cst::NodeBuilder::empty(
                                                            start_position,
                                                        ),
                                                        error: Some(error),
                                                    }
                                                }
                                                pass => pass,
                                            }
                                        } {
                                            err @ Fail { .. } => break err,
                                            Pass { builder, .. } => {
                                                result.push(builder);
                                                let save = stream.position();
                                                match self.parse_token_with_trivia(
                                                    stream,
                                                    Self::scan_comma,
                                                    TokenKind::Comma,
                                                ) {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            builder: cst::NodeBuilder::multiple(
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { builder, .. } => result.push(builder),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    Fail { error } => {
                                        stream.set_position(start_position);
                                        Pass {
                                            builder: cst::NodeBuilder::empty(start_position),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                err @ Fail { .. } => err,
                                Pass {
                                    builder: expr_node,
                                    error: expr_error,
                                } => {
                                    match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_close_paren,
                                        TokenKind::CloseParen,
                                    ) {
                                        Fail { error } => Fail {
                                            error: error.maybe_merge_with(expr_error),
                                        },
                                        Pass {
                                            builder: close_node,
                                            ..
                                        } => Pass {
                                            builder: cst::NodeBuilder::multiple(vec![
                                                open_node, expr_node, close_node,
                                            ]),
                                            error: None,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_equal,
                    TokenKind::Equal,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_expression(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_tuple_deconstruction_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_tuple_deconstruction_statement_0_4_11(stream)
            .with_kind(RuleKind::TupleDeconstructionStatement)
    }

    // TupleExpression = «OpenParen» Expression? («Comma» Expression?)* «CloseParen»;

    #[allow(dead_code)]
    fn parse_tuple_expression_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(stream, Self::scan_open_paren, TokenKind::OpenParen)
            {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
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
                                            builder: cst::NodeBuilder::empty(start_position),
                                            error: Some(error),
                                        }
                                    }
                                    pass => pass,
                                }
                            } {
                                err @ Fail { .. } => break err,
                                Pass { builder, .. } => {
                                    result.push(builder);
                                    let save = stream.position();
                                    match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_comma,
                                        TokenKind::Comma,
                                    ) {
                                        Fail { error } => {
                                            stream.set_position(save);
                                            break Pass {
                                                builder: cst::NodeBuilder::multiple(result),
                                                error: Some(error),
                                            };
                                        }
                                        Pass { builder, .. } => result.push(builder),
                                    }
                                }
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_paren,
                                TokenKind::CloseParen,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_tuple_expression(&self, stream: &mut Stream) -> ParserResult {
        self.parse_tuple_expression_0_4_11(stream)
            .with_kind(RuleKind::TupleExpression)
    }

    // (* v0.5.3 *)
    // TypeExpression = «TypeKeyword» «OpenParen» TypeName «CloseParen»;

    #[allow(dead_code)]
    fn parse_type_expression_0_5_3(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_type_keyword,
                TokenKind::TypeKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_paren,
                    TokenKind::OpenParen,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => match self.parse_type_name(stream) {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_paren,
                                TokenKind::CloseParen,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    },
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_type_expression(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_5_3 {
            Some(self.parse_type_expression_0_5_3(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_type_expression(&self, stream: &mut Stream) -> Option<ParserResult> {
        self.dispatch_parse_type_expression(stream)
            .map(|body| body.with_kind(RuleKind::TypeExpression))
    }

    #[inline]
    pub(crate) fn parse_type_expression(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_type_expression(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // TypeName = ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath;
    // ArrayTypeName = TypeName «OpenBracket» Expression? «CloseBracket»;

    #[allow(dead_code)]
    fn parse_type_name_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            enum Pratt {
                Operator {
                    kind: RuleKind,
                    builder: cst::NodeBuilder,
                    left_binding_power: u8,
                    right_binding_power: u8,
                },
                Builder(cst::NodeBuilder),
            }
            let mut elements = Vec::new();
            if let Some(error) = loop {
                match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_function_type(stream) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_mapping_type(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_elementary_type(stream) {
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
                    Fail { error } => break Some(error),
                    Pass { builder, .. } => elements.push(Pratt::Builder(builder)),
                }
                loop {
                    let start_position = stream.position();
                    match match {
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_open_bracket,
                            TokenKind::OpenBracket,
                        ) {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: open_node, ..
                            } => {
                                match {
                                    let start_position = stream.position();
                                    match self.parse_expression(stream) {
                                        Fail { error } => {
                                            stream.set_position(start_position);
                                            Pass {
                                                builder: cst::NodeBuilder::empty(start_position),
                                                error: Some(error),
                                            }
                                        }
                                        pass => pass,
                                    }
                                } {
                                    err @ Fail { .. } => err,
                                    Pass {
                                        builder: expr_node,
                                        error: expr_error,
                                    } => {
                                        match self.parse_token_with_trivia(
                                            stream,
                                            Self::scan_close_bracket,
                                            TokenKind::CloseBracket,
                                        ) {
                                            Fail { error } => Fail {
                                                error: error.maybe_merge_with(expr_error),
                                            },
                                            Pass {
                                                builder: close_node,
                                                ..
                                            } => Pass {
                                                builder: cst::NodeBuilder::multiple(vec![
                                                    open_node, expr_node, close_node,
                                                ]),
                                                error: None,
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    } {
                        Pass { builder, .. } => Ok(Pratt::Operator {
                            builder,
                            kind: RuleKind::ArrayTypeName,
                            left_binding_power: 1u8,
                            right_binding_power: 255,
                        }),
                        Fail { error } => Err(error),
                    } {
                        Err(_) => {
                            stream.set_position(start_position);
                            break;
                        }
                        Ok(operator) => elements.push(operator),
                    }
                }
                break None;
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
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                            ) = (left, op)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![left, op]).with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        } else if *left_binding_power == 255 {
                            let op = elements.remove(i);
                            let right = elements.remove(i);
                            if let (
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (op, right)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![op, right]).with_kind(kind);
                                elements.insert(i, Pratt::Builder(builder));
                                i = i.saturating_sub(1);
                            } else {
                                unreachable!()
                            }
                        } else {
                            let left = elements.remove(i - 1);
                            let op = elements.remove(i - 1);
                            let right = elements.remove(i - 1);
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (left, op, right)
                            {
                                let builder = cst::NodeBuilder::multiple(vec![left, op, right])
                                    .with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        }
                    } else {
                        i += 1;
                    }
                }
                if let Pratt::Builder(builder) = elements.pop().unwrap() {
                    Pass {
                        builder,
                        error: None,
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_type_name(&self, stream: &mut Stream) -> ParserResult {
        self.parse_type_name_0_4_11(stream)
            .with_kind(RuleKind::TypeName)
    }

    // UnaryPostfixOperator = «PlusPlus» | «MinusMinus»;

    #[allow(dead_code)]
    fn parse_unary_postfix_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(stream, Self::scan_plus_plus, TokenKind::PlusPlus) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_minus_minus,
                TokenKind::MinusMinus,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_unary_postfix_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_unary_postfix_operator_0_4_11(stream)
            .with_kind(RuleKind::UnaryPostfixOperator)
    }

    // (* v0.4.11 *)
    // UnaryPrefixOperator = «PlusPlus»
    //                     | «MinusMinus»
    //                     | «Tilde»
    //                     | «Bang»
    //                     | «Minus»
    //                     | «Plus»;

    #[allow(dead_code)]
    fn parse_unary_prefix_operator_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(stream, Self::scan_plus_plus, TokenKind::PlusPlus) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_minus_minus,
                TokenKind::MinusMinus,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_tilde, TokenKind::Tilde) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_bang, TokenKind::Bang) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_minus, TokenKind::Minus) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_plus, TokenKind::Plus) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.5.0 *)
    // UnaryPrefixOperator = «PlusPlus»
    //                     | «MinusMinus»
    //                     | «Tilde»
    //                     | «Bang»
    //                     | «Minus»;

    #[allow(dead_code)]
    fn parse_unary_prefix_operator_0_5_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(stream, Self::scan_plus_plus, TokenKind::PlusPlus) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_minus_minus,
                TokenKind::MinusMinus,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_tilde, TokenKind::Tilde) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_bang, TokenKind::Bang) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_minus, TokenKind::Minus) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_unary_prefix_operator(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.parse_unary_prefix_operator_0_5_0(stream)
        } else {
            self.parse_unary_prefix_operator_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_unary_prefix_operator(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_unary_prefix_operator(stream)
            .with_kind(RuleKind::UnaryPrefixOperator)
    }

    // (* v0.8.0 *)
    // UncheckedBlock = «UncheckedKeyword» Block;

    #[allow(dead_code)]
    fn parse_unchecked_block_0_8_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_unchecked_keyword,
                TokenKind::UncheckedKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_unchecked_block(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            Some(self.parse_unchecked_block_0_8_0(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_unchecked_block(&self, stream: &mut Stream) -> Option<ParserResult> {
        self.dispatch_parse_unchecked_block(stream)
            .map(|body| body.with_kind(RuleKind::UncheckedBlock))
    }

    #[inline]
    pub(crate) fn parse_unchecked_block(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_unchecked_block(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.11 *)
    // UnnamedFunctionAttribute = ModifierInvocation
    //                          | OverrideSpecifier
    //                          | «ExternalKeyword»
    //                          | «PayableKeyword»
    //                          | «PureKeyword»
    //                          | «ViewKeyword»;

    #[allow(dead_code)]
    fn parse_unnamed_function_attribute_0_4_11(&self, stream: &mut Stream) -> ParserResult {
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
            match self.parse_token_with_trivia(
                stream,
                Self::scan_external_keyword,
                TokenKind::ExternalKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_payable_keyword,
                TokenKind::PayableKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_pure_keyword,
                TokenKind::PureKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_view_keyword,
                TokenKind::ViewKeyword,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_unnamed_function_attribute(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            None
        } else {
            Some(self.parse_unnamed_function_attribute_0_4_11(stream))
        }
    }

    pub(crate) fn maybe_parse_unnamed_function_attribute(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_unnamed_function_attribute(stream)
            .map(|body| body.with_kind(RuleKind::UnnamedFunctionAttribute))
    }

    #[inline]
    pub(crate) fn parse_unnamed_function_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_unnamed_function_attribute(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.11 *)
    // UnnamedFunctionDefinition = «FunctionKeyword» ParameterList UnnamedFunctionAttribute* («Semicolon» | Block);

    #[allow(dead_code)]
    fn parse_unnamed_function_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_function_keyword,
                TokenKind::FunctionKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_parameter_list(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    match self.parse_unnamed_function_attribute(stream) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            break Pass {
                                builder: if result.is_empty() {
                                    cst::NodeBuilder::empty(start_position)
                                } else {
                                    cst::NodeBuilder::multiple(result)
                                },
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_semicolon,
                    TokenKind::Semicolon,
                ) {
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
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2, result_3]),
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_unnamed_function_definition(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            None
        } else {
            Some(self.parse_unnamed_function_definition_0_4_11(stream))
        }
    }

    pub(crate) fn maybe_parse_unnamed_function_definition(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_unnamed_function_definition(stream)
            .map(|body| body.with_kind(RuleKind::UnnamedFunctionDefinition))
    }

    #[inline]
    pub(crate) fn parse_unnamed_function_definition(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_unnamed_function_definition(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.8.19 *)
    // UserDefinedOperator = «Ampersand»
    //                     | «BangEqual»
    //                     | «Bar»
    //                     | «Caret»
    //                     | «EqualEqual»
    //                     | «GreaterThan»
    //                     | «GreaterThanEqual»
    //                     | «LessThan»
    //                     | «LessThanEqual»
    //                     | «Minus»
    //                     | «Percent»
    //                     | «Plus»
    //                     | «Slash»
    //                     | «Asterisk»
    //                     | «Tilde»;

    #[allow(dead_code)]
    fn parse_user_defined_operator_0_8_19(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_token_with_trivia(stream, Self::scan_ampersand, TokenKind::Ampersand) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_bang_equal, TokenKind::BangEqual)
            {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_bar, TokenKind::Bar) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_caret, TokenKind::Caret) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_equal_equal,
                TokenKind::EqualEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_greater_than,
                TokenKind::GreaterThan,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_greater_than_equal,
                TokenKind::GreaterThanEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_less_than, TokenKind::LessThan) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_less_than_equal,
                TokenKind::LessThanEqual,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_minus, TokenKind::Minus) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_percent, TokenKind::Percent) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_plus, TokenKind::Plus) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_slash, TokenKind::Slash) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_asterisk, TokenKind::Asterisk) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(stream, Self::scan_tilde, TokenKind::Tilde) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_user_defined_operator(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_19 {
            Some(self.parse_user_defined_operator_0_8_19(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_user_defined_operator(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_user_defined_operator(stream)
            .map(|body| body.with_kind(RuleKind::UserDefinedOperator))
    }

    #[inline]
    pub(crate) fn parse_user_defined_operator(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_user_defined_operator(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.8.8 *)
    // UserDefinedValueTypeDefinition = «TypeKeyword» «Identifier» «IsKeyword» ElementaryType «Semicolon»;

    #[allow(dead_code)]
    fn parse_user_defined_value_type_definition_0_8_8(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_type_keyword,
                    TokenKind::TypeKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_is_keyword,
                    TokenKind::IsKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_3 = match self.parse_elementary_type(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![
                        result_0, result_1, result_2, result_3,
                    ]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    fn dispatch_parse_user_defined_value_type_definition(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_8 {
            Some(self.parse_user_defined_value_type_definition_0_8_8(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_user_defined_value_type_definition(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_user_defined_value_type_definition(stream)
            .map(|body| body.with_kind(RuleKind::UserDefinedValueTypeDefinition))
    }

    #[inline]
    pub(crate) fn parse_user_defined_value_type_definition(
        &self,
        stream: &mut Stream,
    ) -> ParserResult {
        self.maybe_parse_user_defined_value_type_definition(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.11 *)
    // UsingDirective = «UsingKeyword» (IdentifierPath | («OpenBrace» IdentifierPath («Comma» IdentifierPath)* «CloseBrace»)) «ForKeyword» («Asterisk» | TypeName) «GlobalKeyword»? «Semicolon»;

    #[allow(dead_code)]
    fn parse_using_directive_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_using_keyword,
                    TokenKind::UsingKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_open_brace,
                            TokenKind::OpenBrace,
                        ) {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: open_node, ..
                            } => {
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match self.parse_identifier_path(stream) {
                                            err @ Fail { .. } => break err,
                                            Pass { builder, .. } => {
                                                result.push(builder);
                                                let save = stream.position();
                                                match self.parse_token_with_trivia(
                                                    stream,
                                                    Self::scan_comma,
                                                    TokenKind::Comma,
                                                ) {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            builder: cst::NodeBuilder::multiple(
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { builder, .. } => result.push(builder),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    err @ Fail { .. } => err,
                                    Pass {
                                        builder: expr_node,
                                        error: expr_error,
                                    } => {
                                        match self.parse_token_with_trivia(
                                            stream,
                                            Self::scan_close_brace,
                                            TokenKind::CloseBrace,
                                        ) {
                                            Fail { error } => Fail {
                                                error: error.maybe_merge_with(expr_error),
                                            },
                                            Pass {
                                                builder: close_node,
                                                ..
                                            } => Pass {
                                                builder: cst::NodeBuilder::multiple(vec![
                                                    open_node, expr_node, close_node,
                                                ]),
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
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_for_keyword,
                    TokenKind::ForKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_asterisk,
                        TokenKind::Asterisk,
                    ) {
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
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let start_position = stream.position();
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_global_keyword,
                        TokenKind::GlobalKeyword,
                    ) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![
                        result_0, result_1, result_2, result_3, result_4,
                    ]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    // (* v0.8.19 *)
    // UsingDirective = «UsingKeyword» (IdentifierPath | («OpenBrace» IdentifierPath («AsKeyword» UserDefinedOperator)? («Comma» IdentifierPath («AsKeyword» UserDefinedOperator)?)* «CloseBrace»)) «ForKeyword» («Asterisk» | TypeName) «GlobalKeyword»? «Semicolon»;

    #[allow(dead_code)]
    fn parse_using_directive_0_8_19(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_using_keyword,
                    TokenKind::UsingKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_open_brace,
                            TokenKind::OpenBrace,
                        ) {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: open_node, ..
                            } => {
                                match {
                                    let mut result = Vec::new();
                                    loop {
                                        match loop {
                                            let mut furthest_error = None;
                                            let result_0 = match self.parse_identifier_path(stream)
                                            {
                                                Pass { builder, error } => {
                                                    furthest_error = error.map(|error| {
                                                        error.maybe_merge_with(furthest_error)
                                                    });
                                                    builder
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
                                                match loop {
                                                    let mut furthest_error = None;
                                                    let result_0 = match self
                                                        .parse_token_with_trivia(
                                                            stream,
                                                            Self::scan_as_keyword,
                                                            TokenKind::AsKeyword,
                                                        ) {
                                                        Pass { builder, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            builder
                                                        }
                                                        Fail { error } => {
                                                            break Fail {
                                                                error: error.maybe_merge_with(
                                                                    furthest_error,
                                                                ),
                                                            }
                                                        }
                                                    };
                                                    let result_1 = match self
                                                        .parse_user_defined_operator(stream)
                                                    {
                                                        Pass { builder, error } => {
                                                            furthest_error = error.map(|error| {
                                                                error.maybe_merge_with(
                                                                    furthest_error,
                                                                )
                                                            });
                                                            builder
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
                                                        builder: cst::NodeBuilder::multiple(vec![
                                                            result_0, result_1,
                                                        ]),
                                                        error: furthest_error,
                                                    };
                                                } {
                                                    Fail { error } => {
                                                        stream.set_position(start_position);
                                                        Pass {
                                                            builder: cst::NodeBuilder::empty(
                                                                start_position,
                                                            ),
                                                            error: Some(error),
                                                        }
                                                    }
                                                    pass => pass,
                                                }
                                            } {
                                                Pass { builder, error } => {
                                                    furthest_error = error.map(|error| {
                                                        error.maybe_merge_with(furthest_error)
                                                    });
                                                    builder
                                                }
                                                Fail { error } => {
                                                    break Fail {
                                                        error: error
                                                            .maybe_merge_with(furthest_error),
                                                    }
                                                }
                                            };
                                            break Pass {
                                                builder: cst::NodeBuilder::multiple(vec![
                                                    result_0, result_1,
                                                ]),
                                                error: furthest_error,
                                            };
                                        } {
                                            err @ Fail { .. } => break err,
                                            Pass { builder, .. } => {
                                                result.push(builder);
                                                let save = stream.position();
                                                match self.parse_token_with_trivia(
                                                    stream,
                                                    Self::scan_comma,
                                                    TokenKind::Comma,
                                                ) {
                                                    Fail { error } => {
                                                        stream.set_position(save);
                                                        break Pass {
                                                            builder: cst::NodeBuilder::multiple(
                                                                result,
                                                            ),
                                                            error: Some(error),
                                                        };
                                                    }
                                                    Pass { builder, .. } => result.push(builder),
                                                }
                                            }
                                        }
                                    }
                                } {
                                    err @ Fail { .. } => err,
                                    Pass {
                                        builder: expr_node,
                                        error: expr_error,
                                    } => {
                                        match self.parse_token_with_trivia(
                                            stream,
                                            Self::scan_close_brace,
                                            TokenKind::CloseBrace,
                                        ) {
                                            Fail { error } => Fail {
                                                error: error.maybe_merge_with(expr_error),
                                            },
                                            Pass {
                                                builder: close_node,
                                                ..
                                            } => Pass {
                                                builder: cst::NodeBuilder::multiple(vec![
                                                    open_node, expr_node, close_node,
                                                ]),
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
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_for_keyword,
                    TokenKind::ForKeyword,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_asterisk,
                        TokenKind::Asterisk,
                    ) {
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
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_4 = match {
                    let start_position = stream.position();
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_global_keyword,
                        TokenKind::GlobalKeyword,
                    ) {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![
                        result_0, result_1, result_2, result_3, result_4,
                    ]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    fn dispatch_parse_using_directive(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_19 {
            self.parse_using_directive_0_8_19(stream)
        } else {
            self.parse_using_directive_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_using_directive(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_using_directive(stream)
            .with_kind(RuleKind::UsingDirective)
    }

    // (* v0.4.11 *)
    // VariableDeclarationStatement = ((TypeName DataLocation?) | «VarKeyword») «Identifier» («Equal» Expression)? «Semicolon»;

    #[allow(dead_code)]
    fn parse_variable_declaration_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match loop {
                        let mut furthest_error = None;
                        let result_0 = match self.parse_type_name(stream) {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
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
                                        builder: cst::NodeBuilder::empty(start_position),
                                        error: Some(error),
                                    }
                                }
                                pass => pass,
                            }
                        } {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_var_keyword,
                        TokenKind::VarKeyword,
                    ) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_1 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                        let result_0 = match self.parse_token_with_trivia(
                            stream,
                            Self::scan_equal,
                            TokenKind::Equal,
                        ) {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_expression(stream) {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    // (* v0.5.0 *)
    // VariableDeclarationStatement = TypeName DataLocation? «Identifier» («Equal» Expression)? «Semicolon»;

    #[allow(dead_code)]
    fn parse_variable_declaration_statement_0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            match loop {
                let mut furthest_error = None;
                let result_0 = match self.parse_type_name(stream) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                let result_2 = match self.parse_token_with_trivia(
                    stream,
                    Self::scan_identifier,
                    TokenKind::Identifier,
                ) {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
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
                        let result_0 = match self.parse_token_with_trivia(
                            stream,
                            Self::scan_equal,
                            TokenKind::Equal,
                        ) {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_expression(stream) {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            stream.set_position(start_position);
                            Pass {
                                builder: cst::NodeBuilder::empty(start_position),
                                error: Some(error),
                            }
                        }
                        pass => pass,
                    }
                } {
                    Pass { builder, error } => {
                        furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                        builder
                    }
                    Fail { error } => {
                        break Fail {
                            error: error.maybe_merge_with(furthest_error),
                        }
                    }
                };
                break Pass {
                    builder: cst::NodeBuilder::multiple(vec![
                        result_0, result_1, result_2, result_3,
                    ]),
                    error: furthest_error,
                };
            } {
                err @ Fail { .. } => err,
                Pass {
                    builder: expr_node,
                    error: expr_error,
                } => {
                    match self.parse_token_with_trivia(
                        stream,
                        Self::scan_semicolon,
                        TokenKind::Semicolon,
                    ) {
                        Fail { error } => Fail {
                            error: error.maybe_merge_with(expr_error),
                        },
                        Pass {
                            builder: terminator_node,
                            ..
                        } => Pass {
                            builder: cst::NodeBuilder::multiple(vec![expr_node, terminator_node]),
                            error: None,
                        },
                    }
                }
            }
        }
    }

    fn dispatch_parse_variable_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.parse_variable_declaration_statement_0_5_0(stream)
        } else {
            self.parse_variable_declaration_statement_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_variable_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_variable_declaration_statement(stream)
            .with_kind(RuleKind::VariableDeclarationStatement)
    }

    // VersionPragma = «SolidityKeyword» VersionPragmaExpression+;

    #[allow(dead_code)]
    fn parse_version_pragma_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_solidity_keyword,
                TokenKind::SolidityKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    match self.parse_version_pragma_expression(stream) {
                        Fail { error } => {
                            if result.is_empty() {
                                break Fail { error };
                            }
                            stream.set_position(start_position);
                            break Pass {
                                builder: cst::NodeBuilder::multiple(result),
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            }
            .with_kind(RuleKind::VersionPragmaExpressionList)
            {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_version_pragma(&self, stream: &mut Stream) -> ParserResult {
        self.parse_version_pragma_0_4_11(stream)
            .with_kind(RuleKind::VersionPragma)
    }

    // VersionPragmaExpression = VersionPragmaAlternatives | VersionPragmaRange | VersionPragmaComparator | VersionPragmaSpecifier;
    // VersionPragmaAlternatives = VersionPragmaExpression «BarBar» VersionPragmaExpression;
    // VersionPragmaRange = VersionPragmaExpression «Minus» VersionPragmaExpression;
    // VersionPragmaComparator = («Caret» | «Tilde» | «Equal» | «LessThan» | «GreaterThan» | «LessThanEqual» | «GreaterThanEqual») VersionPragmaExpression;

    #[allow(dead_code)]
    fn parse_version_pragma_expression_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            enum Pratt {
                Operator {
                    kind: RuleKind,
                    builder: cst::NodeBuilder,
                    left_binding_power: u8,
                    right_binding_power: u8,
                },
                Builder(cst::NodeBuilder),
            }
            let mut elements = Vec::new();
            if let Some(error) = loop {
                loop {
                    let start_position = stream.position();
                    match match loop {
                        let start_position = stream.position();
                        let mut furthest_error;
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_caret,
                            TokenKind::Caret,
                        ) {
                            Fail { error } => furthest_error = error,
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_tilde,
                            TokenKind::Tilde,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_equal,
                            TokenKind::Equal,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_less_than,
                            TokenKind::LessThan,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_greater_than,
                            TokenKind::GreaterThan,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_less_than_equal,
                            TokenKind::LessThanEqual,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        stream.set_position(start_position);
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_greater_than_equal,
                            TokenKind::GreaterThanEqual,
                        ) {
                            Fail { error } => furthest_error.merge_with(error),
                            pass => break pass,
                        }
                        break Fail {
                            error: furthest_error,
                        };
                    } {
                        Pass { builder, .. } => Ok(Pratt::Operator {
                            builder,
                            kind: RuleKind::VersionPragmaComparator,
                            left_binding_power: 255,
                            right_binding_power: 5u8,
                        }),
                        Fail { error } => Err(error),
                    } {
                        Err(_) => {
                            stream.set_position(start_position);
                            break;
                        }
                        Ok(operator) => elements.push(operator),
                    }
                }
                match self.parse_version_pragma_specifier(stream) {
                    Fail { error } => break Some(error),
                    Pass { builder, .. } => elements.push(Pratt::Builder(builder)),
                }
                let start_position = stream.position();
                match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match match self.parse_token_with_trivia(
                        stream,
                        Self::scan_bar_bar,
                        TokenKind::BarBar,
                    ) {
                        Pass { builder, .. } => Ok(Pratt::Operator {
                            builder,
                            kind: RuleKind::VersionPragmaAlternatives,
                            left_binding_power: 1u8,
                            right_binding_power: 1u8 + 1,
                        }),
                        Fail { error } => Err(error),
                    } {
                        Err(error) => furthest_error = error,
                        ok => break ok,
                    }
                    stream.set_position(start_position);
                    match {
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_minus,
                            TokenKind::Minus,
                        ) {
                            Pass { builder, .. } => Ok(Pratt::Operator {
                                builder,
                                kind: RuleKind::VersionPragmaRange,
                                left_binding_power: 3u8,
                                right_binding_power: 3u8 + 1,
                            }),
                            Fail { error } => Err(error),
                        }
                    } {
                        Err(error) => furthest_error.merge_with(error),
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
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                            ) = (left, op)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![left, op]).with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        } else if *left_binding_power == 255 {
                            let op = elements.remove(i);
                            let right = elements.remove(i);
                            if let (
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (op, right)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![op, right]).with_kind(kind);
                                elements.insert(i, Pratt::Builder(builder));
                                i = i.saturating_sub(1);
                            } else {
                                unreachable!()
                            }
                        } else {
                            let left = elements.remove(i - 1);
                            let op = elements.remove(i - 1);
                            let right = elements.remove(i - 1);
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (left, op, right)
                            {
                                let builder = cst::NodeBuilder::multiple(vec![left, op, right])
                                    .with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        }
                    } else {
                        i += 1;
                    }
                }
                if let Pratt::Builder(builder) = elements.pop().unwrap() {
                    Pass {
                        builder,
                        error: None,
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_version_pragma_expression(&self, stream: &mut Stream) -> ParserResult {
        self.parse_version_pragma_expression_0_4_11(stream)
    }

    // VersionPragmaSpecifier = «VersionPragmaValue» («Period» «VersionPragmaValue»)*;

    #[allow(dead_code)]
    fn parse_version_pragma_specifier_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut result = Vec::new();
            loop {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_version_pragma_value,
                    TokenKind::VersionPragmaValue,
                ) {
                    err @ Fail { .. } => break err,
                    Pass { builder, .. } => {
                        result.push(builder);
                        let save = stream.position();
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_period,
                            TokenKind::Period,
                        ) {
                            Fail { error } => {
                                stream.set_position(save);
                                break Pass {
                                    builder: cst::NodeBuilder::multiple(result),
                                    error: Some(error),
                                };
                            }
                            Pass { builder, .. } => result.push(builder),
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_version_pragma_specifier(&self, stream: &mut Stream) -> ParserResult {
        self.parse_version_pragma_specifier_0_4_11(stream)
            .with_kind(RuleKind::VersionPragmaSpecifier)
    }

    // WhileStatement = «WhileKeyword» «OpenParen» Expression «CloseParen» Statement;

    #[allow(dead_code)]
    fn parse_while_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_while_keyword,
                TokenKind::WhileKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_paren,
                    TokenKind::OpenParen,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => match self.parse_expression(stream) {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_paren,
                                TokenKind::CloseParen,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    },
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_statement(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_while_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_while_statement_0_4_11(stream)
            .with_kind(RuleKind::WhileStatement)
    }

    // YulAssignmentStatement = YulIdentifierPath («Comma» YulIdentifierPath)* «ColonEqual» YulExpression;

    #[allow(dead_code)]
    fn parse_yul_assignment_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match {
                let mut result = Vec::new();
                loop {
                    match self.parse_yul_identifier_path(stream) {
                        err @ Fail { .. } => break err,
                        Pass { builder, .. } => {
                            result.push(builder);
                            let save = stream.position();
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_comma,
                                TokenKind::Comma,
                            ) {
                                Fail { error } => {
                                    stream.set_position(save);
                                    break Pass {
                                        builder: cst::NodeBuilder::multiple(result),
                                        error: Some(error),
                                    };
                                }
                                Pass { builder, .. } => result.push(builder),
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_colon_equal,
                TokenKind::ColonEqual,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_yul_expression(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_yul_assignment_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_assignment_statement_0_4_11(stream)
            .with_kind(RuleKind::YulAssignmentStatement)
    }

    // YulBlock = «OpenBrace» YulStatement* «CloseBrace»;

    #[allow(dead_code)]
    fn parse_yul_block_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            match self.parse_token_with_trivia(stream, Self::scan_open_brace, TokenKind::OpenBrace)
            {
                err @ Fail { .. } => err,
                Pass {
                    builder: open_node, ..
                } => {
                    match {
                        let mut result = Vec::new();
                        loop {
                            let start_position = stream.position();
                            match self.parse_yul_statement(stream) {
                                Fail { error } => {
                                    stream.set_position(start_position);
                                    break Pass {
                                        builder: if result.is_empty() {
                                            cst::NodeBuilder::empty(start_position)
                                        } else {
                                            cst::NodeBuilder::multiple(result)
                                        },
                                        error: Some(error),
                                    };
                                }
                                Pass { builder, .. } => result.push(builder),
                            }
                        }
                    } {
                        err @ Fail { .. } => err,
                        Pass {
                            builder: expr_node,
                            error: expr_error,
                        } => {
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_close_brace,
                                TokenKind::CloseBrace,
                            ) {
                                Fail { error } => Fail {
                                    error: error.maybe_merge_with(expr_error),
                                },
                                Pass {
                                    builder: close_node,
                                    ..
                                } => Pass {
                                    builder: cst::NodeBuilder::multiple(vec![
                                        open_node, expr_node, close_node,
                                    ]),
                                    error: None,
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_yul_block(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_block_0_4_11(stream)
            .with_kind(RuleKind::YulBlock)
    }

    // YulBreakStatement = «BreakKeyword»;

    #[allow(dead_code)]
    fn parse_yul_break_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, Self::scan_break_keyword, TokenKind::BreakKeyword)
    }

    #[inline]
    pub(crate) fn parse_yul_break_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_break_statement_0_4_11(stream)
            .with_kind(RuleKind::YulBreakStatement)
    }

    // YulContinueStatement = «ContinueKeyword»;

    #[allow(dead_code)]
    fn parse_yul_continue_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            Self::scan_continue_keyword,
            TokenKind::ContinueKeyword,
        )
    }

    #[inline]
    pub(crate) fn parse_yul_continue_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_continue_statement_0_4_11(stream)
            .with_kind(RuleKind::YulContinueStatement)
    }

    // YulDeclarationStatement = «LetKeyword» YulIdentifierPath («Comma» YulIdentifierPath)* («ColonEqual» YulExpression)?;

    #[allow(dead_code)]
    fn parse_yul_declaration_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_let_keyword,
                TokenKind::LetKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                        Pass { builder, .. } => {
                            result.push(builder);
                            let save = stream.position();
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_comma,
                                TokenKind::Comma,
                            ) {
                                Fail { error } => {
                                    stream.set_position(save);
                                    break Pass {
                                        builder: cst::NodeBuilder::multiple(result),
                                        error: Some(error),
                                    };
                                }
                                Pass { builder, .. } => result.push(builder),
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    let result_0 = match self.parse_token_with_trivia(
                        stream,
                        Self::scan_colon_equal,
                        TokenKind::ColonEqual,
                    ) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    let result_1 = match self.parse_yul_expression(stream) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    break Pass {
                        builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                        error: furthest_error,
                    };
                } {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_yul_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_declaration_statement_0_4_11(stream)
            .with_kind(RuleKind::YulDeclarationStatement)
    }

    // YulExpression = YulFunctionCallExpression | YulLiteral | YulIdentifierPath;
    // YulFunctionCallExpression = YulExpression «OpenParen» (YulExpression («Comma» YulExpression)*)? «CloseParen»;

    #[allow(dead_code)]
    fn parse_yul_expression_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            enum Pratt {
                Operator {
                    kind: RuleKind,
                    builder: cst::NodeBuilder,
                    left_binding_power: u8,
                    right_binding_power: u8,
                },
                Builder(cst::NodeBuilder),
            }
            let mut elements = Vec::new();
            if let Some(error) = loop {
                match loop {
                    let start_position = stream.position();
                    let mut furthest_error;
                    match self.parse_yul_literal(stream) {
                        Fail { error } => furthest_error = error,
                        pass => break pass,
                    }
                    stream.set_position(start_position);
                    match self.parse_yul_identifier_path(stream) {
                        Fail { error } => furthest_error.merge_with(error),
                        pass => break pass,
                    }
                    break Fail {
                        error: furthest_error,
                    };
                } {
                    Fail { error } => break Some(error),
                    Pass { builder, .. } => elements.push(Pratt::Builder(builder)),
                }
                loop {
                    let start_position = stream.position();
                    match match {
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_open_paren,
                            TokenKind::OpenParen,
                        ) {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: open_node, ..
                            } => {
                                match {
                                    let start_position = stream.position();
                                    match {
                                        let mut result = Vec::new();
                                        loop {
                                            match self.parse_yul_expression(stream) {
                                                err @ Fail { .. } => break err,
                                                Pass { builder, .. } => {
                                                    result.push(builder);
                                                    let save = stream.position();
                                                    match self.parse_token_with_trivia(
                                                        stream,
                                                        Self::scan_comma,
                                                        TokenKind::Comma,
                                                    ) {
                                                        Fail { error } => {
                                                            stream.set_position(save);
                                                            break Pass {
                                                                builder: cst::NodeBuilder::multiple(
                                                                    result,
                                                                ),
                                                                error: Some(error),
                                                            };
                                                        }
                                                        Pass { builder, .. } => {
                                                            result.push(builder)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } {
                                        Fail { error } => {
                                            stream.set_position(start_position);
                                            Pass {
                                                builder: cst::NodeBuilder::empty(start_position),
                                                error: Some(error),
                                            }
                                        }
                                        pass => pass,
                                    }
                                } {
                                    err @ Fail { .. } => err,
                                    Pass {
                                        builder: expr_node,
                                        error: expr_error,
                                    } => {
                                        match self.parse_token_with_trivia(
                                            stream,
                                            Self::scan_close_paren,
                                            TokenKind::CloseParen,
                                        ) {
                                            Fail { error } => Fail {
                                                error: error.maybe_merge_with(expr_error),
                                            },
                                            Pass {
                                                builder: close_node,
                                                ..
                                            } => Pass {
                                                builder: cst::NodeBuilder::multiple(vec![
                                                    open_node, expr_node, close_node,
                                                ]),
                                                error: None,
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    } {
                        Pass { builder, .. } => Ok(Pratt::Operator {
                            builder,
                            kind: RuleKind::YulFunctionCallExpression,
                            left_binding_power: 1u8,
                            right_binding_power: 255,
                        }),
                        Fail { error } => Err(error),
                    } {
                        Err(_) => {
                            stream.set_position(start_position);
                            break;
                        }
                        Ok(operator) => elements.push(operator),
                    }
                }
                break None;
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
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                            ) = (left, op)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![left, op]).with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        } else if *left_binding_power == 255 {
                            let op = elements.remove(i);
                            let right = elements.remove(i);
                            if let (
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (op, right)
                            {
                                let builder =
                                    cst::NodeBuilder::multiple(vec![op, right]).with_kind(kind);
                                elements.insert(i, Pratt::Builder(builder));
                                i = i.saturating_sub(1);
                            } else {
                                unreachable!()
                            }
                        } else {
                            let left = elements.remove(i - 1);
                            let op = elements.remove(i - 1);
                            let right = elements.remove(i - 1);
                            if let (
                                Pratt::Builder(left),
                                Pratt::Operator {
                                    builder: op, kind, ..
                                },
                                Pratt::Builder(right),
                            ) = (left, op, right)
                            {
                                let builder = cst::NodeBuilder::multiple(vec![left, op, right])
                                    .with_kind(kind);
                                elements.insert(i - 1, Pratt::Builder(builder));
                                i = i.saturating_sub(2);
                            } else {
                                unreachable!()
                            }
                        }
                    } else {
                        i += 1;
                    }
                }
                if let Pratt::Builder(builder) = elements.pop().unwrap() {
                    Pass {
                        builder,
                        error: None,
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_yul_expression(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_expression_0_4_11(stream)
            .with_kind(RuleKind::YulExpression)
    }

    // YulForStatement = «ForKeyword» YulBlock YulExpression YulBlock YulBlock;

    #[allow(dead_code)]
    fn parse_yul_for_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_for_keyword,
                TokenKind::ForKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_yul_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_yul_expression(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_3 = match self.parse_yul_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_4 = match self.parse_yul_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![
                    result_0, result_1, result_2, result_3, result_4,
                ]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_yul_for_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_for_statement_0_4_11(stream)
            .with_kind(RuleKind::YulForStatement)
    }

    // YulFunctionDefinition = «FunctionKeyword» «YulIdentifier» «OpenParen» («YulIdentifier» («Comma» «YulIdentifier»)*)? «CloseParen» («MinusGreaterThan» «YulIdentifier» («Comma» «YulIdentifier»)*)? YulBlock;

    #[allow(dead_code)]
    fn parse_yul_function_definition_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_function_keyword,
                TokenKind::FunctionKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_token_with_trivia(
                stream,
                Self::scan_yul_identifier,
                TokenKind::YulIdentifier,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_open_paren,
                    TokenKind::OpenParen,
                ) {
                    err @ Fail { .. } => err,
                    Pass {
                        builder: open_node, ..
                    } => {
                        match {
                            let start_position = stream.position();
                            match {
                                let mut result = Vec::new();
                                loop {
                                    match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_yul_identifier,
                                        TokenKind::YulIdentifier,
                                    ) {
                                        err @ Fail { .. } => break err,
                                        Pass { builder, .. } => {
                                            result.push(builder);
                                            let save = stream.position();
                                            match self.parse_token_with_trivia(
                                                stream,
                                                Self::scan_comma,
                                                TokenKind::Comma,
                                            ) {
                                                Fail { error } => {
                                                    stream.set_position(save);
                                                    break Pass {
                                                        builder: cst::NodeBuilder::multiple(result),
                                                        error: Some(error),
                                                    };
                                                }
                                                Pass { builder, .. } => result.push(builder),
                                            }
                                        }
                                    }
                                }
                            }
                            .with_kind(RuleKind::Arguments)
                            {
                                Fail { error } => {
                                    stream.set_position(start_position);
                                    Pass {
                                        builder: cst::NodeBuilder::empty(start_position),
                                        error: Some(error),
                                    }
                                }
                                pass => pass,
                            }
                        } {
                            err @ Fail { .. } => err,
                            Pass {
                                builder: expr_node,
                                error: expr_error,
                            } => {
                                match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_close_paren,
                                    TokenKind::CloseParen,
                                ) {
                                    Fail { error } => Fail {
                                        error: error.maybe_merge_with(expr_error),
                                    },
                                    Pass {
                                        builder: close_node,
                                        ..
                                    } => Pass {
                                        builder: cst::NodeBuilder::multiple(vec![
                                            open_node, expr_node, close_node,
                                        ]),
                                        error: None,
                                    },
                                }
                            }
                        }
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                    let result_0 = match self.parse_token_with_trivia(
                        stream,
                        Self::scan_minus_greater_than,
                        TokenKind::MinusGreaterThan,
                    ) {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
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
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_yul_identifier,
                                TokenKind::YulIdentifier,
                            ) {
                                err @ Fail { .. } => break err,
                                Pass { builder, .. } => {
                                    result.push(builder);
                                    let save = stream.position();
                                    match self.parse_token_with_trivia(
                                        stream,
                                        Self::scan_comma,
                                        TokenKind::Comma,
                                    ) {
                                        Fail { error } => {
                                            stream.set_position(save);
                                            break Pass {
                                                builder: cst::NodeBuilder::multiple(result),
                                                error: Some(error),
                                            };
                                        }
                                        Pass { builder, .. } => result.push(builder),
                                    }
                                }
                            }
                        }
                    }
                    .with_kind(RuleKind::Results)
                    {
                        Pass { builder, error } => {
                            furthest_error =
                                error.map(|error| error.maybe_merge_with(furthest_error));
                            builder
                        }
                        Fail { error } => {
                            break Fail {
                                error: error.maybe_merge_with(furthest_error),
                            }
                        }
                    };
                    break Pass {
                        builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                        error: furthest_error,
                    };
                } {
                    Fail { error } => {
                        stream.set_position(start_position);
                        Pass {
                            builder: cst::NodeBuilder::empty(start_position),
                            error: Some(error),
                        }
                    }
                    pass => pass,
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_4 = match self.parse_yul_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![
                    result_0, result_1, result_2, result_3, result_4,
                ]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_yul_function_definition(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_function_definition_0_4_11(stream)
            .with_kind(RuleKind::YulFunctionDefinition)
    }

    // YulIdentifierPath = «YulIdentifier» («Period» «YulIdentifier»)*;

    #[allow(dead_code)]
    fn parse_yul_identifier_path_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut result = Vec::new();
            loop {
                match self.parse_token_with_trivia(
                    stream,
                    Self::scan_yul_identifier,
                    TokenKind::YulIdentifier,
                ) {
                    err @ Fail { .. } => break err,
                    Pass { builder, .. } => {
                        result.push(builder);
                        let save = stream.position();
                        match self.parse_token_with_trivia(
                            stream,
                            Self::scan_period,
                            TokenKind::Period,
                        ) {
                            Fail { error } => {
                                stream.set_position(save);
                                break Pass {
                                    builder: cst::NodeBuilder::multiple(result),
                                    error: Some(error),
                                };
                            }
                            Pass { builder, .. } => result.push(builder),
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn parse_yul_identifier_path(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_identifier_path_0_4_11(stream)
            .with_kind(RuleKind::YulIdentifierPath)
    }

    // YulIfStatement = «IfKeyword» YulExpression YulBlock;

    #[allow(dead_code)]
    fn parse_yul_if_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_if_keyword,
                TokenKind::IfKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_yul_expression(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_2 = match self.parse_yul_block(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_yul_if_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_if_statement_0_4_11(stream)
            .with_kind(RuleKind::YulIfStatement)
    }

    // (* v0.6.0 *)
    // YulLeaveStatement = «LeaveKeyword»;

    #[allow(dead_code)]
    fn parse_yul_leave_statement_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, Self::scan_leave_keyword, TokenKind::LeaveKeyword)
    }

    fn dispatch_parse_yul_leave_statement(&self, stream: &mut Stream) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.parse_yul_leave_statement_0_6_0(stream))
        } else {
            None
        }
    }

    pub(crate) fn maybe_parse_yul_leave_statement(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        self.dispatch_parse_yul_leave_statement(stream)
            .map(|body| body.with_kind(RuleKind::YulLeaveStatement))
    }

    #[inline]
    pub(crate) fn parse_yul_leave_statement(&self, stream: &mut Stream) -> ParserResult {
        self.maybe_parse_yul_leave_statement(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // YulLiteral = BooleanLiteral
    //            | «YulHexLiteral»
    //            | «YulDecimalLiteral»
    //            | «HexStringLiteral»
    //            | «AsciiStringLiteral»;

    #[allow(dead_code)]
    fn parse_yul_literal_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_boolean_literal(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_yul_hex_literal,
                TokenKind::YulHexLiteral,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_yul_decimal_literal,
                TokenKind::YulDecimalLiteral,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_hex_string_literal,
                TokenKind::HexStringLiteral,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_token_with_trivia(
                stream,
                Self::scan_ascii_string_literal,
                TokenKind::AsciiStringLiteral,
            ) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_yul_literal(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_literal_0_4_11(stream)
            .with_kind(RuleKind::YulLiteral)
    }

    // (* v0.4.11 *)
    // YulStatement = YulBlock
    //              | YulFunctionDefinition
    //              | YulDeclarationStatement
    //              | YulAssignmentStatement
    //              | YulIfStatement
    //              | YulForStatement
    //              | YulSwitchStatement
    //              | YulBreakStatement
    //              | YulContinueStatement
    //              | YulExpression;

    #[allow(dead_code)]
    fn parse_yul_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_yul_block(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_yul_function_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_yul_declaration_statement(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_yul_assignment_statement(stream) {
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
            match self.parse_yul_break_statement(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_yul_continue_statement(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_yul_expression(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    // (* v0.6.0 *)
    // YulStatement = YulBlock
    //              | YulFunctionDefinition
    //              | YulDeclarationStatement
    //              | YulAssignmentStatement
    //              | YulIfStatement
    //              | YulForStatement
    //              | YulSwitchStatement
    //              | YulLeaveStatement
    //              | YulBreakStatement
    //              | YulContinueStatement
    //              | YulExpression;

    #[allow(dead_code)]
    fn parse_yul_statement_0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let start_position = stream.position();
            let mut furthest_error;
            match self.parse_yul_block(stream) {
                Fail { error } => furthest_error = error,
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_yul_function_definition(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_yul_declaration_statement(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            stream.set_position(start_position);
            match self.parse_yul_assignment_statement(stream) {
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
            stream.set_position(start_position);
            match self.parse_yul_expression(stream) {
                Fail { error } => furthest_error.merge_with(error),
                pass => break pass,
            }
            break Fail {
                error: furthest_error,
            };
        }
    }

    fn dispatch_parse_yul_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.parse_yul_statement_0_6_0(stream)
        } else {
            self.parse_yul_statement_0_4_11(stream)
        }
    }

    #[inline]
    pub(crate) fn parse_yul_statement(&self, stream: &mut Stream) -> ParserResult {
        self.dispatch_parse_yul_statement(stream)
            .with_kind(RuleKind::YulStatement)
    }

    // YulSwitchStatement = «SwitchKeyword» YulExpression (((«CaseKeyword» YulLiteral) | «DefaultKeyword») YulBlock)+;

    #[allow(dead_code)]
    fn parse_yul_switch_statement_0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut furthest_error = None;
            let result_0 = match self.parse_token_with_trivia(
                stream,
                Self::scan_switch_keyword,
                TokenKind::SwitchKeyword,
            ) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            let result_1 = match self.parse_yul_expression(stream) {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
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
                                let result_0 = match self.parse_token_with_trivia(
                                    stream,
                                    Self::scan_case_keyword,
                                    TokenKind::CaseKeyword,
                                ) {
                                    Pass { builder, error } => {
                                        furthest_error = error
                                            .map(|error| error.maybe_merge_with(furthest_error));
                                        builder
                                    }
                                    Fail { error } => {
                                        break Fail {
                                            error: error.maybe_merge_with(furthest_error),
                                        }
                                    }
                                };
                                let result_1 = match self.parse_yul_literal(stream) {
                                    Pass { builder, error } => {
                                        furthest_error = error
                                            .map(|error| error.maybe_merge_with(furthest_error));
                                        builder
                                    }
                                    Fail { error } => {
                                        break Fail {
                                            error: error.maybe_merge_with(furthest_error),
                                        }
                                    }
                                };
                                break Pass {
                                    builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                                    error: furthest_error,
                                };
                            } {
                                Fail { error } => furthest_error = error,
                                pass => break pass,
                            }
                            stream.set_position(start_position);
                            match self.parse_token_with_trivia(
                                stream,
                                Self::scan_default_keyword,
                                TokenKind::DefaultKeyword,
                            ) {
                                Fail { error } => furthest_error.merge_with(error),
                                pass => break pass,
                            }
                            break Fail {
                                error: furthest_error,
                            };
                        } {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        let result_1 = match self.parse_yul_block(stream) {
                            Pass { builder, error } => {
                                furthest_error =
                                    error.map(|error| error.maybe_merge_with(furthest_error));
                                builder
                            }
                            Fail { error } => {
                                break Fail {
                                    error: error.maybe_merge_with(furthest_error),
                                }
                            }
                        };
                        break Pass {
                            builder: cst::NodeBuilder::multiple(vec![result_0, result_1]),
                            error: furthest_error,
                        };
                    } {
                        Fail { error } => {
                            if result.is_empty() {
                                break Fail { error };
                            }
                            stream.set_position(start_position);
                            break Pass {
                                builder: cst::NodeBuilder::multiple(result),
                                error: Some(error),
                            };
                        }
                        Pass { builder, .. } => result.push(builder),
                    }
                }
            } {
                Pass { builder, error } => {
                    furthest_error = error.map(|error| error.maybe_merge_with(furthest_error));
                    builder
                }
                Fail { error } => {
                    break Fail {
                        error: error.maybe_merge_with(furthest_error),
                    }
                }
            };
            break Pass {
                builder: cst::NodeBuilder::multiple(vec![result_0, result_1, result_2]),
                error: furthest_error,
            };
        }
    }

    #[inline]
    pub(crate) fn parse_yul_switch_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_yul_switch_statement_0_4_11(stream)
            .with_kind(RuleKind::YulSwitchStatement)
    }
}
