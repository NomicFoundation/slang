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
pub type LocatedType = Located<char, ErrorType>;
type PResult<O> = (
    Vec<LocatedType>,
    Result<(O, Option<LocatedType>), LocatedType>,
);
type StreamOf<'a> = Stream<'a, char, SpanType>;
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
    declare_token!(DecimalLiteral);
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
    declare_token!(HexLiteral);
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
    declare_rule!(PayableType);
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
    declare_token!(YulDecimalLiteral);
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
    macro_rules! scan_terminal {
        ($ literal : literal) => {
            just($literal).ignored()
        };
        ($ filter : expr) => {
            filter($filter).ignored()
        };
    }
    #[allow(unused_macros)]
    macro_rules ! scan_choice { ($ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr) , *)) } ; }
    #[allow(unused_macros)]
    macro_rules ! scan_seq { ($ head : expr , $ ($ tail : expr) , +) => { $ head . ignored () . then_ignore (scan_seq ! ($ ($ tail) , +)) } ; ($ head : expr) => { $ head . ignored () } ; }
    #[allow(unused_macros)]
    macro_rules! scan_zero_or_more {
        ($ expr : expr) => {
            $expr.repeated().ignored()
        };
    }
    #[allow(unused_macros)]
    macro_rules! scan_one_or_more {
        ($ expr : expr) => {
            $expr.repeated().at_least(1).ignored()
        };
    }
    #[allow(unused_macros)]
    macro_rules! scan_repeated {
        ($ expr : expr , $ min : literal , $ max : literal) => {
            $expr.repeated().at_least($min).at_most($max).ignored()
        };
    }
    #[allow(unused_macros)]
    macro_rules! scan_optional {
        ($ expr : expr) => {
            $expr.or_not().ignored()
        };
    }
    #[allow(unused_macros)]
    macro_rules! scan_separated_by {
        ($ expr : expr , $ separator : expr) => {
            $expr.then_ignore($separator.then_ignore($expr).repeated())
        };
    }
    #[allow(unused_macros)]
    macro_rules! scan_make_node {
        ($ expr : expr) => {
            $expr.map_with_span(|_, span: SpanType| lex::Node::chars(span))
        };
    }
    #[allow(unused_macros)]
    macro_rules! lex_terminal {
        ($ kind : ident , $ literal : literal) => {
            just($literal).map_with_span(|_, span: SpanType| {
                lex::Node::named(TokenKind::$kind, lex::Node::chars(span))
            })
        };
        ($ kind : ident , $ filter : expr) => {
            filter($filter).map_with_span(|_, span: SpanType| {
                lex::Node::named(TokenKind::$kind, lex::Node::chars(span))
            })
        };
        ($ literal : literal) => {
            just($literal).map_with_span(|_, span: SpanType| lex::Node::chars(span))
        };
        ($ filter : expr) => {
            filter($filter).map_with_span(|_, span: SpanType| lex::Node::chars(span))
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
            $expr.repeated().map(lex::Node::sequence)
        };
    }
    #[allow(unused_macros)]
    macro_rules! lex_one_or_more {
        ($ kind : ident , $ expr : expr) => {
            lex_one_or_more!($expr).map(|element| lex::Node::named(TokenKind::$kind, element))
        };
        ($ expr : expr) => {
            $expr.repeated().at_least(1).map(lex::Node::sequence)
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
                .map(lex::Node::sequence)
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
    macro_rules! define_token {
        ($ kind : ident , $ expr : expr) => {
            $kind.define($expr.map(|node| lex::Node::named(TokenKind::$kind, node)));
            parsers.insert(
                ProductionKind::$kind,
                Parser::new(
                    $kind
                        .clone()
                        .map(cst::Node::top_level_token)
                        .then_ignore(end())
                        .boxed(),
                ),
            );
        };
    }
    #[allow(unused_macros)]
    macro_rules! with_trivia {
        ($ expr : expr) => {
            LeadingTrivia
                .clone()
                .then($expr)
                .then(TrailingTrivia.clone())
        };
    }
    #[allow(unused_macros)]
    macro_rules! trivia_terminal {
        ($ kind : ident , $ literal : literal) => {
            just($literal).map_with_span(|_, span: SpanType| {
                cst::Node::trivia_token(TokenKind::$kind, lex::Node::chars_unwrapped(span))
            })
        };
        ($ kind : ident , $ filter : expr) => {
            filter($filter).map_with_span(|_, span: SpanType| {
                cst::Node::trivia_token(TokenKind::$kind, lex::Node::chars_unwrapped(span))
            })
        };
    }
    #[allow(unused_macros)]
    macro_rules! terminal {
        ($ kind : ident , $ literal : literal) => {
            with_trivia!(just($literal).map_with_span(|_, span: SpanType| span)).map(
                |((leading_trivia, range), trailing_trivia)| {
                    cst::Node::token(
                        TokenKind::$kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                },
            )
        };
        ($ kind : ident , $ filter : expr) => {
            with_trivia!(filter($filter).map_with_span(|_, span: SpanType| span)).map(
                |((leading_trivia, range), trailing_trivia)| {
                    cst::Node::token(
                        TokenKind::$kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                },
            )
        };
    }
    #[allow(unused_macros)]
    macro_rules ! trivia_token { ($ token_rule : ident) => { rule ! ($ token_rule) . map (| token : Option < Rc < lex :: Node >> | { let token = token . unwrap () ; if let lex :: NodeContents :: Named (kind , element) = & token . contents { cst :: Node :: trivia_token (* kind , element . clone ()) } else { unreachable ! ("a token rule should always return a named token, but rule {} returned {:?}" , stringify ! ($ token_rule) , token) } }) } ; }
    #[allow(unused_macros)]
    macro_rules ! token { ($ token_rule : ident) => { with_trivia ! ($ token_rule . clone ()) . map (| ((leading_trivia , token) , trailing_trivia) : ((_ , Option < Rc < lex :: Node >>) , _) | { let token = token . unwrap () ; if let lex :: NodeContents :: Named (kind , element) = & token . contents { cst :: Node :: token (* kind , element . clone () , leading_trivia , trailing_trivia) } else { unreachable ! ("a token rule should always return a named token, but rule {} returned {:?}" , stringify ! ($ token_rule) , token) } }) } ; }
    #[allow(unused_macros)]
    macro_rules! rule {
        ($ rule : ident) => {
            $rule.clone()
        };
    }
    #[allow(unused_macros)]
    macro_rules ! choice { ($ kind : ident , $ ($ expr : expr) , *) => { choice :: < _ , ErrorType > (($ ($ expr) , *)) } ; ($ head : expr , $ ($ tail : expr) , +) => { choice :: < _ , ErrorType > (($ head , $ ($ tail) , +)) } ; ($ expr : expr) => { $ expr } ; }
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
    macro_rules! delimited_by {
        ($ kind : ident , $ open : expr , $ expr : expr , $ close : expr) => {
            seq!($kind, $open, $expr, $close)
        };
        ($ open : expr , $ expr : expr , $ close : expr) => {
            seq!($open, $expr, $close)
        };
    }
    #[allow(unused_macros)]
    macro_rules! define_rule {
        ($ kind : ident , $ expr : expr) => {
            $kind.define($expr.map(|node| cst::Node::rule(RuleKind::$kind, vec![node])));
            parsers.insert(
                ProductionKind::$kind,
                Parser::new(
                    $kind
                        .clone()
                        .map(|node| cst::Node::top_level_rule(RuleKind::$kind, node))
                        .then_ignore(end())
                        .boxed(),
                ),
            );
        };
    }

    // Define all productions ---------------------------

    // ABICoderPragma = 'abicoder' «Identifier» ;
    define_rule!(
        ABICoderPragma,
        seq!(
            with_trivia!(just("abicoder")
                .to(TokenKind::Abicoder)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier)
        )
    );

    // AddSubExpression = Expression ( '+' | '-' ) Expression ;
    define_rule!(
        AddSubExpression,
        seq!(
            rule!(Expression),
            with_trivia!(choice!(
                just("+").to(TokenKind::Plus),
                just("-").to(TokenKind::Minus)
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // AddressType = 'address' [ 'payable' ] ;
    define_rule!(
        AddressType,
        seq!(
            with_trivia!(just("address")
                .to(TokenKind::Address)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            optional!(with_trivia!(just("payable")
                .to(TokenKind::Payable)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }))
        )
    );

    // AndExpression = Expression '&&' Expression ;
    define_rule!(
        AndExpression,
        seq!(
            rule!(Expression),
            with_trivia!(just("&&")
                .to(TokenKind::AmpersandAmpersand)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // ArgumentList = '(' [ PositionalArgumentList | NamedArgumentList ] ')' ;
    define_rule!(
        ArgumentList,
        delimited_by!(
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
            terminal!(OpenBracket, "["),
            separated_by!(rule!(Expression), terminal!(Comma, ",")),
            terminal!(CloseBracket, "]")
        )
    );

    // «AsciiEscape» = 'n' | 'r' | 't' | '\'' | '"' | '\\' | '\u{a}' | '\u{d}' ;
    define_token!(
        AsciiEscape,
        scan_make_node!(scan_terminal!(|&c: &char| c == 'n'
            || c == 'r'
            || c == 't'
            || c == '\''
            || c == '"'
            || c == '\\'
            || c == '\n'
            || c == '\r'))
    );

    // «AsciiStringLiteral» = «SingleQuotedAsciiStringLiteral» | «DoubleQuotedAsciiStringLiteral» ;
    define_token!(
        AsciiStringLiteral,
        scan_make_node!(scan_choice!(
            scan_seq!(
                scan_terminal!("'"),
                scan_zero_or_more!(scan_choice!(
                    scan_one_or_more!(scan_terminal!(|&c: &char| (' ' <= c && c <= '~')
                        && c != '\''
                        && c != '\\')),
                    scan_seq!(
                        scan_terminal!('\\'),
                        scan_choice!(
                            choice!(
                                just("\n").ignored(),
                                just("\r").ignored(),
                                just("\"").ignored(),
                                just("'").ignored(),
                                just("\\").ignored(),
                                just("n").ignored(),
                                just("r").ignored(),
                                just("t").ignored()
                            ),
                            scan_seq!(
                                scan_terminal!('x'),
                                scan_repeated!(
                                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                        || ('a' <= c && c <= 'f')
                                        || ('A' <= c && c <= 'F')),
                                    2usize,
                                    2usize
                                )
                            ),
                            scan_seq!(
                                scan_terminal!('u'),
                                scan_repeated!(
                                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                        || ('a' <= c && c <= 'f')
                                        || ('A' <= c && c <= 'F')),
                                    4usize,
                                    4usize
                                )
                            )
                        )
                    )
                )),
                scan_terminal!("'")
            ),
            scan_seq!(
                scan_terminal!("\""),
                scan_zero_or_more!(scan_choice!(
                    scan_one_or_more!(scan_terminal!(|&c: &char| (' ' <= c && c <= '~')
                        && c != '"'
                        && c != '\\')),
                    scan_seq!(
                        scan_terminal!('\\'),
                        scan_choice!(
                            choice!(
                                just("\n").ignored(),
                                just("\r").ignored(),
                                just("\"").ignored(),
                                just("'").ignored(),
                                just("\\").ignored(),
                                just("n").ignored(),
                                just("r").ignored(),
                                just("t").ignored()
                            ),
                            scan_seq!(
                                scan_terminal!('x'),
                                scan_repeated!(
                                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                        || ('a' <= c && c <= 'f')
                                        || ('A' <= c && c <= 'F')),
                                    2usize,
                                    2usize
                                )
                            ),
                            scan_seq!(
                                scan_terminal!('u'),
                                scan_repeated!(
                                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                        || ('a' <= c && c <= 'f')
                                        || ('A' <= c && c <= 'F')),
                                    4usize,
                                    4usize
                                )
                            )
                        )
                    )
                )),
                scan_terminal!("\"")
            )
        ))
    );

    // AssemblyFlags = '(' «DoubleQuotedAsciiStringLiteral»  { ',' «DoubleQuotedAsciiStringLiteral» } ')' ;
    define_rule!(
        AssemblyFlags,
        delimited_by!(
            terminal!(OpenParen, "("),
            separated_by!(
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
            with_trivia!(just("assembly")
                .to(TokenKind::Assembly)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            optional!(with_trivia!(just("\"evmasm\"")
                .to(TokenKind::DoubleQuoteEvmasmDoubleQuote)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })),
            optional!(rule!(AssemblyFlags)),
            rule!(YulBlock)
        )
    );

    // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
    define_rule!(
        AssignmentExpression,
        seq!(
            rule!(Expression),
            with_trivia!(choice!(
                just("%=").to(TokenKind::PercentEqual),
                just("&=").to(TokenKind::AmpersandEqual),
                just("*=").to(TokenKind::StarEqual),
                just("+=").to(TokenKind::PlusEqual),
                just("-=").to(TokenKind::MinusEqual),
                just("/=").to(TokenKind::SlashEqual),
                just("<<=").to(TokenKind::LessLessEqual),
                just("=").to(TokenKind::Equal),
                just(">>").ignore_then(choice!(
                    just("=").to(TokenKind::GreaterGreaterEqual),
                    just(">=").to(TokenKind::GreaterGreaterGreaterEqual)
                )),
                just("^=").to(TokenKind::CaretEqual),
                just("|=").to(TokenKind::PipeEqual)
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // BitAndExpression = Expression '&' Expression ;
    define_rule!(
        BitAndExpression,
        seq!(
            rule!(Expression),
            with_trivia!(just("&")
                .to(TokenKind::Ampersand)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // BitOrExpression = Expression '|' Expression ;
    define_rule!(
        BitOrExpression,
        seq!(
            rule!(Expression),
            with_trivia!(just("|")
                .to(TokenKind::Pipe)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // BitXOrExpression = Expression '^' Expression ;
    define_rule!(
        BitXOrExpression,
        seq!(
            rule!(Expression),
            with_trivia!(just("^")
                .to(TokenKind::Caret)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // Block = '{' { Statement | UncheckedBlock } '}' ;
    define_rule!(
        Block,
        delimited_by!(
            terminal!(OpenBrace, "{"),
            zero_or_more!(choice!(rule!(Statement), rule!(UncheckedBlock))),
            terminal!(CloseBrace, "}")
        )
    );

    // «BooleanLiteral» = 'true' | 'false' ;
    define_token!(
        BooleanLiteral,
        scan_make_node!(choice!(just("false").ignored(), just("true").ignored()))
    );

    // BreakStatement = 'break' ';' ;
    define_rule!(
        BreakStatement,
        seq!(
            with_trivia!(just("break")
                .to(TokenKind::Break)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
    define_rule!(
        CatchClause,
        seq!(
            with_trivia!(just("catch")
                .to(TokenKind::Catch)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            optional!(seq!(optional!(token!(Identifier)), rule!(ParameterList))),
            rule!(Block)
        )
    );

    // ConditionalExpression = Expression '?' Expression ':' Expression ;
    define_rule!(
        ConditionalExpression,
        seq!(
            rule!(Expression),
            seq!(
                with_trivia!(just("?")
                    .to(TokenKind::Question)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Expression),
                with_trivia!(just(":")
                    .to(TokenKind::Colon)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Expression)
            )
        )
    );

    // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
    define_rule!(
        ConstantDefinition,
        seq!(
            rule!(TypeName),
            with_trivia!(just("constant")
                .to(TokenKind::Constant)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            with_trivia!(just("=")
                .to(TokenKind::Equal)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
    define_rule!(
        ConstructorAttribute,
        choice!(
            rule!(ModifierInvocation),
            with_trivia!(choice!(
                just("internal").to(TokenKind::Internal),
                just("p").ignore_then(choice!(
                    just("ayable").to(TokenKind::Payable),
                    just("ublic").to(TokenKind::Public)
                ))
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // ConstructorDefinition = 'constructor' ParameterList { ConstructorAttribute } Block ;
    define_rule!(
        ConstructorDefinition,
        seq!(
            with_trivia!(just("constructor")
                .to(TokenKind::Constructor)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(ParameterList),
            zero_or_more!(rule!(ConstructorAttribute)),
            rule!(Block)
        )
    );

    // ContinueStatement = 'continue' ';' ;
    define_rule!(
        ContinueStatement,
        seq!(
            with_trivia!(just("continue")
                .to(TokenKind::Continue)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // ContractBodyElement = UsingDirective | ConstructorDefinition | FunctionDefinition | FallbackFunctionDefinition | ReceiveFunctionDefinition | ModifierDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | EventDefinition | ErrorDefinition | StateVariableDeclaration ;
    define_rule!(
        ContractBodyElement,
        choice!(
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
            optional!(with_trivia!(just("abstract")
                .to(TokenKind::Abstract)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })),
            with_trivia!(just("contract")
                .to(TokenKind::Contract)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            optional!(rule!(InheritanceSpecifierList)),
            delimited_by!(
                terminal!(OpenBrace, "{"),
                zero_or_more!(rule!(ContractBodyElement)),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // DataLocation = 'memory' | 'storage' | 'calldata' ;
    define_rule!(
        DataLocation,
        with_trivia!(choice!(
            just("calldata").to(TokenKind::Calldata),
            just("memory").to(TokenKind::Memory),
            just("storage").to(TokenKind::Storage)
        )
        .map_with_span(|kind, span: SpanType| (kind, span)))
        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
            cst::Node::token(
                kind,
                lex::Node::chars_unwrapped(range),
                leading_trivia,
                trailing_trivia,
            )
        })
    );

    // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalNumber» ;
    define_token!(
        DecimalExponent,
        scan_make_node!(scan_seq!(
            scan_terminal!(|&c: &char| c == 'e' || c == 'E'),
            scan_optional!(scan_terminal!('-')),
            scan_separated_by!(
                scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
                scan_terminal!("_")
            )
        ))
    );

    // «DecimalLiteral» = ( «DecimalNumber» [ '.' «DecimalNumber» ] | '.' «DecimalNumber» ) [ «DecimalExponent» ] ;
    define_token!(
        DecimalLiteral,
        scan_make_node!(scan_seq!(
            scan_choice!(
                scan_seq!(
                    scan_separated_by!(
                        scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
                        scan_terminal!("_")
                    ),
                    scan_optional!(scan_seq!(
                        scan_terminal!('.'),
                        scan_separated_by!(
                            scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
                            scan_terminal!("_")
                        )
                    ))
                ),
                scan_seq!(
                    scan_terminal!('.'),
                    scan_separated_by!(
                        scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
                        scan_terminal!("_")
                    )
                )
            ),
            scan_optional!(scan_seq!(
                scan_terminal!(|&c: &char| c == 'e' || c == 'E'),
                scan_optional!(scan_terminal!('-')),
                scan_separated_by!(
                    scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
                    scan_terminal!("_")
                )
            ))
        ))
    );

    // «DecimalNumber» = 1…*{ '0'…'9' }  { '_' 1…*{ '0'…'9' } } ;
    define_token!(
        DecimalNumber,
        scan_make_node!(scan_separated_by!(
            scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
            scan_terminal!("_")
        ))
    );

    // Definition = ContractDefinition | InterfaceDefinition | LibraryDefinition | FunctionDefinition | ConstantDefinition | StructDefinition | EnumDefinition | UserDefinedValueTypeDefinition | ErrorDefinition ;
    define_rule!(
        Definition,
        choice!(
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

    // DeleteStatement = 'delete' Expression ';' ;
    define_rule!(
        DeleteStatement,
        seq!(
            with_trivia!(just("delete")
                .to(TokenKind::Delete)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // Directive = PragmaDirective | ImportDirective | UsingDirective ;
    define_rule!(
        Directive,
        choice!(
            rule!(PragmaDirective),
            rule!(ImportDirective),
            rule!(UsingDirective)
        )
    );

    // DoWhileStatement = 'do' Statement 'while' '(' Expression ')' ';' ;
    define_rule!(
        DoWhileStatement,
        seq!(
            with_trivia!(just("do")
                .to(TokenKind::Do)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Statement),
            with_trivia!(just("while")
                .to(TokenKind::While)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            delimited_by!(
                terminal!(OpenParen, "("),
                rule!(Expression),
                terminal!(CloseParen, ")")
            ),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // «DoubleQuotedAsciiStringLiteral» = '"' { 1…*{ '\u{20}'…'~' - ( '"' | '\\' ) } | «EscapeSequence» } '"' ;
    define_token!(
        DoubleQuotedAsciiStringLiteral,
        scan_make_node!(scan_seq!(
            scan_terminal!("\""),
            scan_zero_or_more!(scan_choice!(
                scan_one_or_more!(scan_terminal!(|&c: &char| (' ' <= c && c <= '~')
                    && c != '"'
                    && c != '\\')),
                scan_seq!(
                    scan_terminal!('\\'),
                    scan_choice!(
                        choice!(
                            just("\n").ignored(),
                            just("\r").ignored(),
                            just("\"").ignored(),
                            just("'").ignored(),
                            just("\\").ignored(),
                            just("n").ignored(),
                            just("r").ignored(),
                            just("t").ignored()
                        ),
                        scan_seq!(
                            scan_terminal!('x'),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                2usize,
                                2usize
                            )
                        ),
                        scan_seq!(
                            scan_terminal!('u'),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                4usize,
                                4usize
                            )
                        )
                    )
                )
            )),
            scan_terminal!("\"")
        ))
    );

    // «DoubleQuotedUnicodeStringLiteral» = 'unicode"' { 1…*{ ¬( '"' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '"' ;
    define_token!(
        DoubleQuotedUnicodeStringLiteral,
        scan_make_node!(scan_seq!(
            scan_terminal!("unicode\""),
            scan_zero_or_more!(scan_choice!(
                scan_one_or_more!(scan_terminal!(|&c: &char| c != '"'
                    && c != '\\'
                    && c != '\n'
                    && c != '\r')),
                scan_seq!(
                    scan_terminal!('\\'),
                    scan_choice!(
                        choice!(
                            just("\n").ignored(),
                            just("\r").ignored(),
                            just("\"").ignored(),
                            just("'").ignored(),
                            just("\\").ignored(),
                            just("n").ignored(),
                            just("r").ignored(),
                            just("t").ignored()
                        ),
                        scan_seq!(
                            scan_terminal!('x'),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                2usize,
                                2usize
                            )
                        ),
                        scan_seq!(
                            scan_terminal!('u'),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                4usize,
                                4usize
                            )
                        )
                    )
                )
            )),
            scan_terminal!("\"")
        ))
    );

    // ElementaryType = 'bool' | 'string' | AddressType | PayableType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
    define_rule!(
        ElementaryType,
        choice!(
            with_trivia!(choice!(
                just("bool").to(TokenKind::Bool),
                just("string").to(TokenKind::String)
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(AddressType),
            rule!(PayableType),
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
            with_trivia!(just("emit")
                .to(TokenKind::Emit)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(IdentifierPath),
            rule!(ArgumentList),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // EndOfFileTrivia = { «Whitespace» | «MultilineComment» | «SingleLineComment» } ;
    define_rule!(
        EndOfFileTrivia,
        zero_or_more!(choice!(
            trivia_token!(Whitespace),
            trivia_token!(MultilineComment),
            trivia_token!(SingleLineComment)
        ))
    );

    // «EndOfLine» = 1…*{ '\u{d}' | '\u{a}' } ;
    define_token!(
        EndOfLine,
        scan_make_node!(scan_one_or_more!(scan_terminal!(
            |&c: &char| c == '\r' || c == '\n'
        )))
    );

    // EnumDefinition = 'enum' «Identifier» '{' «Identifier»  { ',' «Identifier» } '}' ;
    define_rule!(
        EnumDefinition,
        seq!(
            with_trivia!(just("enum")
                .to(TokenKind::Enum)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            delimited_by!(
                terminal!(OpenBrace, "{"),
                separated_by!(token!(Identifier), terminal!(Comma, ",")),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // EqualityComparisonExpression = Expression ( '==' | '!=' ) Expression ;
    define_rule!(
        EqualityComparisonExpression,
        seq!(
            rule!(Expression),
            with_trivia!(choice!(
                just("!=").to(TokenKind::BangEqual),
                just("==").to(TokenKind::EqualEqual)
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
    define_rule!(
        ErrorDefinition,
        seq!(
            with_trivia!(just("error")
                .to(TokenKind::Error)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(rule!(ErrorParameter), terminal!(Comma, ","))),
                terminal!(CloseParen, ")")
            ),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // ErrorParameter = TypeName [ «Identifier» ] ;
    define_rule!(
        ErrorParameter,
        seq!(rule!(TypeName), optional!(token!(Identifier)))
    );

    // «EscapeSequence» = '\\' ( «AsciiEscape» | «HexByteEscape» | «UnicodeEscape» ) ;
    define_token!(
        EscapeSequence,
        scan_make_node!(scan_seq!(
            scan_terminal!('\\'),
            scan_choice!(
                choice!(
                    just("\n").ignored(),
                    just("\r").ignored(),
                    just("\"").ignored(),
                    just("'").ignored(),
                    just("\\").ignored(),
                    just("n").ignored(),
                    just("r").ignored(),
                    just("t").ignored()
                ),
                scan_seq!(
                    scan_terminal!('x'),
                    scan_repeated!(
                        scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                            || ('a' <= c && c <= 'f')
                            || ('A' <= c && c <= 'F')),
                        2usize,
                        2usize
                    )
                ),
                scan_seq!(
                    scan_terminal!('u'),
                    scan_repeated!(
                        scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                            || ('a' <= c && c <= 'f')
                            || ('A' <= c && c <= 'F')),
                        4usize,
                        4usize
                    )
                )
            )
        ))
    );

    // EventDefinition = 'event' «Identifier» '(' [ EventParameter  { ',' EventParameter } ] ')' [ 'anonymous' ] ';' ;
    define_rule!(
        EventDefinition,
        seq!(
            with_trivia!(just("event")
                .to(TokenKind::Event)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(rule!(EventParameter), terminal!(Comma, ","))),
                terminal!(CloseParen, ")")
            ),
            optional!(with_trivia!(just("anonymous")
                .to(TokenKind::Anonymous)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
    define_rule!(
        EventParameter,
        seq!(
            rule!(TypeName),
            optional!(with_trivia!(just("indexed")
                .to(TokenKind::Indexed)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })),
            optional!(token!(Identifier))
        )
    );

    // ExperimentalPragma = 'experimental' «Identifier» ;
    define_rule!(
        ExperimentalPragma,
        seq!(
            with_trivia!(just("experimental")
                .to(TokenKind::Experimental)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier)
        )
    );

    // (* 0.4.11 *) ExponentiationExpression = Expression '**' Expression ;
    // (* 0.6.0 *) ExponentiationExpression = Expression '**' Expression ;
    define_rule!(
        ExponentiationExpression,
        if version_0_6_0 <= version {
            seq!(
                rule!(Expression),
                with_trivia!(just("**")
                    .to(TokenKind::StarStar)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Expression)
            )
            .boxed()
        } else {
            seq!(
                rule!(Expression),
                with_trivia!(just("**")
                    .to(TokenKind::StarStar)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Expression)
            )
            .boxed()
        }
    );

    // (* 0.4.11 *) Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    // (* 0.6.0 *) Expression = AssignmentExpression | ConditionalExpression | OrExpression | AndExpression | EqualityComparisonExpression | OrderComparisonExpression | BitOrExpression | BitXOrExpression | BitAndExpression | ShiftExpression | AddSubExpression | MulDivModExpression | ExponentiationExpression | UnarySuffixExpression | UnaryPrefixExpression | FunctionCallExpression | MemberAccessExpression | IndexAccessExpression | PrimaryExpression ;
    define_rule!(
        Expression,
        if version_0_6_0 <= version {
            {
                enum Pratt {
                    Operator {
                        kind: RuleKind,
                        node: Option<Rc<cst::Node>>,
                        left_binding_power: u8,
                        right_binding_power: u8,
                    },
                    Node(Option<Rc<cst::Node>>),
                }
                let prefix_operator = with_trivia!(choice!(
                    just("!").to((TokenKind::Bang, 255, 29u8, RuleKind::UnaryPrefixExpression)),
                    just("++").to((
                        TokenKind::PlusPlus,
                        255,
                        29u8,
                        RuleKind::UnaryPrefixExpression
                    )),
                    just("-").ignore_then(choice!(
                        just("-").to((
                            TokenKind::MinusMinus,
                            255,
                            29u8,
                            RuleKind::UnaryPrefixExpression
                        )),
                        empty().to((TokenKind::Minus, 255, 29u8, RuleKind::UnaryPrefixExpression))
                    )),
                    just("~").to((TokenKind::Tilde, 255, 29u8, RuleKind::UnaryPrefixExpression))
                )
                .map_with_span(|payload, span: SpanType| (payload, span)))
                .map(
                    |(
                        (
                            leading_trivia,
                            ((token_kind, left_binding_power, right_binding_power, kind), range),
                        ),
                        trailing_trivia,
                    )| Pratt::Operator {
                        node: cst::Node::token(
                            token_kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        ),
                        kind,
                        left_binding_power,
                        right_binding_power,
                    },
                );
                let suffix_operator = choice!(
                    with_trivia!(choice!(
                        just("++").to((
                            TokenKind::PlusPlus,
                            27u8,
                            255,
                            RuleKind::UnarySuffixExpression
                        )),
                        just("--").to((
                            TokenKind::MinusMinus,
                            27u8,
                            255,
                            RuleKind::UnarySuffixExpression
                        ))
                    )
                    .map_with_span(|payload, span: SpanType| (payload, span)))
                    .map(
                        |(
                            (
                                leading_trivia,
                                (
                                    (token_kind, left_binding_power, right_binding_power, kind),
                                    range,
                                ),
                            ),
                            trailing_trivia,
                        )| Pratt::Operator {
                            node: cst::Node::token(
                                token_kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia
                            ),
                            kind,
                            left_binding_power,
                            right_binding_power
                        }
                    ),
                    seq!(
                        with_trivia!(just("?")
                            .to(TokenKind::Question)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        }),
                        rule!(Expression),
                        with_trivia!(just(":")
                            .to(TokenKind::Colon)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        }),
                        rule!(Expression)
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::ConditionalExpression,
                        left_binding_power: 3u8,
                        right_binding_power: 255
                    }),
                    seq!(optional!(rule!(NamedArgumentList)), rule!(ArgumentList)).map(|node| {
                        Pratt::Operator {
                            node,
                            kind: RuleKind::FunctionCallExpression,
                            left_binding_power: 31u8,
                            right_binding_power: 255,
                        }
                    }),
                    seq!(
                        with_trivia!(just(".")
                            .to(TokenKind::Period)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        }),
                        choice!(
                            token!(Identifier),
                            with_trivia!(just("address")
                                .to(TokenKind::Address)
                                .map_with_span(|kind, span: SpanType| (kind, span)))
                            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                                cst::Node::token(
                                    kind,
                                    lex::Node::chars_unwrapped(range),
                                    leading_trivia,
                                    trailing_trivia,
                                )
                            })
                        )
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::MemberAccessExpression,
                        left_binding_power: 33u8,
                        right_binding_power: 255
                    }),
                    delimited_by!(
                        terminal!(OpenBracket, "["),
                        seq!(
                            optional!(rule!(Expression)),
                            optional!(seq!(
                                with_trivia!(just(":")
                                    .to(TokenKind::Colon)
                                    .map_with_span(|kind, span: SpanType| (kind, span)))
                                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                                    cst::Node::token(
                                        kind,
                                        lex::Node::chars_unwrapped(range),
                                        leading_trivia,
                                        trailing_trivia,
                                    )
                                }),
                                optional!(rule!(Expression))
                            ))
                        ),
                        terminal!(CloseBracket, "]")
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::IndexAccessExpression,
                        left_binding_power: 35u8,
                        right_binding_power: 255
                    })
                );
                let primary_expression = rule!(PrimaryExpression);
                let prefixes_primary_suffixes = prefix_operator
                    .repeated()
                    .then(primary_expression)
                    .then(suffix_operator.repeated());
                type PPS = ((Vec<Pratt>, Option<Rc<cst::Node>>), Vec<Pratt>);
                let binary_operator = with_trivia!(choice!(
                    just("!=").to((
                        TokenKind::BangEqual,
                        9u8,
                        9u8 + 1,
                        RuleKind::EqualityComparisonExpression
                    )),
                    just("%").ignore_then(choice!(
                        just("=").to((
                            TokenKind::PercentEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((
                            TokenKind::Percent,
                            23u8,
                            23u8 + 1,
                            RuleKind::MulDivModExpression
                        ))
                    )),
                    just("&").ignore_then(choice!(
                        just("&").to((
                            TokenKind::AmpersandAmpersand,
                            7u8,
                            7u8 + 1,
                            RuleKind::AndExpression
                        )),
                        just("=").to((
                            TokenKind::AmpersandEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((
                            TokenKind::Ampersand,
                            17u8,
                            17u8 + 1,
                            RuleKind::BitAndExpression
                        ))
                    )),
                    just("*").ignore_then(choice!(
                        just("*").to((
                            TokenKind::StarStar,
                            25u8 + 1,
                            25u8,
                            RuleKind::ExponentiationExpression
                        )),
                        just("=").to((
                            TokenKind::StarEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((
                            TokenKind::Star,
                            23u8,
                            23u8 + 1,
                            RuleKind::MulDivModExpression
                        ))
                    )),
                    just("+").ignore_then(choice!(
                        just("=").to((
                            TokenKind::PlusEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((TokenKind::Plus, 21u8, 21u8 + 1, RuleKind::AddSubExpression))
                    )),
                    just("-").ignore_then(choice!(
                        just("=").to((
                            TokenKind::MinusEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((TokenKind::Minus, 21u8, 21u8 + 1, RuleKind::AddSubExpression))
                    )),
                    just("/").ignore_then(choice!(
                        just("=").to((
                            TokenKind::SlashEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((
                            TokenKind::Slash,
                            23u8,
                            23u8 + 1,
                            RuleKind::MulDivModExpression
                        ))
                    )),
                    just("<").ignore_then(choice!(
                        just("<").ignore_then(choice!(
                            just("=").to((
                                TokenKind::LessLessEqual,
                                1u8,
                                1u8 + 1,
                                RuleKind::AssignmentExpression
                            )),
                            empty().to((
                                TokenKind::LessLess,
                                19u8,
                                19u8 + 1,
                                RuleKind::ShiftExpression
                            ))
                        )),
                        just("=").to((
                            TokenKind::LessEqual,
                            11u8,
                            11u8 + 1,
                            RuleKind::OrderComparisonExpression
                        )),
                        empty().to((
                            TokenKind::Less,
                            11u8,
                            11u8 + 1,
                            RuleKind::OrderComparisonExpression
                        ))
                    )),
                    just("=").ignore_then(choice!(
                        just("=").to((
                            TokenKind::EqualEqual,
                            9u8,
                            9u8 + 1,
                            RuleKind::EqualityComparisonExpression
                        )),
                        empty().to((
                            TokenKind::Equal,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        ))
                    )),
                    just(">").ignore_then(choice!(
                        just("=").to((
                            TokenKind::GreaterEqual,
                            11u8,
                            11u8 + 1,
                            RuleKind::OrderComparisonExpression
                        )),
                        just(">").ignore_then(choice!(
                            just("=").to((
                                TokenKind::GreaterGreaterEqual,
                                1u8,
                                1u8 + 1,
                                RuleKind::AssignmentExpression
                            )),
                            just(">").ignore_then(choice!(
                                just("=").to((
                                    TokenKind::GreaterGreaterGreaterEqual,
                                    1u8,
                                    1u8 + 1,
                                    RuleKind::AssignmentExpression
                                )),
                                empty().to((
                                    TokenKind::GreaterGreaterGreater,
                                    19u8,
                                    19u8 + 1,
                                    RuleKind::ShiftExpression
                                ))
                            )),
                            empty().to((
                                TokenKind::GreaterGreater,
                                19u8,
                                19u8 + 1,
                                RuleKind::ShiftExpression
                            ))
                        )),
                        empty().to((
                            TokenKind::Greater,
                            11u8,
                            11u8 + 1,
                            RuleKind::OrderComparisonExpression
                        ))
                    )),
                    just("^").ignore_then(choice!(
                        just("=").to((
                            TokenKind::CaretEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((TokenKind::Caret, 15u8, 15u8 + 1, RuleKind::BitXOrExpression))
                    )),
                    just("|").ignore_then(choice!(
                        just("=").to((
                            TokenKind::PipeEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        just("|").to((TokenKind::PipePipe, 5u8, 5u8 + 1, RuleKind::OrExpression)),
                        empty().to((TokenKind::Pipe, 13u8, 13u8 + 1, RuleKind::BitOrExpression))
                    ))
                )
                .map_with_span(|payload, span: SpanType| (payload, span)))
                .map(
                    |(
                        (
                            leading_trivia,
                            ((token_kind, left_binding_power, right_binding_power, kind), range),
                        ),
                        trailing_trivia,
                    )| Pratt::Operator {
                        node: cst::Node::token(
                            token_kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        ),
                        kind,
                        left_binding_power,
                        right_binding_power,
                    },
                );
                prefixes_primary_suffixes
                    .clone()
                    .then(binary_operator.then(prefixes_primary_suffixes).repeated())
                    .map(|(pps, tail): (PPS, Vec<(Pratt, PPS)>)| {
                        let mut elements = Vec::new();
                        let ((prefixes, expr), suffixes) = pps;
                        elements.extend(prefixes.into_iter());
                        elements.push(Pratt::Node(expr));
                        elements.extend(suffixes.into_iter());
                        for (binary_operator, pps) in tail.into_iter() {
                            elements.push(binary_operator);
                            let ((prefixes, expr), suffixes) = pps;
                            elements.extend(prefixes.into_iter());
                            elements.push(Pratt::Node(expr));
                            elements.extend(suffixes.into_iter());
                        }
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
                                        Pratt::Node(left),
                                        Pratt::Operator { node: op, kind, .. },
                                    ) = (left, op)
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
                            node
                        } else {
                            unreachable!()
                        }
                    })
            }
            .boxed()
        } else {
            {
                enum Pratt {
                    Operator {
                        kind: RuleKind,
                        node: Option<Rc<cst::Node>>,
                        left_binding_power: u8,
                        right_binding_power: u8,
                    },
                    Node(Option<Rc<cst::Node>>),
                }
                let prefix_operator = with_trivia!(choice!(
                    just("!").to((TokenKind::Bang, 255, 29u8, RuleKind::UnaryPrefixExpression)),
                    just("++").to((
                        TokenKind::PlusPlus,
                        255,
                        29u8,
                        RuleKind::UnaryPrefixExpression
                    )),
                    just("-").ignore_then(choice!(
                        just("-").to((
                            TokenKind::MinusMinus,
                            255,
                            29u8,
                            RuleKind::UnaryPrefixExpression
                        )),
                        empty().to((TokenKind::Minus, 255, 29u8, RuleKind::UnaryPrefixExpression))
                    )),
                    just("~").to((TokenKind::Tilde, 255, 29u8, RuleKind::UnaryPrefixExpression))
                )
                .map_with_span(|payload, span: SpanType| (payload, span)))
                .map(
                    |(
                        (
                            leading_trivia,
                            ((token_kind, left_binding_power, right_binding_power, kind), range),
                        ),
                        trailing_trivia,
                    )| Pratt::Operator {
                        node: cst::Node::token(
                            token_kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        ),
                        kind,
                        left_binding_power,
                        right_binding_power,
                    },
                );
                let suffix_operator = choice!(
                    with_trivia!(choice!(
                        just("++").to((
                            TokenKind::PlusPlus,
                            27u8,
                            255,
                            RuleKind::UnarySuffixExpression
                        )),
                        just("--").to((
                            TokenKind::MinusMinus,
                            27u8,
                            255,
                            RuleKind::UnarySuffixExpression
                        ))
                    )
                    .map_with_span(|payload, span: SpanType| (payload, span)))
                    .map(
                        |(
                            (
                                leading_trivia,
                                (
                                    (token_kind, left_binding_power, right_binding_power, kind),
                                    range,
                                ),
                            ),
                            trailing_trivia,
                        )| Pratt::Operator {
                            node: cst::Node::token(
                                token_kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia
                            ),
                            kind,
                            left_binding_power,
                            right_binding_power
                        }
                    ),
                    seq!(
                        with_trivia!(just("?")
                            .to(TokenKind::Question)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        }),
                        rule!(Expression),
                        with_trivia!(just(":")
                            .to(TokenKind::Colon)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        }),
                        rule!(Expression)
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::ConditionalExpression,
                        left_binding_power: 3u8,
                        right_binding_power: 255
                    }),
                    seq!(optional!(rule!(NamedArgumentList)), rule!(ArgumentList)).map(|node| {
                        Pratt::Operator {
                            node,
                            kind: RuleKind::FunctionCallExpression,
                            left_binding_power: 31u8,
                            right_binding_power: 255,
                        }
                    }),
                    seq!(
                        with_trivia!(just(".")
                            .to(TokenKind::Period)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        }),
                        choice!(
                            token!(Identifier),
                            with_trivia!(just("address")
                                .to(TokenKind::Address)
                                .map_with_span(|kind, span: SpanType| (kind, span)))
                            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                                cst::Node::token(
                                    kind,
                                    lex::Node::chars_unwrapped(range),
                                    leading_trivia,
                                    trailing_trivia,
                                )
                            })
                        )
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::MemberAccessExpression,
                        left_binding_power: 33u8,
                        right_binding_power: 255
                    }),
                    delimited_by!(
                        terminal!(OpenBracket, "["),
                        seq!(
                            optional!(rule!(Expression)),
                            optional!(seq!(
                                with_trivia!(just(":")
                                    .to(TokenKind::Colon)
                                    .map_with_span(|kind, span: SpanType| (kind, span)))
                                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                                    cst::Node::token(
                                        kind,
                                        lex::Node::chars_unwrapped(range),
                                        leading_trivia,
                                        trailing_trivia,
                                    )
                                }),
                                optional!(rule!(Expression))
                            ))
                        ),
                        terminal!(CloseBracket, "]")
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::IndexAccessExpression,
                        left_binding_power: 35u8,
                        right_binding_power: 255
                    })
                );
                let primary_expression = rule!(PrimaryExpression);
                let prefixes_primary_suffixes = prefix_operator
                    .repeated()
                    .then(primary_expression)
                    .then(suffix_operator.repeated());
                type PPS = ((Vec<Pratt>, Option<Rc<cst::Node>>), Vec<Pratt>);
                let binary_operator = with_trivia!(choice!(
                    just("!=").to((
                        TokenKind::BangEqual,
                        9u8,
                        9u8 + 1,
                        RuleKind::EqualityComparisonExpression
                    )),
                    just("%").ignore_then(choice!(
                        just("=").to((
                            TokenKind::PercentEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((
                            TokenKind::Percent,
                            23u8,
                            23u8 + 1,
                            RuleKind::MulDivModExpression
                        ))
                    )),
                    just("&").ignore_then(choice!(
                        just("&").to((
                            TokenKind::AmpersandAmpersand,
                            7u8,
                            7u8 + 1,
                            RuleKind::AndExpression
                        )),
                        just("=").to((
                            TokenKind::AmpersandEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((
                            TokenKind::Ampersand,
                            17u8,
                            17u8 + 1,
                            RuleKind::BitAndExpression
                        ))
                    )),
                    just("*").ignore_then(choice!(
                        just("*").to((
                            TokenKind::StarStar,
                            25u8,
                            25u8 + 1,
                            RuleKind::ExponentiationExpression
                        )),
                        just("=").to((
                            TokenKind::StarEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((
                            TokenKind::Star,
                            23u8,
                            23u8 + 1,
                            RuleKind::MulDivModExpression
                        ))
                    )),
                    just("+").ignore_then(choice!(
                        just("=").to((
                            TokenKind::PlusEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((TokenKind::Plus, 21u8, 21u8 + 1, RuleKind::AddSubExpression))
                    )),
                    just("-").ignore_then(choice!(
                        just("=").to((
                            TokenKind::MinusEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((TokenKind::Minus, 21u8, 21u8 + 1, RuleKind::AddSubExpression))
                    )),
                    just("/").ignore_then(choice!(
                        just("=").to((
                            TokenKind::SlashEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((
                            TokenKind::Slash,
                            23u8,
                            23u8 + 1,
                            RuleKind::MulDivModExpression
                        ))
                    )),
                    just("<").ignore_then(choice!(
                        just("<").ignore_then(choice!(
                            just("=").to((
                                TokenKind::LessLessEqual,
                                1u8,
                                1u8 + 1,
                                RuleKind::AssignmentExpression
                            )),
                            empty().to((
                                TokenKind::LessLess,
                                19u8,
                                19u8 + 1,
                                RuleKind::ShiftExpression
                            ))
                        )),
                        just("=").to((
                            TokenKind::LessEqual,
                            11u8,
                            11u8 + 1,
                            RuleKind::OrderComparisonExpression
                        )),
                        empty().to((
                            TokenKind::Less,
                            11u8,
                            11u8 + 1,
                            RuleKind::OrderComparisonExpression
                        ))
                    )),
                    just("=").ignore_then(choice!(
                        just("=").to((
                            TokenKind::EqualEqual,
                            9u8,
                            9u8 + 1,
                            RuleKind::EqualityComparisonExpression
                        )),
                        empty().to((
                            TokenKind::Equal,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        ))
                    )),
                    just(">").ignore_then(choice!(
                        just("=").to((
                            TokenKind::GreaterEqual,
                            11u8,
                            11u8 + 1,
                            RuleKind::OrderComparisonExpression
                        )),
                        just(">").ignore_then(choice!(
                            just("=").to((
                                TokenKind::GreaterGreaterEqual,
                                1u8,
                                1u8 + 1,
                                RuleKind::AssignmentExpression
                            )),
                            just(">").ignore_then(choice!(
                                just("=").to((
                                    TokenKind::GreaterGreaterGreaterEqual,
                                    1u8,
                                    1u8 + 1,
                                    RuleKind::AssignmentExpression
                                )),
                                empty().to((
                                    TokenKind::GreaterGreaterGreater,
                                    19u8,
                                    19u8 + 1,
                                    RuleKind::ShiftExpression
                                ))
                            )),
                            empty().to((
                                TokenKind::GreaterGreater,
                                19u8,
                                19u8 + 1,
                                RuleKind::ShiftExpression
                            ))
                        )),
                        empty().to((
                            TokenKind::Greater,
                            11u8,
                            11u8 + 1,
                            RuleKind::OrderComparisonExpression
                        ))
                    )),
                    just("^").ignore_then(choice!(
                        just("=").to((
                            TokenKind::CaretEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        empty().to((TokenKind::Caret, 15u8, 15u8 + 1, RuleKind::BitXOrExpression))
                    )),
                    just("|").ignore_then(choice!(
                        just("=").to((
                            TokenKind::PipeEqual,
                            1u8,
                            1u8 + 1,
                            RuleKind::AssignmentExpression
                        )),
                        just("|").to((TokenKind::PipePipe, 5u8, 5u8 + 1, RuleKind::OrExpression)),
                        empty().to((TokenKind::Pipe, 13u8, 13u8 + 1, RuleKind::BitOrExpression))
                    ))
                )
                .map_with_span(|payload, span: SpanType| (payload, span)))
                .map(
                    |(
                        (
                            leading_trivia,
                            ((token_kind, left_binding_power, right_binding_power, kind), range),
                        ),
                        trailing_trivia,
                    )| Pratt::Operator {
                        node: cst::Node::token(
                            token_kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        ),
                        kind,
                        left_binding_power,
                        right_binding_power,
                    },
                );
                prefixes_primary_suffixes
                    .clone()
                    .then(binary_operator.then(prefixes_primary_suffixes).repeated())
                    .map(|(pps, tail): (PPS, Vec<(Pratt, PPS)>)| {
                        let mut elements = Vec::new();
                        let ((prefixes, expr), suffixes) = pps;
                        elements.extend(prefixes.into_iter());
                        elements.push(Pratt::Node(expr));
                        elements.extend(suffixes.into_iter());
                        for (binary_operator, pps) in tail.into_iter() {
                            elements.push(binary_operator);
                            let ((prefixes, expr), suffixes) = pps;
                            elements.extend(prefixes.into_iter());
                            elements.push(Pratt::Node(expr));
                            elements.extend(suffixes.into_iter());
                        }
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
                                        Pratt::Node(left),
                                        Pratt::Operator { node: op, kind, .. },
                                    ) = (left, op)
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
                            node
                        } else {
                            unreachable!()
                        }
                    })
            }
            .boxed()
        }
    );

    // ExpressionStatement = Expression ';' ;
    define_rule!(
        ExpressionStatement,
        seq!(
            rule!(Expression),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
    define_rule!(
        FallbackFunctionAttribute,
        choice!(
            rule!(ModifierInvocation),
            rule!(OverrideSpecifier),
            with_trivia!(choice!(
                just("external").to(TokenKind::External),
                just("p").ignore_then(choice!(
                    just("ayable").to(TokenKind::Payable),
                    just("ure").to(TokenKind::Pure)
                )),
                just("vi").ignore_then(choice!(
                    just("ew").to(TokenKind::View),
                    just("rtual").to(TokenKind::Virtual)
                ))
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // FallbackFunctionDefinition = 'fallback' ParameterList { FallbackFunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    define_rule!(
        FallbackFunctionDefinition,
        seq!(
            with_trivia!(just("fallback")
                .to(TokenKind::Fallback)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(ParameterList),
            zero_or_more!(rule!(FallbackFunctionAttribute)),
            optional!(seq!(
                with_trivia!(just("returns")
                    .to(TokenKind::Returns)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(ParameterList)
            )),
            choice!(
                with_trivia!(just(";")
                    .to(TokenKind::Semicolon)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Block)
            )
        )
    );

    // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
    define_token!(
        FixedBytesType,
        scan_make_node!(scan_seq!(
            just("bytes").ignored(),
            choice!(
                just("1").ignore_then(choice!(
                    just("0").ignored(),
                    just("1").ignored(),
                    just("2").ignored(),
                    just("3").ignored(),
                    just("4").ignored(),
                    just("5").ignored(),
                    just("6").ignored(),
                    just("7").ignored(),
                    just("8").ignored(),
                    just("9").ignored(),
                    empty()
                )),
                just("2").ignore_then(choice!(
                    just("0").ignored(),
                    just("1").ignored(),
                    just("2").ignored(),
                    just("3").ignored(),
                    just("4").ignored(),
                    just("5").ignored(),
                    just("6").ignored(),
                    just("7").ignored(),
                    just("8").ignored(),
                    just("9").ignored(),
                    empty()
                )),
                just("3").ignore_then(choice!(
                    just("0").ignored(),
                    just("1").ignored(),
                    just("2").ignored(),
                    empty()
                )),
                just("4").ignored(),
                just("5").ignored(),
                just("6").ignored(),
                just("7").ignored(),
                just("8").ignored(),
                just("9").ignored()
            )
        ))
    );

    // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
    define_rule!(
        ForStatement,
        seq!(
            with_trivia!(just("for")
                .to(TokenKind::For)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            delimited_by!(
                terminal!(OpenParen, "("),
                seq!(
                    choice!(
                        rule!(SimpleStatement),
                        with_trivia!(just(";")
                            .to(TokenKind::Semicolon)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        })
                    ),
                    choice!(
                        rule!(ExpressionStatement),
                        with_trivia!(just(";")
                            .to(TokenKind::Semicolon)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        })
                    ),
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
            rule!(ModifierInvocation),
            rule!(OverrideSpecifier),
            with_trivia!(choice!(
                just("external").to(TokenKind::External),
                just("internal").to(TokenKind::Internal),
                just("p").ignore_then(choice!(
                    just("ayable").to(TokenKind::Payable),
                    just("rivate").to(TokenKind::Private),
                    just("u").ignore_then(choice!(
                        just("blic").to(TokenKind::Public),
                        just("re").to(TokenKind::Pure)
                    ))
                )),
                just("vi").ignore_then(choice!(
                    just("ew").to(TokenKind::View),
                    just("rtual").to(TokenKind::Virtual)
                ))
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // FunctionCallExpression = Expression [ NamedArgumentList ] ArgumentList ;
    define_rule!(
        FunctionCallExpression,
        seq!(
            rule!(Expression),
            optional!(rule!(NamedArgumentList)),
            rule!(ArgumentList)
        )
    );

    // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    define_rule!(
        FunctionDefinition,
        seq!(
            with_trivia!(just("function")
                .to(TokenKind::Function)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            choice!(
                token!(Identifier),
                with_trivia!(choice!(
                    just("fallback").to(TokenKind::Fallback),
                    just("receive").to(TokenKind::Receive)
                )
                .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                })
            ),
            rule!(ParameterList),
            zero_or_more!(rule!(FunctionAttribute)),
            optional!(seq!(
                with_trivia!(just("returns")
                    .to(TokenKind::Returns)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(ParameterList)
            )),
            choice!(
                with_trivia!(just(";")
                    .to(TokenKind::Semicolon)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Block)
            )
        )
    );

    // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
    define_rule!(
        FunctionType,
        seq!(
            with_trivia!(just("function")
                .to(TokenKind::Function)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(ParameterList),
            zero_or_more!(with_trivia!(choice!(
                just("external").to(TokenKind::External),
                just("internal").to(TokenKind::Internal),
                just("p").ignore_then(choice!(
                    just("ayable").to(TokenKind::Payable),
                    just("rivate").to(TokenKind::Private),
                    just("u").ignore_then(choice!(
                        just("blic").to(TokenKind::Public),
                        just("re").to(TokenKind::Pure)
                    ))
                )),
                just("view").to(TokenKind::View)
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })),
            optional!(seq!(
                with_trivia!(just("returns")
                    .to(TokenKind::Returns)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(ParameterList)
            ))
        )
    );

    // «HexByteEscape» = 'x' 2…2*{ «HexCharacter» } ;
    define_token!(
        HexByteEscape,
        scan_make_node!(scan_seq!(
            scan_terminal!('x'),
            scan_repeated!(
                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                    || ('a' <= c && c <= 'f')
                    || ('A' <= c && c <= 'F')),
                2usize,
                2usize
            )
        ))
    );

    // «HexCharacter» = '0'…'9' | 'a'…'f' | 'A'…'F' ;
    define_token!(
        HexCharacter,
        scan_make_node!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
            || ('a' <= c && c <= 'f')
            || ('A' <= c && c <= 'F')))
    );

    // «HexLiteral» = '0x' 1…*{ «HexCharacter» }  { '_' 1…*{ «HexCharacter» } } ;
    define_token!(
        HexLiteral,
        scan_make_node!(scan_seq!(
            just("0x").ignored(),
            scan_separated_by!(
                scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                    || ('a' <= c && c <= 'f')
                    || ('A' <= c && c <= 'F'))),
                scan_terminal!("_")
            )
        ))
    );

    // «HexStringLiteral» = 'hex' ( '"' [ «PossiblySeparatedPairsOfHexDigits» ] '"' | '\'' [ «PossiblySeparatedPairsOfHexDigits» ] '\'' ) ;
    define_token!(
        HexStringLiteral,
        scan_make_node!(scan_seq!(
            just("hex").ignored(),
            scan_choice!(
                scan_seq!(
                    scan_terminal!("\""),
                    scan_optional!(scan_seq!(
                        scan_repeated!(
                            scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                || ('a' <= c && c <= 'f')
                                || ('A' <= c && c <= 'F')),
                            2usize,
                            2usize
                        ),
                        scan_zero_or_more!(scan_seq!(
                            scan_optional!(scan_terminal!('_')),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                2usize,
                                2usize
                            )
                        ))
                    )),
                    scan_terminal!("\"")
                ),
                scan_seq!(
                    scan_terminal!("'"),
                    scan_optional!(scan_seq!(
                        scan_repeated!(
                            scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                || ('a' <= c && c <= 'f')
                                || ('A' <= c && c <= 'F')),
                            2usize,
                            2usize
                        ),
                        scan_zero_or_more!(scan_seq!(
                            scan_optional!(scan_terminal!('_')),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                2usize,
                                2usize
                            )
                        ))
                    )),
                    scan_terminal!("'")
                )
            )
        ))
    );

    // «Identifier» = «RawIdentifier» - «Keyword» ;
    define_token!(
        Identifier,
        scan_make_node!(difference(
            scan_seq!(
                scan_terminal!(|&c: &char| c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z')),
                scan_zero_or_more!(scan_terminal!(|&c: &char| c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z')
                    || ('0' <= c && c <= '9')))
            ),
            scan_choice!(
                choice!(just("false").ignored(), just("true").ignored()),
                scan_seq!(
                    just("bytes").ignored(),
                    choice!(
                        just("1").ignore_then(choice!(
                            just("0").ignored(),
                            just("1").ignored(),
                            just("2").ignored(),
                            just("3").ignored(),
                            just("4").ignored(),
                            just("5").ignored(),
                            just("6").ignored(),
                            just("7").ignored(),
                            just("8").ignored(),
                            just("9").ignored(),
                            empty()
                        )),
                        just("2").ignore_then(choice!(
                            just("0").ignored(),
                            just("1").ignored(),
                            just("2").ignored(),
                            just("3").ignored(),
                            just("4").ignored(),
                            just("5").ignored(),
                            just("6").ignored(),
                            just("7").ignored(),
                            just("8").ignored(),
                            just("9").ignored(),
                            empty()
                        )),
                        just("3").ignore_then(choice!(
                            just("0").ignored(),
                            just("1").ignored(),
                            just("2").ignored(),
                            empty()
                        )),
                        just("4").ignored(),
                        just("5").ignored(),
                        just("6").ignored(),
                        just("7").ignored(),
                        just("8").ignored(),
                        just("9").ignored()
                    )
                ),
                choice!(
                    just("a").ignore_then(choice!(
                        just("fter").ignored(),
                        just("lias").ignored(),
                        just("pply").ignored(),
                        just("uto").ignored()
                    )),
                    just("byte").ignored(),
                    just("c").ignore_then(choice!(just("ase").ignored(), just("opyof").ignored())),
                    just("d").ignore_then(choice!(
                        just("ays").ignored(),
                        just("ef")
                            .ignore_then(choice!(just("ault").ignored(), just("ine").ignored()))
                    )),
                    just("ether").ignored(),
                    just("fin").ignore_then(choice!(just("al").ignored(), just("ney").ignored())),
                    just("gwei").ignored(),
                    just("hours").ignored(),
                    just("i").ignore_then(choice!(
                        just("mplements").ignored(),
                        just("n").ignore_then(choice!(just("line").ignored(), empty()))
                    )),
                    just("let").ignored(),
                    just("m").ignore_then(choice!(
                        just("a")
                            .ignore_then(choice!(just("cro").ignored(), just("tch").ignored())),
                        just("inutes").ignored(),
                        just("utable").ignored()
                    )),
                    just("null").ignored(),
                    just("of").ignored(),
                    just("p")
                        .ignore_then(choice!(just("artial").ignored(), just("romise").ignored())),
                    just("re").ignore_then(choice!(
                        just("ference").ignored(),
                        just("locatable").ignored()
                    )),
                    just("s").ignore_then(choice!(
                        just("e")
                            .ignore_then(choice!(just("aled").ignored(), just("conds").ignored())),
                        just("izeof").ignored(),
                        just("tatic").ignored(),
                        just("upports").ignored(),
                        just("witch").ignored(),
                        just("zabo").ignored()
                    )),
                    just("type").ignore_then(choice!(just("def").ignored(), just("of").ignored())),
                    just("var").ignored(),
                    just("we").ignore_then(choice!(just("eks").ignored(), just("i").ignored())),
                    just("years").ignored()
                ),
                scan_seq!(
                    just("int").ignored(),
                    scan_optional!(choice!(
                        just("1").ignore_then(choice!(
                            just("04").ignored(),
                            just("12").ignored(),
                            just("2")
                                .ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                            just("36").ignored(),
                            just("44").ignored(),
                            just("52").ignored(),
                            just("6").ignore_then(choice!(
                                just("0").ignored(),
                                just("8").ignored(),
                                empty()
                            )),
                            just("76").ignored(),
                            just("84").ignored(),
                            just("92").ignored()
                        )),
                        just("2").ignore_then(choice!(
                            just("0")
                                .ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                            just("16").ignored(),
                            just("24").ignored(),
                            just("32").ignored(),
                            just("4").ignore_then(choice!(
                                just("0").ignored(),
                                just("8").ignored(),
                                empty()
                            )),
                            just("56").ignored()
                        )),
                        just("32").ignored(),
                        just("4").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                        just("56").ignored(),
                        just("64").ignored(),
                        just("72").ignored(),
                        just("8").ignore_then(choice!(
                            just("0").ignored(),
                            just("8").ignored(),
                            empty()
                        )),
                        just("96").ignored()
                    ))
                ),
                scan_seq!(
                    scan_terminal!('u'),
                    scan_seq!(
                        just("int").ignored(),
                        scan_optional!(choice!(
                            just("1").ignore_then(choice!(
                                just("04").ignored(),
                                just("12").ignored(),
                                just("2")
                                    .ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                                just("36").ignored(),
                                just("44").ignored(),
                                just("52").ignored(),
                                just("6").ignore_then(choice!(
                                    just("0").ignored(),
                                    just("8").ignored(),
                                    empty()
                                )),
                                just("76").ignored(),
                                just("84").ignored(),
                                just("92").ignored()
                            )),
                            just("2").ignore_then(choice!(
                                just("0")
                                    .ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                                just("16").ignored(),
                                just("24").ignored(),
                                just("32").ignored(),
                                just("4").ignore_then(choice!(
                                    just("0").ignored(),
                                    just("8").ignored(),
                                    empty()
                                )),
                                just("56").ignored()
                            )),
                            just("32").ignored(),
                            just("4")
                                .ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                            just("56").ignored(),
                            just("64").ignored(),
                            just("72").ignored(),
                            just("8").ignore_then(choice!(
                                just("0").ignored(),
                                just("8").ignored(),
                                empty()
                            )),
                            just("96").ignored()
                        ))
                    )
                ),
                choice!(
                    just("a").ignore_then(choice!(
                        just("bstract").ignored(),
                        just("ddress").ignored(),
                        just("nonymous").ignored(),
                        just("s").ignore_then(choice!(just("sembly").ignored(), empty()))
                    )),
                    just("b").ignore_then(choice!(just("ool").ignored(), just("reak").ignored())),
                    just("c").ignore_then(choice!(
                        just("a")
                            .ignore_then(choice!(just("lldata").ignored(), just("tch").ignored())),
                        just("on").ignore_then(choice!(
                            just("st").ignore_then(choice!(
                                just("ant").ignored(),
                                just("ructor").ignored()
                            )),
                            just("t").ignore_then(choice!(
                                just("inue").ignored(),
                                just("ract").ignored()
                            ))
                        ))
                    )),
                    just("d").ignore_then(choice!(just("elete").ignored(), just("o").ignored())),
                    just("e").ignore_then(choice!(
                        just("lse").ignored(),
                        just("mit").ignored(),
                        just("num").ignored(),
                        just("vent").ignored(),
                        just("xternal").ignored()
                    )),
                    just("f").ignore_then(choice!(
                        just("al")
                            .ignore_then(choice!(just("lback").ignored(), just("se").ignored())),
                        just("ixed").ignored(),
                        just("or").ignored(),
                        just("unction").ignored()
                    )),
                    just("hex").ignored(),
                    just("i").ignore_then(choice!(
                        just("f").ignored(),
                        just("m").ignore_then(choice!(
                            just("mutable").ignored(),
                            just("port").ignored()
                        )),
                        just("n").ignore_then(choice!(
                            just("dexed").ignored(),
                            just("ter").ignore_then(choice!(
                                just("face").ignored(),
                                just("nal").ignored()
                            ))
                        )),
                        just("s").ignored()
                    )),
                    just("library").ignored(),
                    just("m").ignore_then(choice!(
                        just("apping").ignored(),
                        just("emory").ignored(),
                        just("odifier").ignored()
                    )),
                    just("new").ignored(),
                    just("override").ignored(),
                    just("p").ignore_then(choice!(
                        just("ayable").ignored(),
                        just("r")
                            .ignore_then(choice!(just("agma").ignored(), just("ivate").ignored())),
                        just("u")
                            .ignore_then(choice!(just("blic").ignored(), just("re").ignored()))
                    )),
                    just("re").ignore_then(choice!(
                        just("ceive").ignored(),
                        just("turn").ignore_then(choice!(just("s").ignored(), empty()))
                    )),
                    just("st").ignore_then(choice!(
                        just("orage").ignored(),
                        just("r")
                            .ignore_then(choice!(just("ing").ignored(), just("uct").ignored()))
                    )),
                    just("t").ignore_then(choice!(
                        just("r").ignore_then(choice!(just("ue").ignored(), just("y").ignored())),
                        just("ype").ignored()
                    )),
                    just("u").ignore_then(choice!(
                        just("fixed").ignored(),
                        just("nchecked").ignored(),
                        just("sing").ignored()
                    )),
                    just("vi").ignore_then(choice!(just("ew").ignored(), just("rtual").ignored())),
                    just("while").ignored()
                )
            )
        ))
    );

    // «IdentifierPart» = «IdentifierStart» | '0'…'9' ;
    define_token!(
        IdentifierPart,
        scan_make_node!(scan_terminal!(|&c: &char| c == '_'
            || c == '$'
            || ('a' <= c && c <= 'z')
            || ('A' <= c && c <= 'Z')
            || ('0' <= c && c <= '9')))
    );

    // IdentifierPath = «Identifier»  { '.' «Identifier» } ;
    define_rule!(
        IdentifierPath,
        separated_by!(token!(Identifier), terminal!(Period, "."))
    );

    // «IdentifierStart» = '_' | '$' | 'a'…'z' | 'A'…'Z' ;
    define_token!(
        IdentifierStart,
        scan_make_node!(scan_terminal!(|&c: &char| c == '_'
            || c == '$'
            || ('a' <= c && c <= 'z')
            || ('A' <= c && c <= 'Z')))
    );

    // IfStatement = 'if' '(' Expression ')' Statement [ 'else' Statement ] ;
    define_rule!(
        IfStatement,
        seq!(
            with_trivia!(just("if")
                .to(TokenKind::If)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            delimited_by!(
                terminal!(OpenParen, "("),
                rule!(Expression),
                terminal!(CloseParen, ")")
            ),
            rule!(Statement),
            optional!(seq!(
                with_trivia!(just("else")
                    .to(TokenKind::Else)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Statement)
            ))
        )
    );

    // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
    define_rule!(
        ImportDirective,
        seq!(
            with_trivia!(just("import")
                .to(TokenKind::Import)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            choice!(
                rule!(SimpleImportDirective),
                rule!(StarImportDirective),
                rule!(SelectingImportDirective)
            ),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // ImportPath = «AsciiStringLiteral» ;
    define_rule!(ImportPath, token!(AsciiStringLiteral));

    // IndexAccessExpression = Expression '[' [ Expression ] [ ':' [ Expression ] ] ']' ;
    define_rule!(
        IndexAccessExpression,
        seq!(
            rule!(Expression),
            delimited_by!(
                terminal!(OpenBracket, "["),
                seq!(
                    optional!(rule!(Expression)),
                    optional!(seq!(
                        with_trivia!(just(":")
                            .to(TokenKind::Colon)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        }),
                        optional!(rule!(Expression))
                    ))
                ),
                terminal!(CloseBracket, "]")
            )
        )
    );

    // InheritanceSpecifier = IdentifierPath [ ArgumentList ] ;
    define_rule!(
        InheritanceSpecifier,
        seq!(rule!(IdentifierPath), optional!(rule!(ArgumentList)))
    );

    // InheritanceSpecifierList = 'is' InheritanceSpecifier  { ',' InheritanceSpecifier } ;
    define_rule!(
        InheritanceSpecifierList,
        seq!(
            with_trivia!(just("is")
                .to(TokenKind::Is)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            separated_by!(rule!(InheritanceSpecifier), terminal!(Comma, ","))
        )
    );

    // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    define_rule!(
        InterfaceDefinition,
        seq!(
            with_trivia!(just("interface")
                .to(TokenKind::Interface)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            optional!(rule!(InheritanceSpecifierList)),
            delimited_by!(
                terminal!(OpenBrace, "{"),
                zero_or_more!(rule!(ContractBodyElement)),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // «Keyword» = «BooleanLiteral» | «FixedBytesType» | «NumberUnit» | «ReservedKeyword» | «SignedIntegerType» | «UnsignedIntegerType» | 'abstract' | 'address' | 'anonymous' | 'as' | 'assembly' | 'bool' | 'break' | 'calldata' | 'catch' | 'constant' | 'constructor' | 'continue' | 'contract' | 'delete' | 'do' | 'else' | 'emit' | 'enum' | 'event' | 'external' | 'fallback' | 'false' | 'fixed' | 'for' | 'function' | 'hex' | 'if' | 'immutable' | 'import' | 'indexed' | 'interface' | 'internal' | 'is' | 'library' | 'mapping' | 'memory' | 'modifier' | 'new' | 'override' | 'payable' | 'pragma' | 'private' | 'public' | 'pure' | 'receive' | 'return' | 'returns' | 'storage' | 'string' | 'struct' | 'true' | 'try' | 'type' | 'ufixed' | 'unchecked' | 'using' | 'view' | 'virtual' | 'while' ;
    define_token!(
        Keyword,
        scan_make_node!(scan_choice!(
            choice!(just("false").ignored(), just("true").ignored()),
            scan_seq!(
                just("bytes").ignored(),
                choice!(
                    just("1").ignore_then(choice!(
                        just("0").ignored(),
                        just("1").ignored(),
                        just("2").ignored(),
                        just("3").ignored(),
                        just("4").ignored(),
                        just("5").ignored(),
                        just("6").ignored(),
                        just("7").ignored(),
                        just("8").ignored(),
                        just("9").ignored(),
                        empty()
                    )),
                    just("2").ignore_then(choice!(
                        just("0").ignored(),
                        just("1").ignored(),
                        just("2").ignored(),
                        just("3").ignored(),
                        just("4").ignored(),
                        just("5").ignored(),
                        just("6").ignored(),
                        just("7").ignored(),
                        just("8").ignored(),
                        just("9").ignored(),
                        empty()
                    )),
                    just("3").ignore_then(choice!(
                        just("0").ignored(),
                        just("1").ignored(),
                        just("2").ignored(),
                        empty()
                    )),
                    just("4").ignored(),
                    just("5").ignored(),
                    just("6").ignored(),
                    just("7").ignored(),
                    just("8").ignored(),
                    just("9").ignored()
                )
            ),
            choice!(
                just("a").ignore_then(choice!(
                    just("fter").ignored(),
                    just("lias").ignored(),
                    just("pply").ignored(),
                    just("uto").ignored()
                )),
                just("byte").ignored(),
                just("c").ignore_then(choice!(just("ase").ignored(), just("opyof").ignored())),
                just("d").ignore_then(choice!(
                    just("ays").ignored(),
                    just("ef").ignore_then(choice!(just("ault").ignored(), just("ine").ignored()))
                )),
                just("ether").ignored(),
                just("fin").ignore_then(choice!(just("al").ignored(), just("ney").ignored())),
                just("gwei").ignored(),
                just("hours").ignored(),
                just("i").ignore_then(choice!(
                    just("mplements").ignored(),
                    just("n").ignore_then(choice!(just("line").ignored(), empty()))
                )),
                just("let").ignored(),
                just("m").ignore_then(choice!(
                    just("a").ignore_then(choice!(just("cro").ignored(), just("tch").ignored())),
                    just("inutes").ignored(),
                    just("utable").ignored()
                )),
                just("null").ignored(),
                just("of").ignored(),
                just("p").ignore_then(choice!(just("artial").ignored(), just("romise").ignored())),
                just("re").ignore_then(choice!(
                    just("ference").ignored(),
                    just("locatable").ignored()
                )),
                just("s").ignore_then(choice!(
                    just("e").ignore_then(choice!(just("aled").ignored(), just("conds").ignored())),
                    just("izeof").ignored(),
                    just("tatic").ignored(),
                    just("upports").ignored(),
                    just("witch").ignored(),
                    just("zabo").ignored()
                )),
                just("type").ignore_then(choice!(just("def").ignored(), just("of").ignored())),
                just("var").ignored(),
                just("we").ignore_then(choice!(just("eks").ignored(), just("i").ignored())),
                just("years").ignored()
            ),
            scan_seq!(
                just("int").ignored(),
                scan_optional!(choice!(
                    just("1").ignore_then(choice!(
                        just("04").ignored(),
                        just("12").ignored(),
                        just("2").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                        just("36").ignored(),
                        just("44").ignored(),
                        just("52").ignored(),
                        just("6").ignore_then(choice!(
                            just("0").ignored(),
                            just("8").ignored(),
                            empty()
                        )),
                        just("76").ignored(),
                        just("84").ignored(),
                        just("92").ignored()
                    )),
                    just("2").ignore_then(choice!(
                        just("0").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                        just("16").ignored(),
                        just("24").ignored(),
                        just("32").ignored(),
                        just("4").ignore_then(choice!(
                            just("0").ignored(),
                            just("8").ignored(),
                            empty()
                        )),
                        just("56").ignored()
                    )),
                    just("32").ignored(),
                    just("4").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                    just("56").ignored(),
                    just("64").ignored(),
                    just("72").ignored(),
                    just("8").ignore_then(choice!(
                        just("0").ignored(),
                        just("8").ignored(),
                        empty()
                    )),
                    just("96").ignored()
                ))
            ),
            scan_seq!(
                scan_terminal!('u'),
                scan_seq!(
                    just("int").ignored(),
                    scan_optional!(choice!(
                        just("1").ignore_then(choice!(
                            just("04").ignored(),
                            just("12").ignored(),
                            just("2")
                                .ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                            just("36").ignored(),
                            just("44").ignored(),
                            just("52").ignored(),
                            just("6").ignore_then(choice!(
                                just("0").ignored(),
                                just("8").ignored(),
                                empty()
                            )),
                            just("76").ignored(),
                            just("84").ignored(),
                            just("92").ignored()
                        )),
                        just("2").ignore_then(choice!(
                            just("0")
                                .ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                            just("16").ignored(),
                            just("24").ignored(),
                            just("32").ignored(),
                            just("4").ignore_then(choice!(
                                just("0").ignored(),
                                just("8").ignored(),
                                empty()
                            )),
                            just("56").ignored()
                        )),
                        just("32").ignored(),
                        just("4").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                        just("56").ignored(),
                        just("64").ignored(),
                        just("72").ignored(),
                        just("8").ignore_then(choice!(
                            just("0").ignored(),
                            just("8").ignored(),
                            empty()
                        )),
                        just("96").ignored()
                    ))
                )
            ),
            choice!(
                just("a").ignore_then(choice!(
                    just("bstract").ignored(),
                    just("ddress").ignored(),
                    just("nonymous").ignored(),
                    just("s").ignore_then(choice!(just("sembly").ignored(), empty()))
                )),
                just("b").ignore_then(choice!(just("ool").ignored(), just("reak").ignored())),
                just("c").ignore_then(choice!(
                    just("a").ignore_then(choice!(just("lldata").ignored(), just("tch").ignored())),
                    just("on").ignore_then(choice!(
                        just("st")
                            .ignore_then(choice!(just("ant").ignored(), just("ructor").ignored())),
                        just("t")
                            .ignore_then(choice!(just("inue").ignored(), just("ract").ignored()))
                    ))
                )),
                just("d").ignore_then(choice!(just("elete").ignored(), just("o").ignored())),
                just("e").ignore_then(choice!(
                    just("lse").ignored(),
                    just("mit").ignored(),
                    just("num").ignored(),
                    just("vent").ignored(),
                    just("xternal").ignored()
                )),
                just("f").ignore_then(choice!(
                    just("al").ignore_then(choice!(just("lback").ignored(), just("se").ignored())),
                    just("ixed").ignored(),
                    just("or").ignored(),
                    just("unction").ignored()
                )),
                just("hex").ignored(),
                just("i").ignore_then(choice!(
                    just("f").ignored(),
                    just("m")
                        .ignore_then(choice!(just("mutable").ignored(), just("port").ignored())),
                    just("n").ignore_then(choice!(
                        just("dexed").ignored(),
                        just("ter")
                            .ignore_then(choice!(just("face").ignored(), just("nal").ignored()))
                    )),
                    just("s").ignored()
                )),
                just("library").ignored(),
                just("m").ignore_then(choice!(
                    just("apping").ignored(),
                    just("emory").ignored(),
                    just("odifier").ignored()
                )),
                just("new").ignored(),
                just("override").ignored(),
                just("p").ignore_then(choice!(
                    just("ayable").ignored(),
                    just("r").ignore_then(choice!(just("agma").ignored(), just("ivate").ignored())),
                    just("u").ignore_then(choice!(just("blic").ignored(), just("re").ignored()))
                )),
                just("re").ignore_then(choice!(
                    just("ceive").ignored(),
                    just("turn").ignore_then(choice!(just("s").ignored(), empty()))
                )),
                just("st").ignore_then(choice!(
                    just("orage").ignored(),
                    just("r").ignore_then(choice!(just("ing").ignored(), just("uct").ignored()))
                )),
                just("t").ignore_then(choice!(
                    just("r").ignore_then(choice!(just("ue").ignored(), just("y").ignored())),
                    just("ype").ignored()
                )),
                just("u").ignore_then(choice!(
                    just("fixed").ignored(),
                    just("nchecked").ignored(),
                    just("sing").ignored()
                )),
                just("vi").ignore_then(choice!(just("ew").ignored(), just("rtual").ignored())),
                just("while").ignored()
            )
        ))
    );

    // LeadingTrivia = { «Whitespace» | «EndOfLine» | «MultilineComment» | «SingleLineComment» } ;
    define_rule!(
        LeadingTrivia,
        zero_or_more!(choice!(
            trivia_token!(Whitespace),
            trivia_token!(EndOfLine),
            trivia_token!(MultilineComment),
            trivia_token!(SingleLineComment)
        ))
    );

    // LibraryDefinition = 'library' «Identifier» '{' { ContractBodyElement } '}' ;
    define_rule!(
        LibraryDefinition,
        seq!(
            with_trivia!(just("library")
                .to(TokenKind::Library)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            delimited_by!(
                terminal!(OpenBrace, "{"),
                zero_or_more!(rule!(ContractBodyElement)),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // MappingType = 'mapping' '(' ( ElementaryType | IdentifierPath ) '=>' TypeName ')' ;
    define_rule!(
        MappingType,
        seq!(
            with_trivia!(just("mapping")
                .to(TokenKind::Mapping)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            delimited_by!(
                terminal!(OpenParen, "("),
                seq!(
                    choice!(rule!(ElementaryType), rule!(IdentifierPath)),
                    with_trivia!(just("=>")
                        .to(TokenKind::EqualGreater)
                        .map_with_span(|kind, span: SpanType| (kind, span)))
                    .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                        cst::Node::token(
                            kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        )
                    }),
                    rule!(TypeName)
                ),
                terminal!(CloseParen, ")")
            )
        )
    );

    // MemberAccessExpression = Expression '.' ( «Identifier» | 'address' ) ;
    define_rule!(
        MemberAccessExpression,
        seq!(
            rule!(Expression),
            with_trivia!(just(".")
                .to(TokenKind::Period)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            choice!(
                token!(Identifier),
                with_trivia!(just("address")
                    .to(TokenKind::Address)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                })
            )
        )
    );

    // ModifierAttribute = OverrideSpecifier | 'virtual' ;
    define_rule!(
        ModifierAttribute,
        choice!(
            rule!(OverrideSpecifier),
            with_trivia!(just("virtual")
                .to(TokenKind::Virtual)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
    define_rule!(
        ModifierDefinition,
        seq!(
            with_trivia!(just("modifier")
                .to(TokenKind::Modifier)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            optional!(rule!(ParameterList)),
            zero_or_more!(rule!(ModifierAttribute)),
            choice!(
                with_trivia!(just(";")
                    .to(TokenKind::Semicolon)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Block)
            )
        )
    );

    // ModifierInvocation = IdentifierPath [ ArgumentList ] ;
    define_rule!(
        ModifierInvocation,
        seq!(rule!(IdentifierPath), optional!(rule!(ArgumentList)))
    );

    // MulDivModExpression = Expression ( '*' | '/' | '%' ) Expression ;
    define_rule!(
        MulDivModExpression,
        seq!(
            rule!(Expression),
            with_trivia!(choice!(
                just("%").to(TokenKind::Percent),
                just("*").to(TokenKind::Star),
                just("/").to(TokenKind::Slash)
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // «MultilineComment» = '/*' { ¬'*' | '*' ¬'/' } '*/' ;
    define_token!(
        MultilineComment,
        scan_make_node!(scan_seq!(
            scan_terminal!("/*"),
            scan_zero_or_more!(scan_choice!(
                scan_terminal!(|&c: &char| c != '*'),
                scan_seq!(scan_terminal!('*'), scan_terminal!(|&c: &char| c != '/'))
            )),
            scan_terminal!("*/")
        ))
    );

    // NamedArgument = «Identifier» ':' Expression ;
    define_rule!(
        NamedArgument,
        seq!(
            token!(Identifier),
            with_trivia!(just(":")
                .to(TokenKind::Colon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // NamedArgumentList = '{' [ NamedArgument  { ',' NamedArgument } ] '}' ;
    define_rule!(
        NamedArgumentList,
        delimited_by!(
            terminal!(OpenBrace, "{"),
            optional!(separated_by!(rule!(NamedArgument), terminal!(Comma, ","))),
            terminal!(CloseBrace, "}")
        )
    );

    // NewExpression = 'new' IdentifierPath ArgumentList ;
    define_rule!(
        NewExpression,
        seq!(
            with_trivia!(just("new")
                .to(TokenKind::New)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(IdentifierPath),
            rule!(ArgumentList)
        )
    );

    // «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
    define_token!(
        NumberUnit,
        scan_make_node!(choice!(
            just("days").ignored(),
            just("ether").ignored(),
            just("finney").ignored(),
            just("gwei").ignored(),
            just("hours").ignored(),
            just("minutes").ignored(),
            just("s").ignore_then(choice!(just("econds").ignored(), just("zabo").ignored())),
            just("we").ignore_then(choice!(just("eks").ignored(), just("i").ignored())),
            just("years").ignored()
        ))
    );

    // NumericLiteral = ( «HexLiteral» | «DecimalLiteral» ) [ «NumberUnit» ] ;
    define_rule!(
        NumericLiteral,
        seq!(
            choice!(token!(HexLiteral), token!(DecimalLiteral)),
            optional!(token!(NumberUnit))
        )
    );

    // OrExpression = Expression '||' Expression ;
    define_rule!(
        OrExpression,
        seq!(
            rule!(Expression),
            with_trivia!(just("||")
                .to(TokenKind::PipePipe)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
    define_rule!(
        OrderComparisonExpression,
        seq!(
            rule!(Expression),
            with_trivia!(choice!(
                just("<").ignore_then(choice!(
                    just("=").to(TokenKind::LessEqual),
                    empty().to(TokenKind::Less)
                )),
                just(">").ignore_then(choice!(
                    just("=").to(TokenKind::GreaterEqual),
                    empty().to(TokenKind::Greater)
                ))
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
    define_rule!(
        OverrideSpecifier,
        seq!(
            with_trivia!(just("override")
                .to(TokenKind::Override)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            optional!(delimited_by!(
                terminal!(OpenParen, "("),
                separated_by!(rule!(IdentifierPath), terminal!(Comma, ",")),
                terminal!(CloseParen, ")")
            ))
        )
    );

    // ParameterDeclaration = TypeName [ DataLocation ] [ «Identifier» ] ;
    define_rule!(
        ParameterDeclaration,
        seq!(
            rule!(TypeName),
            optional!(rule!(DataLocation)),
            optional!(token!(Identifier))
        )
    );

    // ParameterList = '(' [ ParameterDeclaration  { ',' ParameterDeclaration } ] ')' ;
    define_rule!(
        ParameterList,
        delimited_by!(
            terminal!(OpenParen, "("),
            optional!(separated_by!(
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
            terminal!(OpenParen, "("),
            separated_by!(optional!(rule!(Expression)), terminal!(Comma, ",")),
            terminal!(CloseParen, ")")
        )
    );

    // PayableType = 'payable' ;
    define_rule!(
        PayableType,
        with_trivia!(just("payable")
            .to(TokenKind::Payable)
            .map_with_span(|kind, span: SpanType| (kind, span)))
        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
            cst::Node::token(
                kind,
                lex::Node::chars_unwrapped(range),
                leading_trivia,
                trailing_trivia,
            )
        })
    );

    // PositionalArgumentList = Expression  { ',' Expression } ;
    define_rule!(
        PositionalArgumentList,
        separated_by!(rule!(Expression), terminal!(Comma, ","))
    );

    // «PossiblySeparatedPairsOfHexDigits» = 2…2*{ «HexCharacter» } { [ '_' ] 2…2*{ «HexCharacter» } } ;
    define_token!(
        PossiblySeparatedPairsOfHexDigits,
        scan_make_node!(scan_seq!(
            scan_repeated!(
                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                    || ('a' <= c && c <= 'f')
                    || ('A' <= c && c <= 'F')),
                2usize,
                2usize
            ),
            scan_zero_or_more!(scan_seq!(
                scan_optional!(scan_terminal!('_')),
                scan_repeated!(
                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                        || ('a' <= c && c <= 'f')
                        || ('A' <= c && c <= 'F')),
                    2usize,
                    2usize
                )
            ))
        ))
    );

    // PragmaDirective = 'pragma' ( VersionPragma | ABICoderPragma | ExperimentalPragma ) ';' ;
    define_rule!(
        PragmaDirective,
        seq!(
            with_trivia!(just("pragma")
                .to(TokenKind::Pragma)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            choice!(
                rule!(VersionPragma),
                rule!(ABICoderPragma),
                rule!(ExperimentalPragma)
            ),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // PrimaryExpression = «Identifier» | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | NumericLiteral | «BooleanLiteral» | NewExpression | TypeExpression | ElementaryType ;
    define_rule!(
        PrimaryExpression,
        choice!(
            token!(Identifier),
            rule!(ParenthesisExpression),
            rule!(ArrayLiteral),
            token!(AsciiStringLiteral),
            token!(UnicodeStringLiteral),
            token!(HexStringLiteral),
            rule!(NumericLiteral),
            token!(BooleanLiteral),
            rule!(NewExpression),
            rule!(TypeExpression),
            rule!(ElementaryType)
        )
    );

    // «RawIdentifier» = «IdentifierStart» { «IdentifierPart» } ;
    define_token!(
        RawIdentifier,
        scan_make_node!(scan_seq!(
            scan_terminal!(|&c: &char| c == '_'
                || c == '$'
                || ('a' <= c && c <= 'z')
                || ('A' <= c && c <= 'Z')),
            scan_zero_or_more!(scan_terminal!(|&c: &char| c == '_'
                || c == '$'
                || ('a' <= c && c <= 'z')
                || ('A' <= c && c <= 'Z')
                || ('0' <= c && c <= '9')))
        ))
    );

    // ReceiveFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'virtual' ;
    define_rule!(
        ReceiveFunctionAttribute,
        choice!(
            rule!(ModifierInvocation),
            rule!(OverrideSpecifier),
            with_trivia!(choice!(
                just("external").to(TokenKind::External),
                just("payable").to(TokenKind::Payable),
                just("virtual").to(TokenKind::Virtual)
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // ReceiveFunctionDefinition = 'receive' ParameterList { ReceiveFunctionAttribute } ( ';' | Block ) ;
    define_rule!(
        ReceiveFunctionDefinition,
        seq!(
            with_trivia!(just("receive")
                .to(TokenKind::Receive)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(ParameterList),
            zero_or_more!(rule!(ReceiveFunctionAttribute)),
            choice!(
                with_trivia!(just(";")
                    .to(TokenKind::Semicolon)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Block)
            )
        )
    );

    // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
    define_token!(
        ReservedKeyword,
        scan_make_node!(choice!(
            just("a").ignore_then(choice!(
                just("fter").ignored(),
                just("lias").ignored(),
                just("pply").ignored(),
                just("uto").ignored()
            )),
            just("byte").ignored(),
            just("c").ignore_then(choice!(just("ase").ignored(), just("opyof").ignored())),
            just("def").ignore_then(choice!(just("ault").ignored(), just("ine").ignored())),
            just("final").ignored(),
            just("i").ignore_then(choice!(
                just("mplements").ignored(),
                just("n").ignore_then(choice!(just("line").ignored(), empty()))
            )),
            just("let").ignored(),
            just("m").ignore_then(choice!(
                just("a").ignore_then(choice!(just("cro").ignored(), just("tch").ignored())),
                just("utable").ignored()
            )),
            just("null").ignored(),
            just("of").ignored(),
            just("p").ignore_then(choice!(just("artial").ignored(), just("romise").ignored())),
            just("re").ignore_then(choice!(
                just("ference").ignored(),
                just("locatable").ignored()
            )),
            just("s").ignore_then(choice!(
                just("ealed").ignored(),
                just("izeof").ignored(),
                just("tatic").ignored(),
                just("upports").ignored(),
                just("witch").ignored()
            )),
            just("type").ignore_then(choice!(just("def").ignored(), just("of").ignored())),
            just("var").ignored()
        ))
    );

    // ReturnStatement = 'return' [ Expression ] ';' ;
    define_rule!(
        ReturnStatement,
        seq!(
            with_trivia!(just("return")
                .to(TokenKind::Return)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            optional!(rule!(Expression)),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
    define_rule!(
        RevertStatement,
        seq!(
            with_trivia!(just("revert")
                .to(TokenKind::Revert)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            optional!(rule!(IdentifierPath)),
            rule!(ArgumentList),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
    define_rule!(
        SelectedImport,
        seq!(
            token!(Identifier),
            optional!(seq!(
                with_trivia!(just("as")
                    .to(TokenKind::As)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                token!(Identifier)
            ))
        )
    );

    // SelectingImportDirective = '{' SelectedImport  { ',' SelectedImport } '}' 'from' ImportPath ;
    define_rule!(
        SelectingImportDirective,
        seq!(
            delimited_by!(
                terminal!(OpenBrace, "{"),
                separated_by!(rule!(SelectedImport), terminal!(Comma, ",")),
                terminal!(CloseBrace, "}")
            ),
            with_trivia!(just("from")
                .to(TokenKind::From)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(ImportPath)
        )
    );

    // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
    define_rule!(
        ShiftExpression,
        seq!(
            rule!(Expression),
            with_trivia!(choice!(
                just("<<").to(TokenKind::LessLess),
                just(">>").ignore_then(choice!(
                    just(">").to(TokenKind::GreaterGreaterGreater),
                    empty().to(TokenKind::GreaterGreater)
                ))
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
    define_token!(
        SignedFixedType,
        scan_make_node!(scan_seq!(
            just("fixed").ignored(),
            scan_optional!(scan_seq!(
                scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
                scan_terminal!('x'),
                scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9')))
            ))
        ))
    );

    // «SignedIntegerType» = 'int' [ '8' | '16' | '24' | '32' | '40' | '48' | '56' | '64' | '72' | '80' | '88' | '96' | '104' | '112' | '120' | '128' | '136' | '144' | '152' | '160' | '168' | '176' | '184' | '192' | '200' | '208' | '216' | '224' | '232' | '240' | '248' | '256' ] ;
    define_token!(
        SignedIntegerType,
        scan_make_node!(scan_seq!(
            just("int").ignored(),
            scan_optional!(choice!(
                just("1").ignore_then(choice!(
                    just("04").ignored(),
                    just("12").ignored(),
                    just("2").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                    just("36").ignored(),
                    just("44").ignored(),
                    just("52").ignored(),
                    just("6").ignore_then(choice!(
                        just("0").ignored(),
                        just("8").ignored(),
                        empty()
                    )),
                    just("76").ignored(),
                    just("84").ignored(),
                    just("92").ignored()
                )),
                just("2").ignore_then(choice!(
                    just("0").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                    just("16").ignored(),
                    just("24").ignored(),
                    just("32").ignored(),
                    just("4").ignore_then(choice!(
                        just("0").ignored(),
                        just("8").ignored(),
                        empty()
                    )),
                    just("56").ignored()
                )),
                just("32").ignored(),
                just("4").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                just("56").ignored(),
                just("64").ignored(),
                just("72").ignored(),
                just("8").ignore_then(choice!(just("0").ignored(), just("8").ignored(), empty())),
                just("96").ignored()
            ))
        ))
    );

    // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
    define_rule!(
        SimpleImportDirective,
        seq!(
            rule!(ImportPath),
            zero_or_more!(seq!(
                with_trivia!(just("as")
                    .to(TokenKind::As)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                token!(Identifier)
            ))
        )
    );

    // SimpleStatement = TupleDeconstructionStatement | VariableDeclarationStatement | ExpressionStatement ;
    define_rule!(
        SimpleStatement,
        choice!(
            rule!(TupleDeconstructionStatement),
            rule!(VariableDeclarationStatement),
            rule!(ExpressionStatement)
        )
    );

    // «SingleLineComment» = '//' { ¬( '\u{d}' | '\u{a}' ) } ;
    define_token!(
        SingleLineComment,
        scan_make_node!(scan_seq!(
            just("//").ignored(),
            scan_zero_or_more!(scan_terminal!(|&c: &char| c != '\r' && c != '\n'))
        ))
    );

    // «SingleQuotedAsciiStringLiteral» = '\'' { 1…*{ '\u{20}'…'~' - ( '\'' | '\\' ) } | «EscapeSequence» } '\'' ;
    define_token!(
        SingleQuotedAsciiStringLiteral,
        scan_make_node!(scan_seq!(
            scan_terminal!("'"),
            scan_zero_or_more!(scan_choice!(
                scan_one_or_more!(scan_terminal!(|&c: &char| (' ' <= c && c <= '~')
                    && c != '\''
                    && c != '\\')),
                scan_seq!(
                    scan_terminal!('\\'),
                    scan_choice!(
                        choice!(
                            just("\n").ignored(),
                            just("\r").ignored(),
                            just("\"").ignored(),
                            just("'").ignored(),
                            just("\\").ignored(),
                            just("n").ignored(),
                            just("r").ignored(),
                            just("t").ignored()
                        ),
                        scan_seq!(
                            scan_terminal!('x'),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                2usize,
                                2usize
                            )
                        ),
                        scan_seq!(
                            scan_terminal!('u'),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                4usize,
                                4usize
                            )
                        )
                    )
                )
            )),
            scan_terminal!("'")
        ))
    );

    // «SingleQuotedUnicodeStringLiteral» = 'unicode\'' { 1…*{ ¬( '\'' | '\\' | '\u{a}' | '\u{d}' ) } | «EscapeSequence» } '\'' ;
    define_token!(
        SingleQuotedUnicodeStringLiteral,
        scan_make_node!(scan_seq!(
            scan_terminal!("unicode'"),
            scan_zero_or_more!(scan_choice!(
                scan_one_or_more!(scan_terminal!(|&c: &char| c != '\''
                    && c != '\\'
                    && c != '\n'
                    && c != '\r')),
                scan_seq!(
                    scan_terminal!('\\'),
                    scan_choice!(
                        choice!(
                            just("\n").ignored(),
                            just("\r").ignored(),
                            just("\"").ignored(),
                            just("'").ignored(),
                            just("\\").ignored(),
                            just("n").ignored(),
                            just("r").ignored(),
                            just("t").ignored()
                        ),
                        scan_seq!(
                            scan_terminal!('x'),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                2usize,
                                2usize
                            )
                        ),
                        scan_seq!(
                            scan_terminal!('u'),
                            scan_repeated!(
                                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                    || ('a' <= c && c <= 'f')
                                    || ('A' <= c && c <= 'F')),
                                4usize,
                                4usize
                            )
                        )
                    )
                )
            )),
            scan_terminal!("'")
        ))
    );

    // SourceUnit = LeadingTrivia { Directive | Definition } EndOfFileTrivia ;
    define_rule!(
        SourceUnit,
        seq!(
            rule!(LeadingTrivia),
            zero_or_more!(choice!(rule!(Directive), rule!(Definition))),
            rule!(EndOfFileTrivia)
        )
    );

    // StarImportDirective = '*' 'as' «Identifier» 'from' ImportPath ;
    define_rule!(
        StarImportDirective,
        seq!(
            with_trivia!(just("*")
                .to(TokenKind::Star)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            with_trivia!(just("as")
                .to(TokenKind::As)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            with_trivia!(just("from")
                .to(TokenKind::From)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(ImportPath)
        )
    );

    // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
    define_rule!(
        StateVariableAttribute,
        choice!(
            rule!(OverrideSpecifier),
            with_trivia!(choice!(
                just("constant").to(TokenKind::Constant),
                just("i").ignore_then(choice!(
                    just("mmutable").to(TokenKind::Immutable),
                    just("nternal").to(TokenKind::Internal)
                )),
                just("p").ignore_then(choice!(
                    just("rivate").to(TokenKind::Private),
                    just("ublic").to(TokenKind::Public)
                ))
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // StateVariableDeclaration = TypeName { StateVariableAttribute } «Identifier» [ '=' Expression ] ';' ;
    define_rule!(
        StateVariableDeclaration,
        seq!(
            rule!(TypeName),
            zero_or_more!(rule!(StateVariableAttribute)),
            token!(Identifier),
            optional!(seq!(
                with_trivia!(just("=")
                    .to(TokenKind::Equal)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Expression)
            )),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // Statement = Block | SimpleStatement | IfStatement | ForStatement | WhileStatement | DoWhileStatement | ContinueStatement | BreakStatement | TryStatement | ReturnStatement | EmitStatement | RevertStatement | DeleteStatement | AssemblyStatement ;
    define_rule!(
        Statement,
        choice!(
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
            with_trivia!(just("struct")
                .to(TokenKind::Struct)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            delimited_by!(
                terminal!(OpenBrace, "{"),
                one_or_more!(rule!(StructMember)),
                terminal!(CloseBrace, "}")
            )
        )
    );

    // StructMember = TypeName «Identifier» ';' ;
    define_rule!(
        StructMember,
        seq!(
            rule!(TypeName),
            token!(Identifier),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // TrailingTrivia = [ { «Whitespace» | «MultilineComment» } ( «EndOfLine» | «SingleLineComment» ) ] ;
    define_rule!(
        TrailingTrivia,
        optional!(seq!(
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
            with_trivia!(just("try")
                .to(TokenKind::Try)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression),
            optional!(seq!(
                with_trivia!(just("returns")
                    .to(TokenKind::Returns)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(ParameterList)
            )),
            rule!(Block),
            one_or_more!(rule!(CatchClause))
        )
    );

    // TupleDeconstructionStatement = '(' [ [ TypeName [ DataLocation ] «Identifier» | [ DataLocation ] «Identifier» ]  { ',' [ TypeName [ DataLocation ] «Identifier» | [ DataLocation ] «Identifier» ] } ] ')' '=' Expression ';' ;
    define_rule!(
        TupleDeconstructionStatement,
        seq!(
            delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(
                    optional!(choice!(
                        seq!(
                            rule!(TypeName),
                            optional!(rule!(DataLocation)),
                            token!(Identifier)
                        ),
                        seq!(optional!(rule!(DataLocation)), token!(Identifier))
                    )),
                    terminal!(Comma, ",")
                )),
                terminal!(CloseParen, ")")
            ),
            with_trivia!(just("=")
                .to(TokenKind::Equal)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // TypeExpression = 'type' '(' TypeName ')' ;
    define_rule!(
        TypeExpression,
        seq!(
            with_trivia!(just("type")
                .to(TokenKind::Type)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            delimited_by!(
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
            choice!(
                rule!(ElementaryType),
                rule!(FunctionType),
                rule!(MappingType),
                rule!(IdentifierPath)
            ),
            zero_or_more!(delimited_by!(
                terminal!(OpenBracket, "["),
                optional!(rule!(Expression)),
                terminal!(CloseBracket, "]")
            ))
        )
    );

    // UnaryPrefixExpression = ( '++' | '--' | '!' | '~' | '-' ) Expression ;
    define_rule!(
        UnaryPrefixExpression,
        seq!(
            with_trivia!(choice!(
                just("!").to(TokenKind::Bang),
                just("++").to(TokenKind::PlusPlus),
                just("-").ignore_then(choice!(
                    just("-").to(TokenKind::MinusMinus),
                    empty().to(TokenKind::Minus)
                )),
                just("~").to(TokenKind::Tilde)
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Expression)
        )
    );

    // UnarySuffixExpression = Expression ( '++' | '--' ) ;
    define_rule!(
        UnarySuffixExpression,
        seq!(
            rule!(Expression),
            with_trivia!(choice!(
                just("++").to(TokenKind::PlusPlus),
                just("--").to(TokenKind::MinusMinus)
            )
            .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // UncheckedBlock = 'unchecked' Block ;
    define_rule!(
        UncheckedBlock,
        seq!(
            with_trivia!(just("unchecked")
                .to(TokenKind::Unchecked)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(Block)
        )
    );

    // «UnicodeEscape» = 'u' 4…4*{ «HexCharacter» } ;
    define_token!(
        UnicodeEscape,
        scan_make_node!(scan_seq!(
            scan_terminal!('u'),
            scan_repeated!(
                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                    || ('a' <= c && c <= 'f')
                    || ('A' <= c && c <= 'F')),
                4usize,
                4usize
            )
        ))
    );

    // «UnicodeStringLiteral» = «SingleQuotedUnicodeStringLiteral» | «DoubleQuotedUnicodeStringLiteral» ;
    define_token!(
        UnicodeStringLiteral,
        scan_make_node!(scan_choice!(
            scan_seq!(
                scan_terminal!("unicode'"),
                scan_zero_or_more!(scan_choice!(
                    scan_one_or_more!(scan_terminal!(|&c: &char| c != '\''
                        && c != '\\'
                        && c != '\n'
                        && c != '\r')),
                    scan_seq!(
                        scan_terminal!('\\'),
                        scan_choice!(
                            choice!(
                                just("\n").ignored(),
                                just("\r").ignored(),
                                just("\"").ignored(),
                                just("'").ignored(),
                                just("\\").ignored(),
                                just("n").ignored(),
                                just("r").ignored(),
                                just("t").ignored()
                            ),
                            scan_seq!(
                                scan_terminal!('x'),
                                scan_repeated!(
                                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                        || ('a' <= c && c <= 'f')
                                        || ('A' <= c && c <= 'F')),
                                    2usize,
                                    2usize
                                )
                            ),
                            scan_seq!(
                                scan_terminal!('u'),
                                scan_repeated!(
                                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                        || ('a' <= c && c <= 'f')
                                        || ('A' <= c && c <= 'F')),
                                    4usize,
                                    4usize
                                )
                            )
                        )
                    )
                )),
                scan_terminal!("'")
            ),
            scan_seq!(
                scan_terminal!("unicode\""),
                scan_zero_or_more!(scan_choice!(
                    scan_one_or_more!(scan_terminal!(|&c: &char| c != '"'
                        && c != '\\'
                        && c != '\n'
                        && c != '\r')),
                    scan_seq!(
                        scan_terminal!('\\'),
                        scan_choice!(
                            choice!(
                                just("\n").ignored(),
                                just("\r").ignored(),
                                just("\"").ignored(),
                                just("'").ignored(),
                                just("\\").ignored(),
                                just("n").ignored(),
                                just("r").ignored(),
                                just("t").ignored()
                            ),
                            scan_seq!(
                                scan_terminal!('x'),
                                scan_repeated!(
                                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                        || ('a' <= c && c <= 'f')
                                        || ('A' <= c && c <= 'F')),
                                    2usize,
                                    2usize
                                )
                            ),
                            scan_seq!(
                                scan_terminal!('u'),
                                scan_repeated!(
                                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                                        || ('a' <= c && c <= 'f')
                                        || ('A' <= c && c <= 'F')),
                                    4usize,
                                    4usize
                                )
                            )
                        )
                    )
                )),
                scan_terminal!("\"")
            )
        ))
    );

    // «UnsignedFixedType» = 'u' «SignedFixedType» ;
    define_token!(
        UnsignedFixedType,
        scan_make_node!(scan_seq!(
            scan_terminal!('u'),
            scan_seq!(
                just("fixed").ignored(),
                scan_optional!(scan_seq!(
                    scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))),
                    scan_terminal!('x'),
                    scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9')))
                ))
            )
        ))
    );

    // «UnsignedIntegerType» = 'u' «SignedIntegerType» ;
    define_token!(
        UnsignedIntegerType,
        scan_make_node!(scan_seq!(
            scan_terminal!('u'),
            scan_seq!(
                just("int").ignored(),
                scan_optional!(choice!(
                    just("1").ignore_then(choice!(
                        just("04").ignored(),
                        just("12").ignored(),
                        just("2").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                        just("36").ignored(),
                        just("44").ignored(),
                        just("52").ignored(),
                        just("6").ignore_then(choice!(
                            just("0").ignored(),
                            just("8").ignored(),
                            empty()
                        )),
                        just("76").ignored(),
                        just("84").ignored(),
                        just("92").ignored()
                    )),
                    just("2").ignore_then(choice!(
                        just("0").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                        just("16").ignored(),
                        just("24").ignored(),
                        just("32").ignored(),
                        just("4").ignore_then(choice!(
                            just("0").ignored(),
                            just("8").ignored(),
                            empty()
                        )),
                        just("56").ignored()
                    )),
                    just("32").ignored(),
                    just("4").ignore_then(choice!(just("0").ignored(), just("8").ignored())),
                    just("56").ignored(),
                    just("64").ignored(),
                    just("72").ignored(),
                    just("8").ignore_then(choice!(
                        just("0").ignored(),
                        just("8").ignored(),
                        empty()
                    )),
                    just("96").ignored()
                ))
            )
        ))
    );

    // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
    define_rule!(
        UserDefinedValueTypeDefinition,
        seq!(
            with_trivia!(just("type")
                .to(TokenKind::Type)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(Identifier),
            with_trivia!(just("is")
                .to(TokenKind::Is)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(ElementaryType),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
    define_rule!(
        UsingDirective,
        seq!(
            with_trivia!(just("using")
                .to(TokenKind::Using)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            choice!(
                rule!(IdentifierPath),
                delimited_by!(
                    terminal!(OpenBrace, "{"),
                    separated_by!(rule!(IdentifierPath), terminal!(Comma, ",")),
                    terminal!(CloseBrace, "}")
                )
            ),
            with_trivia!(just("for")
                .to(TokenKind::For)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            choice!(
                with_trivia!(just("*")
                    .to(TokenKind::Star)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(TypeName)
            ),
            optional!(with_trivia!(just("global")
                .to(TokenKind::Global)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
    define_rule!(
        VariableDeclarationStatement,
        seq!(
            rule!(TypeName),
            optional!(rule!(DataLocation)),
            token!(Identifier),
            optional!(seq!(
                with_trivia!(just("=")
                    .to(TokenKind::Equal)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(Expression)
            )),
            with_trivia!(just(";")
                .to(TokenKind::Semicolon)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            })
        )
    );

    // VersionPragma = 'solidity' 1…*{ VersionPragmaSpecifier } ;
    define_rule!(
        VersionPragma,
        seq!(
            with_trivia!(just("solidity")
                .to(TokenKind::Solidity)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            one_or_more!(rule!(VersionPragmaSpecifier))
        )
    );

    // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
    define_token!(
        VersionPragmaOperator,
        scan_make_node!(choice!(
            just("<").ignore_then(choice!(just("=").ignored(), empty())),
            just("=").ignored(),
            just(">").ignore_then(choice!(just("=").ignored(), empty())),
            just("^").ignored(),
            just("~").ignored()
        ))
    );

    // VersionPragmaSpecifier = [ «VersionPragmaOperator» ] «VersionPragmaValue»  { '.' «VersionPragmaValue» } ;
    define_rule!(
        VersionPragmaSpecifier,
        seq!(
            optional!(token!(VersionPragmaOperator)),
            separated_by!(token!(VersionPragmaValue), terminal!(Period, "."))
        )
    );

    // «VersionPragmaValue» = 1…*{ '0'…'9' | 'x' | 'X' | '*' } ;
    define_token!(
        VersionPragmaValue,
        scan_make_node!(scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c
            && c <= '9')
            || c == 'x'
            || c == 'X'
            || c == '*')))
    );

    // WhileStatement = 'while' '(' Expression ')' Statement ;
    define_rule!(
        WhileStatement,
        seq!(
            with_trivia!(just("while")
                .to(TokenKind::While)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            delimited_by!(
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
        scan_make_node!(scan_one_or_more!(scan_terminal!(
            |&c: &char| c == ' ' || c == '\t'
        )))
    );

    // YulAssignmentStatement = YulIdentifierPath  { ',' YulIdentifierPath } ':=' YulExpression ;
    define_rule!(
        YulAssignmentStatement,
        seq!(
            separated_by!(rule!(YulIdentifierPath), terminal!(Comma, ",")),
            with_trivia!(just(":=")
                .to(TokenKind::ColonEqual)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(YulExpression)
        )
    );

    // YulBlock = '{' { YulStatement } '}' ;
    define_rule!(
        YulBlock,
        delimited_by!(
            terminal!(OpenBrace, "{"),
            zero_or_more!(rule!(YulStatement)),
            terminal!(CloseBrace, "}")
        )
    );

    // YulBreakStatement = 'break' ;
    define_rule!(
        YulBreakStatement,
        with_trivia!(just("break")
            .to(TokenKind::Break)
            .map_with_span(|kind, span: SpanType| (kind, span)))
        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
            cst::Node::token(
                kind,
                lex::Node::chars_unwrapped(range),
                leading_trivia,
                trailing_trivia,
            )
        })
    );

    // YulContinueStatement = 'continue' ;
    define_rule!(
        YulContinueStatement,
        with_trivia!(just("continue")
            .to(TokenKind::Continue)
            .map_with_span(|kind, span: SpanType| (kind, span)))
        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
            cst::Node::token(
                kind,
                lex::Node::chars_unwrapped(range),
                leading_trivia,
                trailing_trivia,
            )
        })
    );

    // «YulDecimalLiteral» = '0' | '1'…'9' { '0'…'9' } ;
    define_token!(
        YulDecimalLiteral,
        scan_make_node!(scan_choice!(
            just("0").ignored(),
            scan_seq!(
                scan_terminal!(|&c: &char| ('1' <= c && c <= '9')),
                scan_zero_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9')))
            )
        ))
    );

    // YulExpression = YulFunctionCall | YulLiteral ;
    define_rule!(
        YulExpression,
        choice!(rule!(YulFunctionCall), rule!(YulLiteral))
    );

    // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
    define_rule!(
        YulForStatement,
        seq!(
            with_trivia!(just("for")
                .to(TokenKind::For)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(YulBlock),
            rule!(YulExpression),
            rule!(YulBlock),
            rule!(YulBlock)
        )
    );

    // YulFunctionCall = YulIdentifierPath [ '(' [ YulExpression  { ',' YulExpression } ] ')' ] ;
    define_rule!(
        YulFunctionCall,
        seq!(
            rule!(YulIdentifierPath),
            optional!(delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(rule!(YulExpression), terminal!(Comma, ","))),
                terminal!(CloseParen, ")")
            ))
        )
    );

    // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
    define_rule!(
        YulFunctionDefinition,
        seq!(
            with_trivia!(just("function")
                .to(TokenKind::Function)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            token!(YulIdentifier),
            delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(
                    Arguments,
                    token!(YulIdentifier),
                    terminal!(Comma, ",")
                )),
                terminal!(CloseParen, ")")
            ),
            optional!(seq!(
                with_trivia!(just("->")
                    .to(TokenKind::MinusGreater)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                separated_by!(Results, token!(YulIdentifier), terminal!(Comma, ","))
            )),
            rule!(YulBlock)
        )
    );

    // «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
    define_token!(
        YulHexLiteral,
        scan_make_node!(scan_seq!(
            just("0x").ignored(),
            scan_one_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9')
                || ('a' <= c && c <= 'f')
                || ('A' <= c && c <= 'F')))
        ))
    );

    // «YulIdentifier» = «RawIdentifier» - «YulKeyword» ;
    define_token!(
        YulIdentifier,
        scan_make_node!(difference(
            scan_seq!(
                scan_terminal!(|&c: &char| c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z')),
                scan_zero_or_more!(scan_terminal!(|&c: &char| c == '_'
                    || c == '$'
                    || ('a' <= c && c <= 'z')
                    || ('A' <= c && c <= 'Z')
                    || ('0' <= c && c <= '9')))
            ),
            choice!(
                just("break").ignored(),
                just("c").ignore_then(choice!(just("ase").ignored(), just("ontinue").ignored())),
                just("default").ignored(),
                just("f").ignore_then(choice!(
                    just("alse").ignored(),
                    just("or").ignored(),
                    just("unction").ignored()
                )),
                just("hex").ignored(),
                just("if").ignored(),
                just("le").ignore_then(choice!(just("ave").ignored(), just("t").ignored())),
                just("switch").ignored(),
                just("true").ignored()
            )
        ))
    );

    // YulIdentifierPath = «YulIdentifier»  { '.' «YulIdentifier» } ;
    define_rule!(
        YulIdentifierPath,
        separated_by!(token!(YulIdentifier), terminal!(Period, "."))
    );

    // YulIfStatement = 'if' YulExpression YulBlock ;
    define_rule!(
        YulIfStatement,
        seq!(
            with_trivia!(just("if")
                .to(TokenKind::If)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(YulExpression),
            rule!(YulBlock)
        )
    );

    // «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
    define_token!(
        YulKeyword,
        scan_make_node!(choice!(
            just("break").ignored(),
            just("c").ignore_then(choice!(just("ase").ignored(), just("ontinue").ignored())),
            just("default").ignored(),
            just("f").ignore_then(choice!(
                just("alse").ignored(),
                just("or").ignored(),
                just("unction").ignored()
            )),
            just("hex").ignored(),
            just("if").ignored(),
            just("le").ignore_then(choice!(just("ave").ignored(), just("t").ignored())),
            just("switch").ignored(),
            just("true").ignored()
        ))
    );

    // YulLeaveStatement = 'leave' ;
    define_rule!(
        YulLeaveStatement,
        with_trivia!(just("leave")
            .to(TokenKind::Leave)
            .map_with_span(|kind, span: SpanType| (kind, span)))
        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
            cst::Node::token(
                kind,
                lex::Node::chars_unwrapped(range),
                leading_trivia,
                trailing_trivia,
            )
        })
    );

    // YulLiteral = «BooleanLiteral» | «YulHexLiteral» | «YulDecimalLiteral» | «HexStringLiteral» | «AsciiStringLiteral» ;
    define_rule!(
        YulLiteral,
        choice!(
            token!(BooleanLiteral),
            token!(YulHexLiteral),
            token!(YulDecimalLiteral),
            token!(HexStringLiteral),
            token!(AsciiStringLiteral)
        )
    );

    // YulStatement = YulBlock | YulVariableDeclaration | YulFunctionDefinition | YulAssignmentStatement | YulFunctionCall | YulIfStatement | YulForStatement | YulSwitchStatement | YulLeaveStatement | YulBreakStatement | YulContinueStatement ;
    define_rule!(
        YulStatement,
        choice!(
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
            with_trivia!(just("switch")
                .to(TokenKind::Switch)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            rule!(YulExpression),
            one_or_more!(seq!(
                choice!(
                    seq!(
                        with_trivia!(just("case")
                            .to(TokenKind::Case)
                            .map_with_span(|kind, span: SpanType| (kind, span)))
                        .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                            cst::Node::token(
                                kind,
                                lex::Node::chars_unwrapped(range),
                                leading_trivia,
                                trailing_trivia,
                            )
                        }),
                        rule!(YulLiteral)
                    ),
                    with_trivia!(just("default")
                        .to(TokenKind::Default)
                        .map_with_span(|kind, span: SpanType| (kind, span)))
                    .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                        cst::Node::token(
                            kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        )
                    })
                ),
                rule!(YulBlock)
            ))
        )
    );

    // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
    define_rule!(
        YulVariableDeclaration,
        seq!(
            with_trivia!(just("let")
                .to(TokenKind::Let)
                .map_with_span(|kind, span: SpanType| (kind, span)))
            .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                cst::Node::token(
                    kind,
                    lex::Node::chars_unwrapped(range),
                    leading_trivia,
                    trailing_trivia,
                )
            }),
            separated_by!(rule!(YulIdentifierPath), terminal!(Comma, ",")),
            optional!(seq!(
                with_trivia!(just(":=")
                    .to(TokenKind::ColonEqual)
                    .map_with_span(|kind, span: SpanType| (kind, span)))
                .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                    cst::Node::token(
                        kind,
                        lex::Node::chars_unwrapped(range),
                        leading_trivia,
                        trailing_trivia,
                    )
                }),
                rule!(YulExpression)
            ))
        )
    );

    // Return the Parsers object ------------------------

    parsers
}
