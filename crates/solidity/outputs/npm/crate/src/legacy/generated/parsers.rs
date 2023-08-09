// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::cst;
use super::language::Language;
use super::parser_helpers::*;
use super::parser_result::*;
use super::stream::*;

use crate::syntax::nodes::{RuleKind, TokenKind};

impl Language {
    fn parse_token_with_trivia<F>(
        &self,
        stream: &mut Stream,
        scanner: F,
        kind: TokenKind,
    ) -> ParserResult
    where
        F: Fn(&Self, &mut Stream) -> bool,
    {
        let mut children = vec![];
        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.leading_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }
        let start = stream.position();
        if !scanner(self, stream) {
            stream.set_position(restore);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();
        children.push(cst::Node::token(kind, stream.content(start.utf8..end.utf8)));
        let restore = stream.position();
        if let ParserResult::Match(r#match) = self.trailing_trivia(stream) {
            children.extend(r#match.nodes);
        } else {
            stream.set_position(restore);
        }
        return ParserResult::r#match(children, vec![]);
    }
    fn parse_token<F>(&self, stream: &mut Stream, scanner: F, kind: TokenKind) -> ParserResult
    where
        F: Fn(&Self, &mut Stream) -> bool,
    {
        let start = stream.position();
        if !scanner(self, stream) {
            stream.set_position(start);
            return ParserResult::no_match(vec![kind]);
        }
        let end = stream.position();
        return ParserResult::r#match(
            vec![cst::Node::token(kind, stream.content(start.utf8..end.utf8))],
            vec![],
        );
    }

