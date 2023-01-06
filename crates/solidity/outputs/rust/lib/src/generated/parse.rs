// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::cst;
use super::kinds::*;
use super::language::Parser;
use super::lex;
#[allow(deprecated)]
use chumsky::debug::{Debugger, Silent, Verbose};
use chumsky::error::Located;
use chumsky::prelude::*;
use chumsky::Error;
use chumsky::Parser as ChumskyParser;
use chumsky::Stream;
use semver::Version;
use std::collections::BTreeMap;
use std::ops::Range;
use std::rc::Rc;
pub type SpanType = Range<usize>;
pub type ErrorType = Simple<char, SpanType>;
type PResult<O> = (
    Vec<Located<char, ErrorType>>,
    Result<(O, Option<Located<char, ErrorType>>), Located<char, ErrorType>>,
);
type StreamOf<'a> = Stream<'a, char, <ErrorType as Error<char>>::Span>;
#[derive(Copy, Clone)]
struct Difference<M, S> {
    minuend: M,
    subtrahend: S,
}
impl<
        O,
        M: ChumskyParser<char, O, Error = ErrorType> + Clone,
        S: ChumskyParser<char, O, Error = ErrorType> + Clone,
    > ChumskyParser<char, O> for Difference<M, S>
{
    type Error = ErrorType;
    #[allow(deprecated)]
    fn parse_inner<D: Debugger>(&self, debugger: &mut D, stream: &mut StreamOf) -> PResult<O> {
        let start_position = stream.save();
        #[allow(deprecated)]
        match debugger.invoke(&self.minuend, stream) {
            result @ (_, Ok((_, _))) => {
                let end_position = stream.save();
                stream.revert(start_position);
                #[allow(deprecated)]
                match debugger.invoke(&self.subtrahend, stream) {
                    (_, Ok(_)) if end_position == stream.save() => {
                        stream.revert(start_position);
                        let (at, span, found) = stream.next();
                        stream.revert(start_position);
                        return (
                            Vec::new(),
                            Err(Located::at(
                                at,
                                ErrorType::expected_input_found(span, Vec::new(), found),
                            )),
                        );
                    }
                    _ => {
                        stream.revert(end_position);
                        return result;
                    }
                }
            }
            result => return result,
        }
    }
    fn parse_inner_verbose(&self, d: &mut Verbose, s: &mut StreamOf) -> PResult<O> {
        #[allow(deprecated)]
        self.parse_inner(d, s)
    }
    fn parse_inner_silent(&self, d: &mut Silent, s: &mut StreamOf) -> PResult<O> {
        #[allow(deprecated)]
        self.parse_inner(d, s)
    }
}
#[allow(dead_code)]
fn difference<M, S, O>(minuend: M, subtrahend: S) -> Difference<M, S>
where
    M: ChumskyParser<char, O, Error = ErrorType> + Clone,
    S: ChumskyParser<char, O, Error = ErrorType>,
{
    Difference {
        minuend,
        subtrahend,
    }
}
#[allow(unused_macros)]
macro_rules! declare_rule {
    ($ kind : ident) => {
        #[allow(non_snake_case)]
        let mut $kind = Recursive::<'static, char, Option<Rc<cst::Node>>, ErrorType>::declare();
    };
}
#[allow(unused_macros)]
macro_rules! declare_token {
    ($ kind : ident) => {
        #[allow(non_snake_case)]
        let mut $kind = Recursive::<'static, char, Option<Rc<lex::Node>>, ErrorType>::declare();
    };
}

