// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::cst;
use super::kinds::*;
use super::language::Language;
use super::parser_helpers::*;
use super::parser_result::*;
use super::stream::*;

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

    // AddSubOperator = PLUS | MINUS;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus,
                    TokenKind::Minus,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::AddSubOperator)
    }

    // AddressType = ADDRESS_KEYWORD PAYABLE_KEYWORD?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn address_type(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::address_keyword,
                    TokenKind::AddressKeyword,
                )) {
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
        }
        .with_kind(RuleKind::AddressType)
    }

    // AndOperator = AMPERSAND_AMPERSAND;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn and_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            &Self::ampersand_ampersand,
            TokenKind::AmpersandAmpersand,
        )
        .with_kind(RuleKind::AndOperator)
    }

    // ArgumentList = OPEN_PAREN (PositionalArgumentList | NamedArgumentList)? CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn argument_list(&self, stream: &mut Stream) -> ParserResult {
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
                            .incorporate_choice_result(self.positional_argument_list(stream))
                        {
                            break;
                        }
                        stream.set_position(start_position);
                        running_result.incorporate_choice_result(self.named_argument_list(stream));
                        break;
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
        .with_kind(RuleKind::ArgumentList)
    }

    // ArrayLiteral = OPEN_BRACKET Expression (COMMA Expression)* CLOSE_BRACKET;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn array_literal(&self, stream: &mut Stream) -> ParserResult {
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
                                    running_result
                                        .incorporate_sequence_result(self.expression(stream));
                                    break;
                                }
                                running_result
                            }) {}
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
                    &Self::close_bracket,
                    TokenKind::CloseBracket,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ArrayLiteral)
    }

    // AssemblyFlags = OPEN_PAREN DOUBLE_QUOTED_ASCII_STRING_LITERAL (COMMA DOUBLE_QUOTED_ASCII_STRING_LITERAL)* CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn assembly_flags(&self, stream: &mut Stream) -> ParserResult {
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
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::double_quoted_ascii_string_literal,
                                TokenKind::DoubleQuotedAsciiStringLiteral,
                            ),
                        ) {
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
                                            &Self::double_quoted_ascii_string_literal,
                                            TokenKind::DoubleQuotedAsciiStringLiteral,
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
        }
        .with_kind(RuleKind::AssemblyFlags)
    }

    // AssemblyStatement = ASSEMBLY_KEYWORD EVMASM? AssemblyFlags? YulBlock;

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
                    self.parse_token_with_trivia(stream, &Self::evmasm, TokenKind::Evmasm),
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.assembly_flags(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.yul_block(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::AssemblyStatement)
    }

    // AssignmentOperator = EQUAL
    //                    | BAR_EQUAL
    //                    | CARET_EQUAL
    //                    | AMPERSAND_EQUAL
    //                    | LESS_THAN_LESS_THAN_EQUAL
    //                    | GREATER_THAN_GREATER_THAN_EQUAL
    //                    | GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL
    //                    | PLUS_EQUAL
    //                    | MINUS_EQUAL
    //                    | ASTERISK_EQUAL
    //                    | SLASH_EQUAL
    //                    | PERCENT_EQUAL;

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
                    &Self::caret_equal,
                    TokenKind::CaretEqual,
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
                    &Self::asterisk_equal,
                    TokenKind::AsteriskEqual,
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::percent_equal,
                    TokenKind::PercentEqual,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::AssignmentOperator)
    }

    // AsteriskImport = ASTERISK ImportAlias FROM_KEYWORD ImportPath;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn asterisk_import(&self, stream: &mut Stream) -> ParserResult {
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
                if !running_result.incorporate_sequence_result(self.import_alias(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::from_keyword,
                    TokenKind::FromKeyword,
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.import_path(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::AsteriskImport)
    }

    // BitAndOperator = AMPERSAND;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bit_and_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::ampersand, TokenKind::Ampersand)
            .with_kind(RuleKind::BitAndOperator)
    }

    // BitOrOperator = BAR;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bit_or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::bar, TokenKind::Bar)
            .with_kind(RuleKind::BitOrOperator)
    }

    // BitXOrOperator = CARET;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn bit_x_or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::caret, TokenKind::Caret)
            .with_kind(RuleKind::BitXOrOperator)
    }

    // (* v0.4.11 *)
    // Block = OPEN_BRACE Statement* CLOSE_BRACE;

    #[allow(dead_code, non_snake_case)]
    fn block__0_4_11(&self, stream: &mut Stream) -> ParserResult {
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
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result(self.statement(stream)) {}
                    running_result
                }) {
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

    // (* v0.8.0 *)
    // Block = OPEN_BRACE (Statement | UncheckedBlock)* CLOSE_BRACE;

    #[allow(dead_code, non_snake_case)]
    fn block__0_8_0(&self, stream: &mut Stream) -> ParserResult {
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
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::no_match(vec![]);
                        let start_position = stream.position();
                        loop {
                            if running_result.incorporate_choice_result(self.statement(stream)) {
                                break;
                            }
                            stream.set_position(start_position);
                            running_result.incorporate_choice_result(self.unchecked_block(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                }) {
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

    pub(crate) fn block(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            self.block__0_8_0(stream)
        } else {
            self.block__0_4_11(stream)
        }
    }

    // BooleanLiteral = TRUE_KEYWORD | FALSE_KEYWORD;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn boolean_literal(&self, stream: &mut Stream) -> ParserResult {
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::false_keyword,
                    TokenKind::FalseKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::BooleanLiteral)
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
    // CatchClause = CATCH_KEYWORD (IDENTIFIER? ParameterList)? Block;

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
                if !running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::identifier,
                                TokenKind::Identifier,
                            ),
                        )) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parameter_list(stream));
                        break;
                    }
                    running_result
                })) {
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

    // ConditionalOperator = QUESTION_MARK Expression COLON Expression;

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
        .with_kind(RuleKind::ConditionalOperator)
    }

    // ConstantDefinition = TypeName CONSTANT_KEYWORD IDENTIFIER EQUAL Expression SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn constant_definition(&self, stream: &mut Stream) -> ParserResult {
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

    // (* v0.4.22 *)
    // ConstructorAttribute = ModifierInvocation
    //                      | INTERNAL_KEYWORD
    //                      | PAYABLE_KEYWORD
    //                      | PUBLIC_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ConstructorAttribute)
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
    // ConstructorDefinition = CONSTRUCTOR_KEYWORD ParameterList ConstructorAttribute* Block;

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
                if !running_result.incorporate_sequence_result(self.parameter_list(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result
                        .incorporate_zero_or_more_result(self.constructor_attribute(stream))
                    {
                    }
                    running_result
                }) {
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
    // «ContractBodyElement» = UsingDirective
    //                       | FunctionDefinition
    //                       | UnnamedFunctionDefinition
    //                       | ModifierDefinition
    //                       | StructDefinition
    //                       | EnumDefinition
    //                       | EventDefinition
    //                       | ErrorDefinition
    //                       | StateVariableDeclaration;

    #[allow(dead_code, non_snake_case)]
    fn contract_body_element__0_4_11(&self, stream: &mut Stream) -> ParserResult {
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
                if running_result.incorporate_choice_result(self.error_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result(self.state_variable_declaration(stream));
                break;
            }
            running_result
        }
    }

    // (* v0.4.22 *)
    // «ContractBodyElement» = UsingDirective
    //                       | ConstructorDefinition
    //                       | FunctionDefinition
    //                       | UnnamedFunctionDefinition
    //                       | ModifierDefinition
    //                       | StructDefinition
    //                       | EnumDefinition
    //                       | EventDefinition
    //                       | ErrorDefinition
    //                       | StateVariableDeclaration;

    #[allow(dead_code, non_snake_case)]
    fn contract_body_element__0_4_22(&self, stream: &mut Stream) -> ParserResult {
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
                if running_result.incorporate_choice_result(self.error_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result(self.state_variable_declaration(stream));
                break;
            }
            running_result
        }
    }

    // (* v0.6.0 *)
    // «ContractBodyElement» = UsingDirective
    //                       | ConstructorDefinition
    //                       | FunctionDefinition
    //                       | FallbackFunctionDefinition
    //                       | ReceiveFunctionDefinition
    //                       | ModifierDefinition
    //                       | StructDefinition
    //                       | EnumDefinition
    //                       | EventDefinition
    //                       | ErrorDefinition
    //                       | StateVariableDeclaration;

    #[allow(dead_code, non_snake_case)]
    fn contract_body_element__0_6_0(&self, stream: &mut Stream) -> ParserResult {
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
                running_result.incorporate_choice_result(self.state_variable_declaration(stream));
                break;
            }
            running_result
        }
    }

    // (* v0.8.8 *)
    // «ContractBodyElement» = UsingDirective
    //                       | ConstructorDefinition
    //                       | FunctionDefinition
    //                       | FallbackFunctionDefinition
    //                       | ReceiveFunctionDefinition
    //                       | ModifierDefinition
    //                       | StructDefinition
    //                       | EnumDefinition
    //                       | UserDefinedValueTypeDefinition
    //                       | EventDefinition
    //                       | ErrorDefinition
    //                       | StateVariableDeclaration;

    #[allow(dead_code, non_snake_case)]
    fn contract_body_element__0_8_8(&self, stream: &mut Stream) -> ParserResult {
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
                if running_result
                    .incorporate_choice_result(self.user_defined_value_type_definition(stream))
                {
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
                running_result.incorporate_choice_result(self.state_variable_declaration(stream));
                break;
            }
            running_result
        }
    }

    pub(crate) fn contract_body_element(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_8 {
            self.contract_body_element__0_8_8(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.contract_body_element__0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_4_22 {
            self.contract_body_element__0_4_22(stream)
        } else {
            self.contract_body_element__0_4_11(stream)
        }
    }

    // (* v0.4.11 *)
    // ContractDefinition = CONTRACT_KEYWORD IDENTIFIER InheritanceSpecifierList? OPEN_BRACE ContractBodyElements CLOSE_BRACE;
    // ContractBodyElements = «ContractBodyElement»*;

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
                    self.inheritance_specifier_list(stream),
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
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            while running_result
                                .incorporate_zero_or_more_result(self.contract_body_element(stream))
                            {
                            }
                            running_result.with_kind(RuleKind::ContractBodyElements)
                        }) {
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
    // ContractDefinition = ABSTRACT_KEYWORD? CONTRACT_KEYWORD IDENTIFIER InheritanceSpecifierList? OPEN_BRACE ContractBodyElements CLOSE_BRACE;
    // ContractBodyElements = «ContractBodyElement»*;

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
                    self.inheritance_specifier_list(stream),
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
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            while running_result
                                .incorporate_zero_or_more_result(self.contract_body_element(stream))
                            {
                            }
                            running_result.with_kind(RuleKind::ContractBodyElements)
                        }) {
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
    // DataLocation = MEMORY_KEYWORD | STORAGE_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::storage_keyword,
                    TokenKind::StorageKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::DataLocation)
    }

    // (* v0.5.0 *)
    // DataLocation = MEMORY_KEYWORD
    //              | STORAGE_KEYWORD
    //              | CALLDATA_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::calldata_keyword,
                    TokenKind::CalldataKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::DataLocation)
    }

    pub(crate) fn data_location(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.data_location__0_5_0(stream)
        } else {
            self.data_location__0_4_11(stream)
        }
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

    #[allow(dead_code, non_snake_case)]
    fn definition__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.constant_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.contract_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.error_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
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
                running_result.incorporate_choice_result(self.struct_definition(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::Definition)
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

    #[allow(dead_code, non_snake_case)]
    fn definition__0_8_8(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.constant_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.contract_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.enum_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.error_definition(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.function_definition(stream)) {
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
                running_result
                    .incorporate_choice_result(self.user_defined_value_type_definition(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::Definition)
    }

    pub(crate) fn definition(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_8 {
            self.definition__0_8_8(stream)
        } else {
            self.definition__0_4_11(stream)
        }
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

    // Directive = PragmaDirective
    //           | ImportDirective
    //           | UsingDirective;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn directive(&self, stream: &mut Stream) -> ParserResult {
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
                running_result.incorporate_choice_result(self.using_directive(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::Directive)
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
    // ElementaryType = BOOL_KEYWORD
    //                | STRING_KEYWORD
    //                | AddressType
    //                | PayableType
    //                | BYTE_TYPE
    //                | FIXED_BYTES_TYPE
    //                | SIGNED_INTEGER_TYPE
    //                | UNSIGNED_INTEGER_TYPE
    //                | SIGNED_FIXED_TYPE
    //                | UNSIGNED_FIXED_TYPE;

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
                if running_result.incorporate_choice_result(self.payable_type(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::byte_type,
                    TokenKind::ByteType,
                )) {
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::unsigned_fixed_type,
                    TokenKind::UnsignedFixedType,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ElementaryType)
    }

    // (* v0.8.0 *)
    // ElementaryType = BOOL_KEYWORD
    //                | STRING_KEYWORD
    //                | AddressType
    //                | PayableType
    //                | FIXED_BYTES_TYPE
    //                | SIGNED_INTEGER_TYPE
    //                | UNSIGNED_INTEGER_TYPE
    //                | SIGNED_FIXED_TYPE
    //                | UNSIGNED_FIXED_TYPE;

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
                if running_result.incorporate_choice_result(self.payable_type(stream)) {
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::unsigned_fixed_type,
                    TokenKind::UnsignedFixedType,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ElementaryType)
    }

    pub(crate) fn elementary_type(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            self.elementary_type__0_8_0(stream)
        } else {
            self.elementary_type__0_4_11(stream)
        }
    }

    // (* v0.4.21 *)
    // EmitStatement = EMIT_KEYWORD IdentifierPath ArgumentList SEMICOLON;

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
                        running_result.incorporate_sequence_result(self.argument_list(stream));
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
                    running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::single_line_comment,
                        TokenKind::SingleLineComment,
                    ));
                    break;
                }
                running_result
            }) {}
            running_result
        }
        .with_kind(RuleKind::EndOfFileTrivia)
    }

    // EnumDefinition = ENUM_KEYWORD IDENTIFIER OPEN_BRACE (IDENTIFIER (COMMA IDENTIFIER)*)? CLOSE_BRACE;

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
                        if !running_result.incorporate_sequence_result(transform_option_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
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
                                    while running_result.incorporate_zero_or_more_result({
                                        let mut running_result =
                                            ParserResult::r#match(vec![], vec![]);
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
                        })) {
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

    // EqualityComparisonOperator = EQUAL_EQUAL | BANG_EQUAL;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::bang_equal,
                    TokenKind::BangEqual,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::EqualityComparisonOperator)
    }

    // ErrorDefinition = ERROR_KEYWORD IDENTIFIER OPEN_PAREN (ErrorParameter (COMMA ErrorParameter)*)? CLOSE_PAREN SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn error_definition(&self, stream: &mut Stream) -> ParserResult {
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
                                    transform_option_result({
                                        let mut running_result =
                                            ParserResult::r#match(vec![], vec![]);
                                        loop {
                                            if !running_result.incorporate_sequence_result(
                                                self.error_parameter(stream),
                                            ) {
                                                break;
                                            }
                                            running_result.incorporate_sequence_result({
                                                let mut running_result =
                                                    ParserResult::r#match(vec![], vec![]);
                                                while running_result
                                                    .incorporate_zero_or_more_result({
                                                        let mut running_result =
                                                            ParserResult::r#match(vec![], vec![]);
                                                        loop {
                                                            if !running_result
                                                                .incorporate_sequence_result(
                                                                    self.parse_token_with_trivia(
                                                                        stream,
                                                                        &Self::comma,
                                                                        TokenKind::Comma,
                                                                    ),
                                                                )
                                                            {
                                                                break;
                                                            }
                                                            running_result
                                                                .incorporate_sequence_result(
                                                                    self.error_parameter(stream),
                                                                );
                                                            break;
                                                        }
                                                        running_result
                                                    })
                                                {
                                                }
                                                running_result
                                            });
                                            break;
                                        }
                                        running_result
                                    }),
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

    // ErrorParameter = TypeName IDENTIFIER?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn error_parameter(&self, stream: &mut Stream) -> ParserResult {
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

    // EventDefinition = EVENT_KEYWORD IDENTIFIER OPEN_PAREN (EventParameter (COMMA EventParameter)*)? CLOSE_PAREN ANONYMOUS_KEYWORD? SEMICOLON;

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
                                    transform_option_result({
                                        let mut running_result =
                                            ParserResult::r#match(vec![], vec![]);
                                        loop {
                                            if !running_result.incorporate_sequence_result(
                                                self.event_parameter(stream),
                                            ) {
                                                break;
                                            }
                                            running_result.incorporate_sequence_result({
                                                let mut running_result =
                                                    ParserResult::r#match(vec![], vec![]);
                                                while running_result
                                                    .incorporate_zero_or_more_result({
                                                        let mut running_result =
                                                            ParserResult::r#match(vec![], vec![]);
                                                        loop {
                                                            if !running_result
                                                                .incorporate_sequence_result(
                                                                    self.parse_token_with_trivia(
                                                                        stream,
                                                                        &Self::comma,
                                                                        TokenKind::Comma,
                                                                    ),
                                                                )
                                                            {
                                                                break;
                                                            }
                                                            running_result
                                                                .incorporate_sequence_result(
                                                                    self.event_parameter(stream),
                                                                );
                                                            break;
                                                        }
                                                        running_result
                                                    })
                                                {
                                                }
                                                running_result
                                            });
                                            break;
                                        }
                                        running_result
                                    }),
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

    // ExperimentalPragma = EXPERIMENTAL_KEYWORD IDENTIFIER;

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
                running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ExperimentalPragma)
    }

    // ExponentiationOperator = ASTERISK_ASTERISK;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn exponentiation_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            &Self::asterisk_asterisk,
            TokenKind::AsteriskAsterisk,
        )
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

    #[allow(dead_code, non_snake_case)]
    fn expression__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut elements: Vec<ParserResult> = Vec::new();
            let result = loop {
                let result = loop {
                    let result = self
                        .unary_prefix_operator(stream)
                        .to_pratt_element_operator(RuleKind::UnaryPrefixExpression, 255, 29u8);
                    match result {
                        ParserResult::PrattOperatorMatch(_) => elements.push(result),
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
                        elements.push(result);
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
                        ParserResult::PrattOperatorMatch(_) => elements.push(result),
                        _ => break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => {
                        break result;
                    }
                }
                let result = loop {
                    let start_position = stream.position();
                    stream.set_position(start_position);
                    let next_result = self.assignment_operator(stream).to_pratt_element_operator(
                        RuleKind::AssignmentExpression,
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
                    let next_result = self.or_operator(stream).to_pratt_element_operator(
                        RuleKind::OrExpression,
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
                        RuleKind::AndExpression,
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
                        .to_pratt_element_operator(
                            RuleKind::EqualityComparisonExpression,
                            9u8,
                            9u8 + 1,
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
                        .order_comparison_operator(stream)
                        .to_pratt_element_operator(
                            RuleKind::OrderComparisonExpression,
                            11u8,
                            11u8 + 1,
                        );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    let next_result = self.bit_or_operator(stream).to_pratt_element_operator(
                        RuleKind::BitOrExpression,
                        13u8,
                        13u8 + 1,
                    );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    let next_result = self.bit_x_or_operator(stream).to_pratt_element_operator(
                        RuleKind::BitXOrExpression,
                        15u8,
                        15u8 + 1,
                    );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    let next_result = self.bit_and_operator(stream).to_pratt_element_operator(
                        RuleKind::BitAndExpression,
                        17u8,
                        17u8 + 1,
                    );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    let next_result = self.shift_operator(stream).to_pratt_element_operator(
                        RuleKind::ShiftExpression,
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
                        RuleKind::AddSubExpression,
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
                    let next_result = self.mul_div_mod_operator(stream).to_pratt_element_operator(
                        RuleKind::MulDivModExpression,
                        23u8,
                        23u8 + 1,
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
                        .exponentiation_operator(stream)
                        .to_pratt_element_operator(
                            RuleKind::ExponentiationExpression,
                            25u8,
                            25u8 + 1,
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
                    ParserResult::PrattOperatorMatch(_) => elements.push(result),
                    _ => break result,
                }
            };
            if elements.is_empty() {
                break result;
            }
            reduce_pratt_elements(
                |children| vec![cst::Node::rule(RuleKind::Expression, children)],
                &mut elements,
            );
            if elements.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    elements
                );
            }
            if let ParserResult::Match(r#match) = elements.remove(0) {
                if let ParserResult::IncompleteMatch(_) = result {
                    break ParserResult::incomplete_match(r#match.nodes, vec![]);
                } else {
                    break ParserResult::r#match(
                        r#match.nodes,
                        r#match.tokens_that_would_have_allowed_more_progress,
                    );
                }
            } else {
                unreachable!("Pratt parser failed to reduce to a single match")
            }
        }
        .with_kind(RuleKind::Expression)
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

    #[allow(dead_code, non_snake_case)]
    fn expression__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut elements: Vec<ParserResult> = Vec::new();
            let result = loop {
                let result = loop {
                    let result = self
                        .unary_prefix_operator(stream)
                        .to_pratt_element_operator(RuleKind::UnaryPrefixExpression, 255, 29u8);
                    match result {
                        ParserResult::PrattOperatorMatch(_) => elements.push(result),
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
                        elements.push(result);
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
                        ParserResult::PrattOperatorMatch(_) => elements.push(result),
                        _ => break result,
                    }
                };
                match result {
                    ParserResult::NoMatch(_) => {}
                    _ => {
                        break result;
                    }
                }
                let result = loop {
                    let start_position = stream.position();
                    stream.set_position(start_position);
                    let next_result = self.assignment_operator(stream).to_pratt_element_operator(
                        RuleKind::AssignmentExpression,
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
                    let next_result = self.or_operator(stream).to_pratt_element_operator(
                        RuleKind::OrExpression,
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
                        RuleKind::AndExpression,
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
                        .to_pratt_element_operator(
                            RuleKind::EqualityComparisonExpression,
                            9u8,
                            9u8 + 1,
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
                        .order_comparison_operator(stream)
                        .to_pratt_element_operator(
                            RuleKind::OrderComparisonExpression,
                            11u8,
                            11u8 + 1,
                        );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    let next_result = self.bit_or_operator(stream).to_pratt_element_operator(
                        RuleKind::BitOrExpression,
                        13u8,
                        13u8 + 1,
                    );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    let next_result = self.bit_x_or_operator(stream).to_pratt_element_operator(
                        RuleKind::BitXOrExpression,
                        15u8,
                        15u8 + 1,
                    );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    let next_result = self.bit_and_operator(stream).to_pratt_element_operator(
                        RuleKind::BitAndExpression,
                        17u8,
                        17u8 + 1,
                    );
                    match next_result {
                        ParserResult::PrattOperatorMatch(_) => break next_result,
                        ParserResult::Match(_) => unreachable!(
                            "ParserResult::Match isn't constructed when parsing operators"
                        ),
                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {}
                    }
                    stream.set_position(start_position);
                    let next_result = self.shift_operator(stream).to_pratt_element_operator(
                        RuleKind::ShiftExpression,
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
                        RuleKind::AddSubExpression,
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
                    let next_result = self.mul_div_mod_operator(stream).to_pratt_element_operator(
                        RuleKind::MulDivModExpression,
                        23u8,
                        23u8 + 1,
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
                        .exponentiation_operator(stream)
                        .to_pratt_element_operator(
                            RuleKind::ExponentiationExpression,
                            25u8 + 1,
                            25u8,
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
                    ParserResult::PrattOperatorMatch(_) => elements.push(result),
                    _ => break result,
                }
            };
            if elements.is_empty() {
                break result;
            }
            reduce_pratt_elements(
                |children| vec![cst::Node::rule(RuleKind::Expression, children)],
                &mut elements,
            );
            if elements.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    elements
                );
            }
            if let ParserResult::Match(r#match) = elements.remove(0) {
                if let ParserResult::IncompleteMatch(_) = result {
                    break ParserResult::incomplete_match(r#match.nodes, vec![]);
                } else {
                    break ParserResult::r#match(
                        r#match.nodes,
                        r#match.tokens_that_would_have_allowed_more_progress,
                    );
                }
            } else {
                unreachable!("Pratt parser failed to reduce to a single match")
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
    // FallbackFunctionAttribute = ModifierInvocation
    //                           | OverrideSpecifier
    //                           | EXTERNAL_KEYWORD
    //                           | PAYABLE_KEYWORD
    //                           | PURE_KEYWORD
    //                           | VIEW_KEYWORD
    //                           | VIRTUAL_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::virtual_keyword,
                    TokenKind::VirtualKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FallbackFunctionAttribute)
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
    // FallbackFunctionDefinition = FALLBACK_KEYWORD ParameterList FallbackFunctionAttribute* (RETURNS_KEYWORD ParameterList)? (SEMICOLON | Block);

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
                if !running_result.incorporate_sequence_result(self.parameter_list(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result
                        .incorporate_zero_or_more_result(self.fallback_function_attribute(stream))
                    {
                    }
                    running_result
                }) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::returns_keyword,
                                TokenKind::ReturnsKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parameter_list(stream));
                        break;
                    }
                    running_result
                })) {
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
                        running_result.incorporate_choice_result(self.block(stream));
                        break;
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

    // ForStatement = FOR_KEYWORD OPEN_PAREN (SimpleStatement | SEMICOLON) (ExpressionStatement | SEMICOLON) Expression? CLOSE_PAREN Statement;

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
                                        running_result.incorporate_choice_result(
                                            self.parse_token_with_trivia(
                                                stream,
                                                &Self::semicolon,
                                                TokenKind::Semicolon,
                                            ),
                                        );
                                        break;
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
                                        running_result.incorporate_choice_result(
                                            self.parse_token_with_trivia(
                                                stream,
                                                &Self::semicolon,
                                                TokenKind::Semicolon,
                                            ),
                                        );
                                        break;
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
    // FunctionAttribute = CONSTANT_KEYWORD
    //                   | EXTERNAL_KEYWORD
    //                   | INTERNAL_KEYWORD
    //                   | ModifierInvocation
    //                   | OverrideSpecifier
    //                   | PAYABLE_KEYWORD
    //                   | PRIVATE_KEYWORD
    //                   | PUBLIC_KEYWORD
    //                   | PURE_KEYWORD
    //                   | VIEW_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn function_attribute__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::view_keyword,
                    TokenKind::ViewKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FunctionAttribute)
    }

    // (* v0.5.0 *)
    // FunctionAttribute = EXTERNAL_KEYWORD
    //                   | INTERNAL_KEYWORD
    //                   | ModifierInvocation
    //                   | OverrideSpecifier
    //                   | PAYABLE_KEYWORD
    //                   | PRIVATE_KEYWORD
    //                   | PUBLIC_KEYWORD
    //                   | PURE_KEYWORD
    //                   | VIEW_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn function_attribute__0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::view_keyword,
                    TokenKind::ViewKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FunctionAttribute)
    }

    // (* v0.6.0 *)
    // FunctionAttribute = EXTERNAL_KEYWORD
    //                   | INTERNAL_KEYWORD
    //                   | ModifierInvocation
    //                   | OverrideSpecifier
    //                   | PAYABLE_KEYWORD
    //                   | PRIVATE_KEYWORD
    //                   | PUBLIC_KEYWORD
    //                   | PURE_KEYWORD
    //                   | VIEW_KEYWORD
    //                   | VIRTUAL_KEYWORD;

    #[allow(dead_code, non_snake_case)]
    fn function_attribute__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::virtual_keyword,
                    TokenKind::VirtualKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FunctionAttribute)
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

    // (* v0.4.11 *)
    // FunctionCallOperator = ArgumentList;

    #[allow(dead_code, non_snake_case)]
    fn function_call_operator__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.argument_list(stream)
            .with_kind(RuleKind::FunctionCallOperator)
    }

    // (* v0.6.2 *)
    // FunctionCallOperator = FunctionCallOptions* ArgumentList;

    #[allow(dead_code, non_snake_case)]
    fn function_call_operator__0_6_2(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result
                        .incorporate_zero_or_more_result(self.function_call_options(stream))
                    {
                    }
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(self.argument_list(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FunctionCallOperator)
    }

    // (* v0.8.0 *)
    // FunctionCallOperator = FunctionCallOptions? ArgumentList;

    #[allow(dead_code, non_snake_case)]
    fn function_call_operator__0_8_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(transform_option_result(
                    self.function_call_options(stream),
                )) {
                    break;
                }
                running_result.incorporate_sequence_result(self.argument_list(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FunctionCallOperator)
    }

    pub(crate) fn function_call_operator(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_0 {
            self.function_call_operator__0_8_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_6_2 {
            self.function_call_operator__0_6_2(stream)
        } else {
            self.function_call_operator__0_4_11(stream)
        }
    }

    // (* v0.6.2 *)
    // FunctionCallOptions = OPEN_BRACE (NamedArgument (COMMA NamedArgument)*)? CLOSE_BRACE;

    #[allow(dead_code, non_snake_case)]
    fn function_call_options__0_6_2(&self, stream: &mut Stream) -> ParserResult {
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
                if !running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(self.named_argument(stream))
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
                                        .incorporate_sequence_result(self.named_argument(stream));
                                    break;
                                }
                                running_result
                            }) {}
                            running_result
                        });
                        break;
                    }
                    running_result
                })) {
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
        .with_kind(RuleKind::FunctionCallOptions)
    }

    #[allow(non_snake_case)]
    pub(crate) fn function_call_options__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_6_2 {
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

    // FunctionDefinition = FUNCTION_KEYWORD (IDENTIFIER | FALLBACK_KEYWORD | RECEIVE_KEYWORD) ParameterList FunctionAttribute* (RETURNS_KEYWORD ParameterList)? (SEMICOLON | Block);

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
                        running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::receive_keyword,
                            TokenKind::ReceiveKeyword,
                        ));
                        break;
                    }
                    running_result
                }) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.parameter_list(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result
                        .incorporate_zero_or_more_result(self.function_attribute(stream))
                    {
                    }
                    running_result
                }) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::returns_keyword,
                                TokenKind::ReturnsKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parameter_list(stream));
                        break;
                    }
                    running_result
                })) {
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
                        running_result.incorporate_choice_result(self.block(stream));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FunctionDefinition)
    }

    // FunctionType = FUNCTION_KEYWORD ParameterList (INTERNAL_KEYWORD | EXTERNAL_KEYWORD | PRIVATE_KEYWORD | PUBLIC_KEYWORD | PURE_KEYWORD | VIEW_KEYWORD | PAYABLE_KEYWORD)* (RETURNS_KEYWORD ParameterList)?;

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
                if !running_result.incorporate_sequence_result(self.parameter_list(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::no_match(vec![]);
                        let start_position = stream.position();
                        loop {
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::internal_keyword,
                                    TokenKind::InternalKeyword,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::external_keyword,
                                    TokenKind::ExternalKeyword,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::private_keyword,
                                    TokenKind::PrivateKeyword,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::public_keyword,
                                    TokenKind::PublicKeyword,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::pure_keyword,
                                    TokenKind::PureKeyword,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::view_keyword,
                                    TokenKind::ViewKeyword,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            running_result.incorporate_choice_result(self.parse_token_with_trivia(
                                stream,
                                &Self::payable_keyword,
                                TokenKind::PayableKeyword,
                            ));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                }) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::returns_keyword,
                                TokenKind::ReturnsKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parameter_list(stream));
                        break;
                    }
                    running_result
                }));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::FunctionType)
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

    // ImportAlias = AS_KEYWORD IDENTIFIER;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn import_alias(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.parse_token_with_trivia(
                    stream,
                    &Self::as_keyword,
                    TokenKind::AsKeyword,
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
        .with_kind(RuleKind::ImportAlias)
    }

    // ImportDirective = IMPORT_KEYWORD (SimpleImport | AsteriskImport | SelectiveImport) SEMICOLON;

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
                                    .incorporate_choice_result(self.simple_import(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                if running_result
                                    .incorporate_choice_result(self.asterisk_import(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                running_result
                                    .incorporate_choice_result(self.selective_import(stream));
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
        .with_kind(RuleKind::ImportDirective)
    }

    // ImportPath = ASCII_STRING_LITERAL;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn import_path(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(
            stream,
            &Self::ascii_string_literal,
            TokenKind::AsciiStringLiteral,
        )
        .with_kind(RuleKind::ImportPath)
    }

    // IndexAccessOperator = OPEN_BRACKET ((Expression (COLON Expression?)?) | (COLON Expression?)) CLOSE_BRACKET;

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
                    let mut running_result = ParserResult::no_match(vec![]);
                    let start_position = stream.position();
                    loop {
                        if running_result.incorporate_choice_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result
                                    .incorporate_sequence_result(self.expression(stream))
                                {
                                    break;
                                }
                                running_result.incorporate_sequence_result(
                                    transform_option_result({
                                        let mut running_result =
                                            ParserResult::r#match(vec![], vec![]);
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
                                    }),
                                );
                                break;
                            }
                            running_result
                        }) {
                            break;
                        }
                        stream.set_position(start_position);
                        running_result.incorporate_choice_result({
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
                        });
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
        .with_kind(RuleKind::IndexAccessOperator)
    }

    // InheritanceSpecifier = IdentifierPath ArgumentList?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn inheritance_specifier(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.identifier_path(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.argument_list(stream),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::InheritanceSpecifier)
    }

    // InheritanceSpecifierList = IS_KEYWORD InheritanceSpecifier (COMMA InheritanceSpecifier)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn inheritance_specifier_list(&self, stream: &mut Stream) -> ParserResult {
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
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result
                            .incorporate_sequence_result(self.inheritance_specifier(stream))
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
                                        self.inheritance_specifier(stream),
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
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::InheritanceSpecifierList)
    }

    // InterfaceDefinition = INTERFACE_KEYWORD IDENTIFIER InheritanceSpecifierList? OPEN_BRACE «ContractBodyElement»* CLOSE_BRACE;

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
                    self.inheritance_specifier_list(stream),
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
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            while running_result
                                .incorporate_zero_or_more_result(self.contract_body_element(stream))
                            {
                            }
                            running_result
                        }) {
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
                    running_result.incorporate_choice_result(self.parse_token(
                        stream,
                        &Self::single_line_comment,
                        TokenKind::SingleLineComment,
                    ));
                    break;
                }
                running_result
            }) {}
            running_result
        }
        .with_kind(RuleKind::LeadingTrivia)
    }

    // LibraryDefinition = LIBRARY_KEYWORD IDENTIFIER OPEN_BRACE «ContractBodyElement»* CLOSE_BRACE;

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
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            while running_result
                                .incorporate_zero_or_more_result(self.contract_body_element(stream))
                            {
                            }
                            running_result
                        }) {
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

    // (* v0.4.11 *)
    // MappingKeyType = (ElementaryType | IdentifierPath);

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
                running_result.incorporate_choice_result(self.identifier_path(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::MappingKeyType)
    }

    // (* v0.8.18 *)
    // MappingKeyType = (ElementaryType | IdentifierPath) IDENTIFIER?;

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
                        running_result.incorporate_choice_result(self.identifier_path(stream));
                        break;
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

    // MemberAccessOperator = PERIOD (IDENTIFIER | ADDRESS_KEYWORD);

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
                        running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::address_keyword,
                            TokenKind::AddressKeyword,
                        ));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::MemberAccessOperator)
    }

    // (* v0.4.11 *)
    // ModifierAttribute = OverrideSpecifier;

    #[allow(dead_code, non_snake_case)]
    fn modifier_attribute__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        self.override_specifier(stream)
            .with_kind(RuleKind::ModifierAttribute)
    }

    // (* v0.6.0 *)
    // ModifierAttribute = OverrideSpecifier | VIRTUAL_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::virtual_keyword,
                    TokenKind::VirtualKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ModifierAttribute)
    }

    pub(crate) fn modifier_attribute(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.modifier_attribute__0_6_0(stream)
        } else {
            self.modifier_attribute__0_4_11(stream)
        }
    }

    // ModifierDefinition = MODIFIER_KEYWORD IDENTIFIER ParameterList? ModifierAttribute* (SEMICOLON | Block);

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
                    self.parameter_list(stream),
                )) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result
                        .incorporate_zero_or_more_result(self.modifier_attribute(stream))
                    {
                    }
                    running_result
                }) {
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
                        running_result.incorporate_choice_result(self.block(stream));
                        break;
                    }
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ModifierDefinition)
    }

    // ModifierInvocation = IdentifierPath ArgumentList?;

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
                    self.argument_list(stream),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ModifierInvocation)
    }

    // MulDivModOperator = ASTERISK
    //                   | SLASH
    //                   | PERCENT;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::percent,
                    TokenKind::Percent,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::MulDivModOperator)
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

    // NamedArgumentList = OPEN_BRACE (NamedArgument (COMMA NamedArgument)*)? CLOSE_BRACE;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn named_argument_list(&self, stream: &mut Stream) -> ParserResult {
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
                if !running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(self.named_argument(stream))
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
                                        .incorporate_sequence_result(self.named_argument(stream));
                                    break;
                                }
                                running_result
                            }) {}
                            running_result
                        });
                        break;
                    }
                    running_result
                })) {
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
        .with_kind(RuleKind::NamedArgumentList)
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
    // NumberUnit = DAYS_KEYWORD
    //            | ETHER_KEYWORD
    //            | FINNEY_KEYWORD
    //            | HOURS_KEYWORD
    //            | MINUTES_KEYWORD
    //            | SECONDS_KEYWORD
    //            | SZABO_KEYWORD
    //            | WEEKS_KEYWORD
    //            | WEI_KEYWORD
    //            | YEARS_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::years_keyword,
                    TokenKind::YearsKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NumberUnit)
    }

    // (* v0.5.0 *)
    // NumberUnit = DAYS_KEYWORD
    //            | ETHER_KEYWORD
    //            | FINNEY_KEYWORD
    //            | HOURS_KEYWORD
    //            | MINUTES_KEYWORD
    //            | SECONDS_KEYWORD
    //            | SZABO_KEYWORD
    //            | WEEKS_KEYWORD
    //            | WEI_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::wei_keyword,
                    TokenKind::WeiKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NumberUnit)
    }

    // (* v0.6.11 *)
    // NumberUnit = DAYS_KEYWORD
    //            | ETHER_KEYWORD
    //            | FINNEY_KEYWORD
    //            | GWEI_KEYWORD
    //            | HOURS_KEYWORD
    //            | MINUTES_KEYWORD
    //            | SECONDS_KEYWORD
    //            | SZABO_KEYWORD
    //            | WEEKS_KEYWORD
    //            | WEI_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::wei_keyword,
                    TokenKind::WeiKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NumberUnit)
    }

    // (* v0.7.0 *)
    // NumberUnit = DAYS_KEYWORD
    //            | ETHER_KEYWORD
    //            | GWEI_KEYWORD
    //            | HOURS_KEYWORD
    //            | MINUTES_KEYWORD
    //            | SECONDS_KEYWORD
    //            | WEEKS_KEYWORD
    //            | WEI_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::wei_keyword,
                    TokenKind::WeiKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::NumberUnit)
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
    // NumericExpression = (HEX_LITERAL | DECIMAL_LITERAL) NumberUnit?;

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
                        running_result.incorporate_choice_result(self.parse_token_with_trivia(
                            stream,
                            &Self::decimal_literal,
                            TokenKind::DecimalLiteral,
                        ));
                        break;
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
    // NumericExpression = HEX_LITERAL | (DECIMAL_LITERAL NumberUnit?);

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
                running_result.incorporate_choice_result({
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
                });
                break;
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

    // OrOperator = BAR_BAR;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn or_operator(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::bar_bar, TokenKind::BarBar)
            .with_kind(RuleKind::OrOperator)
    }

    // OrderComparisonOperator = LESS_THAN
    //                         | GREATER_THAN
    //                         | LESS_THAN_EQUAL
    //                         | GREATER_THAN_EQUAL;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than_equal,
                    TokenKind::GreaterThanEqual,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::OrderComparisonOperator)
    }

    // OverrideSpecifier = OVERRIDE_KEYWORD (OPEN_PAREN IdentifierPath (COMMA IdentifierPath)* CLOSE_PAREN)?;

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
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result
                                    .incorporate_sequence_result(self.identifier_path(stream))
                                {
                                    break;
                                }
                                running_result.incorporate_sequence_result({
                                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                                    while running_result.incorporate_zero_or_more_result({
                                        let mut running_result =
                                            ParserResult::r#match(vec![], vec![]);
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
                                                self.identifier_path(stream),
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
                }));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::OverrideSpecifier)
    }

    // ParameterDeclaration = TypeName DataLocation? IDENTIFIER?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parameter_declaration(&self, stream: &mut Stream) -> ParserResult {
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
        .with_kind(RuleKind::ParameterDeclaration)
    }

    // ParameterList = OPEN_PAREN (ParameterDeclaration (COMMA ParameterDeclaration)*)? CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn parameter_list(&self, stream: &mut Stream) -> ParserResult {
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
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result
                            .incorporate_sequence_result(self.parameter_declaration(stream))
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
                                        self.parameter_declaration(stream),
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
        .with_kind(RuleKind::ParameterList)
    }

    // PayableType = PAYABLE_KEYWORD;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn payable_type(&self, stream: &mut Stream) -> ParserResult {
        self.parse_token_with_trivia(stream, &Self::payable_keyword, TokenKind::PayableKeyword)
            .with_kind(RuleKind::PayableType)
    }

    // PositionalArgumentList = Expression (COMMA Expression)*;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn positional_argument_list(&self, stream: &mut Stream) -> ParserResult {
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
        .with_kind(RuleKind::PositionalArgumentList)
    }

    // PragmaDirective = PRAGMA_KEYWORD (VersionPragma | ABICoderPragma | ExperimentalPragma) SEMICOLON;

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
                                    .incorporate_choice_result(self.version_pragma(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                if running_result
                                    .incorporate_choice_result(self.abi_coder_pragma(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                running_result
                                    .incorporate_choice_result(self.experimental_pragma(stream));
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
    //                   | IDENTIFIER;

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
                if running_result.incorporate_choice_result(self.array_literal(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.boolean_literal(stream)) {
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::PrimaryExpression)
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
    //                   | IDENTIFIER;

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
                if running_result.incorporate_choice_result(self.array_literal(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.boolean_literal(stream)) {
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::identifier,
                    TokenKind::Identifier,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::PrimaryExpression)
    }

    pub(crate) fn primary_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_3 {
            self.primary_expression__0_5_3(stream)
        } else {
            self.primary_expression__0_4_11(stream)
        }
    }

    // (* v0.6.0 *)
    // ReceiveFunctionAttribute = ModifierInvocation
    //                          | OverrideSpecifier
    //                          | EXTERNAL_KEYWORD
    //                          | PAYABLE_KEYWORD
    //                          | VIRTUAL_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::virtual_keyword,
                    TokenKind::VirtualKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ReceiveFunctionAttribute)
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
    // ReceiveFunctionDefinition = RECEIVE_KEYWORD ParameterList ReceiveFunctionAttribute* (SEMICOLON | Block);

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
                if !running_result.incorporate_sequence_result(self.parameter_list(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result
                        .incorporate_zero_or_more_result(self.receive_function_attribute(stream))
                    {
                    }
                    running_result
                }) {
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
                        running_result.incorporate_choice_result(self.block(stream));
                        break;
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

    // RevertStatement = REVERT_KEYWORD IdentifierPath? ArgumentList SEMICOLON;

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
                        running_result.incorporate_sequence_result(self.argument_list(stream));
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

    // SelectiveImport = OPEN_BRACE IDENTIFIER ImportAlias? (COMMA IDENTIFIER ImportAlias?)* CLOSE_BRACE FROM_KEYWORD ImportPath;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn selective_import(&self, stream: &mut Stream) -> ParserResult {
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
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result({
                                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                                    loop {
                                        if !running_result.incorporate_sequence_result(
                                            self.parse_token_with_trivia(
                                                stream,
                                                &Self::identifier,
                                                TokenKind::Identifier,
                                            ),
                                        ) {
                                            break;
                                        }
                                        running_result.incorporate_sequence_result(
                                            transform_option_result(self.import_alias(stream)),
                                        );
                                        break;
                                    }
                                    running_result
                                }) {
                                    break;
                                }
                                running_result.incorporate_sequence_result({
                                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                                    while running_result.incorporate_zero_or_more_result({
                                        let mut running_result =
                                            ParserResult::r#match(vec![], vec![]);
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
                                            running_result.incorporate_sequence_result({
                                                let mut running_result =
                                                    ParserResult::r#match(vec![], vec![]);
                                                loop {
                                                    if !running_result.incorporate_sequence_result(
                                                        self.parse_token_with_trivia(
                                                            stream,
                                                            &Self::identifier,
                                                            TokenKind::Identifier,
                                                        ),
                                                    ) {
                                                        break;
                                                    }
                                                    running_result.incorporate_sequence_result(
                                                        transform_option_result(
                                                            self.import_alias(stream),
                                                        ),
                                                    );
                                                    break;
                                                }
                                                running_result
                                            });
                                            break;
                                        }
                                        running_result
                                    }) {}
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
                running_result.incorporate_sequence_result(self.import_path(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::SelectiveImport)
    }

    // ShiftOperator = LESS_THAN_LESS_THAN
    //               | GREATER_THAN_GREATER_THAN
    //               | GREATER_THAN_GREATER_THAN_GREATER_THAN;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::greater_than_greater_than_greater_than,
                    TokenKind::GreaterThanGreaterThanGreaterThan,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::ShiftOperator)
    }

    // SimpleImport = ImportPath ImportAlias?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn simple_import(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result(self.import_path(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result(transform_option_result(
                    self.import_alias(stream),
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::SimpleImport)
    }

    // SimpleStatement = TupleDeconstructionStatement
    //                 | VariableDeclarationStatement
    //                 | ExpressionStatement;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn simple_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result
                    .incorporate_choice_result(self.tuple_deconstruction_statement(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                if running_result
                    .incorporate_choice_result(self.variable_declaration_statement(stream))
                {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result(self.expression_statement(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::SimpleStatement)
    }

    // SourceUnit = (Directive | Definition)* EndOfFileTrivia?;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn source_unit(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result({
                        let mut running_result = ParserResult::no_match(vec![]);
                        let start_position = stream.position();
                        loop {
                            if running_result.incorporate_choice_result(self.directive(stream)) {
                                break;
                            }
                            stream.set_position(start_position);
                            running_result.incorporate_choice_result(self.definition(stream));
                            break;
                        }
                        running_result
                    }) {}
                    running_result
                }) {
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
    // StateVariableAttribute = OverrideSpecifier
    //                        | CONSTANT_KEYWORD
    //                        | INTERNAL_KEYWORD
    //                        | PRIVATE_KEYWORD
    //                        | PUBLIC_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::StateVariableAttribute)
    }

    // (* v0.6.5 *)
    // StateVariableAttribute = OverrideSpecifier
    //                        | CONSTANT_KEYWORD
    //                        | IMMUTABLE_KEYWORD
    //                        | INTERNAL_KEYWORD
    //                        | PRIVATE_KEYWORD
    //                        | PUBLIC_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::public_keyword,
                    TokenKind::PublicKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::StateVariableAttribute)
    }

    pub(crate) fn state_variable_attribute(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_5 {
            self.state_variable_attribute__0_6_5(stream)
        } else {
            self.state_variable_attribute__0_4_11(stream)
        }
    }

    // StateVariableDeclaration = TypeName StateVariableAttribute* IDENTIFIER (EQUAL Expression)? SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn state_variable_declaration(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(self.type_name(stream)) {
                            break;
                        }
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            while running_result.incorporate_zero_or_more_result(
                                self.state_variable_attribute(stream),
                            ) {}
                            running_result
                        }) {
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

    #[allow(dead_code, non_snake_case)]
    fn statement__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.block(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.simple_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
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
                if running_result.incorporate_choice_result(self.return_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.throw_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.revert_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.delete_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result(self.assembly_statement(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::Statement)
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

    #[allow(dead_code, non_snake_case)]
    fn statement__0_4_21(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.block(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.simple_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
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
                if running_result.incorporate_choice_result(self.return_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.emit_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.throw_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.revert_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.delete_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result(self.assembly_statement(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::Statement)
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

    #[allow(dead_code, non_snake_case)]
    fn statement__0_5_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.block(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.simple_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
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
                if running_result.incorporate_choice_result(self.return_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.emit_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.revert_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.delete_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result(self.assembly_statement(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::Statement)
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

    #[allow(dead_code, non_snake_case)]
    fn statement__0_6_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.block(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.simple_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
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
                if running_result.incorporate_choice_result(self.try_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.return_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.emit_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.revert_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result(self.delete_statement(stream)) {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result(self.assembly_statement(stream));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::Statement)
    }

    pub(crate) fn statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_6_0 {
            self.statement__0_6_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.statement__0_5_0(stream)
        } else if self.version_is_equal_to_or_greater_than_0_4_21 {
            self.statement__0_4_21(stream)
        } else {
            self.statement__0_4_11(stream)
        }
    }

    // (* v0.4.11 *)
    // StringExpression = HEX_STRING_LITERAL+ | ASCII_STRING_LITERAL+;

    #[allow(dead_code, non_snake_case)]
    fn string_expression__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_one_or_more_result(
                        self.parse_token_with_trivia(
                            stream,
                            &Self::hex_string_literal,
                            TokenKind::HexStringLiteral,
                        ),
                    ) {}
                    running_result
                }) {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_one_or_more_result(
                        self.parse_token_with_trivia(
                            stream,
                            &Self::ascii_string_literal,
                            TokenKind::AsciiStringLiteral,
                        ),
                    ) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::StringExpression)
    }

    // (* v0.7.0 *)
    // StringExpression = HEX_STRING_LITERAL+ | ASCII_STRING_LITERAL+ | UNICODE_STRING_LITERAL+;

    #[allow(dead_code, non_snake_case)]
    fn string_expression__0_7_0(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_one_or_more_result(
                        self.parse_token_with_trivia(
                            stream,
                            &Self::hex_string_literal,
                            TokenKind::HexStringLiteral,
                        ),
                    ) {}
                    running_result
                }) {
                    break;
                }
                stream.set_position(start_position);
                if running_result.incorporate_choice_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_one_or_more_result(
                        self.parse_token_with_trivia(
                            stream,
                            &Self::ascii_string_literal,
                            TokenKind::AsciiStringLiteral,
                        ),
                    ) {}
                    running_result
                }) {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_one_or_more_result(
                        self.parse_token_with_trivia(
                            stream,
                            &Self::unicode_string_literal,
                            TokenKind::UnicodeStringLiteral,
                        ),
                    ) {}
                    running_result
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::StringExpression)
    }

    pub(crate) fn string_expression(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_7_0 {
            self.string_expression__0_7_0(stream)
        } else {
            self.string_expression__0_4_11(stream)
        }
    }

    // StructDefinition = STRUCT_KEYWORD IDENTIFIER OPEN_BRACE StructMember+ CLOSE_BRACE;

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
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            while running_result
                                .incorporate_one_or_more_result(self.struct_member(stream))
                            {
                            }
                            running_result
                        }) {
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
    // TryStatement = TRY_KEYWORD Expression (RETURNS_KEYWORD ParameterList)? Block CatchClause+;

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
                if !running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::returns_keyword,
                                TokenKind::ReturnsKeyword,
                            ),
                        ) {
                            break;
                        }
                        running_result.incorporate_sequence_result(self.parameter_list(stream));
                        break;
                    }
                    running_result
                })) {
                    break;
                }
                if !running_result.incorporate_sequence_result(self.block(stream)) {
                    break;
                }
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_one_or_more_result(self.catch_clause(stream)) {
                    }
                    running_result
                });
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

    // TupleDeconstructionStatement = OPEN_PAREN (((TypeName DataLocation? IDENTIFIER) | (DataLocation? IDENTIFIER))? (COMMA ((TypeName DataLocation? IDENTIFIER) | (DataLocation? IDENTIFIER))?)*)? CLOSE_PAREN EQUAL Expression SEMICOLON;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn tuple_deconstruction_statement(&self, stream: &mut Stream) -> ParserResult {
        { let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: open_paren , TokenKind :: OpenParen)) { break ; } if ! running_result . incorporate_sequence_result (transform_option_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (transform_option_result ({ let mut running_result = ParserResult :: no_match (vec ! []) ; let start_position = stream . position () ; loop { if running_result . incorporate_choice_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . type_name (stream)) { break ; } if ! running_result . incorporate_sequence_result (transform_option_result (self . data_location (stream))) { break ; } running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: identifier , TokenKind :: Identifier)) ; break ; } running_result }) { break ; } stream . set_position (start_position) ; running_result . incorporate_choice_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (transform_option_result (self . data_location (stream))) { break ; } running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: identifier , TokenKind :: Identifier)) ; break ; } running_result }) ; break ; } running_result })) { break ; } running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; while running_result . incorporate_zero_or_more_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: comma , TokenKind :: Comma)) { break ; } running_result . incorporate_sequence_result (transform_option_result ({ let mut running_result = ParserResult :: no_match (vec ! []) ; let start_position = stream . position () ; loop { if running_result . incorporate_choice_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . type_name (stream)) { break ; } if ! running_result . incorporate_sequence_result (transform_option_result (self . data_location (stream))) { break ; } running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: identifier , TokenKind :: Identifier)) ; break ; } running_result }) { break ; } stream . set_position (start_position) ; running_result . incorporate_choice_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (transform_option_result (self . data_location (stream))) { break ; } running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: identifier , TokenKind :: Identifier)) ; break ; } running_result }) ; break ; } running_result })) ; break ; } running_result }) { } running_result }) ; break ; } running_result })) { break ; } running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: close_paren , TokenKind :: CloseParen)) ; break ; } running_result }) { break ; } if ! running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: equal , TokenKind :: Equal)) { break ; } running_result . incorporate_sequence_result (self . expression (stream)) ; break ; } running_result }) { break ; } running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: semicolon , TokenKind :: Semicolon)) ; break ; } running_result } . with_kind (RuleKind :: TupleDeconstructionStatement)
    }

    // TupleExpression = OPEN_PAREN Expression? (COMMA Expression?)* CLOSE_PAREN;

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
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(transform_option_result(
                            self.expression(stream),
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
                                        transform_option_result(self.expression(stream)),
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
        }
        .with_kind(RuleKind::TupleExpression)
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

    // TypeName = ArrayTypeName | FunctionType | MappingType | ElementaryType | IdentifierPath;
    // ArrayTypeName = TypeName OPEN_BRACKET Expression? CLOSE_BRACKET;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn type_name(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut elements: Vec<ParserResult> = Vec::new();
            let result = loop {
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
                            running_result.incorporate_choice_result(self.identifier_path(stream));
                            break;
                        }
                        running_result
                    };
                    if result.is_match() {
                        elements.push(result);
                    } else {
                        break result;
                    }
                }
                let result = loop {
                    let result = {
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::open_bracket,
                                    TokenKind::OpenBracket,
                                ),
                            ) {
                                break;
                            }
                            if !running_result.incorporate_sequence_result(transform_option_result(
                                self.expression(stream),
                            )) {
                                break;
                            }
                            running_result.incorporate_sequence_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::close_bracket,
                                    TokenKind::CloseBracket,
                                ),
                            );
                            break;
                        }
                        running_result
                    }
                    .to_pratt_element_operator(
                        RuleKind::ArrayTypeName,
                        1u8,
                        255,
                    );
                    match result {
                        ParserResult::PrattOperatorMatch(_) => elements.push(result),
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
            if elements.is_empty() {
                break result;
            }
            reduce_pratt_elements(
                |children| vec![cst::Node::rule(RuleKind::TypeName, children)],
                &mut elements,
            );
            if elements.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    elements
                );
            }
            if let ParserResult::Match(r#match) = elements.remove(0) {
                if let ParserResult::IncompleteMatch(_) = result {
                    break ParserResult::incomplete_match(r#match.nodes, vec![]);
                } else {
                    break ParserResult::r#match(
                        r#match.nodes,
                        r#match.tokens_that_would_have_allowed_more_progress,
                    );
                }
            } else {
                unreachable!("Pratt parser failed to reduce to a single match")
            }
        }
        .with_kind(RuleKind::TypeName)
    }

    // UnaryPostfixOperator = PLUS_PLUS | MINUS_MINUS;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus_minus,
                    TokenKind::MinusMinus,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UnaryPostfixOperator)
    }

    // (* v0.4.11 *)
    // UnaryPrefixOperator = PLUS_PLUS
    //                     | MINUS_MINUS
    //                     | TILDE
    //                     | BANG
    //                     | MINUS
    //                     | PLUS;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::plus,
                    TokenKind::Plus,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UnaryPrefixOperator)
    }

    // (* v0.5.0 *)
    // UnaryPrefixOperator = PLUS_PLUS
    //                     | MINUS_MINUS
    //                     | TILDE
    //                     | BANG
    //                     | MINUS;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::minus,
                    TokenKind::Minus,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UnaryPrefixOperator)
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

    // (* v0.4.11 *)
    // UnnamedFunctionAttribute = ModifierInvocation
    //                          | OverrideSpecifier
    //                          | EXTERNAL_KEYWORD
    //                          | PAYABLE_KEYWORD
    //                          | PURE_KEYWORD
    //                          | VIEW_KEYWORD;

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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::view_keyword,
                    TokenKind::ViewKeyword,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UnnamedFunctionAttribute)
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
    // UnnamedFunctionDefinition = FUNCTION_KEYWORD ParameterList UnnamedFunctionAttribute* (SEMICOLON | Block);

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
                if !running_result.incorporate_sequence_result(self.parameter_list(stream)) {
                    break;
                }
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result
                        .incorporate_zero_or_more_result(self.unnamed_function_attribute(stream))
                    {
                    }
                    running_result
                }) {
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
                        running_result.incorporate_choice_result(self.block(stream));
                        break;
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

    // (* v0.8.19 *)
    // UserDefinedOperator = AMPERSAND
    //                     | BANG_EQUAL
    //                     | BAR
    //                     | CARET
    //                     | EQUAL_EQUAL
    //                     | GREATER_THAN
    //                     | GREATER_THAN_EQUAL
    //                     | LESS_THAN
    //                     | LESS_THAN_EQUAL
    //                     | MINUS
    //                     | PERCENT
    //                     | PLUS
    //                     | SLASH
    //                     | ASTERISK
    //                     | TILDE;

    #[allow(dead_code, non_snake_case)]
    fn user_defined_operator__0_8_19(&self, stream: &mut Stream) -> ParserResult {
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
                    &Self::asterisk,
                    TokenKind::Asterisk,
                )) {
                    break;
                }
                stream.set_position(start_position);
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::tilde,
                    TokenKind::Tilde,
                ));
                break;
            }
            running_result
        }
        .with_kind(RuleKind::UserDefinedOperator)
    }

    #[allow(non_snake_case)]
    pub(crate) fn user_defined_operator__sparse_dispatch(
        &self,
        stream: &mut Stream,
    ) -> Option<ParserResult> {
        if self.version_is_equal_to_or_greater_than_0_8_19 {
            Some(self.user_defined_operator__0_8_19(stream))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn user_defined_operator(&self, stream: &mut Stream) -> ParserResult {
        self.user_defined_operator__sparse_dispatch(stream)
            .expect("Validation should have checked that references are valid between versions")
    }

    // (* v0.8.8 *)
    // UserDefinedValueTypeDefinition = TYPE_KEYWORD IDENTIFIER IS_KEYWORD ElementaryType SEMICOLON;

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

    // (* v0.4.11 *)
    // UsingDirective = USING_KEYWORD (IdentifierPath | (OPEN_BRACE IdentifierPath (COMMA IdentifierPath)* CLOSE_BRACE)) FOR_KEYWORD (ASTERISK | TypeName) GLOBAL_KEYWORD? SEMICOLON;

    #[allow(dead_code, non_snake_case)]
    fn using_directive__0_4_11(&self, stream: &mut Stream) -> ParserResult {
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
                                    .incorporate_choice_result(self.identifier_path(stream))
                                {
                                    break;
                                }
                                stream.set_position(start_position);
                                running_result.incorporate_choice_result({
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
                                        if !running_result.incorporate_sequence_result({
                                            let mut running_result =
                                                ParserResult::r#match(vec![], vec![]);
                                            loop {
                                                if !running_result.incorporate_sequence_result(
                                                    self.identifier_path(stream),
                                                ) {
                                                    break;
                                                }
                                                running_result.incorporate_sequence_result({
                                                    let mut running_result =
                                                        ParserResult::r#match(vec![], vec![]);
                                                    while running_result
                                                        .incorporate_zero_or_more_result({
                                                            let mut running_result =
                                                                ParserResult::r#match(
                                                                    vec![],
                                                                    vec![],
                                                                );
                                                            loop {
                                                                if !running_result
                                                                    .incorporate_sequence_result(
                                                                    self.parse_token_with_trivia(
                                                                        stream,
                                                                        &Self::comma,
                                                                        TokenKind::Comma,
                                                                    ),
                                                                ) {
                                                                    break;
                                                                }
                                                                running_result
                                                                    .incorporate_sequence_result(
                                                                        self.identifier_path(
                                                                            stream,
                                                                        ),
                                                                    );
                                                                break;
                                                            }
                                                            running_result
                                                        })
                                                    {
                                                    }
                                                    running_result
                                                });
                                                break;
                                            }
                                            running_result
                                        }) {
                                            break;
                                        }
                                        running_result.incorporate_sequence_result(
                                            self.parse_token_with_trivia(
                                                stream,
                                                &Self::close_brace,
                                                TokenKind::CloseBrace,
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
                                running_result.incorporate_choice_result(self.type_name(stream));
                                break;
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

    // (* v0.8.19 *)
    // UsingDirective = USING_KEYWORD (IdentifierPath | (OPEN_BRACE IdentifierPath (AS_KEYWORD UserDefinedOperator)? (COMMA IdentifierPath (AS_KEYWORD UserDefinedOperator)?)* CLOSE_BRACE)) FOR_KEYWORD (ASTERISK | TypeName) GLOBAL_KEYWORD? SEMICOLON;

    #[allow(dead_code, non_snake_case)]
    fn using_directive__0_8_19(&self, stream: &mut Stream) -> ParserResult {
        { let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: using_keyword , TokenKind :: UsingKeyword)) { break ; } if ! running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: no_match (vec ! []) ; let start_position = stream . position () ; loop { if running_result . incorporate_choice_result (self . identifier_path (stream)) { break ; } stream . set_position (start_position) ; running_result . incorporate_choice_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: open_brace , TokenKind :: OpenBrace)) { break ; } if ! running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . identifier_path (stream)) { break ; } running_result . incorporate_sequence_result (transform_option_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: as_keyword , TokenKind :: AsKeyword)) { break ; } running_result . incorporate_sequence_result (self . user_defined_operator (stream)) ; break ; } running_result })) ; break ; } running_result }) { break ; } running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; while running_result . incorporate_zero_or_more_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: comma , TokenKind :: Comma)) { break ; } running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . identifier_path (stream)) { break ; } running_result . incorporate_sequence_result (transform_option_result ({ let mut running_result = ParserResult :: r#match (vec ! [] , vec ! []) ; loop { if ! running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: as_keyword , TokenKind :: AsKeyword)) { break ; } running_result . incorporate_sequence_result (self . user_defined_operator (stream)) ; break ; } running_result })) ; break ; } running_result }) ; break ; } running_result }) { } running_result }) ; break ; } running_result }) { break ; } running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: close_brace , TokenKind :: CloseBrace)) ; break ; } running_result }) ; break ; } running_result }) { break ; } if ! running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: for_keyword , TokenKind :: ForKeyword)) { break ; } if ! running_result . incorporate_sequence_result ({ let mut running_result = ParserResult :: no_match (vec ! []) ; let start_position = stream . position () ; loop { if running_result . incorporate_choice_result (self . parse_token_with_trivia (stream , & Self :: asterisk , TokenKind :: Asterisk)) { break ; } stream . set_position (start_position) ; running_result . incorporate_choice_result (self . type_name (stream)) ; break ; } running_result }) { break ; } running_result . incorporate_sequence_result (transform_option_result (self . parse_token_with_trivia (stream , & Self :: global_keyword , TokenKind :: GlobalKeyword))) ; break ; } running_result }) { break ; } running_result . incorporate_sequence_result (self . parse_token_with_trivia (stream , & Self :: semicolon , TokenKind :: Semicolon)) ; break ; } running_result } . with_kind (RuleKind :: UsingDirective)
    }

    pub(crate) fn using_directive(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_8_19 {
            self.using_directive__0_8_19(stream)
        } else {
            self.using_directive__0_4_11(stream)
        }
    }

    // (* v0.4.11 *)
    // VariableDeclarationStatement = ((TypeName DataLocation?) | VAR_KEYWORD) IDENTIFIER (EQUAL Expression)? SEMICOLON;

    #[allow(dead_code, non_snake_case)]
    fn variable_declaration_statement__0_4_11(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result({
                            let mut running_result = ParserResult::no_match(vec![]);
                            let start_position = stream.position();
                            loop {
                                if running_result.incorporate_choice_result({
                                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                                    loop {
                                        if !running_result
                                            .incorporate_sequence_result(self.type_name(stream))
                                        {
                                            break;
                                        }
                                        running_result.incorporate_sequence_result(
                                            transform_option_result(self.data_location(stream)),
                                        );
                                        break;
                                    }
                                    running_result
                                }) {
                                    break;
                                }
                                stream.set_position(start_position);
                                running_result.incorporate_choice_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::var_keyword,
                                        TokenKind::VarKeyword,
                                    ),
                                );
                                break;
                            }
                            running_result
                        }) {
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
        .with_kind(RuleKind::VariableDeclarationStatement)
    }

    // (* v0.5.0 *)
    // VariableDeclarationStatement = TypeName DataLocation? IDENTIFIER (EQUAL Expression)? SEMICOLON;

    #[allow(dead_code, non_snake_case)]
    fn variable_declaration_statement__0_5_0(&self, stream: &mut Stream) -> ParserResult {
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
                            self.data_location(stream),
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
        .with_kind(RuleKind::VariableDeclarationStatement)
    }

    pub(crate) fn variable_declaration_statement(&self, stream: &mut Stream) -> ParserResult {
        if self.version_is_equal_to_or_greater_than_0_5_0 {
            self.variable_declaration_statement__0_5_0(stream)
        } else {
            self.variable_declaration_statement__0_4_11(stream)
        }
    }

    // VersionPragma = SOLIDITY_KEYWORD VersionPragmaExpressionList;
    // VersionPragmaExpressionList = «VersionPragmaExpression»+;

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
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result
                        .incorporate_one_or_more_result(self.version_pragma_expression(stream))
                    {
                    }
                    running_result.with_kind(RuleKind::VersionPragmaExpressionList)
                });
                break;
            }
            running_result
        }
        .with_kind(RuleKind::VersionPragma)
    }

    // «VersionPragmaExpression» = VersionPragmaAlternatives
    //                           | VersionPragmaRange
    //                           | VersionPragmaComparator
    //                           | VersionPragmaSpecifier;
    // VersionPragmaAlternatives = «VersionPragmaExpression» BAR_BAR «VersionPragmaExpression»;
    // VersionPragmaRange = «VersionPragmaExpression» MINUS «VersionPragmaExpression»;
    // VersionPragmaComparator = (CARET | TILDE | EQUAL | LESS_THAN | GREATER_THAN | LESS_THAN_EQUAL | GREATER_THAN_EQUAL) «VersionPragmaExpression»;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn version_pragma_expression(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut elements: Vec<ParserResult> = Vec::new();
            let result = loop {
                let result = loop {
                    let result = {
                        let mut running_result = ParserResult::no_match(vec![]);
                        let start_position = stream.position();
                        loop {
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::caret,
                                    TokenKind::Caret,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::tilde,
                                    TokenKind::Tilde,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::equal,
                                    TokenKind::Equal,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::less_than,
                                    TokenKind::LessThan,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::greater_than,
                                    TokenKind::GreaterThan,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            if running_result.incorporate_choice_result(
                                self.parse_token_with_trivia(
                                    stream,
                                    &Self::less_than_equal,
                                    TokenKind::LessThanEqual,
                                ),
                            ) {
                                break;
                            }
                            stream.set_position(start_position);
                            running_result.incorporate_choice_result(self.parse_token_with_trivia(
                                stream,
                                &Self::greater_than_equal,
                                TokenKind::GreaterThanEqual,
                            ));
                            break;
                        }
                        running_result
                    }
                    .to_pratt_element_operator(
                        RuleKind::VersionPragmaComparator,
                        255,
                        5u8,
                    );
                    match result {
                        ParserResult::PrattOperatorMatch(_) => elements.push(result),
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
                        elements.push(result);
                    } else {
                        break result;
                    }
                }
                let result = loop {
                    let start_position = stream.position();
                    stream.set_position(start_position);
                    let next_result = self
                        .parse_token_with_trivia(stream, &Self::bar_bar, TokenKind::BarBar)
                        .to_pratt_element_operator(
                            RuleKind::VersionPragmaAlternatives,
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
                        .parse_token_with_trivia(stream, &Self::minus, TokenKind::Minus)
                        .to_pratt_element_operator(RuleKind::VersionPragmaRange, 3u8, 3u8 + 1);
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
                    ParserResult::PrattOperatorMatch(_) => elements.push(result),
                    _ => break result,
                }
            };
            if elements.is_empty() {
                break result;
            }
            reduce_pratt_elements(|children| children, &mut elements);
            if elements.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    elements
                );
            }
            if let ParserResult::Match(r#match) = elements.remove(0) {
                if let ParserResult::IncompleteMatch(_) = result {
                    break ParserResult::incomplete_match(r#match.nodes, vec![]);
                } else {
                    break ParserResult::r#match(
                        r#match.nodes,
                        r#match.tokens_that_would_have_allowed_more_progress,
                    );
                }
            } else {
                unreachable!("Pratt parser failed to reduce to a single match")
            }
        }
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

    // YulAssignmentStatement = YulIdentifierPath (COMMA YulIdentifierPath)* COLON_EQUAL YulExpression;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_assignment_statement(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::r#match(vec![], vec![]);
            loop {
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result
                            .incorporate_sequence_result(self.yul_identifier_path(stream))
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
                                        self.yul_identifier_path(stream),
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
                }) {
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

    // YulBlock = OPEN_BRACE YulStatement* CLOSE_BRACE;

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
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_zero_or_more_result(self.yul_statement(stream))
                    {
                    }
                    running_result
                }) {
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

    // YulDeclarationStatement = LET_KEYWORD YulIdentifierPath (COMMA YulIdentifierPath)* (COLON_EQUAL YulExpression)?;

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
                if !running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result
                            .incorporate_sequence_result(self.yul_identifier_path(stream))
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
                                        self.yul_identifier_path(stream),
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
                }) {
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

    // YulExpression = YulFunctionCallExpression | YulLiteral | YulIdentifierPath;
    // YulFunctionCallExpression = YulExpression OPEN_PAREN (YulExpression (COMMA YulExpression)*)? CLOSE_PAREN;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_expression(&self, stream: &mut Stream) -> ParserResult {
        loop {
            let mut elements: Vec<ParserResult> = Vec::new();
            let result = loop {
                {
                    let result = {
                        let mut running_result = ParserResult::no_match(vec![]);
                        let start_position = stream.position();
                        loop {
                            if running_result.incorporate_choice_result(self.yul_literal(stream)) {
                                break;
                            }
                            stream.set_position(start_position);
                            running_result
                                .incorporate_choice_result(self.yul_identifier_path(stream));
                            break;
                        }
                        running_result
                    };
                    if result.is_match() {
                        elements.push(result);
                    } else {
                        break result;
                    }
                }
                let result = loop {
                    let result = {
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
                                {
                                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                                    loop {
                                        if !running_result.incorporate_sequence_result(
                                            self.yul_expression(stream),
                                        ) {
                                            break;
                                        }
                                        running_result.incorporate_sequence_result({
                                            let mut running_result =
                                                ParserResult::r#match(vec![], vec![]);
                                            while running_result.incorporate_zero_or_more_result({
                                                let mut running_result =
                                                    ParserResult::r#match(vec![], vec![]);
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
                                                        self.yul_expression(stream),
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
                                },
                            )) {
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
                    }
                    .to_pratt_element_operator(
                        RuleKind::YulFunctionCallExpression,
                        1u8,
                        255,
                    );
                    match result {
                        ParserResult::PrattOperatorMatch(_) => elements.push(result),
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
            if elements.is_empty() {
                break result;
            }
            reduce_pratt_elements(
                |children| vec![cst::Node::rule(RuleKind::YulExpression, children)],
                &mut elements,
            );
            if elements.len() != 1 {
                unreachable!(
                    "Pratt parser failed to reduce to a single result: {:?}",
                    elements
                );
            }
            if let ParserResult::Match(r#match) = elements.remove(0) {
                if let ParserResult::IncompleteMatch(_) = result {
                    break ParserResult::incomplete_match(r#match.nodes, vec![]);
                } else {
                    break ParserResult::r#match(
                        r#match.nodes,
                        r#match.tokens_that_would_have_allowed_more_progress,
                    );
                }
            } else {
                unreachable!("Pratt parser failed to reduce to a single match")
            }
        }
        .with_kind(RuleKind::YulExpression)
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

    // YulFunctionDefinition = FUNCTION_KEYWORD YUL_IDENTIFIER OPEN_PAREN Arguments? CLOSE_PAREN (MINUS_GREATER_THAN Results)? YulBlock;
    // Arguments = YUL_IDENTIFIER (COMMA YUL_IDENTIFIER)*;
    // Results = YUL_IDENTIFIER (COMMA YUL_IDENTIFIER)*;

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
                        if !running_result.incorporate_sequence_result(transform_option_result({
                            let mut running_result = ParserResult::r#match(vec![], vec![]);
                            loop {
                                if !running_result.incorporate_sequence_result(
                                    self.parse_token_with_trivia(
                                        stream,
                                        &Self::yul_identifier,
                                        TokenKind::YulIdentifier,
                                    ),
                                ) {
                                    break;
                                }
                                running_result.incorporate_sequence_result({
                                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                                    while running_result.incorporate_zero_or_more_result({
                                        let mut running_result =
                                            ParserResult::r#match(vec![], vec![]);
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
                            running_result.with_kind(RuleKind::Arguments)
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
                }) {
                    break;
                }
                if !running_result.incorporate_sequence_result(transform_option_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    loop {
                        if !running_result.incorporate_sequence_result(
                            self.parse_token_with_trivia(
                                stream,
                                &Self::minus_greater_than,
                                TokenKind::MinusGreaterThan,
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
                                        &Self::yul_identifier,
                                        TokenKind::YulIdentifier,
                                    ),
                                ) {
                                    break;
                                }
                                running_result.incorporate_sequence_result({
                                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                                    while running_result.incorporate_zero_or_more_result({
                                        let mut running_result =
                                            ParserResult::r#match(vec![], vec![]);
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
                            running_result.with_kind(RuleKind::Results)
                        });
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

    // YulLiteral = BooleanLiteral
    //            | YUL_HEX_LITERAL
    //            | YUL_DECIMAL_LITERAL
    //            | HEX_STRING_LITERAL
    //            | ASCII_STRING_LITERAL;

    #[allow(dead_code)]
    #[allow(unused_assignments, unused_parens)]
    pub(crate) fn yul_literal(&self, stream: &mut Stream) -> ParserResult {
        {
            let mut running_result = ParserResult::no_match(vec![]);
            let start_position = stream.position();
            loop {
                if running_result.incorporate_choice_result(self.boolean_literal(stream)) {
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
                running_result.incorporate_choice_result(self.parse_token_with_trivia(
                    stream,
                    &Self::ascii_string_literal,
                    TokenKind::AsciiStringLiteral,
                ));
                break;
            }
            running_result
        }
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
                running_result.incorporate_choice_result(self.yul_expression(stream));
                break;
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
                running_result.incorporate_choice_result(self.yul_expression(stream));
                break;
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

    // YulSwitchStatement = SWITCH_KEYWORD YulExpression (((CASE_KEYWORD YulLiteral) | DEFAULT_KEYWORD) YulBlock)+;

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
                running_result.incorporate_sequence_result({
                    let mut running_result = ParserResult::r#match(vec![], vec![]);
                    while running_result.incorporate_one_or_more_result({
                        let mut running_result = ParserResult::r#match(vec![], vec![]);
                        loop {
                            if !running_result.incorporate_sequence_result({
                                let mut running_result = ParserResult::no_match(vec![]);
                                let start_position = stream.position();
                                loop {
                                    if running_result.incorporate_choice_result({
                                        let mut running_result =
                                            ParserResult::r#match(vec![], vec![]);
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
                                            running_result.incorporate_sequence_result(
                                                self.yul_literal(stream),
                                            );
                                            break;
                                        }
                                        running_result
                                    }) {
                                        break;
                                    }
                                    stream.set_position(start_position);
                                    running_result.incorporate_choice_result(
                                        self.parse_token_with_trivia(
                                            stream,
                                            &Self::default_keyword,
                                            TokenKind::DefaultKeyword,
                                        ),
                                    );
                                    break;
                                }
                                running_result
                            }) {
                                break;
                            }
                            running_result.incorporate_sequence_result(self.yul_block(stream));
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
        .with_kind(RuleKind::YulSwitchStatement)
    }
}