    // ABICoderPragma = ABICODER_KEYWORD IDENTIFIER;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn abi_coder_pragma(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::abicoder_keyword,
                    TokenKind::AbicoderKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ABICoderPragma)
    }

    // «AddSubOperator» = PLUS | MINUS;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn add_sub_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::plus,
                    TokenKind::Plus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus,
                    TokenKind::Minus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // AddressType = (ADDRESS_KEYWORD PAYABLE_KEYWORD?) | PAYABLE_KEYWORD;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn address_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::address_keyword,
                                TokenKind::AddressKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(transform_option_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::payable_keyword,
                                TokenKind::PayableKeyword,
                            ),
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::payable_keyword,
                    TokenKind::PayableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
        .with_kind(RuleKind::AddressType)
    }

    // «AndOperator» = AMPERSAND_AMPERSAND;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn and_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            &Self::ampersand_ampersand,
            TokenKind::AmpersandAmpersand,
        )
    }

    // ArgumentsDeclaration = OPEN_PAREN (PositionalArgumentsList | NamedArgumentsDeclaration)? CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn arguments_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_paren,
                    TokenKind::OpenParen,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result
                            .incorporate_choice_result(self.positional_arguments_list(stream))
                        {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result
                            .incorporate_choice_result(self.named_arguments_declaration(stream))
                        {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                })) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_paren,
                    TokenKind::CloseParen,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ArgumentsDeclaration)
    }

    // ArrayExpression = OPEN_BRACKET ArrayValuesList CLOSE_BRACKET;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn array_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_bracket,
                    TokenKind::OpenBracket,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.array_values_list(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_bracket,
                    TokenKind::CloseBracket,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ArrayExpression)
    }

    // «ArrayTypeNameOperator» = OPEN_BRACKET Expression? CLOSE_BRACKET;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn array_type_name_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_bracket,
                    TokenKind::OpenBracket,
                )) {
                    break;
                }
                if !running_result
                    .incorporate_sequence_result(transform_option_result(self.expression(stream)))
                {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_bracket,
                    TokenKind::CloseBracket,
                ));
                break;
            }
            running_result
        }
    }

    // ArrayValuesList = Expression (COMMA Expression)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn array_values_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.expression(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(self.expression(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ArrayValuesList)
    }

    // AsciiStringLiteralsList = ASCII_STRING_LITERAL+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn ascii_string_literals_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.parse_token_with_trivia(
                stream,
                &Self::ascii_string_literal,
                TokenKind::AsciiStringLiteral,
            )) {}
            running_result
        }
        .with_kind(RuleKind::AsciiStringLiteralsList)
    }

    // AssemblyFlagsList = ASCII_STRING_LITERAL (COMMA ASCII_STRING_LITERAL)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn assembly_flags_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ascii_string_literal,
                    TokenKind::AsciiStringLiteral,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::ascii_string_literal,
                                    TokenKind::AsciiStringLiteral,
                                ),
                            );
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::AssemblyFlagsList)
    }

    // AssemblyStatement = ASSEMBLY_KEYWORD ASCII_STRING_LITERAL? (OPEN_PAREN AssemblyFlagsList CLOSE_PAREN)? YulBlock;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn assembly_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::assembly_keyword,
                    TokenKind::AssemblyKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token_with_trivia(
                        stream,
                        &Self::ascii_string_literal,
                        TokenKind::AsciiStringLiteral,
                    ),
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_paren,
                                TokenKind::OpenParen,
                            ),
                        ) {
                            break;
                        }
                        if !running_result
                            .incorporate_sequence_result(self.assembly_flags_list(stream))
                        {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_paren,
                            TokenKind::CloseParen,
                        ));
                        break;
                    }
                    running_result
                })) {
                    break;
                }
                running_result.incorporate_sequence_result(self.yul_block(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::AssemblyStatement)
    }

    // «AssignmentOperator» = EQUAL
    //                      | BAR_EQUAL
    //                      | PLUS_EQUAL
    //                      | MINUS_EQUAL
    //                      | CARET_EQUAL
    //                      | SLASH_EQUAL
    //                      | PERCENT_EQUAL
    //                      | ASTERISK_EQUAL
    //                      | AMPERSAND_EQUAL
    //                      | LESS_THAN_LESS_THAN_EQUAL
    //                      | GREATER_THAN_GREATER_THAN_EQUAL
    //                      | GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn assignment_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::equal,
                    TokenKind::Equal,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::bar_equal,
                    TokenKind::BarEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::plus_equal,
                    TokenKind::PlusEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus_equal,
                    TokenKind::MinusEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::caret_equal,
                    TokenKind::CaretEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::slash_equal,
                    TokenKind::SlashEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::percent_equal,
                    TokenKind::PercentEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::asterisk_equal,
                    TokenKind::AsteriskEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ampersand_equal,
                    TokenKind::AmpersandEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::less_than_less_than_equal,
                    TokenKind::LessThanLessThanEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than_greater_than_equal,
                    TokenKind::GreaterThanGreaterThanEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than_greater_than_greater_than_equal,
                    TokenKind::GreaterThanGreaterThanGreaterThanEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // «BitwiseAndOperator» = AMPERSAND;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bitwise_and_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::ampersand, TokenKind::Ampersand)
    }

    // «BitwiseOrOperator» = BAR;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bitwise_or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::bar, TokenKind::Bar)
    }

    // «BitwiseXOrOperator» = CARET;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bitwise_x_or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::caret, TokenKind::Caret)
    }

    // Block = OPEN_BRACE StatementsList? CLOSE_BRACE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn block(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_brace,
                    TokenKind::OpenBrace,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.statements_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_brace,
                    TokenKind::CloseBrace,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::Block)
    }

    // «BooleanExpression» = TRUE_KEYWORD | FALSE_KEYWORD;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn boolean_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::true_keyword,
                    TokenKind::TrueKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::false_keyword,
                    TokenKind::FalseKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // BreakStatement = BREAK_KEYWORD SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn break_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::break_keyword,
                    TokenKind::BreakKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::BreakStatement)
    }

    // (* v0.6.0 *)
    // CatchClause = CATCH_KEYWORD CatchClauseError? Block;

    #[allow(dead_code, non_snake_case)]
    fn catch_clause__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::catch_keyword,
                    TokenKind::CatchKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.catch_clause_error(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.block(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::CatchClause)
    }

    #[allow(non_snake_case)]
    pub(crate) fn catch_clause__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.catch_clause__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn catch_clause(&self, stream: &mut Stream) -> ParserResult {
        self.catch_clause__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.6.0 *)
    // CatchClauseError = IDENTIFIER? ParametersDeclaration;

    #[allow(dead_code, non_snake_case)]
    fn catch_clause_error__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token_with_trivia(stream, &Self::identifier, TokenKind::Identifier),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parameters_declaration(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::CatchClauseError)
    }

    #[allow(non_snake_case)]
    pub(crate) fn catch_clause_error__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.catch_clause_error__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn catch_clause_error(&self, stream: &mut Stream) -> ParserResult {
        self.catch_clause_error__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.6.0 *)
    // CatchClausesList = CatchClause+;

    #[allow(dead_code, non_snake_case)]
    fn catch_clauses_list__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.catch_clause(stream)) {}
            running_result
        }
        .with_kind(RuleKind::CatchClausesList)
    }

    #[allow(non_snake_case)]
    pub(crate) fn catch_clauses_list__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.catch_clauses_list__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn catch_clauses_list(&self, stream: &mut Stream) -> ParserResult {
        self.catch_clauses_list__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «ConditionalOperator» = QUESTION_MARK Expression COLON Expression;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn conditional_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::question_mark,
                    TokenKind::QuestionMark,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.expression(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::colon,
                    TokenKind::Colon,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.expression(stream));
                break;
            }
            running_result
        }
    }

    // (* v0.7.4 *)
    // ConstantDefinition = TypeName CONSTANT_KEYWORD IDENTIFIER EQUAL Expression SEMICOLON;

    #[allow(dead_code, non_snake_case)]
    fn constant_definition__0_7_4(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::constant_keyword,
                                TokenKind::ConstantKeyword,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::identifier,
                                TokenKind::Identifier,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(stream, &Self::equal, TokenKind::Equal),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.expression(stream));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ConstantDefinition)
    }

    #[allow(non_snake_case)]
    pub(crate) fn constant_definition__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_7_4 {
            Some(self.constant_definition__0_7_4(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn constant_definition(&self, stream: &mut Stream) -> ParserResult {
        self.constant_definition__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.22 *)
    // «ConstructorAttribute» = ModifierInvocation
    //                        | INTERNAL_KEYWORD
    //                        | PAYABLE_KEYWORD
    //                        | PUBLIC_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn constructor_attribute__0_4_22(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.modifier_invocation(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::internal_keyword,
                    TokenKind::InternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::payable_keyword,
                    TokenKind::PayableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn constructor_attribute__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_4_22 {
            Some(self.constructor_attribute__0_4_22(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn constructor_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.constructor_attribute__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.22 *)
    // ConstructorAttributesList = «ConstructorAttribute»+;

    #[allow(dead_code, non_snake_case)]
    fn constructor_attributes_list__0_4_22(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.constructor_attribute(stream))
            {
            }
            running_result
        }
        .with_kind(RuleKind::ConstructorAttributesList)
    }

    #[allow(non_snake_case)]
    pub(crate) fn constructor_attributes_list__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_4_22 {
            Some(self.constructor_attributes_list__0_4_22(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn constructor_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        self.constructor_attributes_list__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.22 *)
    // ConstructorDefinition = CONSTRUCTOR_KEYWORD ParametersDeclaration ConstructorAttributesList? Block;

    #[allow(dead_code, non_snake_case)]
    fn constructor_definition__0_4_22(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::constructor_keyword,
                    TokenKind::ConstructorKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parameters_declaration(stream))
                {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.constructor_attributes_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.block(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ConstructorDefinition)
    }

    #[allow(non_snake_case)]
    pub(crate) fn constructor_definition__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_4_22 {
            Some(self.constructor_definition__0_4_22(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn constructor_definition(&self, stream: &mut Stream) -> ParserResult {
        self.constructor_definition__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // ContinueStatement = CONTINUE_KEYWORD SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn continue_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::continue_keyword,
                    TokenKind::ContinueKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ContinueStatement)
    }

    // (* v0.4.11 *)
    // ContractDefinition = CONTRACT_KEYWORD IDENTIFIER InheritanceSpecifier? OPEN_BRACE ContractMembersList? CLOSE_BRACE;

    #[allow(dead_code, non_snake_case)]
    fn contract_definition__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::contract_keyword,
                    TokenKind::ContractKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.inheritance_specifier(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_brace,
                                TokenKind::OpenBrace,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.contract_members_list(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_brace,
                            TokenKind::CloseBrace,
                        ));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ContractDefinition)
    }

    // (* v0.6.0 *)
    // ContractDefinition = ABSTRACT_KEYWORD? CONTRACT_KEYWORD IDENTIFIER InheritanceSpecifier? OPEN_BRACE ContractMembersList? CLOSE_BRACE;

    #[allow(dead_code, non_snake_case)]
    fn contract_definition__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token_with_trivia(
                        stream,
                        &Self::abstract_keyword,
                        TokenKind::AbstractKeyword,
                    ),
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::contract_keyword,
                    TokenKind::ContractKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.inheritance_specifier(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_brace,
                                TokenKind::OpenBrace,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.contract_members_list(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_brace,
                            TokenKind::CloseBrace,
                        ));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ContractDefinition)
    }

    pub(crate) fn contract_definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.contract_definition__0_6_0(stream)
        } else {
            self.contract_definition__0_4_11(stream)
        }
    }

    // (* v0.4.11 *)
    // «ContractMember» = UsingDirective
    //                  | FunctionDefinition
    //                  | UnnamedFunctionDefinition
    //                  | ModifierDefinition
    //                  | StructDefinition
    //                  | EnumDefinition
    //                  | EventDefinition
    //                  | StateVariableDefinition;

    #[allow(dead_code, non_snake_case)]
    fn contract_member__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.using_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.unnamed_function_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.modifier_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.event_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.state_variable_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.4.22 *)
    // «ContractMember» = UsingDirective
    //                  | ConstructorDefinition
    //                  | FunctionDefinition
    //                  | UnnamedFunctionDefinition
    //                  | ModifierDefinition
    //                  | StructDefinition
    //                  | EnumDefinition
    //                  | EventDefinition
    //                  | StateVariableDefinition;

    #[allow(dead_code, non_snake_case)]
    fn contract_member__0_4_22(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.using_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.constructor_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.unnamed_function_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.modifier_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.event_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.state_variable_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.6.0 *)
    // «ContractMember» = UsingDirective
    //                  | ConstructorDefinition
    //                  | FunctionDefinition
    //                  | FallbackFunctionDefinition
    //                  | ReceiveFunctionDefinition
    //                  | ModifierDefinition
    //                  | StructDefinition
    //                  | EnumDefinition
    //                  | EventDefinition
    //                  | StateVariableDefinition;

    #[allow(dead_code, non_snake_case)]
    fn contract_member__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.using_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.constructor_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.fallback_function_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.receive_function_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.modifier_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.event_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.state_variable_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.8.4 *)
    // «ContractMember» = UsingDirective
    //                  | ConstructorDefinition
    //                  | FunctionDefinition
    //                  | FallbackFunctionDefinition
    //                  | ReceiveFunctionDefinition
    //                  | ModifierDefinition
    //                  | StructDefinition
    //                  | EnumDefinition
    //                  | EventDefinition
    //                  | ErrorDefinition
    //                  | StateVariableDefinition;

    #[allow(dead_code, non_snake_case)]
    fn contract_member__0_8_4(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.using_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.constructor_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.fallback_function_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.receive_function_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.modifier_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.event_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.error_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.state_variable_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.8.8 *)
    // «ContractMember» = UsingDirective
    //                  | ConstructorDefinition
    //                  | FunctionDefinition
    //                  | FallbackFunctionDefinition
    //                  | ReceiveFunctionDefinition
    //                  | ModifierDefinition
    //                  | StructDefinition
    //                  | EnumDefinition
    //                  | EventDefinition
    //                  | ErrorDefinition
    //                  | StateVariableDefinition
    //                  | UserDefinedValueTypeDefinition;

    #[allow(dead_code, non_snake_case)]
    fn contract_member__0_8_8(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.using_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.constructor_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.fallback_function_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.receive_function_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.modifier_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.event_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.error_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.state_variable_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.user_defined_value_type_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn contract_member(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_8 {
            self.contract_member__0_8_8(stream)
        } else if self.version_is_equal_to_or_greater_than_0_8_4 {
            self.contract_member__0_8_4(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.contract_member__0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_4_22 {
            self.contract_member__0_4_22(stream)
        } else {
            self.contract_member__0_4_11(stream)
        }
    }

    // ContractMembersList = «ContractMember»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn contract_members_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.contract_member(stream)) {}
            running_result
        }
        .with_kind(RuleKind::ContractMembersList)
    }

    // (* v0.4.11 *)
    // «ControlStatement» = IfStatement
    //                    | ForStatement
    //                    | WhileStatement
    //                    | DoWhileStatement
    //                    | ContinueStatement
    //                    | BreakStatement
    //                    | DeleteStatement
    //                    | ReturnStatement
    //                    | RevertStatement
    //                    | ThrowStatement;

    #[allow(dead_code, non_snake_case)]
    fn control_statement__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.if_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.for_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.while_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.do_while_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.continue_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.break_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.delete_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.return_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.revert_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.throw_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.4.21 *)
    // «ControlStatement» = IfStatement
    //                    | ForStatement
    //                    | WhileStatement
    //                    | DoWhileStatement
    //                    | ContinueStatement
    //                    | BreakStatement
    //                    | DeleteStatement
    //                    | ReturnStatement
    //                    | RevertStatement
    //                    | ThrowStatement
    //                    | EmitStatement;

    #[allow(dead_code, non_snake_case)]
    fn control_statement__0_4_21(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.if_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.for_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.while_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.do_while_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.continue_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.break_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.delete_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.return_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.revert_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.throw_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.emit_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.5.0 *)
    // «ControlStatement» = IfStatement
    //                    | ForStatement
    //                    | WhileStatement
    //                    | DoWhileStatement
    //                    | ContinueStatement
    //                    | BreakStatement
    //                    | DeleteStatement
    //                    | ReturnStatement
    //                    | RevertStatement
    //                    | EmitStatement;

    #[allow(dead_code, non_snake_case)]
    fn control_statement__0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.if_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.for_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.while_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.do_while_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.continue_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.break_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.delete_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.return_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.revert_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.emit_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.6.0 *)
    // «ControlStatement» = IfStatement
    //                    | ForStatement
    //                    | WhileStatement
    //                    | DoWhileStatement
    //                    | ContinueStatement
    //                    | BreakStatement
    //                    | DeleteStatement
    //                    | ReturnStatement
    //                    | RevertStatement
    //                    | EmitStatement
    //                    | TryStatement;

    #[allow(dead_code, non_snake_case)]
    fn control_statement__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.if_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.for_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.while_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.do_while_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.continue_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.break_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.delete_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.return_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.revert_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.emit_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.try_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn control_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.control_statement__0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.control_statement__0_5_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_4_21 {
            self.control_statement__0_4_21(stream)
        } else {
            self.control_statement__0_4_11(stream)
        }
    }

    // (* v0.4.11 *)
    // «DataLocation» = MEMORY_KEYWORD | STORAGE_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn data_location__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::memory_keyword,
                    TokenKind::MemoryKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::storage_keyword,
                    TokenKind::StorageKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.5.0 *)
    // «DataLocation» = MEMORY_KEYWORD | STORAGE_KEYWORD | CALLDATA_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn data_location__0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::memory_keyword,
                    TokenKind::MemoryKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::storage_keyword,
                    TokenKind::StorageKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::calldata_keyword,
                    TokenKind::CalldataKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn data_location(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.data_location__0_5_0(stream)
        } else {
            self.data_location__0_4_11(stream)
        }
    }

    // DeconstructionImport = OPEN_BRACE DeconstructionImportSymbolsList CLOSE_BRACE FROM_KEYWORD ASCII_STRING_LITERAL;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn deconstruction_import(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_brace,
                                TokenKind::OpenBrace,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.deconstruction_import_symbols_list(stream),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_brace,
                            TokenKind::CloseBrace,
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::from_keyword,
                    TokenKind::FromKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ascii_string_literal,
                    TokenKind::AsciiStringLiteral,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::DeconstructionImport)
    }

    // DeconstructionImportSymbol = IDENTIFIER (AS_KEYWORD IDENTIFIER)?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn deconstruction_import_symbol(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::as_keyword,
                                TokenKind::AsKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::identifier,
                            TokenKind::Identifier,
                        ));
                        break;
                    }
                    running_result
                }));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::DeconstructionImportSymbol)
    }

    // DeconstructionImportSymbolsList = DeconstructionImportSymbol (COMMA DeconstructionImportSymbol)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn deconstruction_import_symbols_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result
                    .incorporate_sequence_result(self.deconstruction_import_symbol(stream))
                {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(
                                self.deconstruction_import_symbol(stream),
                            );
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::DeconstructionImportSymbolsList)
    }

    // DeleteStatement = DELETE_KEYWORD Expression SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn delete_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::delete_keyword,
                                TokenKind::DeleteKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.expression(stream));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::DeleteStatement)
    }

    // DoWhileStatement = DO_KEYWORD Statement WHILE_KEYWORD OPEN_PAREN Expression CLOSE_PAREN SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn do_while_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::do_keyword,
                                TokenKind::DoKeyword,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(self.statement(stream)) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::while_keyword,
                                TokenKind::WhileKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::open_paren,
                                        TokenKind::OpenParen,
                                    ),
                                ) {
                                    break;
                                }
                                if !running_result
                                    .incorporate_sequence_result(self.expression(stream))
                                {
                                    break;
                                }
                                running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::close_paren,
                                        TokenKind::CloseParen,
                                    ),
                                );
                                break;
                            }
                            running_result
                        });
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::DoWhileStatement)
    }

    // (* v0.4.11 *)
    // «ElementaryType» = BOOL_KEYWORD
    //                  | BYTE_KEYWORD
    //                  | STRING_KEYWORD
    //                  | AddressType
    //                  | FIXED_BYTES_TYPE
    //                  | SIGNED_INTEGER_TYPE
    //                  | UNSIGNED_INTEGER_TYPE
    //                  | SIGNED_FIXED_TYPE
    //                  | UNSIGNED_FIXED_TYPE;

    #[allow(dead_code, non_snake_case)]
    fn elementary_type__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::bool_keyword,
                    TokenKind::BoolKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::byte_keyword,
                    TokenKind::ByteKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::string_keyword,
                    TokenKind::StringKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.address_type(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::fixed_bytes_type,
                    TokenKind::FixedBytesType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::signed_integer_type,
                    TokenKind::SignedIntegerType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::unsigned_integer_type,
                    TokenKind::UnsignedIntegerType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::signed_fixed_type,
                    TokenKind::SignedFixedType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::unsigned_fixed_type,
                    TokenKind::UnsignedFixedType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.8.0 *)
    // «ElementaryType» = BOOL_KEYWORD
    //                  | STRING_KEYWORD
    //                  | AddressType
    //                  | FIXED_BYTES_TYPE
    //                  | SIGNED_INTEGER_TYPE
    //                  | UNSIGNED_INTEGER_TYPE
    //                  | SIGNED_FIXED_TYPE
    //                  | UNSIGNED_FIXED_TYPE;

    #[allow(dead_code, non_snake_case)]
    fn elementary_type__0_8_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::bool_keyword,
                    TokenKind::BoolKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::string_keyword,
                    TokenKind::StringKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.address_type(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::fixed_bytes_type,
                    TokenKind::FixedBytesType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::signed_integer_type,
                    TokenKind::SignedIntegerType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::unsigned_integer_type,
                    TokenKind::UnsignedIntegerType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::signed_fixed_type,
                    TokenKind::SignedFixedType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::unsigned_fixed_type,
                    TokenKind::UnsignedFixedType,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn elementary_type(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            self.elementary_type__0_8_0(stream)
        } else {
            self.elementary_type__0_4_11(stream)
        }
    }

    // (* v0.4.21 *)
    // EmitStatement = EMIT_KEYWORD IdentifierPath ArgumentsDeclaration SEMICOLON;

    #[allow(dead_code, non_snake_case)]
    fn emit_statement__0_4_21(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::emit_keyword,
                                TokenKind::EmitKeyword,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(self.identifier_path(stream))
                        {
                            break;
                        }
                        running_result
                            .incorporate_sequence_result(self.arguments_declaration(stream));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::EmitStatement)
    }

    #[allow(non_snake_case)]
    pub(crate) fn emit_statement__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_4_21 {
            Some(self.emit_statement__0_4_21(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn emit_statement(&self, stream: &mut Stream) -> ParserResult {
        self.emit_statement__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // EndOfFileTrivia = (WHITESPACE | END_OF_LINE | MULTILINE_COMMENT | SINGLE_LINE_COMMENT)+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn end_of_file_trivia(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result({
                let mut running_result = ParserResult::no_match(vec![]);
                let start_position = stream.position();
                loop {
                    if running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::whitespace,
                        TokenKind::Whitespace,
                    )) {
                        break;
                    }
                    stream.set_position(start_position);
                    if running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::end_of_line,
                        TokenKind::EndOfLine,
                    )) {
                        break;
                    }
                    stream.set_position(start_position);
                    if running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::multiline_comment,
                        TokenKind::MultilineComment,
                    )) {
                        break;
                    }
                    stream.set_position(start_position);
                    if running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::single_line_comment,
                        TokenKind::SingleLineComment,
                    )) {
                        break;
                    }
                    stream.set_position(start_position);
                    break;
                }
                if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                    incomplete_match.consume_stream(stream);
                }
                running_result
            }) {}
            running_result
        }
        .with_kind(RuleKind::EndOfFileTrivia)
    }

    // EnumDefinition = ENUM_KEYWORD IDENTIFIER OPEN_BRACE IdentifiersList? CLOSE_BRACE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn enum_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::enum_keyword,
                    TokenKind::EnumKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_brace,
                                TokenKind::OpenBrace,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.identifiers_list(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_brace,
                            TokenKind::CloseBrace,
                        ));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::EnumDefinition)
    }

    // «EqualityComparisonOperator» = EQUAL_EQUAL | BANG_EQUAL;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn equality_comparison_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::equal_equal,
                    TokenKind::EqualEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::bang_equal,
                    TokenKind::BangEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.8.4 *)
    // ErrorDefinition = ERROR_KEYWORD IDENTIFIER OPEN_PAREN ErrorParametersList? CLOSE_PAREN SEMICOLON;

    #[allow(dead_code, non_snake_case)]
    fn error_definition__0_8_4(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::error_keyword,
                                TokenKind::ErrorKeyword,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::identifier,
                                TokenKind::Identifier,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::open_paren,
                                        TokenKind::OpenParen,
                                    ),
                                ) {
                                    break;
                                }
                                if !running_result.incorporate_sequence_result(
                                    transform_option_result(self.error_parameters_list(stream)),
                                ) {
                                    break;
                                }
                                running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::close_paren,
                                        TokenKind::CloseParen,
                                    ),
                                );
                                break;
                            }
                            running_result
                        });
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ErrorDefinition)
    }

    #[allow(non_snake_case)]
    pub(crate) fn error_definition__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_4 {
            Some(self.error_definition__0_8_4(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn error_definition(&self, stream: &mut Stream) -> ParserResult {
        self.error_definition__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.8.4 *)
    // ErrorParameter = TypeName IDENTIFIER?;

    #[allow(dead_code, non_snake_case)]
    fn error_parameter__0_8_4(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token_with_trivia(stream, &Self::identifier, TokenKind::Identifier),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ErrorParameter)
    }

    #[allow(non_snake_case)]
    pub(crate) fn error_parameter__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_4 {
            Some(self.error_parameter__0_8_4(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn error_parameter(&self, stream: &mut Stream) -> ParserResult {
        self.error_parameter__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.8.4 *)
    // ErrorParametersList = ErrorParameter (COMMA ErrorParameter)*;

    #[allow(dead_code, non_snake_case)]
    fn error_parameters_list__0_8_4(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.error_parameter(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result
                                .incorporate_sequence_result(self.error_parameter(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ErrorParametersList)
    }

    #[allow(non_snake_case)]
    pub(crate) fn error_parameters_list__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_4 {
            Some(self.error_parameters_list__0_8_4(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn error_parameters_list(&self, stream: &mut Stream) -> ParserResult {
        self.error_parameters_list__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // EventDefinition = EVENT_KEYWORD IDENTIFIER OPEN_PAREN EventParametersList? CLOSE_PAREN ANONYMOUS_KEYWORD? SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn event_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::event_keyword,
                                TokenKind::EventKeyword,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::identifier,
                                TokenKind::Identifier,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::open_paren,
                                        TokenKind::OpenParen,
                                    ),
                                ) {
                                    break;
                                }
                                if !running_result.incorporate_sequence_result(
                                    transform_option_result(self.event_parameters_list(stream)),
                                ) {
                                    break;
                                }
                                running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::close_paren,
                                        TokenKind::CloseParen,
                                    ),
                                );
                                break;
                            }
                            running_result
                        }) {
                            break;
                        }
                        running_result.incorporate_sequence_result(transform_option_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::anonymous_keyword,
                                TokenKind::AnonymousKeyword,
                            ),
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::EventDefinition)
    }

    // EventParameter = TypeName INDEXED_KEYWORD? IDENTIFIER?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn event_parameter(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token_with_trivia(
                        stream,
                        &Self::indexed_keyword,
                        TokenKind::IndexedKeyword,
                    ),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token_with_trivia(stream, &Self::identifier, TokenKind::Identifier),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::EventParameter)
    }

    // EventParametersList = EventParameter (COMMA EventParameter)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn event_parameters_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.event_parameter(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result
                                .incorporate_sequence_result(self.event_parameter(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::EventParametersList)
    }

    // ExperimentalPragma = EXPERIMENTAL_KEYWORD (ASCII_STRING_LITERAL | IDENTIFIER);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn experimental_pragma(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::experimental_keyword,
                    TokenKind::ExperimentalKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::ascii_string_literal,
                            TokenKind::AsciiStringLiteral,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::identifier,
                            TokenKind::Identifier,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ExperimentalPragma)
    }

    // «ExponentiationOperator» = ASTERISK_ASTERISK;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn exponentiation_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            &Self::asterisk_asterisk,
            TokenKind::AsteriskAsterisk,
        )
    }

    // (* v0.4.11 *)
    // Expression = BinaryExpression (* Expression «AssignmentOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | ConditionalExpression (* Expression «ConditionalOperator» *) (* Unary Operator, Postfix *)
    //            | BinaryExpression (* Expression «OrOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «AndOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «EqualityComparisonOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «OrderComparisonOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «BitwiseOrOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «BitwiseXOrOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «BitwiseAndOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «ShiftOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «AddSubOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «MulDivModOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «ExponentiationOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | UnaryPostfixExpression (* Expression «UnaryPostfixOperator» *) (* Unary Operator, Postfix *)
    //            | UnaryPrefixExpression (* «UnaryPrefixOperator» Expression *) (* Unary Operator, Prefix *)
    //            | FunctionCallExpression (* Expression «FunctionCallOperator» *) (* Unary Operator, Postfix *)
    //            | MemberAccessExpression (* Expression «MemberAccessOperator» *) (* Unary Operator, Postfix *)
    //            | IndexAccessExpression (* Expression «IndexAccessOperator» *) (* Unary Operator, Postfix *)
    //            | «PrimaryExpression»;
    // BinaryExpression = Expression «AssignmentOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «OrOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «AndOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «EqualityComparisonOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «OrderComparisonOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «BitwiseOrOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «BitwiseXOrOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «BitwiseAndOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «ShiftOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «AddSubOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «MulDivModOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «ExponentiationOperator» Expression (* Binary Operator, Left Associative *);
    // ConditionalExpression = Expression «ConditionalOperator» (* Unary Operator, Postfix *);
    // UnaryPostfixExpression = Expression «UnaryPostfixOperator» (* Unary Operator, Postfix *);
    // UnaryPrefixExpression = «UnaryPrefixOperator» Expression (* Unary Operator, Prefix *);
    // FunctionCallExpression = Expression «FunctionCallOperator» (* Unary Operator, Postfix *);
    // MemberAccessExpression = Expression «MemberAccessOperator» (* Unary Operator, Postfix *);
    // IndexAccessExpression = Expression «IndexAccessOperator» (* Unary Operator, Postfix *);

    #[allow(dead_code, non_snake_case)]
    fn expression__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut results: Vec<ParserResult> = Vec::new();
            let initial_result = loop {
                let result = loop {
                    let result = self
                        .unary_prefix_operator(stream)
                        .to_pratt_element_operator(RuleKind::UnaryPrefixExpression, 255, 29u8);
                    match result {
                        ParserResult::PrattOperatorMatch(_) => results.push(result),
                        _ => break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => {
                        break result;
                    }
                }
                {
                    let result = self.primary_expression(stream);
                    if result.is_match() {
                        results.push(result);
                    } else {
                        break result;
                    }
                }
                let result = loop {
                    let result = loop {
                        let start_position = stream.position();
                        stream.set_position(start_position);
                        let next_result = self
                            .conditional_operator(stream)
                            .to_pratt_element_operator(RuleKind::ConditionalExpression, 3u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .unary_postfix_operator(stream)
                            .to_pratt_element_operator(RuleKind::UnaryPostfixExpression, 27u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .function_call_operator(stream)
                            .to_pratt_element_operator(RuleKind::FunctionCallExpression, 31u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .member_access_operator(stream)
                            .to_pratt_element_operator(RuleKind::MemberAccessExpression, 33u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .index_access_operator(stream)
                            .to_pratt_element_operator(RuleKind::IndexAccessExpression, 35u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        break ParserResult::no_match(vec![]);
                    };
                    match result {
                        ParserResult::PrattOperatorMatch(_) => results.push(result),
                        _ => break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => {
                        break result;
                    }
                }
                let result =
                    loop {
                        let start_position = stream.position();
                        stream.set_position(start_position);
                        let next_result = self
                            .assignment_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 1u8, 1u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self.or_operator(stream).to_pratt_element_operator(
                            RuleKind::BinaryExpression,
                            5u8,
                            5u8 + 1,
                        );
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self.and_operator(stream).to_pratt_element_operator(
                            RuleKind::BinaryExpression,
                            7u8,
                            7u8 + 1,
                        );
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .equality_comparison_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 9u8, 9u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .order_comparison_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 11u8, 11u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .bitwise_or_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 13u8, 13u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .bitwise_x_or_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 15u8, 15u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .bitwise_and_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 17u8, 17u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self.shift_operator(stream).to_pratt_element_operator(
                            RuleKind::BinaryExpression,
                            19u8,
                            19u8 + 1,
                        );
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self.add_sub_operator(stream).to_pratt_element_operator(
                            RuleKind::BinaryExpression,
                            21u8,
                            21u8 + 1,
                        );
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .mul_div_mod_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 23u8, 23u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .exponentiation_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 25u8, 25u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        break ParserResult::no_match(vec![]);
                    };
                match result {
                    ParserResult::PrattOperatorMatch(_) => results.push(result),
                    _ => break result,
                }
            };
            if results.is_empty() {
                break initial_result;
            }
            reduce_pratt_elements(
                |children| vec![cst::Node::rule(RuleKind::Expression, children)],
                &mut results,
            );
            if results.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    results
                );
            }
            match results.remove(0) {
                ParserResult::Match(r#match) => {
                    if let ParserResult::IncompleteMatch(_) = initial_result {
                        break ParserResult::incomplete_match(r#match.nodes, vec![]);
                    } else {
                        break ParserResult::r#match(
                            r#match.nodes,
                            r#match.tokens_that_would_have_allowed_more_progress,
                        );
                    }
                }
                ParserResult::IncompleteMatch(incomplete_match) => {
                    if let ParserResult::IncompleteMatch(initial_incomplete_match) = initial_result
                    {
                        let mut nodes = incomplete_match.nodes;
                        nodes.extend(initial_incomplete_match.nodes);
                        break ParserResult::incomplete_match(
                            nodes,
                            initial_incomplete_match.tokens_that_would_have_allowed_more_progress,
                        );
                    } else {
                        break ParserResult::IncompleteMatch(incomplete_match);
                    }
                }
                _ => unreachable!("Pratt parser produced an invalid result"),
            }
        }
        .with_kind(RuleKind::Expression)
    }

    // (* v0.6.0 *)
    // Expression = BinaryExpression (* Expression «AssignmentOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | ConditionalExpression (* Expression «ConditionalOperator» *) (* Unary Operator, Postfix *)
    //            | BinaryExpression (* Expression «OrOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «AndOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «EqualityComparisonOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «OrderComparisonOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «BitwiseOrOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «BitwiseXOrOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «BitwiseAndOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «ShiftOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «AddSubOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «MulDivModOperator» Expression *) (* Binary Operator, Left Associative *)
    //            | BinaryExpression (* Expression «ExponentiationOperator» Expression *) (* Binary Operator, Right Associative *)
    //            | UnaryPostfixExpression (* Expression «UnaryPostfixOperator» *) (* Unary Operator, Postfix *)
    //            | UnaryPrefixExpression (* «UnaryPrefixOperator» Expression *) (* Unary Operator, Prefix *)
    //            | FunctionCallExpression (* Expression «FunctionCallOperator» *) (* Unary Operator, Postfix *)
    //            | MemberAccessExpression (* Expression «MemberAccessOperator» *) (* Unary Operator, Postfix *)
    //            | IndexAccessExpression (* Expression «IndexAccessOperator» *) (* Unary Operator, Postfix *)
    //            | «PrimaryExpression»;
    // BinaryExpression = Expression «AssignmentOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «OrOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «AndOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «EqualityComparisonOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «OrderComparisonOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «BitwiseOrOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «BitwiseXOrOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «BitwiseAndOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «ShiftOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «AddSubOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «MulDivModOperator» Expression (* Binary Operator, Left Associative *);
    // BinaryExpression = Expression «ExponentiationOperator» Expression (* Binary Operator, Right Associative *);
    // ConditionalExpression = Expression «ConditionalOperator» (* Unary Operator, Postfix *);
    // UnaryPostfixExpression = Expression «UnaryPostfixOperator» (* Unary Operator, Postfix *);
    // UnaryPrefixExpression = «UnaryPrefixOperator» Expression (* Unary Operator, Prefix *);
    // FunctionCallExpression = Expression «FunctionCallOperator» (* Unary Operator, Postfix *);
    // MemberAccessExpression = Expression «MemberAccessOperator» (* Unary Operator, Postfix *);
    // IndexAccessExpression = Expression «IndexAccessOperator» (* Unary Operator, Postfix *);

    #[allow(dead_code, non_snake_case)]
    fn expression__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut results: Vec<ParserResult> = Vec::new();
            let initial_result = loop {
                let result = loop {
                    let result = self
                        .unary_prefix_operator(stream)
                        .to_pratt_element_operator(RuleKind::UnaryPrefixExpression, 255, 29u8);
                    match result {
                        ParserResult::PrattOperatorMatch(_) => results.push(result),
                        _ => break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => {
                        break result;
                    }
                }
                {
                    let result = self.primary_expression(stream);
                    if result.is_match() {
                        results.push(result);
                    } else {
                        break result;
                    }
                }
                let result = loop {
                    let result = loop {
                        let start_position = stream.position();
                        stream.set_position(start_position);
                        let next_result = self
                            .conditional_operator(stream)
                            .to_pratt_element_operator(RuleKind::ConditionalExpression, 3u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .unary_postfix_operator(stream)
                            .to_pratt_element_operator(RuleKind::UnaryPostfixExpression, 27u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .function_call_operator(stream)
                            .to_pratt_element_operator(RuleKind::FunctionCallExpression, 31u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .member_access_operator(stream)
                            .to_pratt_element_operator(RuleKind::MemberAccessExpression, 33u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .index_access_operator(stream)
                            .to_pratt_element_operator(RuleKind::IndexAccessExpression, 35u8, 255);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        break ParserResult::no_match(vec![]);
                    };
                    match result {
                        ParserResult::PrattOperatorMatch(_) => results.push(result),
                        _ => break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => {
                        break result;
                    }
                }
                let result =
                    loop {
                        let start_position = stream.position();
                        stream.set_position(start_position);
                        let next_result = self
                            .assignment_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 1u8, 1u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self.or_operator(stream).to_pratt_element_operator(
                            RuleKind::BinaryExpression,
                            5u8,
                            5u8 + 1,
                        );
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self.and_operator(stream).to_pratt_element_operator(
                            RuleKind::BinaryExpression,
                            7u8,
                            7u8 + 1,
                        );
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .equality_comparison_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 9u8, 9u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .order_comparison_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 11u8, 11u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .bitwise_or_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 13u8, 13u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .bitwise_x_or_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 15u8, 15u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .bitwise_and_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 17u8, 17u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self.shift_operator(stream).to_pratt_element_operator(
                            RuleKind::BinaryExpression,
                            19u8,
                            19u8 + 1,
                        );
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self.add_sub_operator(stream).to_pratt_element_operator(
                            RuleKind::BinaryExpression,
                            21u8,
                            21u8 + 1,
                        );
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .mul_div_mod_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 23u8, 23u8 + 1);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        let next_result = self
                            .exponentiation_operator(stream)
                            .to_pratt_element_operator(RuleKind::BinaryExpression, 25u8 + 1, 25u8);
                        match next_result {
                            ParserResult::PrattOperatorMatch(_) => break next_result,
                            ParserResult::Match(_) => unreachable!(
                                "ParserResult::Match isn't constructed when parsing operators"
                            ),
                            ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                        }
                        stream.set_position(start_position);
                        break ParserResult::no_match(vec![]);
                    };
                match result {
                    ParserResult::PrattOperatorMatch(_) => results.push(result),
                    _ => break result,
                }
            };
            if results.is_empty() {
                break initial_result;
            }
            reduce_pratt_elements(
                |children| vec![cst::Node::rule(RuleKind::Expression, children)],
                &mut results,
            );
            if results.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    results
                );
            }
            match results.remove(0) {
                ParserResult::Match(r#match) => {
                    if let ParserResult::IncompleteMatch(_) = initial_result {
                        break ParserResult::incomplete_match(r#match.nodes, vec![]);
                    } else {
                        break ParserResult::r#match(
                            r#match.nodes,
                            r#match.tokens_that_would_have_allowed_more_progress,
                        );
                    }
                }
                ParserResult::IncompleteMatch(incomplete_match) => {
                    if let ParserResult::IncompleteMatch(initial_incomplete_match) = initial_result
                    {
                        let mut nodes = incomplete_match.nodes;
                        nodes.extend(initial_incomplete_match.nodes);
                        break ParserResult::incomplete_match(
                            nodes,
                            initial_incomplete_match.tokens_that_would_have_allowed_more_progress,
                        );
                    } else {
                        break ParserResult::IncompleteMatch(incomplete_match);
                    }
                }
                _ => unreachable!("Pratt parser produced an invalid result"),
            }
        }
        .with_kind(RuleKind::Expression)
    }

    pub(crate) fn expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.expression__0_6_0(stream)
        } else {
            self.expression__0_4_11(stream)
        }
    }

    // ExpressionStatement = Expression SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn expression_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.expression(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ExpressionStatement)
    }

    // (* v0.6.0 *)
    // «FallbackFunctionAttribute» = ModifierInvocation
    //                             | OverrideSpecifier
    //                             | EXTERNAL_KEYWORD
    //                             | PAYABLE_KEYWORD
    //                             | PURE_KEYWORD
    //                             | VIEW_KEYWORD
    //                             | VIRTUAL_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn fallback_function_attribute__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.modifier_invocation(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.override_specifier(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::external_keyword,
                    TokenKind::ExternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::payable_keyword,
                    TokenKind::PayableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::pure_keyword,
                    TokenKind::PureKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::view_keyword,
                    TokenKind::ViewKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::virtual_keyword,
                    TokenKind::VirtualKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn fallback_function_attribute__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.fallback_function_attribute__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn fallback_function_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.fallback_function_attribute__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.6.0 *)
    // FallbackFunctionAttributesList = «FallbackFunctionAttribute»+;

    #[allow(dead_code, non_snake_case)]
    fn fallback_function_attributes_list__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result
                .incorporate_one_or_more_result(self.fallback_function_attribute(stream))
            {
            }
            running_result
        }
        .with_kind(RuleKind::FallbackFunctionAttributesList)
    }

    #[allow(non_snake_case)]
    pub(crate) fn fallback_function_attributes_list__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.fallback_function_attributes_list__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn fallback_function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        self.fallback_function_attributes_list__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.6.0 *)
    // FallbackFunctionDefinition = FALLBACK_KEYWORD ParametersDeclaration FallbackFunctionAttributesList? ReturnsDeclaration? (SEMICOLON | Block);

    #[allow(dead_code, non_snake_case)]
    fn fallback_function_definition__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::fallback_keyword,
                    TokenKind::FallbackKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parameters_declaration(stream))
                {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.fallback_function_attributes_list(stream),
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.returns_declaration(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::semicolon,
                            TokenKind::Semicolon,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.block(stream)) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FallbackFunctionDefinition)
    }

    #[allow(non_snake_case)]
    pub(crate) fn fallback_function_definition__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.fallback_function_definition__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn fallback_function_definition(&self, stream: &mut Stream) -> ParserResult {
        self.fallback_function_definition__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // ForStatement = FOR_KEYWORD OPEN_PAREN («SimpleStatement» | SEMICOLON) (ExpressionStatement | SEMICOLON) Expression? CLOSE_PAREN Statement;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn for_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::for_keyword,
                    TokenKind::ForKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_paren,
                                TokenKind::OpenParen,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result({
                                    let mut running_result = ParserResult::no_match(vec![]);
                                    let start_position = stream.position();
                                    loop {
                                        if running_result.incorporate_choice_result(
                                            self.simple_statement(stream),
                                        ) {
                                            break;
                                        }
                                        stream.set_position(start_position);
                                        if running_result.incorporate_choice_result(
                                            self.parse_token_with_trivia(
                                                stream,
                                                &Self::semicolon,
                                                TokenKind::Semicolon,
                                            ),
                                        ) {
                                            break;
                                        }
                                        stream.set_position(start_position);
                                        break;
                                    }
                                    if let ParserResult::IncompleteMatch(incomplete_match) =
                                        &running_result
                                    {
                                        incomplete_match.consume_stream(stream);
                                    }
                                    running_result
                                }) {
                                    break;
                                }
                                if !running_result.incorporate_sequence_result({
                                    let mut running_result = ParserResult::no_match(vec![]);
                                    let start_position = stream.position();
                                    loop {
                                        if running_result.incorporate_choice_result(
                                            self.expression_statement(stream),
                                        ) {
                                            break;
                                        }
                                        stream.set_position(start_position);
                                        if running_result.incorporate_choice_result(
                                            self.parse_token_with_trivia(
                                                stream,
                                                &Self::semicolon,
                                                TokenKind::Semicolon,
                                            ),
                                        ) {
                                            break;
                                        }
                                        stream.set_position(start_position);
                                        break;
                                    }
                                    if let ParserResult::IncompleteMatch(incomplete_match) =
                                        &running_result
                                    {
                                        incomplete_match.consume_stream(stream);
                                    }
                                    running_result
                                }) {
                                    break;
                                }
                                running_result.incorporate_sequence_result(
                                    transform_option_result(self.expression(stream)),
                                );
                                break;
                            }
                            running_result
                        }) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_paren,
                            TokenKind::CloseParen,
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.statement(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ForStatement)
    }

    // (* v0.4.11 *)
    // «FunctionAttribute» = ModifierInvocation
    //                     | OverrideSpecifier
    //                     | CONSTANT_KEYWORD
    //                     | EXTERNAL_KEYWORD
    //                     | INTERNAL_KEYWORD
    //                     | PAYABLE_KEYWORD
    //                     | PRIVATE_KEYWORD
    //                     | PUBLIC_KEYWORD
    //                     | PURE_KEYWORD
    //                     | VIEW_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn function_attribute__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.modifier_invocation(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.override_specifier(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::constant_keyword,
                    TokenKind::ConstantKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::external_keyword,
                    TokenKind::ExternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::internal_keyword,
                    TokenKind::InternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::payable_keyword,
                    TokenKind::PayableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::private_keyword,
                    TokenKind::PrivateKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::pure_keyword,
                    TokenKind::PureKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::view_keyword,
                    TokenKind::ViewKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.5.0 *)
    // «FunctionAttribute» = ModifierInvocation
    //                     | OverrideSpecifier
    //                     | EXTERNAL_KEYWORD
    //                     | INTERNAL_KEYWORD
    //                     | PAYABLE_KEYWORD
    //                     | PRIVATE_KEYWORD
    //                     | PUBLIC_KEYWORD
    //                     | PURE_KEYWORD
    //                     | VIEW_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn function_attribute__0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.modifier_invocation(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.override_specifier(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::external_keyword,
                    TokenKind::ExternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::internal_keyword,
                    TokenKind::InternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::payable_keyword,
                    TokenKind::PayableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::private_keyword,
                    TokenKind::PrivateKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::pure_keyword,
                    TokenKind::PureKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::view_keyword,
                    TokenKind::ViewKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.6.0 *)
    // «FunctionAttribute» = ModifierInvocation
    //                     | OverrideSpecifier
    //                     | EXTERNAL_KEYWORD
    //                     | INTERNAL_KEYWORD
    //                     | PAYABLE_KEYWORD
    //                     | PRIVATE_KEYWORD
    //                     | PUBLIC_KEYWORD
    //                     | PURE_KEYWORD
    //                     | VIEW_KEYWORD
    //                     | VIRTUAL_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn function_attribute__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.modifier_invocation(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.override_specifier(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::external_keyword,
                    TokenKind::ExternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::internal_keyword,
                    TokenKind::InternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::payable_keyword,
                    TokenKind::PayableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::private_keyword,
                    TokenKind::PrivateKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::pure_keyword,
                    TokenKind::PureKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::view_keyword,
                    TokenKind::ViewKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::virtual_keyword,
                    TokenKind::VirtualKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn function_attribute(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.function_attribute__0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.function_attribute__0_5_0(stream)
        } else {
            self.function_attribute__0_4_11(stream)
        }
    }

    // FunctionAttributesList = «FunctionAttribute»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.function_attribute(stream)) {}
            running_result
        }
        .with_kind(RuleKind::FunctionAttributesList)
    }

    // (* v0.4.11 *)
    // «FunctionCallOperator» = ArgumentsDeclaration;

    #[allow(dead_code, non_snake_case)]
    fn function_call_operator__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.arguments_declaration(stream)
    }

    // (* v0.6.2 *)
    // «FunctionCallOperator» = FunctionCallOptions? ArgumentsDeclaration;

    #[allow(dead_code, non_snake_case)]
    fn function_call_operator__0_6_2(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.function_call_options(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.arguments_declaration(stream));
                break;
            }
            running_result
        }
    }

    pub(crate) fn function_call_operator(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_2 {
            self.function_call_operator__0_6_2(stream)
        } else {
            self.function_call_operator__0_4_11(stream)
        }
    }

    // (* v0.6.2 *)
    // FunctionCallOptions = NamedArgumentsDeclaration+;

    #[allow(dead_code, non_snake_case)]
    fn function_call_options__0_6_2(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result
                .incorporate_one_or_more_result(self.named_arguments_declaration(stream))
            {
            }
            running_result
        }
        .with_kind(RuleKind::FunctionCallOptions)
    }

    // (* v0.8.0 *)
    // FunctionCallOptions = NamedArgumentsDeclaration;

    #[allow(dead_code, non_snake_case)]
    fn function_call_options__0_8_0(&self, stream: &mut Stream) -> ParserResult {
        self.named_arguments_declaration(stream)
            .with_kind(RuleKind::FunctionCallOptions)
    }

    #[allow(non_snake_case)]
    pub(crate) fn function_call_options__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            Some(self.function_call_options__0_8_0(stream))
        } else if self.version_is_equal_to_or_greater_than_0_6_2 {
            Some(self.function_call_options__0_6_2(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn function_call_options(&self, stream: &mut Stream) -> ParserResult {
        self.function_call_options__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // FunctionDefinition = FUNCTION_KEYWORD (IDENTIFIER | FALLBACK_KEYWORD | RECEIVE_KEYWORD) ParametersDeclaration FunctionAttributesList? ReturnsDeclaration? (SEMICOLON | Block);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn function_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::function_keyword,
                    TokenKind::FunctionKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::identifier,
                            TokenKind::Identifier,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::fallback_keyword,
                            TokenKind::FallbackKeyword,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::receive_keyword,
                            TokenKind::ReceiveKeyword,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                }) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parameters_declaration(stream))
                {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.function_attributes_list(stream),
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.returns_declaration(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::semicolon,
                            TokenKind::Semicolon,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.block(stream)) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FunctionDefinition)
    }

    // FunctionType = FUNCTION_KEYWORD ParametersDeclaration FunctionTypeAttributesList? ReturnsDeclaration?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn function_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::function_keyword,
                    TokenKind::FunctionKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parameters_declaration(stream))
                {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.function_type_attributes_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.returns_declaration(stream),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FunctionType)
    }

    // «FunctionTypeAttribute» = INTERNAL_KEYWORD
    //                         | EXTERNAL_KEYWORD
    //                         | PRIVATE_KEYWORD
    //                         | PUBLIC_KEYWORD
    //                         | PURE_KEYWORD
    //                         | VIEW_KEYWORD
    //                         | PAYABLE_KEYWORD;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn function_type_attribute(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::internal_keyword,
                    TokenKind::InternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::external_keyword,
                    TokenKind::ExternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::private_keyword,
                    TokenKind::PrivateKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::pure_keyword,
                    TokenKind::PureKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::view_keyword,
                    TokenKind::ViewKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::payable_keyword,
                    TokenKind::PayableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // FunctionTypeAttributesList = «FunctionTypeAttribute»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn function_type_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result
                .incorporate_one_or_more_result(self.function_type_attribute(stream))
            {}
            running_result
        }
        .with_kind(RuleKind::FunctionTypeAttributesList)
    }

    // HexStringLiteralsList = HEX_STRING_LITERAL+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn hex_string_literals_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.parse_token_with_trivia(
                stream,
                &Self::hex_string_literal,
                TokenKind::HexStringLiteral,
            )) {}
            running_result
        }
        .with_kind(RuleKind::HexStringLiteralsList)
    }

    // IdentifierPath = IDENTIFIER (PERIOD IDENTIFIER)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn identifier_path(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::period,
                                    TokenKind::Period,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::identifier,
                                    TokenKind::Identifier,
                                ),
                            );
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::IdentifierPath)
    }

    // IdentifierPathsList = IdentifierPath (COMMA IdentifierPath)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn identifier_paths_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.identifier_path(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result
                                .incorporate_sequence_result(self.identifier_path(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::IdentifierPathsList)
    }

    // IdentifiersList = IDENTIFIER (COMMA IDENTIFIER)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn identifiers_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::identifier,
                                    TokenKind::Identifier,
                                ),
                            );
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::IdentifiersList)
    }

    // IfStatement = IF_KEYWORD OPEN_PAREN Expression CLOSE_PAREN Statement (ELSE_KEYWORD Statement)?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn if_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::if_keyword,
                    TokenKind::IfKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_paren,
                                TokenKind::OpenParen,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(self.expression(stream)) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_paren,
                            TokenKind::CloseParen,
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.statement(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::else_keyword,
                                TokenKind::ElseKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.statement(stream));
                        break;
                    }
                    running_result
                }));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::IfStatement)
    }

    // ImportDirective = IMPORT_KEYWORD (PathImport | NamedImport | DeconstructionImport) SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn import_directive(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::import_keyword,
                                TokenKind::ImportKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::no_match(vec![]);
                            let start_position = stream.position();
                            loop {
                                if running_result
                                    .incorporate_choice_result(self.path_import(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                if running_result
                                    .incorporate_choice_result(self.named_import(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                if running_result
                                    .incorporate_choice_result(self.deconstruction_import(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                break;
                            }
                            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result
                            {
                                incomplete_match.consume_stream(stream);
                            }
                            running_result
                        });
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ImportDirective)
    }

    // «IndexAccessOperator» = OPEN_BRACKET Expression? (COLON Expression?)? CLOSE_BRACKET;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn index_access_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_bracket,
                    TokenKind::OpenBracket,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.expression(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(transform_option_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::colon,
                                        TokenKind::Colon,
                                    ),
                                ) {
                                    break;
                                }
                                running_result.incorporate_sequence_result(
                                    transform_option_result(self.expression(stream)),
                                );
                                break;
                            }
                            running_result
                        }));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_bracket,
                    TokenKind::CloseBracket,
                ));
                break;
            }
            running_result
        }
    }

    // InheritanceSpecifier = IS_KEYWORD InheritanceTypesList;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn inheritance_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::is_keyword,
                    TokenKind::IsKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.inheritance_types_list(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::InheritanceSpecifier)
    }

    // InheritanceType = IdentifierPath ArgumentsDeclaration?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn inheritance_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.identifier_path(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.arguments_declaration(stream),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::InheritanceType)
    }

    // InheritanceTypesList = InheritanceType (COMMA InheritanceType)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn inheritance_types_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.inheritance_type(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result
                                .incorporate_sequence_result(self.inheritance_type(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::InheritanceTypesList)
    }

    // InterfaceDefinition = INTERFACE_KEYWORD IDENTIFIER InheritanceSpecifier? OPEN_BRACE InterfaceMembersList? CLOSE_BRACE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn interface_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::interface_keyword,
                    TokenKind::InterfaceKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.inheritance_specifier(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_brace,
                                TokenKind::OpenBrace,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.interface_members_list(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_brace,
                            TokenKind::CloseBrace,
                        ));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::InterfaceDefinition)
    }

    // InterfaceMembersList = «ContractMember»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn interface_members_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.contract_member(stream)) {}
            running_result
        }
        .with_kind(RuleKind::InterfaceMembersList)
    }

    // LeadingTrivia = (WHITESPACE | END_OF_LINE | MULTILINE_COMMENT | SINGLE_LINE_COMMENT)+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn leading_trivia(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result({
                let mut running_result = ParserResult::no_match(vec![]);
                let start_position = stream.position();
                loop {
                    if running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::whitespace,
                        TokenKind::Whitespace,
                    )) {
                        break;
                    }
                    stream.set_position(start_position);
                    if running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::end_of_line,
                        TokenKind::EndOfLine,
                    )) {
                        break;
                    }
                    stream.set_position(start_position);
                    if running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::multiline_comment,
                        TokenKind::MultilineComment,
                    )) {
                        break;
                    }
                    stream.set_position(start_position);
                    if running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::single_line_comment,
                        TokenKind::SingleLineComment,
                    )) {
                        break;
                    }
                    stream.set_position(start_position);
                    break;
                }
                if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                    incomplete_match.consume_stream(stream);
                }
                running_result
            }) {}
            running_result
        }
        .with_kind(RuleKind::LeadingTrivia)
    }

    // LibraryDefinition = LIBRARY_KEYWORD IDENTIFIER OPEN_BRACE LibraryMembersList? CLOSE_BRACE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn library_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::library_keyword,
                    TokenKind::LibraryKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_brace,
                                TokenKind::OpenBrace,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.library_members_list(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_brace,
                            TokenKind::CloseBrace,
                        ));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::LibraryDefinition)
    }

    // LibraryMembersList = «ContractMember»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn library_members_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.contract_member(stream)) {}
            running_result
        }
        .with_kind(RuleKind::LibraryMembersList)
    }

    // (* v0.4.11 *)
    // MappingKeyType = «ElementaryType» | IdentifierPath;

    #[allow(dead_code, non_snake_case)]
    fn mapping_key_type__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.elementary_type(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.identifier_path(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
        .with_kind(RuleKind::MappingKeyType)
    }

    // (* v0.8.18 *)
    // MappingKeyType = («ElementaryType» | IdentifierPath) IDENTIFIER?;

    #[allow(dead_code, non_snake_case)]
    fn mapping_key_type__0_8_18(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.elementary_type(stream)) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.identifier_path(stream)) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token_with_trivia(stream, &Self::identifier, TokenKind::Identifier),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::MappingKeyType)
    }

    pub(crate) fn mapping_key_type(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_18 {
            self.mapping_key_type__0_8_18(stream)
        } else {
            self.mapping_key_type__0_4_11(stream)
        }
    }

    // MappingType = MAPPING_KEYWORD OPEN_PAREN MappingKeyType EQUAL_GREATER_THAN MappingValueType CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn mapping_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::mapping_keyword,
                    TokenKind::MappingKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_paren,
                                TokenKind::OpenParen,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result
                                    .incorporate_sequence_result(self.mapping_key_type(stream))
                                {
                                    break;
                                }
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::equal_greater_than,
                                        TokenKind::EqualGreaterThan,
                                    ),
                                ) {
                                    break;
                                }
                                running_result
                                    .incorporate_sequence_result(self.mapping_value_type(stream));
                                break;
                            }
                            running_result
                        }) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_paren,
                            TokenKind::CloseParen,
                        ));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::MappingType)
    }

    // (* v0.4.11 *)
    // MappingValueType = TypeName;

    #[allow(dead_code, non_snake_case)]
    fn mapping_value_type__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.type_name(stream).with_kind(RuleKind::MappingValueType)
    }

    // (* v0.8.18 *)
    // MappingValueType = TypeName IDENTIFIER?;

    #[allow(dead_code, non_snake_case)]
    fn mapping_value_type__0_8_18(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token_with_trivia(stream, &Self::identifier, TokenKind::Identifier),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::MappingValueType)
    }

    pub(crate) fn mapping_value_type(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_18 {
            self.mapping_value_type__0_8_18(stream)
        } else {
            self.mapping_value_type__0_4_11(stream)
        }
    }

    // «MemberAccessOperator» = PERIOD (IDENTIFIER | ADDRESS_KEYWORD);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn member_access_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::period,
                    TokenKind::Period,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::identifier,
                            TokenKind::Identifier,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::address_keyword,
                            TokenKind::AddressKeyword,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                });
                break;
            }
            running_result
        }
    }

    // (* v0.4.11 *)
    // «ModifierAttribute» = OverrideSpecifier;

    #[allow(dead_code, non_snake_case)]
    fn modifier_attribute__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.override_specifier(stream)
    }

    // (* v0.6.0 *)
    // «ModifierAttribute» = OverrideSpecifier | VIRTUAL_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn modifier_attribute__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.override_specifier(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::virtual_keyword,
                    TokenKind::VirtualKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn modifier_attribute(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.modifier_attribute__0_6_0(stream)
        } else {
            self.modifier_attribute__0_4_11(stream)
        }
    }

    // ModifierAttributesList = «ModifierAttribute»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn modifier_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.modifier_attribute(stream)) {}
            running_result
        }
        .with_kind(RuleKind::ModifierAttributesList)
    }

    // ModifierDefinition = MODIFIER_KEYWORD IDENTIFIER ParametersDeclaration? ModifierAttributesList? (SEMICOLON | Block);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn modifier_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::modifier_keyword,
                    TokenKind::ModifierKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.parameters_declaration(stream),
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.modifier_attributes_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::semicolon,
                            TokenKind::Semicolon,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.block(stream)) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ModifierDefinition)
    }

    // ModifierInvocation = IdentifierPath ArgumentsDeclaration?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn modifier_invocation(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.identifier_path(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.arguments_declaration(stream),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ModifierInvocation)
    }

    // «MulDivModOperator» = ASTERISK | SLASH | PERCENT;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn mul_div_mod_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::asterisk,
                    TokenKind::Asterisk,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::slash,
                    TokenKind::Slash,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::percent,
                    TokenKind::Percent,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // NamedArgument = IDENTIFIER COLON Expression;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn named_argument(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::colon,
                    TokenKind::Colon,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.expression(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NamedArgument)
    }

    // NamedArgumentsDeclaration = OPEN_BRACE NamedArgumentsList? CLOSE_BRACE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn named_arguments_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_brace,
                    TokenKind::OpenBrace,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.named_arguments_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_brace,
                    TokenKind::CloseBrace,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NamedArgumentsDeclaration)
    }

    // NamedArgumentsList = NamedArgument (COMMA NamedArgument)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn named_arguments_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.named_argument(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(self.named_argument(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NamedArgumentsList)
    }

    // NamedImport = ASTERISK AS_KEYWORD IDENTIFIER FROM_KEYWORD ASCII_STRING_LITERAL;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn named_import(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::asterisk,
                    TokenKind::Asterisk,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::as_keyword,
                    TokenKind::AsKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::from_keyword,
                    TokenKind::FromKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ascii_string_literal,
                    TokenKind::AsciiStringLiteral,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NamedImport)
    }

    // NewExpression = NEW_KEYWORD TypeName;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn new_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::new_keyword,
                    TokenKind::NewKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.type_name(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NewExpression)
    }

    // (* v0.4.11 *)
    // «NumberUnit» = DAYS_KEYWORD
    //              | ETHER_KEYWORD
    //              | FINNEY_KEYWORD
    //              | HOURS_KEYWORD
    //              | MINUTES_KEYWORD
    //              | SECONDS_KEYWORD
    //              | SZABO_KEYWORD
    //              | WEEKS_KEYWORD
    //              | WEI_KEYWORD
    //              | YEARS_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn number_unit__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::days_keyword,
                    TokenKind::DaysKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ether_keyword,
                    TokenKind::EtherKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::finney_keyword,
                    TokenKind::FinneyKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::hours_keyword,
                    TokenKind::HoursKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minutes_keyword,
                    TokenKind::MinutesKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::seconds_keyword,
                    TokenKind::SecondsKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::szabo_keyword,
                    TokenKind::SzaboKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::weeks_keyword,
                    TokenKind::WeeksKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::wei_keyword,
                    TokenKind::WeiKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::years_keyword,
                    TokenKind::YearsKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.5.0 *)
    // «NumberUnit» = DAYS_KEYWORD
    //              | ETHER_KEYWORD
    //              | FINNEY_KEYWORD
    //              | HOURS_KEYWORD
    //              | MINUTES_KEYWORD
    //              | SECONDS_KEYWORD
    //              | SZABO_KEYWORD
    //              | WEEKS_KEYWORD
    //              | WEI_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn number_unit__0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::days_keyword,
                    TokenKind::DaysKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ether_keyword,
                    TokenKind::EtherKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::finney_keyword,
                    TokenKind::FinneyKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::hours_keyword,
                    TokenKind::HoursKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minutes_keyword,
                    TokenKind::MinutesKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::seconds_keyword,
                    TokenKind::SecondsKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::szabo_keyword,
                    TokenKind::SzaboKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::weeks_keyword,
                    TokenKind::WeeksKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::wei_keyword,
                    TokenKind::WeiKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.6.11 *)
    // «NumberUnit» = DAYS_KEYWORD
    //              | ETHER_KEYWORD
    //              | FINNEY_KEYWORD
    //              | GWEI_KEYWORD
    //              | HOURS_KEYWORD
    //              | MINUTES_KEYWORD
    //              | SECONDS_KEYWORD
    //              | SZABO_KEYWORD
    //              | WEEKS_KEYWORD
    //              | WEI_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn number_unit__0_6_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::days_keyword,
                    TokenKind::DaysKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ether_keyword,
                    TokenKind::EtherKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::finney_keyword,
                    TokenKind::FinneyKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::gwei_keyword,
                    TokenKind::GweiKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::hours_keyword,
                    TokenKind::HoursKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minutes_keyword,
                    TokenKind::MinutesKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::seconds_keyword,
                    TokenKind::SecondsKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::szabo_keyword,
                    TokenKind::SzaboKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::weeks_keyword,
                    TokenKind::WeeksKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::wei_keyword,
                    TokenKind::WeiKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.7.0 *)
    // «NumberUnit» = DAYS_KEYWORD
    //              | ETHER_KEYWORD
    //              | GWEI_KEYWORD
    //              | HOURS_KEYWORD
    //              | MINUTES_KEYWORD
    //              | SECONDS_KEYWORD
    //              | WEEKS_KEYWORD
    //              | WEI_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn number_unit__0_7_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::days_keyword,
                    TokenKind::DaysKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ether_keyword,
                    TokenKind::EtherKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::gwei_keyword,
                    TokenKind::GweiKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::hours_keyword,
                    TokenKind::HoursKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minutes_keyword,
                    TokenKind::MinutesKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::seconds_keyword,
                    TokenKind::SecondsKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::weeks_keyword,
                    TokenKind::WeeksKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::wei_keyword,
                    TokenKind::WeiKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn number_unit(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            self.number_unit__0_7_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_11 {
            self.number_unit__0_6_11(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.number_unit__0_5_0(stream)
        } else {
            self.number_unit__0_4_11(stream)
        }
    }

    // (* v0.4.11 *)
    // NumericExpression = (HEX_LITERAL | DECIMAL_LITERAL) «NumberUnit»?;

    #[allow(dead_code, non_snake_case)]
    fn numeric_expression__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::hex_literal,
                            TokenKind::HexLiteral,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::decimal_literal,
                            TokenKind::DecimalLiteral,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                }) {
                    break;
                }
                running_result
                    .incorporate_sequence_result(transform_option_result(self.number_unit(stream)));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NumericExpression)
    }

    // (* v0.5.0 *)
    // NumericExpression = HEX_LITERAL | (DECIMAL_LITERAL «NumberUnit»?);

    #[allow(dead_code, non_snake_case)]
    fn numeric_expression__0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::hex_literal,
                    TokenKind::HexLiteral,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::decimal_literal,
                                TokenKind::DecimalLiteral,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(transform_option_result(
                            self.number_unit(stream),
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
        .with_kind(RuleKind::NumericExpression)
    }

    pub(crate) fn numeric_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.numeric_expression__0_5_0(stream)
        } else {
            self.numeric_expression__0_4_11(stream)
        }
    }

    // «OrOperator» = BAR_BAR;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::bar_bar, TokenKind::BarBar)
    }

    // «OrderComparisonOperator» = LESS_THAN
    //                           | GREATER_THAN
    //                           | LESS_THAN_EQUAL
    //                           | GREATER_THAN_EQUAL;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn order_comparison_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::less_than,
                    TokenKind::LessThan,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than,
                    TokenKind::GreaterThan,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::less_than_equal,
                    TokenKind::LessThanEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than_equal,
                    TokenKind::GreaterThanEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // OverrideSpecifier = OVERRIDE_KEYWORD (OPEN_PAREN IdentifierPathsList? CLOSE_PAREN)?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn override_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::override_keyword,
                    TokenKind::OverrideKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_paren,
                                TokenKind::OpenParen,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.identifier_paths_list(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_paren,
                            TokenKind::CloseParen,
                        ));
                        break;
                    }
                    running_result
                }));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::OverrideSpecifier)
    }

    // Parameter = TypeName «DataLocation»? IDENTIFIER?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parameter(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.data_location(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token_with_trivia(stream, &Self::identifier, TokenKind::Identifier),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::Parameter)
    }

    // ParametersDeclaration = OPEN_PAREN ParametersList? CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parameters_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_paren,
                    TokenKind::OpenParen,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.parameters_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_paren,
                    TokenKind::CloseParen,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ParametersDeclaration)
    }

    // ParametersList = Parameter (COMMA Parameter)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parameters_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parameter(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(self.parameter(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ParametersList)
    }

    // PathImport = ASCII_STRING_LITERAL (AS_KEYWORD IDENTIFIER)?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn path_import(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ascii_string_literal,
                    TokenKind::AsciiStringLiteral,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::as_keyword,
                                TokenKind::AsKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::identifier,
                            TokenKind::Identifier,
                        ));
                        break;
                    }
                    running_result
                }));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::PathImport)
    }

    // PositionalArgumentsList = Expression (COMMA Expression)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn positional_arguments_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.expression(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(self.expression(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::PositionalArgumentsList)
    }

    // PragmaDirective = PRAGMA_KEYWORD (ABICoderPragma | ExperimentalPragma | VersionPragma) SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn pragma_directive(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::pragma_keyword,
                                TokenKind::PragmaKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::no_match(vec![]);
                            let start_position = stream.position();
                            loop {
                                if running_result
                                    .incorporate_choice_result(self.abi_coder_pragma(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                if running_result
                                    .incorporate_choice_result(self.experimental_pragma(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                if running_result
                                    .incorporate_choice_result(self.version_pragma(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                break;
                            }
                            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result
                            {
                                incomplete_match.consume_stream(stream);
                            }
                            running_result
                        });
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::PragmaDirective)
    }

    // (* v0.4.11 *)
    // «PrimaryExpression» = NewExpression
    //                     | TupleExpression
    //                     | ArrayExpression
    //                     | «BooleanExpression»
    //                     | NumericExpression
    //                     | «StringExpression»
    //                     | «ElementaryType»
    //                     | IDENTIFIER;

    #[allow(dead_code, non_snake_case)]
    fn primary_expression__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.new_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.tuple_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.array_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.boolean_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.numeric_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.string_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.elementary_type(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.5.3 *)
    // «PrimaryExpression» = NewExpression
    //                     | TupleExpression
    //                     | TypeExpression
    //                     | ArrayExpression
    //                     | «BooleanExpression»
    //                     | NumericExpression
    //                     | «StringExpression»
    //                     | «ElementaryType»
    //                     | IDENTIFIER;

    #[allow(dead_code, non_snake_case)]
    fn primary_expression__0_5_3(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.new_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.tuple_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.type_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.array_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.boolean_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.numeric_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.string_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.elementary_type(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn primary_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_3 {
            self.primary_expression__0_5_3(stream)
        } else {
            self.primary_expression__0_4_11(stream)
        }
    }

    // (* v0.6.0 *)
    // «ReceiveFunctionAttribute» = ModifierInvocation
    //                            | OverrideSpecifier
    //                            | EXTERNAL_KEYWORD
    //                            | PAYABLE_KEYWORD
    //                            | VIRTUAL_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn receive_function_attribute__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.modifier_invocation(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.override_specifier(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::external_keyword,
                    TokenKind::ExternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::payable_keyword,
                    TokenKind::PayableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::virtual_keyword,
                    TokenKind::VirtualKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn receive_function_attribute__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.receive_function_attribute__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn receive_function_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.receive_function_attribute__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.6.0 *)
    // ReceiveFunctionAttributesList = «ReceiveFunctionAttribute»+;

    #[allow(dead_code, non_snake_case)]
    fn receive_function_attributes_list__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result
                .incorporate_one_or_more_result(self.receive_function_attribute(stream))
            {
            }
            running_result
        }
        .with_kind(RuleKind::ReceiveFunctionAttributesList)
    }

    #[allow(non_snake_case)]
    pub(crate) fn receive_function_attributes_list__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.receive_function_attributes_list__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn receive_function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        self.receive_function_attributes_list__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.6.0 *)
    // ReceiveFunctionDefinition = RECEIVE_KEYWORD ParametersDeclaration ReceiveFunctionAttributesList? (SEMICOLON | Block);

    #[allow(dead_code, non_snake_case)]
    fn receive_function_definition__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::receive_keyword,
                    TokenKind::ReceiveKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parameters_declaration(stream))
                {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.receive_function_attributes_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::semicolon,
                            TokenKind::Semicolon,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.block(stream)) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ReceiveFunctionDefinition)
    }

    #[allow(non_snake_case)]
    pub(crate) fn receive_function_definition__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.receive_function_definition__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn receive_function_definition(&self, stream: &mut Stream) -> ParserResult {
        self.receive_function_definition__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // ReturnStatement = RETURN_KEYWORD Expression? SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn return_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::return_keyword,
                                TokenKind::ReturnKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(transform_option_result(
                            self.expression(stream),
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ReturnStatement)
    }

    // ReturnsDeclaration = RETURNS_KEYWORD ParametersDeclaration;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn returns_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::returns_keyword,
                    TokenKind::ReturnsKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parameters_declaration(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ReturnsDeclaration)
    }

    // RevertStatement = REVERT_KEYWORD IdentifierPath? ArgumentsDeclaration SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn revert_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::revert_keyword,
                                TokenKind::RevertKeyword,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.identifier_path(stream),
                        )) {
                            break;
                        }
                        running_result
                            .incorporate_sequence_result(self.arguments_declaration(stream));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::RevertStatement)
    }

    // «ShiftOperator» = LESS_THAN_LESS_THAN
    //                 | GREATER_THAN_GREATER_THAN
    //                 | GREATER_THAN_GREATER_THAN_GREATER_THAN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn shift_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::less_than_less_than,
                    TokenKind::LessThanLessThan,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than_greater_than,
                    TokenKind::GreaterThanGreaterThan,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than_greater_than_greater_than,
                    TokenKind::GreaterThanGreaterThanGreaterThan,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // «SimpleStatement» = ExpressionStatement
    //                   | VariableDeclarationStatement
    //                   | TupleDeconstructionStatement;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn simple_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.expression_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.variable_declaration_statement(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.tuple_deconstruction_statement(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // SourceUnit = SourceUnitMembersList? EndOfFileTrivia?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn source_unit(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.source_unit_members_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.end_of_file_trivia(stream),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::SourceUnit)
    }

    // (* v0.4.11 *)
    // «SourceUnitMember» = PragmaDirective
    //                    | ImportDirective
    //                    | ContractDefinition
    //                    | InterfaceDefinition
    //                    | LibraryDefinition;

    #[allow(dead_code, non_snake_case)]
    fn source_unit_member__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.pragma_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.import_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.contract_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.interface_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.library_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.6.0 *)
    // «SourceUnitMember» = PragmaDirective
    //                    | ImportDirective
    //                    | ContractDefinition
    //                    | InterfaceDefinition
    //                    | LibraryDefinition
    //                    | StructDefinition
    //                    | EnumDefinition;

    #[allow(dead_code, non_snake_case)]
    fn source_unit_member__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.pragma_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.import_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.contract_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.interface_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.library_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.7.1 *)
    // «SourceUnitMember» = PragmaDirective
    //                    | ImportDirective
    //                    | ContractDefinition
    //                    | InterfaceDefinition
    //                    | LibraryDefinition
    //                    | StructDefinition
    //                    | EnumDefinition
    //                    | FunctionDefinition;

    #[allow(dead_code, non_snake_case)]
    fn source_unit_member__0_7_1(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.pragma_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.import_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.contract_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.interface_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.library_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.7.4 *)
    // «SourceUnitMember» = PragmaDirective
    //                    | ImportDirective
    //                    | ContractDefinition
    //                    | InterfaceDefinition
    //                    | LibraryDefinition
    //                    | StructDefinition
    //                    | EnumDefinition
    //                    | FunctionDefinition
    //                    | ConstantDefinition;

    #[allow(dead_code, non_snake_case)]
    fn source_unit_member__0_7_4(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.pragma_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.import_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.contract_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.interface_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.library_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.constant_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.8.4 *)
    // «SourceUnitMember» = PragmaDirective
    //                    | ImportDirective
    //                    | ContractDefinition
    //                    | InterfaceDefinition
    //                    | LibraryDefinition
    //                    | StructDefinition
    //                    | EnumDefinition
    //                    | FunctionDefinition
    //                    | ConstantDefinition
    //                    | ErrorDefinition;

    #[allow(dead_code, non_snake_case)]
    fn source_unit_member__0_8_4(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.pragma_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.import_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.contract_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.interface_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.library_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.constant_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.error_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.8.8 *)
    // «SourceUnitMember» = PragmaDirective
    //                    | ImportDirective
    //                    | ContractDefinition
    //                    | InterfaceDefinition
    //                    | LibraryDefinition
    //                    | StructDefinition
    //                    | EnumDefinition
    //                    | FunctionDefinition
    //                    | ConstantDefinition
    //                    | ErrorDefinition
    //                    | UserDefinedValueTypeDefinition;

    #[allow(dead_code, non_snake_case)]
    fn source_unit_member__0_8_8(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.pragma_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.import_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.contract_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.interface_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.library_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.constant_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.error_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.user_defined_value_type_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.8.13 *)
    // «SourceUnitMember» = PragmaDirective
    //                    | ImportDirective
    //                    | ContractDefinition
    //                    | InterfaceDefinition
    //                    | LibraryDefinition
    //                    | StructDefinition
    //                    | EnumDefinition
    //                    | FunctionDefinition
    //                    | ConstantDefinition
    //                    | ErrorDefinition
    //                    | UserDefinedValueTypeDefinition
    //                    | UsingDirective;

    #[allow(dead_code, non_snake_case)]
    fn source_unit_member__0_8_13(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.pragma_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.import_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.contract_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.interface_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.library_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.struct_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.constant_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.error_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.user_defined_value_type_definition(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.using_directive(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn source_unit_member(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_13 {
            self.source_unit_member__0_8_13(stream)
        } else if self.version_is_equal_to_or_greater_than_0_8_8 {
            self.source_unit_member__0_8_8(stream)
        } else if self.version_is_equal_to_or_greater_than_0_8_4 {
            self.source_unit_member__0_8_4(stream)
        } else if self.version_is_equal_to_or_greater_than_0_7_4 {
            self.source_unit_member__0_7_4(stream)
        } else if self.version_is_equal_to_or_greater_than_0_7_1 {
            self.source_unit_member__0_7_1(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.source_unit_member__0_6_0(stream)
        } else {
            self.source_unit_member__0_4_11(stream)
        }
    }

    // SourceUnitMembersList = «SourceUnitMember»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn source_unit_members_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.source_unit_member(stream)) {}
            running_result
        }
        .with_kind(RuleKind::SourceUnitMembersList)
    }

    // (* v0.4.11 *)
    // «StateVariableAttribute» = OverrideSpecifier
    //                          | CONSTANT_KEYWORD
    //                          | INTERNAL_KEYWORD
    //                          | PRIVATE_KEYWORD
    //                          | PUBLIC_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn state_variable_attribute__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.override_specifier(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::constant_keyword,
                    TokenKind::ConstantKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::internal_keyword,
                    TokenKind::InternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::private_keyword,
                    TokenKind::PrivateKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.6.5 *)
    // «StateVariableAttribute» = OverrideSpecifier
    //                          | CONSTANT_KEYWORD
    //                          | IMMUTABLE_KEYWORD
    //                          | INTERNAL_KEYWORD
    //                          | PRIVATE_KEYWORD
    //                          | PUBLIC_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn state_variable_attribute__0_6_5(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.override_specifier(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::constant_keyword,
                    TokenKind::ConstantKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::immutable_keyword,
                    TokenKind::ImmutableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::internal_keyword,
                    TokenKind::InternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::private_keyword,
                    TokenKind::PrivateKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn state_variable_attribute(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_5 {
            self.state_variable_attribute__0_6_5(stream)
        } else {
            self.state_variable_attribute__0_4_11(stream)
        }
    }

    // StateVariableAttributesList = «StateVariableAttribute»+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn state_variable_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result
                .incorporate_one_or_more_result(self.state_variable_attribute(stream))
            {}
            running_result
        }
        .with_kind(RuleKind::StateVariableAttributesList)
    }

    // StateVariableDefinition = TypeName StateVariableAttributesList? IDENTIFIER (EQUAL Expression)? SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn state_variable_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.state_variable_attributes_list(stream),
                        )) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::identifier,
                                TokenKind::Identifier,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(transform_option_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::equal,
                                        TokenKind::Equal,
                                    ),
                                ) {
                                    break;
                                }
                                running_result.incorporate_sequence_result(self.expression(stream));
                                break;
                            }
                            running_result
                        }));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::StateVariableDefinition)
    }

    // (* v0.4.11 *)
    // Statement = «SimpleStatement»
    //           | «ControlStatement»
    //           | AssemblyStatement
    //           | Block;

    #[allow(dead_code, non_snake_case)]
    fn statement__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.simple_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.control_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.assembly_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.block(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
        .with_kind(RuleKind::Statement)
    }

    // (* v0.8.0 *)
    // Statement = «SimpleStatement»
    //           | «ControlStatement»
    //           | AssemblyStatement
    //           | Block
    //           | UncheckedBlock;

    #[allow(dead_code, non_snake_case)]
    fn statement__0_8_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.simple_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.control_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.assembly_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.block(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.unchecked_block(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
        .with_kind(RuleKind::Statement)
    }

    pub(crate) fn statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            self.statement__0_8_0(stream)
        } else {
            self.statement__0_4_11(stream)
        }
    }

    // StatementsList = Statement+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn statements_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.statement(stream)) {}
            running_result
        }
        .with_kind(RuleKind::StatementsList)
    }

    // (* v0.4.11 *)
    // «StringExpression» = HexStringLiteralsList | AsciiStringLiteralsList;

    #[allow(dead_code, non_snake_case)]
    fn string_expression__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.hex_string_literals_list(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.ascii_string_literals_list(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.7.0 *)
    // «StringExpression» = HexStringLiteralsList
    //                    | AsciiStringLiteralsList
    //                    | UnicodeStringLiteralsList;

    #[allow(dead_code, non_snake_case)]
    fn string_expression__0_7_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.hex_string_literals_list(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.ascii_string_literals_list(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.unicode_string_literals_list(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn string_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            self.string_expression__0_7_0(stream)
        } else {
            self.string_expression__0_4_11(stream)
        }
    }

    // StructDefinition = STRUCT_KEYWORD IDENTIFIER OPEN_BRACE StructMembersList? CLOSE_BRACE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn struct_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::struct_keyword,
                    TokenKind::StructKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_brace,
                                TokenKind::OpenBrace,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.struct_members_list(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_brace,
                            TokenKind::CloseBrace,
                        ));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::StructDefinition)
    }

    // StructMember = TypeName IDENTIFIER SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn struct_member(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::identifier,
                            TokenKind::Identifier,
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::StructMember)
    }

    // StructMembersList = StructMember+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn struct_members_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.struct_member(stream)) {}
            running_result
        }
        .with_kind(RuleKind::StructMembersList)
    }

    // (* v0.4.11 *)
    // ThrowStatement = THROW_KEYWORD SEMICOLON;

    #[allow(dead_code, non_snake_case)]
    fn throw_statement__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::throw_keyword,
                    TokenKind::ThrowKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ThrowStatement)
    }

    #[allow(non_snake_case)]
    pub(crate) fn throw_statement__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            None
        } else {
            Some(self.throw_statement__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn throw_statement(&self, stream: &mut Stream) -> ParserResult {
        self.throw_statement__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // TrailingTrivia = WHITESPACE? SINGLE_LINE_COMMENT? END_OF_LINE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn trailing_trivia(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token(stream, &Self::whitespace, TokenKind::Whitespace),
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.parse_token(
                        stream,
                        &Self::single_line_comment,
                        TokenKind::SingleLineComment,
                    ),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token(
                    stream,
                    &Self::end_of_line,
                    TokenKind::EndOfLine,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::TrailingTrivia)
    }

    // (* v0.6.0 *)
    // TryStatement = TRY_KEYWORD Expression ReturnsDeclaration? Block CatchClausesList;

    #[allow(dead_code, non_snake_case)]
    fn try_statement__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::try_keyword,
                    TokenKind::TryKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.expression(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.returns_declaration(stream),
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.block(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(self.catch_clauses_list(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::TryStatement)
    }

    #[allow(non_snake_case)]
    pub(crate) fn try_statement__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.try_statement__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn try_statement(&self, stream: &mut Stream) -> ParserResult {
        self.try_statement__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // TupleDeconstructionStatement = OPEN_PAREN TupleMembersList? CLOSE_PAREN EQUAL Expression SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn tuple_deconstruction_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::open_paren,
                                        TokenKind::OpenParen,
                                    ),
                                ) {
                                    break;
                                }
                                if !running_result.incorporate_sequence_result(
                                    transform_option_result(self.tuple_members_list(stream)),
                                ) {
                                    break;
                                }
                                running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::close_paren,
                                        TokenKind::CloseParen,
                                    ),
                                );
                                break;
                            }
                            running_result
                        }) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(stream, &Self::equal, TokenKind::Equal),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.expression(stream));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::TupleDeconstructionStatement)
    }

    // TupleExpression = OPEN_PAREN TupleValuesList CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn tuple_expression(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_paren,
                    TokenKind::OpenParen,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.tuple_values_list(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_paren,
                    TokenKind::CloseParen,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::TupleExpression)
    }

    // TupleMember = ((TypeName «DataLocation»? IDENTIFIER) | («DataLocation»? IDENTIFIER))?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn tuple_member(&self, stream: &mut Stream) -> ParserResult {
        transform_option_result({
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.data_location(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::identifier,
                            TokenKind::Identifier,
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.data_location(stream),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::identifier,
                            TokenKind::Identifier,
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        })
        .with_kind(RuleKind::TupleMember)
    }

    // TupleMembersList = TupleMember (COMMA TupleMember)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn tuple_members_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.tuple_member(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(self.tuple_member(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::TupleMembersList)
    }

    // TupleValuesList = Expression? (COMMA Expression?)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn tuple_values_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result
                    .incorporate_sequence_result(transform_option_result(self.expression(stream)))
                {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(transform_option_result(
                                self.expression(stream),
                            ));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::TupleValuesList)
    }

    // (* v0.5.3 *)
    // TypeExpression = TYPE_KEYWORD OPEN_PAREN TypeName CLOSE_PAREN;

    #[allow(dead_code, non_snake_case)]
    fn type_expression__0_5_3(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::type_keyword,
                    TokenKind::TypeKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_paren,
                                TokenKind::OpenParen,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_paren,
                            TokenKind::CloseParen,
                        ));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::TypeExpression)
    }

    #[allow(non_snake_case)]
    pub(crate) fn type_expression__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_5_3 {
            Some(self.type_expression__0_5_3(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn type_expression(&self, stream: &mut Stream) -> ParserResult {
        self.type_expression__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // TypeName = ArrayTypeName (* TypeName «ArrayTypeNameOperator» *) (* Unary Operator, Postfix *)
    //          | FunctionType
    //          | MappingType
    //          | «ElementaryType»
    //          | IdentifierPath;
    // ArrayTypeName = TypeName «ArrayTypeNameOperator» (* Unary Operator, Postfix *);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn type_name(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut results: Vec<ParserResult> = Vec::new();
            let initial_result = loop {
                {
                    let result = {
                        let mut running_result = ParserResult::no_match(vec![]);
                        let start_position = stream.position();
                        loop {
                            if running_result.incorporate_choice_result(self.function_type(stream))
                            {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(self.mapping_type(stream)) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result
                                .incorporate_choice_result(self.elementary_type(stream))
                            {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result
                                .incorporate_choice_result(self.identifier_path(stream))
                            {
                                break;
                            }
                            stream.set_position(start_position);
                            break;
                        }
                        if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                            incomplete_match.consume_stream(stream);
                        }
                        running_result
                    };
                    if result.is_match() {
                        results.push(result);
                    } else {
                        break result;
                    }
                }
                let result = loop {
                    let result = self
                        .array_type_name_operator(stream)
                        .to_pratt_element_operator(RuleKind::ArrayTypeName, 1u8, 255);
                    match result {
                        ParserResult::PrattOperatorMatch(_) => results.push(result),
                        _ => break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => {
                        break result;
                    }
                }
                break ParserResult::no_match(vec![]);
            };
            if results.is_empty() {
                break initial_result;
            }
            reduce_pratt_elements(
                |children| vec![cst::Node::rule(RuleKind::TypeName, children)],
                &mut results,
            );
            if results.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    results
                );
            }
            match results.remove(0) {
                ParserResult::Match(r#match) => {
                    if let ParserResult::IncompleteMatch(_) = initial_result {
                        break ParserResult::incomplete_match(r#match.nodes, vec![]);
                    } else {
                        break ParserResult::r#match(
                            r#match.nodes,
                            r#match.tokens_that_would_have_allowed_more_progress,
                        );
                    }
                }
                ParserResult::IncompleteMatch(incomplete_match) => {
                    if let ParserResult::IncompleteMatch(initial_incomplete_match) = initial_result
                    {
                        let mut nodes = incomplete_match.nodes;
                        nodes.extend(initial_incomplete_match.nodes);
                        break ParserResult::incomplete_match(
                            nodes,
                            initial_incomplete_match.tokens_that_would_have_allowed_more_progress,
                        );
                    } else {
                        break ParserResult::IncompleteMatch(incomplete_match);
                    }
                }
                _ => unreachable!("Pratt parser produced an invalid result"),
            }
        }
        .with_kind(RuleKind::TypeName)
    }

    // «UnaryPostfixOperator» = PLUS_PLUS | MINUS_MINUS;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn unary_postfix_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::plus_plus,
                    TokenKind::PlusPlus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus_minus,
                    TokenKind::MinusMinus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.4.11 *)
    // «UnaryPrefixOperator» = PLUS_PLUS | MINUS_MINUS | TILDE | BANG | MINUS | PLUS;

    #[allow(dead_code, non_snake_case)]
    fn unary_prefix_operator__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::plus_plus,
                    TokenKind::PlusPlus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus_minus,
                    TokenKind::MinusMinus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::tilde,
                    TokenKind::Tilde,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::bang,
                    TokenKind::Bang,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus,
                    TokenKind::Minus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::plus,
                    TokenKind::Plus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // (* v0.5.0 *)
    // «UnaryPrefixOperator» = PLUS_PLUS | MINUS_MINUS | TILDE | BANG | MINUS;

    #[allow(dead_code, non_snake_case)]
    fn unary_prefix_operator__0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::plus_plus,
                    TokenKind::PlusPlus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus_minus,
                    TokenKind::MinusMinus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::tilde,
                    TokenKind::Tilde,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::bang,
                    TokenKind::Bang,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus,
                    TokenKind::Minus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    pub(crate) fn unary_prefix_operator(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.unary_prefix_operator__0_5_0(stream)
        } else {
            self.unary_prefix_operator__0_4_11(stream)
        }
    }

    // (* v0.8.0 *)
    // UncheckedBlock = UNCHECKED_KEYWORD Block;

    #[allow(dead_code, non_snake_case)]
    fn unchecked_block__0_8_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::unchecked_keyword,
                    TokenKind::UncheckedKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.block(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UncheckedBlock)
    }

    #[allow(non_snake_case)]
    pub(crate) fn unchecked_block__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            Some(self.unchecked_block__0_8_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn unchecked_block(&self, stream: &mut Stream) -> ParserResult {
        self.unchecked_block__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.7.0 *)
    // UnicodeStringLiteralsList = UNICODE_STRING_LITERAL+;

    #[allow(dead_code, non_snake_case)]
    fn unicode_string_literals_list__0_7_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.parse_token_with_trivia(
                stream,
                &Self::unicode_string_literal,
                TokenKind::UnicodeStringLiteral,
            )) {}
            running_result
        }
        .with_kind(RuleKind::UnicodeStringLiteralsList)
    }

    #[allow(non_snake_case)]
    pub(crate) fn unicode_string_literals_list__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            Some(self.unicode_string_literals_list__0_7_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn unicode_string_literals_list(&self, stream: &mut Stream) -> ParserResult {
        self.unicode_string_literals_list__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.11 *)
    // «UnnamedFunctionAttribute» = ModifierInvocation
    //                            | OverrideSpecifier
    //                            | EXTERNAL_KEYWORD
    //                            | PAYABLE_KEYWORD
    //                            | PURE_KEYWORD
    //                            | VIEW_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn unnamed_function_attribute__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.modifier_invocation(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.override_specifier(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::external_keyword,
                    TokenKind::ExternalKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::payable_keyword,
                    TokenKind::PayableKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::pure_keyword,
                    TokenKind::PureKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::view_keyword,
                    TokenKind::ViewKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn unnamed_function_attribute__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            None
        } else {
            Some(self.unnamed_function_attribute__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn unnamed_function_attribute(&self, stream: &mut Stream) -> ParserResult {
        self.unnamed_function_attribute__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.11 *)
    // UnnamedFunctionAttributesList = «UnnamedFunctionAttribute»+;

    #[allow(dead_code, non_snake_case)]
    fn unnamed_function_attributes_list__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result
                .incorporate_one_or_more_result(self.unnamed_function_attribute(stream))
            {
            }
            running_result
        }
        .with_kind(RuleKind::UnnamedFunctionAttributesList)
    }

    #[allow(non_snake_case)]
    pub(crate) fn unnamed_function_attributes_list__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            None
        } else {
            Some(self.unnamed_function_attributes_list__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn unnamed_function_attributes_list(&self, stream: &mut Stream) -> ParserResult {
        self.unnamed_function_attributes_list__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.4.11 *)
    // UnnamedFunctionDefinition = FUNCTION_KEYWORD ParametersDeclaration UnnamedFunctionAttributesList? (SEMICOLON | Block);

    #[allow(dead_code, non_snake_case)]
    fn unnamed_function_definition__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::function_keyword,
                    TokenKind::FunctionKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parameters_declaration(stream))
                {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.unnamed_function_attributes_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::semicolon,
                            TokenKind::Semicolon,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.block(stream)) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UnnamedFunctionDefinition)
    }

    #[allow(non_snake_case)]
    pub(crate) fn unnamed_function_definition__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            None
        } else {
            Some(self.unnamed_function_definition__0_4_11(stream))
        }
    }

    #[inline]
    pub(crate) fn unnamed_function_definition(&self, stream: &mut Stream) -> ParserResult {
        self.unnamed_function_definition__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.8.8 *)
    // UserDefinedValueTypeDefinition = TYPE_KEYWORD IDENTIFIER IS_KEYWORD «ElementaryType» SEMICOLON;

    #[allow(dead_code, non_snake_case)]
    fn user_defined_value_type_definition__0_8_8(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::type_keyword,
                                TokenKind::TypeKeyword,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::identifier,
                                TokenKind::Identifier,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::is_keyword,
                                TokenKind::IsKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.elementary_type(stream));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UserDefinedValueTypeDefinition)
    }

    #[allow(non_snake_case)]
    pub(crate) fn user_defined_value_type_definition__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_8 {
            Some(self.user_defined_value_type_definition__0_8_8(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn user_defined_value_type_definition(&self, stream: &mut Stream) -> ParserResult {
        self.user_defined_value_type_definition__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // UsingDirective = USING_KEYWORD (UsingDirectivePath | UsingDirectiveDeconstruction) FOR_KEYWORD (ASTERISK | TypeName) GLOBAL_KEYWORD? SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn using_directive(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::using_keyword,
                                TokenKind::UsingKeyword,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::no_match(vec![]);
                            let start_position = stream.position();
                            loop {
                                if running_result
                                    .incorporate_choice_result(self.using_directive_path(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                if running_result.incorporate_choice_result(
                                    self.using_directive_deconstruction(stream),
                                ) {
                                    break;
                                }
                                stream.set_position(start_position);
                                break;
                            }
                            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result
                            {
                                incomplete_match.consume_stream(stream);
                            }
                            running_result
                        }) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::for_keyword,
                                TokenKind::ForKeyword,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::no_match(vec![]);
                            let start_position = stream.position();
                            loop {
                                if running_result.incorporate_choice_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::asterisk,
                                        TokenKind::Asterisk,
                                    ),
                                ) {
                                    break;
                                }
                                stream.set_position(start_position);
                                if running_result.incorporate_choice_result(self.type_name(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                break;
                            }
                            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result
                            {
                                incomplete_match.consume_stream(stream);
                            }
                            running_result
                        }) {
                            break;
                        }
                        running_result.incorporate_sequence_result(transform_option_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::global_keyword,
                                TokenKind::GlobalKeyword,
                            ),
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UsingDirective)
    }

    // UsingDirectiveDeconstruction = OPEN_BRACE UsingDirectiveSymbolsList CLOSE_BRACE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn using_directive_deconstruction(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_brace,
                    TokenKind::OpenBrace,
                )) {
                    break;
                }
                if !running_result
                    .incorporate_sequence_result(self.using_directive_symbols_list(stream))
                {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_brace,
                    TokenKind::CloseBrace,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UsingDirectiveDeconstruction)
    }

    // (* v0.8.19 *)
    // «UsingDirectiveOperator» = AMPERSAND
    //                          | ASTERISK
    //                          | BANG_EQUAL
    //                          | BAR
    //                          | CARET
    //                          | EQUAL_EQUAL
    //                          | GREATER_THAN
    //                          | GREATER_THAN_EQUAL
    //                          | LESS_THAN
    //                          | LESS_THAN_EQUAL
    //                          | MINUS
    //                          | PERCENT
    //                          | PLUS
    //                          | SLASH
    //                          | TILDE;

    #[allow(dead_code, non_snake_case)]
    fn using_directive_operator__0_8_19(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ampersand,
                    TokenKind::Ampersand,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::asterisk,
                    TokenKind::Asterisk,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::bang_equal,
                    TokenKind::BangEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::bar,
                    TokenKind::Bar,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::caret,
                    TokenKind::Caret,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::equal_equal,
                    TokenKind::EqualEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than,
                    TokenKind::GreaterThan,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than_equal,
                    TokenKind::GreaterThanEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::less_than,
                    TokenKind::LessThan,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::less_than_equal,
                    TokenKind::LessThanEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus,
                    TokenKind::Minus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::percent,
                    TokenKind::Percent,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::plus,
                    TokenKind::Plus,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::slash,
                    TokenKind::Slash,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::tilde,
                    TokenKind::Tilde,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn using_directive_operator__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_19 {
            Some(self.using_directive_operator__0_8_19(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn using_directive_operator(&self, stream: &mut Stream) -> ParserResult {
        self.using_directive_operator__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // UsingDirectivePath = IdentifierPath;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn using_directive_path(&self, stream: &mut Stream) -> ParserResult {
        self.identifier_path(stream)
            .with_kind(RuleKind::UsingDirectivePath)
    }

    // (* v0.4.11 *)
    // UsingDirectiveSymbol = IdentifierPath;

    #[allow(dead_code, non_snake_case)]
    fn using_directive_symbol__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.identifier_path(stream)
            .with_kind(RuleKind::UsingDirectiveSymbol)
    }

    // (* v0.8.19 *)
    // UsingDirectiveSymbol = IdentifierPath (AS_KEYWORD «UsingDirectiveOperator»)?;

    #[allow(dead_code, non_snake_case)]
    fn using_directive_symbol__0_8_19(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.identifier_path(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::as_keyword,
                                TokenKind::AsKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result
                            .incorporate_sequence_result(self.using_directive_operator(stream));
                        break;
                    }
                    running_result
                }));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UsingDirectiveSymbol)
    }

    pub(crate) fn using_directive_symbol(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_19 {
            self.using_directive_symbol__0_8_19(stream)
        } else {
            self.using_directive_symbol__0_4_11(stream)
        }
    }

    // UsingDirectiveSymbolsList = UsingDirectiveSymbol (COMMA UsingDirectiveSymbol)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn using_directive_symbols_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.using_directive_symbol(stream))
                {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result
                                .incorporate_sequence_result(self.using_directive_symbol(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UsingDirectiveSymbolsList)
    }

    // (* v0.4.11 *)
    // VariableDeclaration = (VAR_KEYWORD | TypeName) «DataLocation»? IDENTIFIER;

    #[allow(dead_code, non_snake_case)]
    fn variable_declaration__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::var_keyword,
                            TokenKind::VarKeyword,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result(self.type_name(stream)) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                }) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.data_location(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::VariableDeclaration)
    }

    // (* v0.5.0 *)
    // VariableDeclaration = TypeName «DataLocation»? IDENTIFIER;

    #[allow(dead_code, non_snake_case)]
    fn variable_declaration__0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.data_location(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::VariableDeclaration)
    }

    pub(crate) fn variable_declaration(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.variable_declaration__0_5_0(stream)
        } else {
            self.variable_declaration__0_4_11(stream)
        }
    }

    // VariableDeclarationStatement = VariableDeclaration (EQUAL Expression)? SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn variable_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result
                            .incorporate_sequence_result(self.variable_declaration(stream))
                        {
                            break;
                        }
                        running_result.incorporate_sequence_result(transform_option_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::equal,
                                        TokenKind::Equal,
                                    ),
                                ) {
                                    break;
                                }
                                running_result.incorporate_sequence_result(self.expression(stream));
                                break;
                            }
                            running_result
                        }));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::semicolon,
                    TokenKind::Semicolon,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::VariableDeclarationStatement)
    }

    // VersionPragma = SOLIDITY_KEYWORD VersionPragmaExpressionsList;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::solidity_keyword,
                    TokenKind::SolidityKeyword,
                )) {
                    break;
                }
                running_result
                    .incorporate_sequence_result(self.version_pragma_expressions_list(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::VersionPragma)
    }

    // VersionPragmaExpression = VersionPragmaBinaryExpression (* VersionPragmaExpression «VersionPragmaOrOperator» VersionPragmaExpression *) (* Binary Operator, Left Associative *)
    //                         | VersionPragmaBinaryExpression (* VersionPragmaExpression «VersionPragmaRangeOperator» VersionPragmaExpression *) (* Binary Operator, Left Associative *)
    //                         | VersionPragmaUnaryExpression (* «VersionPragmaUnaryOperator» VersionPragmaExpression *) (* Unary Operator, Prefix *)
    //                         | VersionPragmaSpecifier;
    // VersionPragmaBinaryExpression = VersionPragmaExpression «VersionPragmaOrOperator» VersionPragmaExpression (* Binary Operator, Left Associative *);
    // VersionPragmaBinaryExpression = VersionPragmaExpression «VersionPragmaRangeOperator» VersionPragmaExpression (* Binary Operator, Left Associative *);
    // VersionPragmaUnaryExpression = «VersionPragmaUnaryOperator» VersionPragmaExpression (* Unary Operator, Prefix *);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma_expression(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut results: Vec<ParserResult> = Vec::new();
            let initial_result = loop {
                let result = loop {
                    let result = self
                        .version_pragma_unary_operator(stream)
                        .to_pratt_element_operator(
                            RuleKind::VersionPragmaUnaryExpression,
                            255,
                            5u8,
                        );
                    match result {
                        ParserResult::PrattOperatorMatch(_) => results.push(result),
                        _ => break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => {
                        break result;
                    }
                }
                {
                    let result = self.version_pragma_specifier(stream);
                    if result.is_match() {
                        results.push(result);
                    } else {
                        break result;
                    }
                }
                let result = loop {
                    let start_position = stream.position();
                    stream.set_position(start_position);
                    let next_result = self
                        .version_pragma_or_operator(stream)
                        .to_pratt_element_operator(
                            RuleKind::VersionPragmaBinaryExpression,
                            1u8,
                            1u8 + 1,
                        );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    let next_result = self
                        .version_pragma_range_operator(stream)
                        .to_pratt_element_operator(
                            RuleKind::VersionPragmaBinaryExpression,
                            3u8,
                            3u8 + 1,
                        );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    break ParserResult::no_match(vec![]);
                };
                match result {
                    ParserResult::PrattOperatorMatch(_) => results.push(result),
                    _ => break result,
                }
            };
            if results.is_empty() {
                break initial_result;
            }
            reduce_pratt_elements(
                |children| vec![cst::Node::rule(RuleKind::VersionPragmaExpression, children)],
                &mut results,
            );
            if results.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    results
                );
            }
            match results.remove(0) {
                ParserResult::Match(r#match) => {
                    if let ParserResult::IncompleteMatch(_) = initial_result {
                        break ParserResult::incomplete_match(r#match.nodes, vec![]);
                    } else {
                        break ParserResult::r#match(
                            r#match.nodes,
                            r#match.tokens_that_would_have_allowed_more_progress,
                        );
                    }
                }
                ParserResult::IncompleteMatch(incomplete_match) => {
                    if let ParserResult::IncompleteMatch(initial_incomplete_match) = initial_result
                    {
                        let mut nodes = incomplete_match.nodes;
                        nodes.extend(initial_incomplete_match.nodes);
                        break ParserResult::incomplete_match(
                            nodes,
                            initial_incomplete_match.tokens_that_would_have_allowed_more_progress,
                        );
                    } else {
                        break ParserResult::IncompleteMatch(incomplete_match);
                    }
                }
                _ => unreachable!("Pratt parser produced an invalid result"),
            }
        }
        .with_kind(RuleKind::VersionPragmaExpression)
    }

    // VersionPragmaExpressionsList = VersionPragmaExpression+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma_expressions_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result
                .incorporate_one_or_more_result(self.version_pragma_expression(stream))
            {
            }
            running_result
        }
        .with_kind(RuleKind::VersionPragmaExpressionsList)
    }

    // «VersionPragmaOrOperator» = BAR_BAR;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma_or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::bar_bar, TokenKind::BarBar)
    }

    // «VersionPragmaRangeOperator» = MINUS;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma_range_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::minus, TokenKind::Minus)
    }

    // VersionPragmaSpecifier = VERSION_PRAGMA_VALUE (PERIOD VERSION_PRAGMA_VALUE)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::version_pragma_value,
                    TokenKind::VersionPragmaValue,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::period,
                                    TokenKind::Period,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::version_pragma_value,
                                    TokenKind::VersionPragmaValue,
                                ),
                            );
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::VersionPragmaSpecifier)
    }

    // «VersionPragmaUnaryOperator» = CARET
    //                              | TILDE
    //                              | EQUAL
    //                              | LESS_THAN
    //                              | GREATER_THAN
    //                              | LESS_THAN_EQUAL
    //                              | GREATER_THAN_EQUAL;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma_unary_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::caret,
                    TokenKind::Caret,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::tilde,
                    TokenKind::Tilde,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::equal,
                    TokenKind::Equal,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::less_than,
                    TokenKind::LessThan,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than,
                    TokenKind::GreaterThan,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::less_than_equal,
                    TokenKind::LessThanEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than_equal,
                    TokenKind::GreaterThanEqual,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // WhileStatement = WHILE_KEYWORD OPEN_PAREN Expression CLOSE_PAREN Statement;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn while_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::while_keyword,
                    TokenKind::WhileKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::open_paren,
                                TokenKind::OpenParen,
                            ),
                        ) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result(self.expression(stream)) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                            stream,
                            &Self::close_paren,
                            TokenKind::CloseParen,
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.statement(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::WhileStatement)
    }

    // YulAssignmentStatement = YulIdentifierPathsList COLON_EQUAL YulExpression;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_assignment_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result
                    .incorporate_sequence_result(self.yul_identifier_paths_list(stream))
                {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::colon_equal,
                    TokenKind::ColonEqual,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.yul_expression(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulAssignmentStatement)
    }

    // YulBlock = OPEN_BRACE YulStatementsList? CLOSE_BRACE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_block(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_brace,
                    TokenKind::OpenBrace,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.yul_statements_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_brace,
                    TokenKind::CloseBrace,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulBlock)
    }

    // YulBreakStatement = BREAK_KEYWORD;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_break_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::break_keyword, TokenKind::BreakKeyword)
            .with_kind(RuleKind::YulBreakStatement)
    }

    // YulContinueStatement = CONTINUE_KEYWORD;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_continue_statement(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::continue_keyword, TokenKind::ContinueKeyword)
            .with_kind(RuleKind::YulContinueStatement)
    }

    // YulDeclarationStatement = LET_KEYWORD YulIdentifierPathsList (COLON_EQUAL YulExpression)?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::let_keyword,
                    TokenKind::LetKeyword,
                )) {
                    break;
                }
                if !running_result
                    .incorporate_sequence_result(self.yul_identifier_paths_list(stream))
                {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::colon_equal,
                                TokenKind::ColonEqual,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.yul_expression(stream));
                        break;
                    }
                    running_result
                }));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulDeclarationStatement)
    }

    // YulExpression = YulFunctionCallExpression (* YulExpression «YulFunctionCallOperator» *) (* Unary Operator, Postfix *)
    //               | «YulLiteral»
    //               | YulIdentifierPath;
    // YulFunctionCallExpression = YulExpression «YulFunctionCallOperator» (* Unary Operator, Postfix *);

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_expression(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut results: Vec<ParserResult> = Vec::new();
            let initial_result = loop {
                {
                    let result = {
                        let mut running_result = ParserResult::no_match(vec![]);
                        let start_position = stream.position();
                        loop {
                            if running_result.incorporate_choice_result(self.yul_literal(stream)) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result
                                .incorporate_choice_result(self.yul_identifier_path(stream))
                            {
                                break;
                            }
                            stream.set_position(start_position);
                            break;
                        }
                        if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                            incomplete_match.consume_stream(stream);
                        }
                        running_result
                    };
                    if result.is_match() {
                        results.push(result);
                    } else {
                        break result;
                    }
                }
                let result = loop {
                    let result = self
                        .yul_function_call_operator(stream)
                        .to_pratt_element_operator(RuleKind::YulFunctionCallExpression, 1u8, 255);
                    match result {
                        ParserResult::PrattOperatorMatch(_) => results.push(result),
                        _ => break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => {
                        break result;
                    }
                }
                break ParserResult::no_match(vec![]);
            };
            if results.is_empty() {
                break initial_result;
            }
            reduce_pratt_elements(
                |children| vec![cst::Node::rule(RuleKind::YulExpression, children)],
                &mut results,
            );
            if results.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    results
                );
            }
            match results.remove(0) {
                ParserResult::Match(r#match) => {
                    if let ParserResult::IncompleteMatch(_) = initial_result {
                        break ParserResult::incomplete_match(r#match.nodes, vec![]);
                    } else {
                        break ParserResult::r#match(
                            r#match.nodes,
                            r#match.tokens_that_would_have_allowed_more_progress,
                        );
                    }
                }
                ParserResult::IncompleteMatch(incomplete_match) => {
                    if let ParserResult::IncompleteMatch(initial_incomplete_match) = initial_result
                    {
                        let mut nodes = incomplete_match.nodes;
                        nodes.extend(initial_incomplete_match.nodes);
                        break ParserResult::incomplete_match(
                            nodes,
                            initial_incomplete_match.tokens_that_would_have_allowed_more_progress,
                        );
                    } else {
                        break ParserResult::IncompleteMatch(incomplete_match);
                    }
                }
                _ => unreachable!("Pratt parser produced an invalid result"),
            }
        }
        .with_kind(RuleKind::YulExpression)
    }

    // YulExpressionsList = YulExpression (COMMA YulExpression)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_expressions_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.yul_expression(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(self.yul_expression(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulExpressionsList)
    }

    // YulForStatement = FOR_KEYWORD YulBlock YulExpression YulBlock YulBlock;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_for_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::for_keyword,
                    TokenKind::ForKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.yul_block(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.yul_expression(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.yul_block(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(self.yul_block(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulForStatement)
    }

    // «YulFunctionCallOperator» = OPEN_PAREN YulExpressionsList? CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_function_call_operator(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_paren,
                    TokenKind::OpenParen,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.yul_expressions_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_paren,
                    TokenKind::CloseParen,
                ));
                break;
            }
            running_result
        }
    }

    // YulFunctionDefinition = FUNCTION_KEYWORD YUL_IDENTIFIER YulParametersDeclaration YulReturnsDeclaration? YulBlock;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_function_definition(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::function_keyword,
                    TokenKind::FunctionKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::yul_identifier,
                    TokenKind::YulIdentifier,
                )) {
                    break;
                }
                if !running_result
                    .incorporate_sequence_result(self.yul_parameters_declaration(stream))
                {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.yul_returns_declaration(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.yul_block(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulFunctionDefinition)
    }

    // YulIdentifierPath = YUL_IDENTIFIER (PERIOD YUL_IDENTIFIER)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_identifier_path(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::yul_identifier,
                    TokenKind::YulIdentifier,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::period,
                                    TokenKind::Period,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::yul_identifier,
                                    TokenKind::YulIdentifier,
                                ),
                            );
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulIdentifierPath)
    }

    // YulIdentifierPathsList = YulIdentifierPath (COMMA YulIdentifierPath)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_identifier_paths_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.yul_identifier_path(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result
                                .incorporate_sequence_result(self.yul_identifier_path(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulIdentifierPathsList)
    }

    // YulIdentifiersList = YUL_IDENTIFIER (COMMA YUL_IDENTIFIER)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_identifiers_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::yul_identifier,
                    TokenKind::YulIdentifier,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::comma,
                                    TokenKind::Comma,
                                ),
                            ) {
                                break;
                            }
                            running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::yul_identifier,
                                    TokenKind::YulIdentifier,
                                ),
                            );
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulIdentifiersList)
    }

    // YulIfStatement = IF_KEYWORD YulExpression YulBlock;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_if_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::if_keyword,
                    TokenKind::IfKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.yul_expression(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(self.yul_block(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulIfStatement)
    }

    // (* v0.6.0 *)
    // YulLeaveStatement = LEAVE_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn yul_leave_statement__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::leave_keyword, TokenKind::LeaveKeyword)
            .with_kind(RuleKind::YulLeaveStatement)
    }

    #[allow(non_snake_case)]
    pub(crate) fn yul_leave_statement__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            Some(self.yul_leave_statement__0_6_0(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn yul_leave_statement(&self, stream: &mut Stream) -> ParserResult {
        self.yul_leave_statement__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // «YulLiteral» = TRUE_KEYWORD
    //              | FALSE_KEYWORD
    //              | YUL_HEX_LITERAL
    //              | YUL_DECIMAL_LITERAL
    //              | HEX_STRING_LITERAL
    //              | ASCII_STRING_LITERAL;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_literal(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::true_keyword,
                    TokenKind::TrueKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::false_keyword,
                    TokenKind::FalseKeyword,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::yul_hex_literal,
                    TokenKind::YulHexLiteral,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::yul_decimal_literal,
                    TokenKind::YulDecimalLiteral,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::hex_string_literal,
                    TokenKind::HexStringLiteral,
                )) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ascii_string_literal,
                    TokenKind::AsciiStringLiteral,
                )) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
    }

    // YulParametersDeclaration = OPEN_PAREN YulIdentifiersList? CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_parameters_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::open_paren,
                    TokenKind::OpenParen,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.yul_identifiers_list(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::close_paren,
                    TokenKind::CloseParen,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulParametersDeclaration)
    }

    // YulReturnsDeclaration = MINUS_GREATER_THAN YulIdentifiersList;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_returns_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus_greater_than,
                    TokenKind::MinusGreaterThan,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.yul_identifiers_list(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulReturnsDeclaration)
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

    #[allow(dead_code, non_snake_case)]
    fn yul_statement__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.yul_block(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_declaration_statement(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_assignment_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_if_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_for_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_switch_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_break_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_continue_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
        .with_kind(RuleKind::YulStatement)
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

    #[allow(dead_code, non_snake_case)]
    fn yul_statement__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.yul_block(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_function_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_declaration_statement(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_assignment_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_if_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_for_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_switch_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_leave_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_break_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_continue_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.yul_expression(stream)) {
                    break;
                }
                stream.set_position(start_position);
                break;
            }
            if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                incomplete_match.consume_stream(stream);
            }
            running_result
        }
        .with_kind(RuleKind::YulStatement)
    }

    pub(crate) fn yul_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.yul_statement__0_6_0(stream)
        } else {
            self.yul_statement__0_4_11(stream)
        }
    }

    // YulStatementsList = YulStatement+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_statements_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.yul_statement(stream)) {}
            running_result
        }
        .with_kind(RuleKind::YulStatementsList)
    }

    // YulSwitchCase = (DEFAULT_KEYWORD | (CASE_KEYWORD «YulLiteral»)) YulBlock;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_switch_case(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::default_keyword,
                            TokenKind::DefaultKeyword,
                        )) {
                            break;
                        }
                        stream.set_position(start_position);
                        if running_result.incorporate_choice_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::case_keyword,
                                        TokenKind::CaseKeyword,
                                    ),
                                ) {
                                    break;
                                }
                                running_result
                                    .incorporate_sequence_result(self.yul_literal(stream));
                                break;
                            }
                            running_result
                        }) {
                            break;
                        }
                        stream.set_position(start_position);
                        break;
                    }
                    if let ParserResult::IncompleteMatch(incomplete_match) = &running_result {
                        incomplete_match.consume_stream(stream);
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.yul_block(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulSwitchCase)
    }

    // YulSwitchCasesList = YulSwitchCase+;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_switch_cases_list(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            while running_result.incorporate_one_or_more_result(self.yul_switch_case(stream)) {}
            running_result
        }
        .with_kind(RuleKind::YulSwitchCasesList)
    }

    // YulSwitchStatement = SWITCH_KEYWORD YulExpression YulSwitchCasesList;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_switch_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::switch_keyword,
                    TokenKind::SwitchKeyword,
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.yul_expression(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(self.yul_switch_cases_list(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::YulSwitchStatement)
    }
}