pub fn create_parsers(version: &Version) -> BTreeMap<ProductionKind, Parser> {
    let mut parsers: BTreeMap<ProductionKind, Parser> = BTreeMap::new();

    // Declare all versions -----------------------------

    let version_0_6_0 = &Version::parse("0.6.0").unwrap();

    // Declare all productions --------------------------

    declare_rule!(ABICoderPragma);
    declare_rule!(AddSubExpression);
    declare_rule!(AddressType);
    declare_rule!(AndExpression);
    declare_rule!(ArgumentList);
    declare_rule!(ArrayLiteral);
    declare_token!(AsciiEscape);
    declare_token!(AsciiStringLiteral);
    declare_rule!(AssemblyFlags);
    declare_rule!(AssemblyStatement);
    declare_rule!(AssignmentExpression);
    declare_rule!(BitAndExpression);
    declare_rule!(BitOrExpression);
    declare_rule!(BitXOrExpression);
    declare_rule!(Block);
    declare_token!(BooleanLiteral);
    declare_rule!(BreakStatement);
    declare_rule!(CatchClause);
    declare_rule!(ConditionalExpression);
    declare_rule!(ConstantDefinition);
    declare_rule!(ConstructorAttribute);
    declare_rule!(ConstructorDefinition);
    declare_rule!(ContinueStatement);
    declare_rule!(ContractBodyElement);
    declare_rule!(ContractDefinition);
    declare_rule!(DataLocation);
    declare_token!(DecimalExponent);
    declare_token!(DecimalFloat);
    declare_token!(DecimalInteger);
    declare_token!(DecimalNumber);
    declare_rule!(Definition);
    declare_rule!(DeleteStatement);
    declare_rule!(Directive);
    declare_rule!(DoWhileStatement);
    declare_token!(DoubleQuotedAsciiStringLiteral);
    declare_token!(DoubleQuotedUnicodeStringLiteral);
    declare_rule!(ElementaryType);
    declare_rule!(EmitStatement);
    declare_rule!(EndOfFileTrivia);
    declare_token!(EndOfLine);
    declare_rule!(EnumDefinition);
    declare_rule!(EqualityComparisonExpression);
    declare_rule!(ErrorDefinition);
    declare_rule!(ErrorParameter);
    declare_token!(EscapeSequence);
    declare_rule!(EventDefinition);
    declare_rule!(EventParameter);
    declare_rule!(ExperimentalPragma);
    declare_rule!(ExponentiationExpression);
    declare_rule!(Expression);
    declare_rule!(ExpressionStatement);
    declare_rule!(FallbackFunctionAttribute);
    declare_rule!(FallbackFunctionDefinition);
    declare_token!(FixedBytesType);
    declare_rule!(ForStatement);
    declare_rule!(FunctionAttribute);
    declare_rule!(FunctionCallExpression);
    declare_rule!(FunctionDefinition);
    declare_rule!(FunctionType);
    declare_token!(HexByteEscape);
    declare_token!(HexCharacter);
    declare_token!(HexNumber);
    declare_token!(HexStringLiteral);
    declare_token!(Identifier);
    declare_token!(IdentifierPart);
    declare_rule!(IdentifierPath);
    declare_token!(IdentifierStart);
    declare_rule!(IfStatement);
    declare_rule!(ImportDirective);
    declare_rule!(ImportPath);
    declare_rule!(IndexAccessExpression);
    declare_rule!(InheritanceSpecifier);
    declare_rule!(InheritanceSpecifierList);
    declare_rule!(InterfaceDefinition);
    declare_token!(Keyword);
    declare_rule!(LeadingTrivia);
    declare_rule!(LibraryDefinition);
    declare_rule!(MappingType);
    declare_rule!(MemberAccessExpression);
    declare_rule!(ModifierAttribute);
    declare_rule!(ModifierDefinition);
    declare_rule!(ModifierInvocation);
    declare_rule!(MulDivModExpression);
    declare_token!(MultilineComment);
    declare_rule!(NamedArgument);
    declare_rule!(NamedArgumentList);
    declare_rule!(NewExpression);
    declare_token!(NumberUnit);
    declare_rule!(NumericLiteral);
    declare_rule!(OrExpression);
    declare_rule!(OrderComparisonExpression);
    declare_rule!(OverrideSpecifier);
    declare_rule!(ParameterDeclaration);
    declare_rule!(ParameterList);
    declare_rule!(ParenthesisExpression);
    declare_rule!(PayableExpression);
    declare_rule!(PositionalArgumentList);
    declare_token!(PossiblySeparatedPairsOfHexDigits);
    declare_rule!(PragmaDirective);
    declare_rule!(PrimaryExpression);
    declare_token!(RawIdentifier);
    declare_rule!(ReceiveFunctionAttribute);
    declare_rule!(ReceiveFunctionDefinition);
    declare_token!(ReservedKeyword);
    declare_rule!(ReturnStatement);
    declare_rule!(RevertStatement);
    declare_rule!(SelectedImport);
    declare_rule!(SelectingImportDirective);
    declare_rule!(ShiftExpression);
    declare_token!(SignedFixedType);
    declare_token!(SignedIntegerType);
    declare_rule!(SimpleImportDirective);
    declare_rule!(SimpleStatement);
    declare_token!(SingleLineComment);
    declare_token!(SingleQuotedAsciiStringLiteral);
    declare_token!(SingleQuotedUnicodeStringLiteral);
    declare_rule!(SourceUnit);
    declare_rule!(StarImportDirective);
    declare_rule!(StateVariableAttribute);
    declare_rule!(StateVariableDeclaration);
    declare_rule!(Statement);
    declare_rule!(StructDefinition);
    declare_rule!(StructMember);
    declare_rule!(TrailingTrivia);
    declare_rule!(TryStatement);
    declare_rule!(TupleDeconstructionStatement);
    declare_rule!(TypeExpression);
    declare_rule!(TypeName);
    declare_rule!(UnaryPrefixExpression);
    declare_rule!(UnarySuffixExpression);
    declare_rule!(UncheckedBlock);
    declare_token!(UnicodeEscape);
    declare_token!(UnicodeStringLiteral);
    declare_token!(UnsignedFixedType);
    declare_token!(UnsignedIntegerType);
    declare_rule!(UserDefinedValueTypeDefinition);
    declare_rule!(UsingDirective);
    declare_rule!(VariableDeclarationStatement);
    declare_rule!(VersionPragma);
    declare_token!(VersionPragmaOperator);
    declare_rule!(VersionPragmaSpecifier);
    declare_token!(VersionPragmaValue);
    declare_rule!(WhileStatement);
    declare_token!(Whitespace);
    declare_rule!(YulAssignmentStatement);
    declare_rule!(YulBlock);
    declare_rule!(YulBreakStatement);
    declare_rule!(YulContinueStatement);
    declare_token!(YulDecimalNumberLiteral);
    declare_rule!(YulExpression);
    declare_rule!(YulForStatement);
    declare_rule!(YulFunctionCall);
    declare_rule!(YulFunctionDefinition);
    declare_token!(YulHexLiteral);
    declare_token!(YulIdentifier);
    declare_rule!(YulIdentifierPath);
    declare_rule!(YulIfStatement);
    declare_token!(YulKeyword);
    declare_rule!(YulLeaveStatement);
    declare_rule!(YulLiteral);
    declare_rule!(YulStatement);
    declare_rule!(YulSwitchStatement);
    declare_rule!(YulVariableDeclaration);

    // Macros -------------------------------------------

    #[allow(unused_macros)]
    macro_rules! lex_terminal {
        ($ kind : ident , $ literal : literal) => {
            just($literal).map_with_span(|_, span: SpanType| {
                lex::Node::named(TokenKind::$kind, lex::Node::chars(span.start()..span.end()))
            })
        };
        ($ kind : ident , $ filter : expr) => {
            filter($filter).map_with_span(|_, span: SpanType| {
                lex::Node::named(TokenKind::$kind, lex::Node::chars(span.start()..span.end()))
            })
        };
        ($ literal : literal) => {
            just($literal)
                .map_with_span(|_, span: SpanType| lex::Node::chars(span.start()..span.end()))
        };
        ($ filter : expr) => {
            filter($filter)
                .map_with_span(|_, span: SpanType| lex::Node::chars(span.start()..span.end()))
        };
    }
    #[allow(unused_macros)]
    macro_rules! lex_rule {
        ($ rule : ident) => {
            $rule.clone()
        };
    }
    #[allow(unused_macros)]
    macro_rules ! lex_choice { ($ kind : ident , $ ($ expr : expr) , *) => { lex_choice ! ($ ($ expr) , *) . map (| element | lex :: Node :: named (TokenKind :: $ kind , element)) } ; ($ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr) , *)) } ; }
    #[allow(unused_macros)]
    macro_rules ! lex_seq { (@ exp $ head : expr , $ ($ tail : expr) , +) => { $ head . then (lex_seq ! (@ exp $ ($ tail) , +)) } ; (@ exp $ head : expr) => { $ head } ; (@ args [$ ($ accum : expr ,) *] , $ current : expr , $ head : expr , $ ($ tail : expr) , +) => { lex_seq ! (@ args [$ ($ accum ,) * $ current . 0 ,] , $ current . 1 , $ ($ tail) , +) } ; (@ args [$ ($ accum : expr ,) *] , $ current : expr , $ head : expr) => { vec ! [$ ($ accum ,) * $ current] } ; ($ kind : ident , $ ($ expr : expr) , +) => { lex_seq ! (@ exp $ ($ expr) , +) . map (| v | lex :: Node :: named (TokenKind :: $ kind , lex :: Node :: sequence (lex_seq ! (@ args [] , v , $ ($ expr) , +)))) } ; ($ ($ expr : expr) , +) => { lex_seq ! (@ exp $ ($ expr) , +) . map (| v | lex :: Node :: sequence (lex_seq ! (@ args [] , v , $ ($ expr) , +))) } ; }
    #[allow(unused_macros)]
    macro_rules! lex_zero_or_more {
        ($ kind : ident , $ expr : expr) => {
            lex_zero_or_more!($expr).map(|element| lex::Node::named(TokenKind::$kind, element))
        };
        ($ expr : expr) => {
            $expr.repeated().map(|v| lex::Node::sequence(v))
        };
    }
    #[allow(unused_macros)]
    macro_rules! lex_one_or_more {
        ($ kind : ident , $ expr : expr) => {
            lex_one_or_more!($expr).map(|element| lex::Node::named(TokenKind::$kind, element))
        };
        ($ expr : expr) => {
            $expr.repeated().at_least(1).map(|v| lex::Node::sequence(v))
        };
    }
    #[allow(unused_macros)]
    macro_rules! lex_repeated {
        ($ kind : ident , $ expr : expr , $ min : literal , $ max : literal) => {
            lex_repeated!($expr, $min, $max)
                .map(|element| lex::Node::named(TokenKind::$kind, element))
        };
        ($ expr : expr , $ min : literal , $ max : literal) => {
            $expr
                .repeated()
                .at_least($min)
                .at_most($max)
                .map(|v| lex::Node::sequence(v))
        };
    }
    #[allow(unused_macros)]
    macro_rules! lex_optional {
        ($ expr : expr) => {
            $expr.or_not().map(|v| v.flatten())
        };
    }
    #[allow(unused_macros)]
    macro_rules! lex_separated_by {
        ($ kind : ident , $ expr : expr , $ separator : expr) => {
            lex_separated_by!($expr, $separator)
                .map(|element| lex::Node::named(TokenKind::$kind, element))
        };
        ($ expr : expr , $ separator : expr) => {
            $expr
                .then($separator.then($expr).repeated())
                .map(|(first, rest)| {
                    let mut v = vec![first];
                    for (separator, expr) in rest {
                        v.push(separator);
                        v.push(expr);
                    }
                    lex::Node::sequence(v)
                })
        };
    }
    #[allow(unused_macros)]
    macro_rules ! lex_trie { ($ kind : ident , $ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr) , *)) . map_with_span (| leaf_kind , span : SpanType | lex :: Node :: named (TokenKind :: $ kind , lex :: Node :: named (leaf_kind , lex :: Node :: chars (span . start () .. span . end ())))) } ; ($ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr) , *)) . map_with_span (| kind , span : SpanType | lex :: Node :: named (kind , lex :: Node :: chars (span . start () .. span . end ()))) } ; }
    #[allow(unused_macros)]
    macro_rules! trieleaf {
        ($ kind : ident , $ string : literal) => {
            just($string).to(TokenKind::$kind)
        };
        ($ kind : ident) => {
            empty().to(TokenKind::$kind)
        };
    }
    #[allow(unused_macros)]
    macro_rules ! trieprefix { ($ string : literal , [$ ($ expr : expr) , *]) => (just ($ string) . ignore_then (choice :: < _ , ErrorType > (($ ($ expr) , *)))) }
    #[allow(unused_macros)]
    macro_rules! trivia_terminal {
        ($ kind : ident , $ literal : literal) => {
            just($literal).map_with_span(|_, span: SpanType| {
                cst::Node::trivia_token(
                    TokenKind::$kind,
                    lex::Node::chars_unwrapped(span.start()..span.end()),
                )
            })
        };
        ($ kind : ident , $ filter : expr) => {
            filter($filter).map_with_span(|_, span: SpanType| {
                cst::Node::trivia_token(
                    TokenKind::$kind,
                    lex::Node::chars_unwrapped(span.start()..span.end()),
                )
            })
        };
    }
    #[allow(unused_macros)]
    macro_rules ! trivia_token { ($ token_rule : ident) => { $ token_rule . clone () . map (| token : Option < Rc < lex :: Node >> | { let token = token . unwrap () ; if let lex :: Node :: Named (kind , element) = token . as_ref () { cst :: Node :: trivia_token (* kind , element . clone ()) } else { unreachable ! ("a token rule should always return a named token, but rule {} returned {:?}" , stringify ! ($ token_rule) , token) } }) } ; }
    #[allow(unused_macros)]
    macro_rules ! trivia_trie { ($ ($ expr : expr) , *) => (choice :: < _ , ErrorType > (($ ($ expr) , *)) . map_with_span (| kind , span : SpanType | cst :: Node :: trivia_token (kind , lex :: Node :: chars_unwrapped (span . start () .. span . end ())))) }
    #[allow(unused_macros)]
    macro_rules! terminal {
        ($ kind : ident , $ literal : literal) => {
            LeadingTrivia
                .clone()
                .then(just($literal).map_with_span(|_, span: SpanType| span.start()..span.end()))
                .then(TrailingTrivia.clone())
                .map(|((leading_trivia, range), trailing_trivia)| {
                    cst::Node::token(
                        TokenKind::$kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                })
        };
        ($ kind : ident , $ filter : expr) => {
            LeadingTrivia
                .clone()
                .then(filter($filter).map_with_span(|_, span: SpanType| span.start()..span.end()))
                .then(TrailingTrivia.clone())
                .map(|((leading_trivia, range), trailing_trivia)| {
                    cst::Node::token(
                        TokenKind::$kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                })
        };
    }
    #[allow(unused_macros)]
    macro_rules ! token { ($ token_rule : ident) => { LeadingTrivia . clone () . then ($ token_rule . clone ()) . then (TrailingTrivia . clone ()) . map (| ((leading_trivia , token) , trailing_trivia) : ((_ , Option < Rc < lex :: Node >>) , _) | { let token = token . unwrap () ; if let lex :: Node :: Named (kind , element) = token . as_ref () { cst :: Node :: token (* kind , element . clone () , leading_trivia , trailing_trivia) } else { unreachable ! ("a token rule should always return a named token, but rule {} returned {:?}" , stringify ! ($ token_rule) , token) } }) } ; }
    #[allow(unused_macros)]
    macro_rules! rule {
        ($ rule : ident) => {
            $rule.clone()
        };
    }
    #[allow(unused_macros)]
    macro_rules ! choice { ($ kind : ident , $ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr) , *)) } ; ($ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr) , *)) } ; }
    #[allow(unused_macros)]
    macro_rules ! seq { (@ exp $ head : expr , $ ($ tail : expr) , +) => { $ head . then (seq ! (@ exp $ ($ tail) , +)) } ; (@ exp $ head : expr) => { $ head } ; (@ args [$ ($ accum : expr ,) *] , $ current : expr , $ head : expr , $ ($ tail : expr) , +) => { seq ! (@ args [$ ($ accum ,) * $ current . 0 ,] , $ current . 1 , $ ($ tail) , +) } ; (@ args [$ ($ accum : expr ,) *] , $ current : expr , $ head : expr) => { vec ! [$ ($ accum ,) * $ current] } ; ($ kind : ident , $ ($ expr : expr) , +) => { seq ! (@ exp $ ($ expr) , +) . map (| v | cst :: Node :: rule (RuleKind :: $ kind , seq ! (@ args [] , v , $ ($ expr) , +))) } ; ($ ($ expr : expr) , +) => { seq ! (@ exp $ ($ expr) , +) . map (| v | cst :: Node :: group (seq ! (@ args [] , v , $ ($ expr) , +))) } ; }
    #[allow(unused_macros)]
    macro_rules! zero_or_more {
        ($ kind : ident , $ expr : expr) => {
            $expr
                .repeated()
                .map(|children| cst::Node::rule(RuleKind::$kind, children))
        };
        ($ expr : expr) => {
            $expr.repeated().map(|children| cst::Node::group(children))
        };
    }
    #[allow(unused_macros)]
    macro_rules! one_or_more {
        ($ kind : ident , $ expr : expr) => {
            $expr
                .repeated()
                .at_least(1)
                .map(|children| cst::Node::rule(RuleKind::$kind, children))
        };
        ($ expr : expr) => {
            $expr
                .repeated()
                .at_least(1)
                .map(|children| cst::Node::group(children))
        };
    }
    #[allow(unused_macros)]
    macro_rules! repeated {
        ($ kind : ident , $ expr : expr , $ min : literal , $ max : literal) => {
            $expr
                .repeated()
                .at_least($min)
                .at_most($max)
                .map(|children| cst::Node::rule(RuleKind::$kind, children))
        };
        ($ expr : expr , $ min : literal , $ max : literal) => {
            $expr
                .repeated()
                .at_least($min)
                .at_most($max)
                .map(|children| cst::Node::group(children))
        };
    }
    #[allow(unused_macros)]
    macro_rules! optional {
        ($ expr : expr) => {
            $expr.or_not().map(|opt| opt.flatten())
        };
    }
    #[allow(unused_macros)]
    macro_rules! separated_by {
        ($ kind : ident , $ expr : expr , $ separator : expr) => {
            $expr
                .then($separator.then($expr).repeated())
                .map(|(first, rest)| {
                    let mut v = vec![first];
                    for (separator, expr) in rest {
                        v.push(separator);
                        v.push(expr);
                    }
                    cst::Node::rule(RuleKind::$kind, v)
                })
        };
        ($ expr : expr , $ separator : expr) => {
            $expr
                .then($separator.then($expr).repeated())
                .map(|(first, rest)| {
                    let mut v = vec![first];
                    for (separator, expr) in rest {
                        v.push(separator);
                        v.push(expr);
                    }
                    cst::Node::group(v)
                })
        };
    }
    #[allow(unused_macros)]
    macro_rules! left_associative_binary_expression {
        ($ kind : ident , $ next_sibling : ident , $ operator : expr) => {
            $next_sibling
                .clone()
                .then($operator.then($next_sibling.clone()).repeated())
                .map(|(first, rest)| {
                    if rest.is_empty() {
                        first
                    } else {
                        rest.into_iter()
                            .fold(first, |left_operand, (operator, right_operand)| {
                                cst::Node::rule(
                                    RuleKind::$kind,
                                    vec![left_operand, operator, right_operand],
                                )
                            })
                    }
                })
        };
    }
    #[allow(unused_macros)]
    macro_rules! right_associative_binary_expression {
        ($ kind : ident , $ next_sibling : ident , $ operator : expr) => {
            $next_sibling
                .clone()
                .then($operator.then($next_sibling.clone()).repeated())
                .map(|(first, rest)| {
                    if rest.is_empty() {
                        first
                    } else {
                        let mut last_operand = first;
                        let mut operand_operator_pairs = vec![];
                        for (operator, right_operand) in rest.into_iter() {
                            let left_operand = std::mem::replace(&mut last_operand, right_operand);
                            operand_operator_pairs.push((left_operand, operator))
                        }
                        operand_operator_pairs.into_iter().rfold(
                            last_operand,
                            |right_operand, (left_operand, operator)| {
                                cst::Node::rule(
                                    RuleKind::$kind,
                                    vec![left_operand, operator, right_operand],
                                )
                            },
                        )
                    }
                })
        };
    }
    #[allow(unused_macros)]
    macro_rules! unary_prefix_expression {
        ($ kind : ident , $ next_sibling : ident , $ operator : expr) => {
            $operator
                .repeated()
                .then($next_sibling.clone())
                .map(|(mut operators, operand)| {
                    if operators.is_empty() {
                        operand
                    } else {
                        operators.reverse();
                        operators
                            .into_iter()
                            .fold(operand, |right_operand, operator| {
                                cst::Node::rule(RuleKind::$kind, vec![operator, right_operand])
                            })
                    }
                })
        };
    }
    #[allow(unused_macros)]
    macro_rules! unary_suffix_expression {
        ($ kind : ident , $ next_sibling : ident , $ operator : expr) => {
            $next_sibling
                .clone()
                .then($operator.repeated())
                .map(|(operand, operators)| {
                    if operators.is_empty() {
                        operand
                    } else {
                        operators
                            .into_iter()
                            .fold(operand, |left_operand, operator| {
                                cst::Node::rule(RuleKind::$kind, vec![left_operand, operator])
                            })
                    }
                })
        };
    }
    #[allow(unused_macros)]
    macro_rules! delimited_by {
        ($ kind : ident , $ open : expr , $ expr : expr , $ close : expr) => {
            seq!($kind, $open, $expr, $close)
        };
        ($ open : expr , $ expr : expr , $ close : expr) => {
            seq!($open, $expr, $close)
        };
    }
    #[allow(unused_macros)]
    macro_rules ! trie { ($ kind : ident , $ ($ expr : expr) , *) => { trie ! ($ ($ expr) , *) . map (| child | cst :: Node :: rule (RuleKind :: $ kind , vec ! [child])) } ; ($ ($ expr : expr) , *) => (LeadingTrivia . clone () . then (choice :: < _ , ErrorType > (($ ($ expr) , *)) . map_with_span (| kind , span : SpanType | (kind , span . start () .. span . end ()))) . then (TrailingTrivia . clone ()) . map (| ((leading_trivia , (kind , range)) , trailing_trivia) | { cst :: Node :: token (kind , lex :: Node :: chars_unwrapped (range) , leading_trivia , trailing_trivia) })) }
    #[allow(unused_macros)]
    macro_rules! define_rule {
        ($ kind : ident , $ expr : expr) => {{
            $kind.define($expr);
        }
        parsers.insert(
            ProductionKind::$kind,
            Parser::new(
                $kind
                    .clone()
                    .map(|node| cst::Node::top_level_rule(RuleKind::$kind, node))
                    .then_ignore(end())
                    .boxed(),
            ),
        );};
    }
    #[allow(unused_macros)]
    macro_rules! define_token {
        ($ kind : ident , $ expr : expr) => {{
            $kind.define($expr);
        }
        parsers.insert(
            ProductionKind::$kind,
            Parser::new(
                $kind
                    .clone()
                    .map(|node| cst::Node::top_level_token(node))
                    .then_ignore(end())
                    .boxed(),
            ),
        );};
    }

    // Define all productions ---------------------------

    // ABICoderPragma = 'abicoder' «Identifier» ;
    define_rule!(
        ABICoderPragma,
        seq!(
            ABICoderPragma,
            terminal!(Abicoder, "abicoder"),
            token!(Identifier)
        )
    );

    // AddSubExpression = Expression ( '+' | '-' ) Expression ;
    define_rule!(
        AddSubExpression,
        left_associative_binary_expression!(
            AddSubExpression,
            MulDivModExpression,
            trie!(trieleaf!(Plus, "+"), trieleaf!(Minus, "-"))
        )
    );

    // AddressType = 'address' [ 'payable' ] ;
    define_rule!(
        AddressType,
        seq!(
            AddressType,
            terminal!(Address, "address"),
            optional!(terminal!(Payable, "payable"))
        )
    );

    // AndExpression = Expression '&&' Expression ;
    define_rule!(
        AndExpression,
        left_associative_binary_expression!(
            AndExpression,
            EqualityComparisonExpression,
            terminal!(AmpersandAmpersand, "&&")
        )
    );

    // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
    define_rule!(
        ArgumentList,
        delimited_by!(
            ArgumentList,
            terminal!(OpenParen, "("),
            optional!(choice!(
                rule!(PositionalArgumentList),
                rule!(NamedArgumentList)
            )),
            terminal!(CloseParen, ")")
        )
    );

    // ArrayLiteral = '[' Expression  { ',' Expression } ']' ;
    define_rule!(
        ArrayLiteral,
        delimited_by!(
            ArrayLiteral,
            terminal!(OpenBracket, "["),
            separated_by!(
                SeparatedExpressions,
                rule!(Expression),
                terminal!(Comma, ",")
            ),
            terminal!(CloseBracket, "]")
        )
    );

    // «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
    define_token!(
        AsciiEscape,
        lex_terminal!(AsciiEscape, |&c: &char| c == 'n'
            || c == 'r'
            || c == 't'
            || c == '\''
            || c == '"'
            || c == '\\'
            || c == '\n'
            || c == '\r')
    );

    // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
    define_token!(
        AsciiStringLiteral,
        lex_choice!(
            AsciiStringLiteral,
            lex_rule!(SingleQuotedAsciiStringLiteral),
            lex_rule!(DoubleQuotedAsciiStringLiteral)
        )
    );

    // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
    define_rule!(
        AssemblyFlags,
        delimited_by!(
            AssemblyFlags,
            terminal!(OpenParen, "("),
            separated_by!(
                SeparatedDoubleQuotedAsciiStringLiterals,
                token!(DoubleQuotedAsciiStringLiteral),
                terminal!(Comma, ",")
            ),
            terminal!(CloseParen, ")")
        )
    );

    // AssemblyStatement = 'assembly' [ '"evmasm"' ] [ AssemblyFlags ] YulBlock ;
    define_rule!(
        AssemblyStatement,
        seq!(
            AssemblyStatement,
            terminal!(Assembly, "assembly"),
            optional!(terminal!(DoubleQuoteEvmasmDoubleQuote, "\"evmasm\"")),
            optional!(rule!(AssemblyFlags)),
            rule!(YulBlock)
        )
    );

    // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
    define_rule!(
        AssignmentExpression,
        left_associative_binary_expression!(
            AssignmentExpression,
            ConditionalExpression,
            trie!(
                trieleaf!(PercentEqual, "%="),
                trieleaf!(AmpersandEqual, "&="),
                trieleaf!(StarEqual, "*="),
                trieleaf!(PlusEqual, "+="),
                trieleaf!(MinusEqual, "-="),
                trieleaf!(SlashEqual, "/="),
                trieleaf!(LessLessEqual, "<<="),
                trieleaf!(Equal, "="),
                trieprefix!(
                    ">>",
                    [
                        trieleaf!(GreaterGreaterEqual, "="),
                        trieleaf!(GreaterGreaterGreaterEqual, ">=")
                    ]
                ),
                trieleaf!(CaretEqual, "^="),
                trieleaf!(PipeEqual, "|=")
            )
        )
    );

    // BitAndExpression = Expression '&' Expression ;
    define_rule!(
        BitAndExpression,
        left_associative_binary_expression!(
            BitAndExpression,
            ShiftExpression,
            terminal!(Ampersand, "&")
        )
    );

    // BitOrExpression = Expression '|' Expression ;
    define_rule!(
        BitOrExpression,
        left_associative_binary_expression!(
            BitOrExpression,
            BitXOrExpression,
            terminal!(Pipe, "|")
        )
    );

    // BitXOrExpression = Expression '^' Expression ;
    define_rule!(
        BitXOrExpression,
        left_associative_binary_expression!(
            BitXOrExpression,
            BitAndExpression,
            terminal!(Caret, "^")
        )
    );

    // Block = '{' { Statement | UncheckedBlock } '}' ;
    define_rule!(
        Block,
        delimited_by!(
            Block,
            terminal!(OpenBrace, "{"),
            zero_or_more!(choice!(rule!(Statement), rule!(UncheckedBlock))),
            terminal!(CloseBrace, "}")
        )
    );

    // «BooleanLiteral» = 'true' | 'false' ;
    define_token!(
        BooleanLiteral,
        lex_trie!(
            BooleanLiteral,
            trieleaf!(False, "false"),
            trieleaf!(True, "true")
        )
    );

    // BreakStatement = 'break' ';' ;
    define_rule!(
        BreakStatement,
        seq!(
            BreakStatement,
            terminal!(Break, "break"),
            terminal!(Semicolon, ";")
        )
    );

    // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
    define_rule!(
        CatchClause,
        seq!(
            CatchClause,
            terminal!(Catch, "catch"),
            optional!(seq!(optional!(token!(Identifier)), rule!(ParameterList))),
            rule!(Block)
        )
    );

    // ConditionalExpression = Expression '?' Expression ':' Expression ;
    define_rule!(
        ConditionalExpression,
        unary_suffix_expression!(
            ConditionalExpression,
            OrExpression,
            seq!(
                terminal!(Question, "?"),
                rule!(Expression),
                terminal!(Colon, ":"),
                rule!(Expression)
            )
        )
    );

    // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
    define_rule!(
        ConstantDefinition,
        seq!(
            ConstantDefinition,
            rule!(TypeName),
            terminal!(Constant, "constant"),
            token!(Identifier),
            terminal!(Equal, "="),
            rule!(Expression),
            terminal!(Semicolon, ";")
        )
    );

    // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
    define_rule!(
        ConstructorAttribute,
        choice!(
            ConstructorAttribute,
            rule!(ModifierInvocation),
            trie!(
                trieleaf!(Internal, "internal"),
                trieprefix!(
                    "p",
                    [trieleaf!(Payable, "ayable"), trieleaf!(Public, "ublic")]
                )
            )
        )
    );

    // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
    define_rule!(
        ConstructorDefinition,
        seq!(
            ConstructorDefinition,
            terminal!(Constructor, "constructor"),
            rule!(ParameterList),
            zero_or_more!(ConstructorAttributes, rule!(ConstructorAttribute)),
            rule!(Block)
        )
    );

    // ContinueStatement = 'continue' ';' ;
    define_rule!(
        ContinueStatement,
        seq!(
            ContinueStatement,
            terminal!(Continue, "continue"),
            terminal!(Semicolon, ";")
        )
    );

    // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
    define_rule!(
        ContractBodyElement,
        choice!(
            ContractBodyElement,
            rule!(UsingDirective),
            rule!(ConstructorDefinition),
            rule!(FunctionDefinition),
            rule!(FallbackFunctionDefinition),
            rule!(ReceiveFunctionDefinition),
            rule!(ModifierDefinition),
            rule!(StructDefinition),
            rule!(EnumDefinition),
            rule!(UserDefinedValueTypeDefinition),
            rule!(EventDefinition),
            rule!(ErrorDefinition),
            rule!(StateVariableDeclaration)
        )
    );

    // ContractDefinition = [ 'abstract' ] 'contract' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    define_rule!(
        ContractDefinition,
        seq!(
            ContractDefinition,
            optional!(terminal!(Abstract, "abstract")),
            terminal!(Contract, "contract"),
            token!(Identifier),
            optional!(rule!(InheritanceSpecifierList)),
            delimited_by!(
                DelimitedContractBodyElements,
                terminal!(OpenBrace, "{"),
                zero_or_more!(ContractBodyElements, rule!(ContractBodyElement)),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // DataLocation = 'memory' | 'storage' | 'calldata' ;
    define_rule!(
        DataLocation,
        trie!(
            DataLocation,
            trieleaf!(Calldata, "calldata"),
            trieleaf!(Memory, "memory"),
            trieleaf!(Storage, "storage")
        )
    );

    // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
    define_token!(
        DecimalExponent,
        lex_seq!(
            DecimalExponent,
            lex_terminal!(|&c: &char| c == 'e' || c == 'E'),
            lex_optional!(lex_terminal!(Minus, '-')),
            lex_rule!(DecimalInteger)
        )
    );

    // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
    define_token!(
        DecimalFloat,
        lex_seq!(
            DecimalFloat,
            lex_optional!(lex_rule!(DecimalInteger)),
            lex_terminal!(Period, '.'),
            lex_rule!(DecimalInteger)
        )
    );

    // «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
    define_token!(
        DecimalInteger,
        lex_seq!(
            DecimalInteger,
            lex_terminal!(|&c: &char| ('0' <= c && c <= '9')),
            lex_zero_or_more!(lex_seq!(
                lex_optional!(lex_terminal!(Underscore, '_')),
                lex_terminal!(|&c: &char| ('0' <= c && c <= '9'))
            ))
        )
    );

    // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
    define_token!(
        DecimalNumber,
        lex_seq!(
            DecimalNumber,
            lex_choice!(lex_rule!(DecimalInteger), lex_rule!(DecimalFloat)),
            lex_optional!(lex_rule!(DecimalExponent))
        )
    );

    // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
    define_rule!(
        Definition,
        choice!(
            Definition,
            rule!(ContractDefinition),
            rule!(InterfaceDefinition),
            rule!(LibraryDefinition),
            rule!(FunctionDefinition),
            rule!(ConstantDefinition),
            rule!(StructDefinition),
            rule!(EnumDefinition),
            rule!(UserDefinedValueTypeDefinition),
            rule!(ErrorDefinition)
        )
    );

    // DeleteStatement = 'delete' «Identifier» ';' ;
    define_rule!(
        DeleteStatement,
        seq!(
            DeleteStatement,
            terminal!(Delete, "delete"),
            token!(Identifier),
            terminal!(Semicolon, ";")
        )
    );

    // Directive = PragmaDirective | ImportDirective | UsingDirective ;
    define_rule!(
        Directive,
        choice!(
            Directive,
            rule!(PragmaDirective),
            rule!(ImportDirective),
            rule!(UsingDirective)
        )
    );

    // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
    define_rule!(
        DoWhileStatement,
        seq!(
            DoWhileStatement,
            terminal!(Do, "do"),
            rule!(Statement),
            terminal!(While, "while"),
            delimited_by!(
                DelimitedExpression,
                terminal!(OpenParen, "("),
                rule!(Expression),
                terminal!(CloseParen, ")")
            ),
            terminal!(Semicolon, ";")
        )
    );

    // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
    define_token!(
        DoubleQuotedAsciiStringLiteral,
        lex_seq!(
            DoubleQuotedAsciiStringLiteral,
            lex_terminal!(DoubleQuote, "\""),
            lex_zero_or_more!(
                Runs,
                lex_choice!(
                    Run,
                    lex_one_or_more!(
                        Chars,
                        lex_terminal!(Char, |&c: &char| (' ' <= c && c <= '~')
                            && c != '"'
                            && c != '\\')
                    ),
                    lex_rule!(EscapeSequence)
                )
            ),
            lex_terminal!(DoubleQuote, "\"")
        )
    );

    // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
    define_token!(
        DoubleQuotedUnicodeStringLiteral,
        lex_seq!(
            DoubleQuotedUnicodeStringLiteral,
            lex_terminal!(UnicodeDoubleQuote, "unicode\""),
            lex_zero_or_more!(
                Runs,
                lex_choice!(
                    Run,
                    lex_one_or_more!(
                        Chars,
                        lex_terminal!(Char, |&c: &char| c != '"'
                            && c != '\\'
                            && c != '\n'
                            && c != '\r')
                    ),
                    lex_rule!(EscapeSequence)
                )
            ),
            lex_terminal!(DoubleQuote, "\"")
        )
    );

    // ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
    define_rule!(
        ElementaryType,
        choice!(
            ElementaryType,
            trie!(trieleaf!(Bool, "bool"), trieleaf!(String, "string")),
            rule!(AddressType),
            token!(FixedBytesType),
            token!(SignedIntegerType),
            token!(UnsignedIntegerType),
            token!(SignedFixedType),
            token!(UnsignedFixedType)
        )
    );

    // EmitStatement = 'emit' IdentifierPath ArgumentList ';' ;
    define_rule!(
        EmitStatement,
        seq!(
            EmitStatement,
            terminal!(Emit, "emit"),
            rule!(IdentifierPath),
            rule!(ArgumentList),
            terminal!(Semicolon, ";")
        )
    );

    // EndOfFileTrivia = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
    define_rule!(
        EndOfFileTrivia,
        zero_or_more!(
            EndOfFileTrivia,
            choice!(
                trivia_token!(Whitespace),
                trivia_token!(MultilineComment),
                trivia_token!(SingleLineComment)
            )
        )
    );

    // «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
    define_token!(
        EndOfLine,
        lex_one_or_more!(EndOfLine, lex_terminal!(|&c: &char| c == '\r' || c == '\n'))
    );

    // EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
    define_rule!(
        EnumDefinition,
        seq!(
            EnumDefinition,
            terminal!(Enum, "enum"),
            token!(Identifier),
            delimited_by!(
                DelimitedSeparatedIdentifiers,
                terminal!(OpenBrace, "{"),
                separated_by!(
                    SeparatedIdentifiers,
                    token!(Identifier),
                    terminal!(Comma, ",")
                ),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
    define_rule!(
        EqualityComparisonExpression,
        left_associative_binary_expression!(
            EqualityComparisonExpression,
            OrderComparisonExpression,
            trie!(trieleaf!(BangEqual, "!="), trieleaf!(EqualEqual, "=="))
        )
    );

    // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
    define_rule!(
        ErrorDefinition,
        seq!(
            ErrorDefinition,
            terminal!(Error, "error"),
            token!(Identifier),
            delimited_by!(
                DelimitedSeparatedErrorParameters,
                terminal!(OpenParen, "("),
                optional!(separated_by!(
                    SeparatedErrorParameters,
                    rule!(ErrorParameter),
                    terminal!(Comma, ",")
                )),
                terminal!(CloseParen, ")")
            ),
            terminal!(Semicolon, ";")
        )
    );

    // ErrorParameter = TypeName [ «Identifier» ] ;
    define_rule!(
        ErrorParameter,
        seq!(
            ErrorParameter,
            rule!(TypeName),
            optional!(token!(Identifier))
        )
    );

    // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
    define_token!(
        EscapeSequence,
        lex_seq!(
            EscapeSequence,
            lex_terminal!(Backslash, '\\'),
            lex_choice!(
                lex_trie!(
                    trieleaf!(Linefeed, "\n"),
                    trieleaf!(CarriageReturn, "\r"),
                    trieleaf!(DoubleQuote, "\""),
                    trieleaf!(Quote, "'"),
                    trieleaf!(Backslash, "\\"),
                    trieleaf!(LatinSmallLetterN, "n"),
                    trieleaf!(LatinSmallLetterR, "r"),
                    trieleaf!(LatinSmallLetterT, "t")
                ),
                lex_rule!(HexByteEscape),
                lex_rule!(UnicodeEscape)
            )
        )
    );

    // EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
    define_rule!(
        EventDefinition,
        seq!(
            EventDefinition,
            terminal!(Event, "event"),
            token!(Identifier),
            delimited_by!(
                DelimitedSeparatedEventParameters,
                terminal!(OpenParen, "("),
                optional!(separated_by!(
                    SeparatedEventParameters,
                    rule!(EventParameter),
                    terminal!(Comma, ",")
                )),
                terminal!(CloseParen, ")")
            ),
            optional!(terminal!(Anonymous, "anonymous")),
            terminal!(Semicolon, ";")
        )
    );

    // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
    define_rule!(
        EventParameter,
        seq!(
            EventParameter,
            rule!(TypeName),
            optional!(terminal!(Indexed, "indexed")),
            optional!(token!(Identifier))
        )
    );

    // ExperimentalPragma = 'experimental' «Identifier» ;
    define_rule!(
        ExperimentalPragma,
        seq!(
            ExperimentalPragma,
            terminal!(Experimental, "experimental"),
            token!(Identifier)
        )
    );

    // (* 0.4.11 *) ExponentiationExpression = Expression '**' Expression ;
    // (* 0.6.0 *) ExponentiationExpression = Expression '**' Expression ;
    define_rule!(
        ExponentiationExpression,
        if version_0_6_0 <= version {
            right_associative_binary_expression!(
                ExponentiationExpression,
                UnarySuffixExpression,
                terminal!(StarStar, "**")
            )
            .boxed()
        } else {
            left_associative_binary_expression!(
                ExponentiationExpression,
                UnarySuffixExpression,
                terminal!(StarStar, "**")
            )
            .boxed()
        }
    );

    // Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    define_rule!(Expression, rule!(AssignmentExpression));

    // ExpressionStatement = Expression ';' ;
    define_rule!(
        ExpressionStatement,
        seq!(
            ExpressionStatement,
            rule!(Expression),
            terminal!(Semicolon, ";")
        )
    );

    // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
    define_rule!(
        FallbackFunctionAttribute,
        choice!(
            FallbackFunctionAttribute,
            rule!(ModifierInvocation),
            rule!(OverrideSpecifier),
            trie!(
                trieleaf!(External, "external"),
                trieprefix!("p", [trieleaf!(Payable, "ayable"), trieleaf!(Pure, "ure")]),
                trieprefix!("vi", [trieleaf!(View, "ew"), trieleaf!(Virtual, "rtual")])
            )
        )
    );

    // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    define_rule!(
        FallbackFunctionDefinition,
        seq!(
            FallbackFunctionDefinition,
            terminal!(Fallback, "fallback"),
            rule!(ParameterList),
            zero_or_more!(FallbackFunctionAttributes, rule!(FallbackFunctionAttribute)),
            optional!(seq!(terminal!(Returns, "returns"), rule!(ParameterList))),
            choice!(terminal!(Semicolon, ";"), rule!(Block))
        )
    );

    // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
    define_token!(
        FixedBytesType,
        lex_seq!(
            FixedBytesType,
            lex_terminal!(Bytes, "bytes"),
            lex_trie!(
                Count,
                trieprefix!(
                    "1",
                    [
                        trieleaf!(OneZero, "0"),
                        trieleaf!(OneOne, "1"),
                        trieleaf!(OneTwo, "2"),
                        trieleaf!(OneThree, "3"),
                        trieleaf!(OneFour, "4"),
                        trieleaf!(OneFive, "5"),
                        trieleaf!(OneSix, "6"),
                        trieleaf!(OneSeven, "7"),
                        trieleaf!(OneEight, "8"),
                        trieleaf!(OneNine, "9"),
                        trieleaf!(One)
                    ]
                ),
                trieprefix!(
                    "2",
                    [
                        trieleaf!(TwoZero, "0"),
                        trieleaf!(TwoOne, "1"),
                        trieleaf!(TwoTwo, "2"),
                        trieleaf!(TwoThree, "3"),
                        trieleaf!(TwoFour, "4"),
                        trieleaf!(TwoFive, "5"),
                        trieleaf!(TwoSix, "6"),
                        trieleaf!(TwoSeven, "7"),
                        trieleaf!(TwoEight, "8"),
                        trieleaf!(TwoNine, "9"),
                        trieleaf!(Two)
                    ]
                ),
                trieprefix!(
                    "3",
                    [
                        trieleaf!(ThreeZero, "0"),
                        trieleaf!(ThreeOne, "1"),
                        trieleaf!(ThreeTwo, "2"),
                        trieleaf!(Three)
                    ]
                ),
                trieleaf!(Four, "4"),
                trieleaf!(Five, "5"),
                trieleaf!(Six, "6"),
                trieleaf!(Seven, "7"),
                trieleaf!(Eight, "8"),
                trieleaf!(Nine, "9")
            )
        )
    );

    // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
    define_rule!(
        ForStatement,
        seq!(
            ForStatement,
            terminal!(For, "for"),
            delimited_by!(
                terminal!(OpenParen, "("),
                seq!(
                    choice!(rule!(SimpleStatement), terminal!(Semicolon, ";")),
                    choice!(rule!(ExpressionStatement), terminal!(Semicolon, ";")),
                    optional!(rule!(Expression))
                ),
                terminal!(CloseParen, ")")
            ),
            rule!(Statement)
        )
    );

    // FunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'internal' | 'payable' | 'private' | 'public' | 'pure' | 'view' | 'virtual' ;
    define_rule!(
        FunctionAttribute,
        choice!(
            FunctionAttribute,
            rule!(ModifierInvocation),
            rule!(OverrideSpecifier),
            trie!(
                trieleaf!(External, "external"),
                trieleaf!(Internal, "internal"),
                trieprefix!(
                    "p",
                    [
                        trieleaf!(Payable, "ayable"),
                        trieleaf!(Private, "rivate"),
                        trieprefix!("u", [trieleaf!(Public, "blic"), trieleaf!(Pure, "re")])
                    ]
                ),
                trieprefix!("vi", [trieleaf!(View, "ew"), trieleaf!(Virtual, "rtual")])
            )
        )
    );

    // FunctionCallExpression = Expression [ '{' NamedArgument  { ',' NamedArgument } '}' ] ArgumentList ;
    define_rule!(
        FunctionCallExpression,
        unary_suffix_expression!(
            FunctionCallExpression,
            MemberAccessExpression,
            seq!(
                optional!(delimited_by!(
                    DelimitedSeparatedNamedArguments,
                    terminal!(OpenBrace, "{"),
                    separated_by!(
                        SeparatedNamedArguments,
                        rule!(NamedArgument),
                        terminal!(Comma, ",")
                    ),
                    terminal!(CloseBrace, "}")
                )),
                rule!(ArgumentList)
            )
        )
    );

    // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    define_rule!(
        FunctionDefinition,
        seq!(
            FunctionDefinition,
            terminal!(Function, "function"),
            choice!(
                token!(Identifier),
                trie!(
                    trieleaf!(Fallback, "fallback"),
                    trieleaf!(Receive, "receive")
                )
            ),
            rule!(ParameterList),
            zero_or_more!(FunctionAttributes, rule!(FunctionAttribute)),
            optional!(seq!(terminal!(Returns, "returns"), rule!(ParameterList))),
            choice!(terminal!(Semicolon, ";"), rule!(Block))
        )
    );

    // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
    define_rule!(
        FunctionType,
        seq!(
            FunctionType,
            terminal!(Function, "function"),
            rule!(ParameterList),
            zero_or_more!(trie!(
                trieleaf!(External, "external"),
                trieleaf!(Internal, "internal"),
                trieprefix!(
                    "p",
                    [
                        trieleaf!(Payable, "ayable"),
                        trieleaf!(Private, "rivate"),
                        trieprefix!("u", [trieleaf!(Public, "blic"), trieleaf!(Pure, "re")])
                    ]
                ),
                trieleaf!(View, "view")
            )),
            optional!(seq!(terminal!(Returns, "returns"), rule!(ParameterList)))
        )
    );

    // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
    define_token!(
        HexByteEscape,
        lex_seq!(
            HexByteEscape,
            lex_terminal!(LatinSmallLetterX, 'x'),
            lex_repeated!(
                lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                    || ('a' <= c && c <= 'f')
                    || ('A' <= c && c <= 'F')),
                2usize,
                2usize
            )
        )
    );

    // «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
    define_token!(
        HexCharacter,
        lex_terminal!(HexCharacter, |&c: &char| ('0' <= c && c <= '9')
            || ('a' <= c && c <= 'f')
            || ('A' <= c && c <= 'F'))
    );

    // «HexNumber» = '0x' «HexCharacter» { [ '_' ] «HexCharacter» } ;
    define_token!(
        HexNumber,
        lex_seq!(
            HexNumber,
            lex_terminal!(ZeroX, "0x"),
            lex_seq!(
                lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                    || ('a' <= c && c <= 'f')
                    || ('A' <= c && c <= 'F')),
                lex_zero_or_more!(lex_seq!(
                    lex_optional!(lex_terminal!(Underscore, '_')),
                    lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                        || ('a' <= c && c <= 'f')
                        || ('A' <= c && c <= 'F'))
                ))
            )
        )
    );

    // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
    define_token!(
        HexStringLiteral,
        lex_seq!(
            HexStringLiteral,
            lex_terminal!(Hex, "hex"),
            lex_choice!(
                lex_seq!(
                    DelimitedPossiblySeparatedPairsOfHexDigits,
                    lex_terminal!(DoubleQuote, "\""),
                    lex_optional!(lex_rule!(PossiblySeparatedPairsOfHexDigits)),
                    lex_terminal!(DoubleQuote, "\"")
                ),
                lex_seq!(
                    DelimitedPossiblySeparatedPairsOfHexDigits,
                    lex_terminal!(Quote, "'"),
                    lex_optional!(lex_rule!(PossiblySeparatedPairsOfHexDigits)),
                    lex_terminal!(Quote, "'")
                )
            )
        )
    );

    // «Identifier» = «RawIdentifier» - «Keyword» ;
    define_token!(
        Identifier,
        difference(lex_rule!(RawIdentifier), lex_rule!(Keyword))
    );

    // «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
    define_token!(
        IdentifierPart,
        lex_terminal!(IdentifierPart, |&c: &char| c == '_'
            || c == '$'
            || ('a' <= c && c <= 'z')
            || ('A' <= c && c <= 'Z')
            || ('0' <= c && c <= '9'))
    );

    // IdentifierPath = «Identifier»  { '.' «Identifier» } ;
    define_rule!(
        IdentifierPath,
        separated_by!(IdentifierPath, token!(Identifier), terminal!(Period, "."))
    );

    // «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
    define_token!(
        IdentifierStart,
        lex_terminal!(IdentifierStart, |&c: &char| c == '_'
            || c == '$'
            || ('a' <= c && c <= 'z')
            || ('A' <= c && c <= 'Z'))
    );

    // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
    define_rule!(
        IfStatement,
        seq!(
            IfStatement,
            terminal!(If, "if"),
            delimited_by!(
                DelimitedExpression,
                terminal!(OpenParen, "("),
                rule!(Expression),
                terminal!(CloseParen, ")")
            ),
            rule!(Statement),
            optional!(seq!(terminal!(Else, "else"), rule!(Statement)))
        )
    );

    // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
    define_rule!(
        ImportDirective,
        seq!(
            ImportDirective,
            terminal!(Import, "import"),
            choice!(
                rule!(SimpleImportDirective),
                rule!(StarImportDirective),
                rule!(SelectingImportDirective)
            ),
            terminal!(Semicolon, ";")
        )
    );

    // ImportPath = «AsciiStringLiteral» ;
    define_rule!(ImportPath, token!(AsciiStringLiteral));

    // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
    define_rule!(
        IndexAccessExpression,
        unary_suffix_expression!(
            IndexAccessExpression,
            PrimaryExpression,
            delimited_by!(
                terminal!(OpenBracket, "["),
                seq!(
                    optional!(rule!(Expression)),
                    optional!(seq!(terminal!(Colon, ":"), optional!(rule!(Expression))))
                ),
                terminal!(CloseBracket, "]")
            )
        )
    );

    // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
    define_rule!(
        InheritanceSpecifier,
        seq!(
            InheritanceSpecifier,
            rule!(IdentifierPath),
            optional!(rule!(ArgumentList))
        )
    );

    // InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
    define_rule!(
        InheritanceSpecifierList,
        seq!(
            InheritanceSpecifierList,
            terminal!(Is, "is"),
            separated_by!(
                SeparatedInheritanceSpecifiers,
                rule!(InheritanceSpecifier),
                terminal!(Comma, ",")
            )
        )
    );

    // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    define_rule!(
        InterfaceDefinition,
        seq!(
            InterfaceDefinition,
            terminal!(Interface, "interface"),
            token!(Identifier),
            optional!(rule!(InheritanceSpecifierList)),
            delimited_by!(
                DelimitedContractBodyElements,
                terminal!(OpenBrace, "{"),
                zero_or_more!(ContractBodyElements, rule!(ContractBodyElement)),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
    define_token!(
        Keyword,
        lex_choice!(
            Keyword,
            lex_trie!(trieleaf!(False, "false"), trieleaf!(True, "true")),
            lex_rule!(FixedBytesType),
            lex_trie!(
                trieprefix!(
                    "a",
                    [
                        trieleaf!(After, "fter"),
                        trieleaf!(Alias, "lias"),
                        trieleaf!(Apply, "pply"),
                        trieleaf!(Auto, "uto")
                    ]
                ),
                trieleaf!(Byte, "byte"),
                trieprefix!("c", [trieleaf!(Case, "ase"), trieleaf!(Copyof, "opyof")]),
                trieprefix!(
                    "d",
                    [
                        trieleaf!(Days, "ays"),
                        trieprefix!("ef", [trieleaf!(Default, "ault"), trieleaf!(Define, "ine")])
                    ]
                ),
                trieleaf!(Ether, "ether"),
                trieprefix!("fin", [trieleaf!(Final, "al"), trieleaf!(Finney, "ney")]),
                trieleaf!(Gwei, "gwei"),
                trieleaf!(Hours, "hours"),
                trieprefix!(
                    "i",
                    [
                        trieleaf!(Implements, "mplements"),
                        trieprefix!("n", [trieleaf!(Inline, "line"), trieleaf!(In)])
                    ]
                ),
                trieleaf!(Let, "let"),
                trieprefix!(
                    "m",
                    [
                        trieprefix!("a", [trieleaf!(Macro, "cro"), trieleaf!(Match, "tch")]),
                        trieleaf!(Minutes, "inutes"),
                        trieleaf!(Mutable, "utable")
                    ]
                ),
                trieleaf!(Null, "null"),
                trieleaf!(Of, "of"),
                trieprefix!(
                    "p",
                    [trieleaf!(Partial, "artial"), trieleaf!(Promise, "romise")]
                ),
                trieprefix!(
                    "re",
                    [
                        trieleaf!(Reference, "ference"),
                        trieleaf!(Relocatable, "locatable")
                    ]
                ),
                trieprefix!(
                    "s",
                    [
                        trieprefix!(
                            "e",
                            [trieleaf!(Sealed, "aled"), trieleaf!(Seconds, "conds")]
                        ),
                        trieleaf!(Sizeof, "izeof"),
                        trieleaf!(Static, "tatic"),
                        trieleaf!(Supports, "upports"),
                        trieleaf!(Switch, "witch"),
                        trieleaf!(Szabo, "zabo")
                    ]
                ),
                trieprefix!("type", [trieleaf!(Typedef, "def"), trieleaf!(Typeof, "of")]),
                trieleaf!(Var, "var"),
                trieprefix!("we", [trieleaf!(Weeks, "eks"), trieleaf!(Wei, "i")]),
                trieleaf!(Years, "years")
            ),
            lex_rule!(SignedIntegerType),
            lex_rule!(UnsignedIntegerType),
            lex_trie!(
                trieprefix!(
                    "a",
                    [
                        trieleaf!(Abstract, "bstract"),
                        trieleaf!(Address, "ddress"),
                        trieleaf!(Anonymous, "nonymous"),
                        trieprefix!("s", [trieleaf!(Assembly, "sembly"), trieleaf!(As)])
                    ]
                ),
                trieprefix!("b", [trieleaf!(Bool, "ool"), trieleaf!(Break, "reak")]),
                trieprefix!(
                    "c",
                    [
                        trieprefix!(
                            "a",
                            [trieleaf!(Calldata, "lldata"), trieleaf!(Catch, "tch")]
                        ),
                        trieprefix!(
                            "on",
                            [
                                trieprefix!(
                                    "st",
                                    [trieleaf!(Constant, "ant"), trieleaf!(Constructor, "ructor")]
                                ),
                                trieprefix!(
                                    "t",
                                    [trieleaf!(Continue, "inue"), trieleaf!(Contract, "ract")]
                                )
                            ]
                        )
                    ]
                ),
                trieprefix!("d", [trieleaf!(Delete, "elete"), trieleaf!(Do, "o")]),
                trieprefix!(
                    "e",
                    [
                        trieleaf!(Else, "lse"),
                        trieleaf!(Emit, "mit"),
                        trieleaf!(Enum, "num"),
                        trieleaf!(Event, "vent"),
                        trieleaf!(External, "xternal")
                    ]
                ),
                trieprefix!(
                    "f",
                    [
                        trieprefix!("al", [trieleaf!(Fallback, "lback"), trieleaf!(False, "se")]),
                        trieleaf!(Fixed, "ixed"),
                        trieleaf!(For, "or"),
                        trieleaf!(Function, "unction")
                    ]
                ),
                trieleaf!(Hex, "hex"),
                trieprefix!(
                    "i",
                    [
                        trieleaf!(If, "f"),
                        trieprefix!(
                            "m",
                            [trieleaf!(Immutable, "mutable"), trieleaf!(Import, "port")]
                        ),
                        trieprefix!(
                            "n",
                            [
                                trieleaf!(Indexed, "dexed"),
                                trieprefix!(
                                    "ter",
                                    [trieleaf!(Interface, "face"), trieleaf!(Internal, "nal")]
                                )
                            ]
                        ),
                        trieleaf!(Is, "s")
                    ]
                ),
                trieleaf!(Library, "library"),
                trieprefix!(
                    "m",
                    [
                        trieleaf!(Mapping, "apping"),
                        trieleaf!(Memory, "emory"),
                        trieleaf!(Modifier, "odifier")
                    ]
                ),
                trieleaf!(New, "new"),
                trieleaf!(Override, "override"),
                trieprefix!(
                    "p",
                    [
                        trieleaf!(Payable, "ayable"),
                        trieprefix!(
                            "r",
                            [trieleaf!(Pragma, "agma"), trieleaf!(Private, "ivate")]
                        ),
                        trieprefix!("u", [trieleaf!(Public, "blic"), trieleaf!(Pure, "re")])
                    ]
                ),
                trieprefix!(
                    "re",
                    [
                        trieleaf!(Receive, "ceive"),
                        trieprefix!("turn", [trieleaf!(Returns, "s"), trieleaf!(Return)])
                    ]
                ),
                trieprefix!(
                    "st",
                    [
                        trieleaf!(Storage, "orage"),
                        trieprefix!("r", [trieleaf!(String, "ing"), trieleaf!(Struct, "uct")])
                    ]
                ),
                trieprefix!(
                    "t",
                    [
                        trieprefix!("r", [trieleaf!(True, "ue"), trieleaf!(Try, "y")]),
                        trieleaf!(Type, "ype")
                    ]
                ),
                trieprefix!(
                    "u",
                    [
                        trieleaf!(Ufixed, "fixed"),
                        trieleaf!(Unchecked, "nchecked"),
                        trieleaf!(Using, "sing")
                    ]
                ),
                trieprefix!("vi", [trieleaf!(View, "ew"), trieleaf!(Virtual, "rtual")]),
                trieleaf!(While, "while")
            )
        )
    );

    // LeadingTrivia = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
    define_rule!(
        LeadingTrivia,
        zero_or_more!(
            LeadingTrivia,
            choice!(
                trivia_token!(Whitespace),
                trivia_token!(EndOfLine),
                trivia_token!(MultilineComment),
                trivia_token!(SingleLineComment)
            )
        )
    );

    // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
    define_rule!(
        LibraryDefinition,
        seq!(
            LibraryDefinition,
            terminal!(Library, "library"),
            token!(Identifier),
            delimited_by!(
                DelimitedContractBodyElements,
                terminal!(OpenBrace, "{"),
                zero_or_more!(ContractBodyElements, rule!(ContractBodyElement)),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
    define_rule!(
        MappingType,
        seq!(
            MappingType,
            terminal!(Mapping, "mapping"),
            delimited_by!(
                terminal!(OpenParen, "("),
                seq!(
                    choice!(rule!(ElementaryType), rule!(IdentifierPath)),
                    terminal!(EqualGreater, "=>"),
                    rule!(TypeName)
                ),
                terminal!(CloseParen, ")")
            )
        )
    );

    // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
    define_rule!(
        MemberAccessExpression,
        unary_suffix_expression!(
            MemberAccessExpression,
            IndexAccessExpression,
            seq!(
                terminal!(Period, "."),
                choice!(token!(Identifier), terminal!(Address, "address"))
            )
        )
    );

    // ModifierAttribute = OverrideSpecifier | 'virtual' ;
    define_rule!(
        ModifierAttribute,
        choice!(
            ModifierAttribute,
            rule!(OverrideSpecifier),
            terminal!(Virtual, "virtual")
        )
    );

    // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
    define_rule!(
        ModifierDefinition,
        seq!(
            ModifierDefinition,
            terminal!(Modifier, "modifier"),
            token!(Identifier),
            optional!(rule!(ParameterList)),
            zero_or_more!(ModifierAttributes, rule!(ModifierAttribute)),
            choice!(terminal!(Semicolon, ";"), rule!(Block))
        )
    );

    // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
    define_rule!(
        ModifierInvocation,
        seq!(
            ModifierInvocation,
            rule!(IdentifierPath),
            optional!(rule!(ArgumentList))
        )
    );

    // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
    define_rule!(
        MulDivModExpression,
        left_associative_binary_expression!(
            MulDivModExpression,
            ExponentiationExpression,
            trie!(
                trieleaf!(Percent, "%"),
                trieleaf!(Star, "*"),
                trieleaf!(Slash, "/")
            )
        )
    );

    // «MultilineComment» = '/*' { ¬'*' | '*' ¬'/' } '*/' ;
    define_token!(
        MultilineComment,
        lex_seq!(
            MultilineComment,
            lex_terminal!(SlashStar, "/*"),
            lex_zero_or_more!(
                Content,
                lex_choice!(
                    lex_terminal!(NotStar, |&c: &char| c != '*'),
                    lex_seq!(
                        lex_terminal!(Star, '*'),
                        lex_terminal!(NotSlash, |&c: &char| c != '/')
                    )
                )
            ),
            lex_terminal!(StarSlash, "*/")
        )
    );

    // NamedArgument = «Identifier» ':' Expression ;
    define_rule!(
        NamedArgument,
        seq!(
            NamedArgument,
            token!(Identifier),
            terminal!(Colon, ":"),
            rule!(Expression)
        )
    );

    // NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
    define_rule!(
        NamedArgumentList,
        delimited_by!(
            NamedArgumentList,
            terminal!(OpenBrace, "{"),
            optional!(separated_by!(
                SeparatedNamedArguments,
                rule!(NamedArgument),
                terminal!(Comma, ",")
            )),
            terminal!(CloseBrace, "}")
        )
    );

    // NewExpression = 'new' IdentifierPath ArgumentList ;
    define_rule!(
        NewExpression,
        seq!(
            NewExpression,
            terminal!(New, "new"),
            rule!(IdentifierPath),
            rule!(ArgumentList)
        )
    );

    // «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
    define_token!(
        NumberUnit,
        lex_trie!(
            NumberUnit,
            trieleaf!(Days, "days"),
            trieleaf!(Ether, "ether"),
            trieleaf!(Finney, "finney"),
            trieleaf!(Gwei, "gwei"),
            trieleaf!(Hours, "hours"),
            trieleaf!(Minutes, "minutes"),
            trieprefix!(
                "s",
                [trieleaf!(Seconds, "econds"), trieleaf!(Szabo, "zabo")]
            ),
            trieprefix!("we", [trieleaf!(Weeks, "eks"), trieleaf!(Wei, "i")]),
            trieleaf!(Years, "years")
        )
    );

    // NumericLiteral = ( «DecimalNumber» | «HexNumber» ) [ «NumberUnit» ] ;
    define_rule!(
        NumericLiteral,
        seq!(
            NumericLiteral,
            choice!(token!(DecimalNumber), token!(HexNumber)),
            optional!(token!(NumberUnit))
        )
    );

    // OrExpression = Expression '||' Expression ;
    define_rule!(
        OrExpression,
        left_associative_binary_expression!(OrExpression, AndExpression, terminal!(PipePipe, "||"))
    );

    // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
    define_rule!(
        OrderComparisonExpression,
        left_associative_binary_expression!(
            OrderComparisonExpression,
            BitOrExpression,
            trie!(
                trieprefix!("<", [trieleaf!(LessEqual, "="), trieleaf!(Less)]),
                trieprefix!(">", [trieleaf!(GreaterEqual, "="), trieleaf!(Greater)])
            )
        )
    );

    // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
    define_rule!(
        OverrideSpecifier,
        seq!(
            OverrideSpecifier,
            terminal!(Override, "override"),
            optional!(delimited_by!(
                DelimitedSeparatedIdentifierPaths,
                terminal!(OpenParen, "("),
                separated_by!(
                    SeparatedIdentifierPaths,
                    rule!(IdentifierPath),
                    terminal!(Comma, ",")
                ),
                terminal!(CloseParen, ")")
            ))
        )
    );

    // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
    define_rule!(
        ParameterDeclaration,
        seq!(
            ParameterDeclaration,
            rule!(TypeName),
            optional!(rule!(DataLocation)),
            optional!(token!(Identifier))
        )
    );

    // ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
    define_rule!(
        ParameterList,
        delimited_by!(
            ParameterList,
            terminal!(OpenParen, "("),
            optional!(separated_by!(
                SeparatedParameterDeclarations,
                rule!(ParameterDeclaration),
                terminal!(Comma, ",")
            )),
            terminal!(CloseParen, ")")
        )
    );

    // ParenthesisExpression = '(' [ Expression ]  { ',' [ Expression ] } ')' ;
    define_rule!(
        ParenthesisExpression,
        delimited_by!(
            ParenthesisExpression,
            terminal!(OpenParen, "("),
            separated_by!(
                SeparatedExpressions,
                optional!(rule!(Expression)),
                terminal!(Comma, ",")
            ),
            terminal!(CloseParen, ")")
        )
    );

    // PayableExpression = 'payable' ArgumentList ;
    define_rule!(
        PayableExpression,
        seq!(
            PayableExpression,
            terminal!(Payable, "payable"),
            rule!(ArgumentList)
        )
    );

    // PositionalArgumentList = Expression  { ',' Expression } ;
    define_rule!(
        PositionalArgumentList,
        separated_by!(
            PositionalArgumentList,
            rule!(Expression),
            terminal!(Comma, ",")
        )
    );

    // «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
    define_token!(
        PossiblySeparatedPairsOfHexDigits,
        lex_seq!(
            PossiblySeparatedPairsOfHexDigits,
            lex_repeated!(
                lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                    || ('a' <= c && c <= 'f')
                    || ('A' <= c && c <= 'F')),
                2usize,
                2usize
            ),
            lex_zero_or_more!(lex_seq!(
                lex_optional!(lex_terminal!(Underscore, '_')),
                lex_repeated!(
                    lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                        || ('a' <= c && c <= 'f')
                        || ('A' <= c && c <= 'F')),
                    2usize,
                    2usize
                )
            ))
        )
    );

    // PragmaDirective = 'pragma' ( VersionPragma | ABICoderPragma | ExperimentalPragma ) ';' ;
    define_rule!(
        PragmaDirective,
        seq!(
            PragmaDirective,
            terminal!(Pragma, "pragma"),
            choice!(
                rule!(VersionPragma),
                rule!(ABICoderPragma),
                rule!(ExperimentalPragma)
            ),
            terminal!(Semicolon, ";")
        )
    );

    // PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | NumericLiteral | «BooleanLiteral» | «Identifier» ;
    define_rule!(
        PrimaryExpression,
        choice!(
            PrimaryExpression,
            rule!(PayableExpression),
            rule!(TypeExpression),
            rule!(NewExpression),
            rule!(ParenthesisExpression),
            rule!(ArrayLiteral),
            token!(AsciiStringLiteral),
            token!(UnicodeStringLiteral),
            token!(HexStringLiteral),
            rule!(NumericLiteral),
            token!(BooleanLiteral),
            token!(Identifier)
        )
    );

    // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
    define_token!(
        RawIdentifier,
        lex_seq!(
            RawIdentifier,
            lex_terminal!(|&c: &char| c == '_'
                || c == '$'
                || ('a' <= c && c <= 'z')
                || ('A' <= c && c <= 'Z')),
            lex_zero_or_more!(lex_terminal!(|&c: &char| c == '_'
                || c == '$'
                || ('a' <= c && c <= 'z')
                || ('A' <= c && c <= 'Z')
                || ('0' <= c && c <= '9')))
        )
    );

    // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
    define_rule!(
        ReceiveFunctionAttribute,
        choice!(
            ReceiveFunctionAttribute,
            rule!(ModifierInvocation),
            rule!(OverrideSpecifier),
            trie!(
                trieleaf!(External, "external"),
                trieleaf!(Payable, "payable"),
                trieleaf!(Virtual, "virtual")
            )
        )
    );

    // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
    define_rule!(
        ReceiveFunctionDefinition,
        seq!(
            ReceiveFunctionDefinition,
            terminal!(Receive, "receive"),
            rule!(ParameterList),
            zero_or_more!(ReceiveFunctionAttributes, rule!(ReceiveFunctionAttribute)),
            choice!(terminal!(Semicolon, ";"), rule!(Block))
        )
    );

    // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
    define_token!(
        ReservedKeyword,
        lex_trie!(
            ReservedKeyword,
            trieprefix!(
                "a",
                [
                    trieleaf!(After, "fter"),
                    trieleaf!(Alias, "lias"),
                    trieleaf!(Apply, "pply"),
                    trieleaf!(Auto, "uto")
                ]
            ),
            trieleaf!(Byte, "byte"),
            trieprefix!("c", [trieleaf!(Case, "ase"), trieleaf!(Copyof, "opyof")]),
            trieprefix!(
                "def",
                [trieleaf!(Default, "ault"), trieleaf!(Define, "ine")]
            ),
            trieleaf!(Final, "final"),
            trieprefix!(
                "i",
                [
                    trieleaf!(Implements, "mplements"),
                    trieprefix!("n", [trieleaf!(Inline, "line"), trieleaf!(In)])
                ]
            ),
            trieleaf!(Let, "let"),
            trieprefix!(
                "m",
                [
                    trieprefix!("a", [trieleaf!(Macro, "cro"), trieleaf!(Match, "tch")]),
                    trieleaf!(Mutable, "utable")
                ]
            ),
            trieleaf!(Null, "null"),
            trieleaf!(Of, "of"),
            trieprefix!(
                "p",
                [trieleaf!(Partial, "artial"), trieleaf!(Promise, "romise")]
            ),
            trieprefix!(
                "re",
                [
                    trieleaf!(Reference, "ference"),
                    trieleaf!(Relocatable, "locatable")
                ]
            ),
            trieprefix!(
                "s",
                [
                    trieleaf!(Sealed, "ealed"),
                    trieleaf!(Sizeof, "izeof"),
                    trieleaf!(Static, "tatic"),
                    trieleaf!(Supports, "upports"),
                    trieleaf!(Switch, "witch")
                ]
            ),
            trieprefix!("type", [trieleaf!(Typedef, "def"), trieleaf!(Typeof, "of")]),
            trieleaf!(Var, "var")
        )
    );

    // ReturnStatement = 'return' [ Expression ] ';' ;
    define_rule!(
        ReturnStatement,
        seq!(
            ReturnStatement,
            terminal!(Return, "return"),
            optional!(rule!(Expression)),
            terminal!(Semicolon, ";")
        )
    );

    // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
    define_rule!(
        RevertStatement,
        seq!(
            RevertStatement,
            terminal!(Revert, "revert"),
            optional!(rule!(IdentifierPath)),
            rule!(ArgumentList),
            terminal!(Semicolon, ";")
        )
    );

    // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
    define_rule!(
        SelectedImport,
        seq!(
            SelectedImport,
            token!(Identifier),
            optional!(seq!(terminal!(As, "as"), token!(Identifier)))
        )
    );

    // SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
    define_rule!(
        SelectingImportDirective,
        seq!(
            SelectingImportDirective,
            delimited_by!(
                DelimitedSeparatedSelectedImports,
                terminal!(OpenBrace, "{"),
                separated_by!(
                    SeparatedSelectedImports,
                    rule!(SelectedImport),
                    terminal!(Comma, ",")
                ),
                terminal!(CloseBrace, "}")
            ),
            terminal!(From, "from"),
            rule!(ImportPath)
        )
    );

    // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
    define_rule!(
        ShiftExpression,
        left_associative_binary_expression!(
            ShiftExpression,
            AddSubExpression,
            trie!(
                trieleaf!(LessLess, "<<"),
                trieprefix!(
                    ">>",
                    [
                        trieleaf!(GreaterGreaterGreater, ">"),
                        trieleaf!(GreaterGreater)
                    ]
                )
            )
        )
    );

    // «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
    define_token!(
        SignedFixedType,
        lex_seq!(
            SignedFixedType,
            lex_terminal!(Fixed, "fixed"),
            lex_optional!(lex_seq!(
                lex_one_or_more!(lex_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
                lex_terminal!(LatinSmallLetterX, 'x'),
                lex_one_or_more!(lex_terminal!(|&c: &char| ('0' <= c && c <= '9')))
            ))
        )
    );

    // «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
    define_token!(
        SignedIntegerType,
        lex_seq!(
            SignedIntegerType,
            lex_terminal!(Int, "int"),
            lex_optional!(lex_trie!(
                ByteCount,
                trieprefix!(
                    "1",
                    [
                        trieleaf!(OneZeroFour, "04"),
                        trieleaf!(OneOneTwo, "12"),
                        trieprefix!(
                            "2",
                            [trieleaf!(OneTwoZero, "0"), trieleaf!(OneTwoEight, "8")]
                        ),
                        trieleaf!(OneThreeSix, "36"),
                        trieleaf!(OneFourFour, "44"),
                        trieleaf!(OneFiveTwo, "52"),
                        trieprefix!(
                            "6",
                            [
                                trieleaf!(OneSixZero, "0"),
                                trieleaf!(OneSixEight, "8"),
                                trieleaf!(OneSix)
                            ]
                        ),
                        trieleaf!(OneSevenSix, "76"),
                        trieleaf!(OneEightFour, "84"),
                        trieleaf!(OneNineTwo, "92")
                    ]
                ),
                trieprefix!(
                    "2",
                    [
                        trieprefix!(
                            "0",
                            [trieleaf!(TwoZeroZero, "0"), trieleaf!(TwoZeroEight, "8")]
                        ),
                        trieleaf!(TwoOneSix, "16"),
                        trieleaf!(TwoTwoFour, "24"),
                        trieleaf!(TwoThreeTwo, "32"),
                        trieprefix!(
                            "4",
                            [
                                trieleaf!(TwoFourZero, "0"),
                                trieleaf!(TwoFourEight, "8"),
                                trieleaf!(TwoFour)
                            ]
                        ),
                        trieleaf!(TwoFiveSix, "56")
                    ]
                ),
                trieleaf!(ThreeTwo, "32"),
                trieprefix!("4", [trieleaf!(FourZero, "0"), trieleaf!(FourEight, "8")]),
                trieleaf!(FiveSix, "56"),
                trieleaf!(SixFour, "64"),
                trieleaf!(SevenTwo, "72"),
                trieprefix!(
                    "8",
                    [
                        trieleaf!(EightZero, "0"),
                        trieleaf!(EightEight, "8"),
                        trieleaf!(Eight)
                    ]
                ),
                trieleaf!(NineSix, "96")
            ))
        )
    );

    // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
    define_rule!(
        SimpleImportDirective,
        seq!(
            SimpleImportDirective,
            rule!(ImportPath),
            zero_or_more!(seq!(terminal!(As, "as"), token!(Identifier)))
        )
    );

    // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
    define_rule!(
        SimpleStatement,
        choice!(
            SimpleStatement,
            rule!(TupleDeconstructionStatement),
            rule!(VariableDeclarationStatement),
            rule!(ExpressionStatement)
        )
    );

    // «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
    define_token!(
        SingleLineComment,
        lex_seq!(
            SingleLineComment,
            lex_terminal!(SlashSlash, "//"),
            lex_zero_or_more!(lex_terminal!(|&c: &char| c != '\r' && c != '\n'))
        )
    );

    // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
    define_token!(
        SingleQuotedAsciiStringLiteral,
        lex_seq!(
            SingleQuotedAsciiStringLiteral,
            lex_terminal!(Quote, "'"),
            lex_zero_or_more!(
                Runs,
                lex_choice!(
                    Run,
                    lex_one_or_more!(
                        Chars,
                        lex_terminal!(Char, |&c: &char| (' ' <= c && c <= '~')
                            && c != '\''
                            && c != '\\')
                    ),
                    lex_rule!(EscapeSequence)
                )
            ),
            lex_terminal!(Quote, "'")
        )
    );

    // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
    define_token!(
        SingleQuotedUnicodeStringLiteral,
        lex_seq!(
            SingleQuotedUnicodeStringLiteral,
            lex_terminal!(UnicodeQuote, "unicode'"),
            lex_zero_or_more!(
                Runs,
                lex_choice!(
                    Run,
                    lex_one_or_more!(
                        Chars,
                        lex_terminal!(Char, |&c: &char| c != '\''
                            && c != '\\'
                            && c != '\n'
                            && c != '\r')
                    ),
                    lex_rule!(EscapeSequence)
                )
            ),
            lex_terminal!(Quote, "'")
        )
    );

    // SourceUnit = LeadingTrivia { Directive | Definition } EndOfFileTrivia ;
    define_rule!(
        SourceUnit,
        seq!(
            SourceUnit,
            rule!(LeadingTrivia),
            zero_or_more!(choice!(rule!(Directive), rule!(Definition))),
            rule!(EndOfFileTrivia)
        )
    );

    // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
    define_rule!(
        StarImportDirective,
        seq!(
            StarImportDirective,
            terminal!(Star, "*"),
            terminal!(As, "as"),
            token!(Identifier),
            terminal!(From, "from"),
            rule!(ImportPath)
        )
    );

    // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
    define_rule!(
        StateVariableAttribute,
        choice!(
            StateVariableAttribute,
            rule!(OverrideSpecifier),
            trie!(
                trieleaf!(Constant, "constant"),
                trieprefix!(
                    "i",
                    [
                        trieleaf!(Immutable, "mmutable"),
                        trieleaf!(Internal, "nternal")
                    ]
                ),
                trieprefix!(
                    "p",
                    [trieleaf!(Private, "rivate"), trieleaf!(Public, "ublic")]
                )
            )
        )
    );

    // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
    define_rule!(
        StateVariableDeclaration,
        seq!(
            StateVariableDeclaration,
            rule!(TypeName),
            zero_or_more!(StateVariableAttributes, rule!(StateVariableAttribute)),
            token!(Identifier),
            optional!(seq!(terminal!(Equal, "="), rule!(Expression))),
            terminal!(Semicolon, ";")
        )
    );

    // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
    define_rule!(
        Statement,
        choice!(
            Statement,
            rule!(Block),
            rule!(SimpleStatement),
            rule!(IfStatement),
            rule!(ForStatement),
            rule!(WhileStatement),
            rule!(DoWhileStatement),
            rule!(ContinueStatement),
            rule!(BreakStatement),
            rule!(TryStatement),
            rule!(ReturnStatement),
            rule!(EmitStatement),
            rule!(RevertStatement),
            rule!(DeleteStatement),
            rule!(AssemblyStatement)
        )
    );

    // StructDefinition = 'struct' «Identifier» '{' 1…*{ StructMember } '}' ;
    define_rule!(
        StructDefinition,
        seq!(
            StructDefinition,
            terminal!(Struct, "struct"),
            token!(Identifier),
            delimited_by!(
                DelimitedStructMembers,
                terminal!(OpenBrace, "{"),
                one_or_more!(StructMembers, rule!(StructMember)),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // StructMember = TypeName «Identifier» ';' ;
    define_rule!(
        StructMember,
        seq!(
            StructMember,
            rule!(TypeName),
            token!(Identifier),
            terminal!(Semicolon, ";")
        )
    );

    // TrailingTrivia = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
    define_rule!(
        TrailingTrivia,
        optional!(seq!(
            TrailingTrivia,
            zero_or_more!(choice!(
                trivia_token!(Whitespace),
                trivia_token!(MultilineComment)
            )),
            choice!(trivia_token!(EndOfLine), trivia_token!(SingleLineComment))
        ))
    );

    // TryStatement = 'try' Expression [ 'returns' ParameterList ] Block 1…*{ CatchClause } ;
    define_rule!(
        TryStatement,
        seq!(
            TryStatement,
            terminal!(Try, "try"),
            rule!(Expression),
            optional!(seq!(terminal!(Returns, "returns"), rule!(ParameterList))),
            rule!(Block),
            one_or_more!(CatchClauses, rule!(CatchClause))
        )
    );

    // TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
    define_rule!(
        TupleDeconstructionStatement,
        seq!(
            TupleDeconstructionStatement,
            delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(
                    optional!(seq!(optional!(rule!(TypeName)), token!(Identifier))),
                    terminal!(Comma, ",")
                )),
                terminal!(CloseParen, ")")
            ),
            terminal!(Equal, "="),
            rule!(Expression),
            terminal!(Semicolon, ";")
        )
    );

    // TypeExpression = 'type' '(' TypeName ')' ;
    define_rule!(
        TypeExpression,
        seq!(
            TypeExpression,
            terminal!(Type, "type"),
            delimited_by!(
                DelimitedTypeName,
                terminal!(OpenParen, "("),
                rule!(TypeName),
                terminal!(CloseParen, ")")
            )
        )
    );

    // TypeName = ( ElementaryType | FunctionType | MappingType | IdentifierPath ) { '[' [ Expression ] ']' } ;
    define_rule!(
        TypeName,
        seq!(
            TypeName,
            choice!(
                rule!(ElementaryType),
                rule!(FunctionType),
                rule!(MappingType),
                rule!(IdentifierPath)
            ),
            zero_or_more!(
                DelimitedExpressions,
                delimited_by!(
                    DelimitedExpression,
                    terminal!(OpenBracket, "["),
                    optional!(rule!(Expression)),
                    terminal!(CloseBracket, "]")
                )
            )
        )
    );

    // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
    define_rule!(
        UnaryPrefixExpression,
        unary_prefix_expression!(
            UnaryPrefixExpression,
            FunctionCallExpression,
            trie!(
                trieleaf!(Bang, "!"),
                trieleaf!(PlusPlus, "++"),
                trieprefix!("-", [trieleaf!(MinusMinus, "-"), trieleaf!(Minus)]),
                trieleaf!(Tilde, "~")
            )
        )
    );

    // UnarySuffixExpression = Expression ( '++' | '--' ) ;
    define_rule!(
        UnarySuffixExpression,
        unary_suffix_expression!(
            UnarySuffixExpression,
            UnaryPrefixExpression,
            trie!(trieleaf!(PlusPlus, "++"), trieleaf!(MinusMinus, "--"))
        )
    );

    // UncheckedBlock = 'unchecked' Block ;
    define_rule!(
        UncheckedBlock,
        seq!(
            UncheckedBlock,
            terminal!(Unchecked, "unchecked"),
            rule!(Block)
        )
    );

    // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
    define_token!(
        UnicodeEscape,
        lex_seq!(
            UnicodeEscape,
            lex_terminal!(LatinSmallLetterU, 'u'),
            lex_repeated!(
                lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                    || ('a' <= c && c <= 'f')
                    || ('A' <= c && c <= 'F')),
                4usize,
                4usize
            )
        )
    );

    // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
    define_token!(
        UnicodeStringLiteral,
        lex_choice!(
            UnicodeStringLiteral,
            lex_rule!(SingleQuotedUnicodeStringLiteral),
            lex_rule!(DoubleQuotedUnicodeStringLiteral)
        )
    );

    // «UnsignedFixedType» = 'u' «SignedFixedType» ;
    define_token!(
        UnsignedFixedType,
        lex_seq!(
            UnsignedFixedType,
            lex_terminal!(LatinSmallLetterU, 'u'),
            lex_rule!(SignedFixedType)
        )
    );

    // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
    define_token!(
        UnsignedIntegerType,
        lex_seq!(
            UnsignedIntegerType,
            lex_terminal!(LatinSmallLetterU, 'u'),
            lex_rule!(SignedIntegerType)
        )
    );

    // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
    define_rule!(
        UserDefinedValueTypeDefinition,
        seq!(
            UserDefinedValueTypeDefinition,
            terminal!(Type, "type"),
            token!(Identifier),
            terminal!(Is, "is"),
            rule!(ElementaryType),
            terminal!(Semicolon, ";")
        )
    );

    // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
    define_rule!(
        UsingDirective,
        seq!(
            UsingDirective,
            terminal!(Using, "using"),
            choice!(
                rule!(IdentifierPath),
                delimited_by!(
                    DelimitedSeparatedIdentifierPaths,
                    terminal!(OpenBrace, "{"),
                    separated_by!(
                        SeparatedIdentifierPaths,
                        rule!(IdentifierPath),
                        terminal!(Comma, ",")
                    ),
                    terminal!(CloseBrace, "}")
                )
            ),
            terminal!(For, "for"),
            choice!(terminal!(Star, "*"), rule!(TypeName)),
            optional!(terminal!(Global, "global")),
            terminal!(Semicolon, ";")
        )
    );

    // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
    define_rule!(
        VariableDeclarationStatement,
        seq!(
            VariableDeclarationStatement,
            rule!(TypeName),
            optional!(rule!(DataLocation)),
            token!(Identifier),
            optional!(seq!(terminal!(Equal, "="), rule!(Expression))),
            terminal!(Semicolon, ";")
        )
    );

    // VersionPragma = 'solidity' 1…*{ VersionPragmaSpecifier } ;
    define_rule!(
        VersionPragma,
        seq!(
            VersionPragma,
            terminal!(Solidity, "solidity"),
            one_or_more!(VersionPragmaSpecifiers, rule!(VersionPragmaSpecifier))
        )
    );

    // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
    define_token!(
        VersionPragmaOperator,
        lex_trie!(
            VersionPragmaOperator,
            trieprefix!("<", [trieleaf!(LessEqual, "="), trieleaf!(Less)]),
            trieleaf!(Equal, "="),
            trieprefix!(">", [trieleaf!(GreaterEqual, "="), trieleaf!(Greater)]),
            trieleaf!(Caret, "^"),
            trieleaf!(Tilde, "~")
        )
    );

    // VersionPragmaSpecifier = [ «VersionPragmaOperator» ] «VersionPragmaValue»  { '.' «VersionPragmaValue» } ;
    define_rule!(
        VersionPragmaSpecifier,
        seq!(
            VersionPragmaSpecifier,
            optional!(token!(VersionPragmaOperator)),
            separated_by!(
                SeparatedVersionPragmaValues,
                token!(VersionPragmaValue),
                terminal!(Period, ".")
            )
        )
    );

    // «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' } ;
    define_token!(
        VersionPragmaValue,
        lex_one_or_more!(
            VersionPragmaValue,
            lex_terminal!(|&c: &char| ('0' <= c && c <= '9') || c == 'x' || c == 'X' || c == '*')
        )
    );

    // WhileStatement = 'while' '(' Expression ')' Statement ;
    define_rule!(
        WhileStatement,
        seq!(
            WhileStatement,
            terminal!(While, "while"),
            delimited_by!(
                DelimitedExpression,
                terminal!(OpenParen, "("),
                rule!(Expression),
                terminal!(CloseParen, ")")
            ),
            rule!(Statement)
        )
    );

    // «Whitespace» = 1…*{ '\u{20}' | '\u{9}' } ;
    define_token!(
        Whitespace,
        lex_one_or_more!(Whitespace, lex_terminal!(|&c: &char| c == ' ' || c == '\t'))
    );

    // YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
    define_rule!(
        YulAssignmentStatement,
        seq!(
            YulAssignmentStatement,
            separated_by!(
                SeparatedYulIdentifierPaths,
                rule!(YulIdentifierPath),
                terminal!(Comma, ",")
            ),
            terminal!(ColonEqual, ":="),
            rule!(YulExpression)
        )
    );

    // YulBlock = '{' { YulStatement } '}' ;
    define_rule!(
        YulBlock,
        delimited_by!(
            YulBlock,
            terminal!(OpenBrace, "{"),
            zero_or_more!(YulStatements, rule!(YulStatement)),
            terminal!(CloseBrace, "}")
        )
    );

    // YulBreakStatement = 'break' ;
    define_rule!(YulBreakStatement, terminal!(Break, "break"));

    // YulContinueStatement = 'continue' ;
    define_rule!(YulContinueStatement, terminal!(Continue, "continue"));

    // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
    define_token!(
        YulDecimalNumberLiteral,
        lex_choice!(
            YulDecimalNumberLiteral,
            lex_terminal!(Zero, "0"),
            lex_seq!(
                lex_terminal!(|&c: &char| ('1' <= c && c <= '9')),
                lex_zero_or_more!(lex_terminal!(|&c: &char| ('0' <= c && c <= '9')))
            )
        )
    );

    // YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
    define_rule!(
        YulExpression,
        choice!(
            YulExpression,
            rule!(YulIdentifierPath),
            rule!(YulFunctionCall),
            rule!(YulLiteral)
        )
    );

    // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
    define_rule!(
        YulForStatement,
        seq!(
            YulForStatement,
            terminal!(For, "for"),
            rule!(YulBlock),
            rule!(YulExpression),
            rule!(YulBlock),
            rule!(YulBlock)
        )
    );

    // YulFunctionCall = «YulIdentifier» '(' [ YulExpression  { ',' YulExpression } ] ')' ;
    define_rule!(
        YulFunctionCall,
        seq!(
            YulFunctionCall,
            token!(YulIdentifier),
            delimited_by!(
                DelimitedSeparatedYulExpressions,
                terminal!(OpenParen, "("),
                optional!(separated_by!(
                    SeparatedYulExpressions,
                    rule!(YulExpression),
                    terminal!(Comma, ",")
                )),
                terminal!(CloseParen, ")")
            )
        )
    );

    // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
    define_rule!(
        YulFunctionDefinition,
        seq!(
            YulFunctionDefinition,
            terminal!(Function, "function"),
            token!(YulIdentifier),
            delimited_by!(
                DelimitedArguments,
                terminal!(OpenParen, "("),
                optional!(separated_by!(
                    Arguments,
                    token!(YulIdentifier),
                    terminal!(Comma, ",")
                )),
                terminal!(CloseParen, ")")
            ),
            optional!(seq!(
                terminal!(MinusGreater, "->"),
                separated_by!(Results, token!(YulIdentifier), terminal!(Comma, ","))
            )),
            rule!(YulBlock)
        )
    );

    // «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
    define_token!(
        YulHexLiteral,
        lex_seq!(
            YulHexLiteral,
            lex_terminal!(ZeroX, "0x"),
            lex_one_or_more!(lex_terminal!(|&c: &char| ('0' <= c && c <= '9')
                || ('a' <= c && c <= 'f')
                || ('A' <= c && c <= 'F')))
        )
    );

    // «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
    define_token!(
        YulIdentifier,
        difference(
            lex_rule!(RawIdentifier),
            lex_trie!(
                trieleaf!(Break, "break"),
                trieprefix!(
                    "c",
                    [trieleaf!(Case, "ase"), trieleaf!(Continue, "ontinue")]
                ),
                trieleaf!(Default, "default"),
                trieprefix!(
                    "f",
                    [
                        trieleaf!(False, "alse"),
                        trieleaf!(For, "or"),
                        trieleaf!(Function, "unction")
                    ]
                ),
                trieleaf!(Hex, "hex"),
                trieleaf!(If, "if"),
                trieprefix!("le", [trieleaf!(Leave, "ave"), trieleaf!(Let, "t")]),
                trieleaf!(Switch, "switch"),
                trieleaf!(True, "true")
            )
        )
    );

    // YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
    define_rule!(
        YulIdentifierPath,
        separated_by!(
            YulIdentifierPath,
            token!(YulIdentifier),
            terminal!(Period, ".")
        )
    );

    // YulIfStatement = 'if' YulExpression YulBlock ;
    define_rule!(
        YulIfStatement,
        seq!(
            YulIfStatement,
            terminal!(If, "if"),
            rule!(YulExpression),
            rule!(YulBlock)
        )
    );

    // «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
    define_token!(
        YulKeyword,
        lex_trie!(
            YulKeyword,
            trieleaf!(Break, "break"),
            trieprefix!(
                "c",
                [trieleaf!(Case, "ase"), trieleaf!(Continue, "ontinue")]
            ),
            trieleaf!(Default, "default"),
            trieprefix!(
                "f",
                [
                    trieleaf!(False, "alse"),
                    trieleaf!(For, "or"),
                    trieleaf!(Function, "unction")
                ]
            ),
            trieleaf!(Hex, "hex"),
            trieleaf!(If, "if"),
            trieprefix!("le", [trieleaf!(Leave, "ave"), trieleaf!(Let, "t")]),
            trieleaf!(Switch, "switch"),
            trieleaf!(True, "true")
        )
    );

    // YulLeaveStatement = 'leave' ;
    define_rule!(YulLeaveStatement, terminal!(Leave, "leave"));

    // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
    define_rule!(
        YulLiteral,
        choice!(
            YulLiteral,
            token!(YulDecimalNumberLiteral),
            token!(YulHexLiteral),
            token!(AsciiStringLiteral),
            token!(BooleanLiteral),
            token!(HexStringLiteral)
        )
    );

    // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
    define_rule!(
        YulStatement,
        choice!(
            YulStatement,
            rule!(YulBlock),
            rule!(YulVariableDeclaration),
            rule!(YulFunctionDefinition),
            rule!(YulAssignmentStatement),
            rule!(YulFunctionCall),
            rule!(YulIfStatement),
            rule!(YulForStatement),
            rule!(YulSwitchStatement),
            rule!(YulLeaveStatement),
            rule!(YulBreakStatement),
            rule!(YulContinueStatement)
        )
    );

    // YulSwitchStatement = 'switch' YulExpression 1…*{ ( 'case' YulLiteral | 'default' ) YulBlock } ;
    define_rule!(
        YulSwitchStatement,
        seq!(
            YulSwitchStatement,
            terminal!(Switch, "switch"),
            rule!(YulExpression),
            one_or_more!(seq!(
                choice!(
                    seq!(terminal!(Case, "case"), rule!(YulLiteral)),
                    terminal!(Default, "default")
                ),
                rule!(YulBlock)
            ))
        )
    );

    // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
    define_rule!(
        YulVariableDeclaration,
        seq!(
            YulVariableDeclaration,
            terminal!(Let, "let"),
            separated_by!(
                SeparatedYulIdentifierPaths,
                rule!(YulIdentifierPath),
                terminal!(Comma, ",")
            ),
            optional!(seq!(terminal!(ColonEqual, ":="), rule!(YulExpression)))
        )
    );

    // Return the Parsers object ------------------------

    parsers
}
