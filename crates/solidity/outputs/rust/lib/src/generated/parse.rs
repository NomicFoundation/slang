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
    macro_rules ! scan_seq { ($ head : expr , $ ($ tail : expr) , +) => { $ head . then_ignore (scan_seq ! ($ ($ tail) , +)) } ; ($ head : expr) => { $ head . ignored () } ; }
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
    macro_rules ! scan_trie { ($ expr : expr) => { $ expr } ; ($ ($ expr : expr) , +) => { choice :: < _ , ErrorType > (($ ($ expr) , +)) } ; }
    #[allow(unused_macros)]
    macro_rules! scan_trieleaf {
        ($ string : literal) => {
            just($string).ignored()
        };
        () => {
            empty()
        };
    }
    #[allow(unused_macros)]
    macro_rules ! scan_trieprefix { ($ string : literal , [$ ($ expr : expr) , +]) => (just ($ string) . ignore_then (choice :: < _ , ErrorType > (($ ($ expr) , +)))) }
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
    macro_rules ! lex_trie { ($ expr : expr) => { $ expr . map_with_span (| _ , span : SpanType | lex :: Node :: chars (span)) } ; ($ ($ expr : expr) , +) => { choice :: < _ , ErrorType > (($ ($ expr) , +)) . map_with_span (| _ , span : SpanType | lex :: Node :: chars (span)) } ; }
    #[allow(unused_macros)]
    macro_rules! lex_trieleaf {
        ($ kind : ident , $ string : literal) => {
            just($string).to(TokenKind::$kind)
        };
        ($ kind : ident) => {
            empty().to(TokenKind::$kind)
        };
    }
    #[allow(unused_macros)]
    macro_rules ! lex_trieprefix { ($ string : literal , [$ ($ expr : expr) , +]) => (just ($ string) . ignore_then (choice :: < _ , ErrorType > (($ ($ expr) , +)))) }
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
    enum Pratt {
        Operator {
            kind: RuleKind,
            node: Option<Rc<cst::Node>>,
            left_binding_power: u8,
            right_binding_power: u8,
        },
        Node(Option<Rc<cst::Node>>),
    }
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
    macro_rules ! trivia_token { ($ token_rule : ident) => { $ token_rule . clone () . map (| token : Option < Rc < lex :: Node >> | { let token = token . unwrap () ; if let lex :: Node :: Named (kind , element) = token . as_ref () { cst :: Node :: trivia_token (* kind , element . clone ()) } else { unreachable ! ("a token rule should always return a named token, but rule {} returned {:?}" , stringify ! ($ token_rule) , token) } }) } ; }
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
    macro_rules ! trie { ($ kind : ident , $ ($ expr : expr) , *) => { trie ! ($ ($ expr) , *) . map (| child | cst :: Node :: rule (RuleKind :: $ kind , vec ! [child])) } ; ($ expr : expr) => { LeadingTrivia . clone () . then ($ expr . map_with_span (| kind , span : SpanType | (kind , span))) . then (TrailingTrivia . clone ()) . map (| ((leading_trivia , (kind , range)) , trailing_trivia) | { cst :: Node :: token (kind , lex :: Node :: chars_unwrapped (range) , leading_trivia , trailing_trivia) }) } ; ($ ($ expr : expr) , +) => { LeadingTrivia . clone () . then (choice :: < _ , ErrorType > (($ ($ expr) , +)) . map_with_span (| kind , span : SpanType | (kind , span))) . then (TrailingTrivia . clone ()) . map (| ((leading_trivia , (kind , range)) , trailing_trivia) | { cst :: Node :: token (kind , lex :: Node :: chars_unwrapped (range) , leading_trivia , trailing_trivia) }) } ; }
    #[allow(unused_macros)]
    macro_rules ! trivia_trie { ($ expr : expr) => { $ expr . map_with_span (| kind , span : SpanType | cst :: Node :: trivia_token (kind , lex :: Node :: chars_unwrapped (span))) } ; ($ ($ expr : expr) , +) => { choice :: < _ , ErrorType > (($ ($ expr) , +)) . map_with_span (| kind , span : SpanType | cst :: Node :: trivia_token (kind , lex :: Node :: chars_unwrapped (span))) } ; }
    #[allow(unused_macros)]
    macro_rules ! trieprefix { ($ string : literal , [$ ($ expr : expr) , +]) => (just ($ string) . ignore_then (choice :: < _ , ErrorType > (($ ($ expr) , +)))) }
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
        seq!(trie!(trieleaf!(Abicoder, "abicoder")), token!(Identifier))
    );

    // AddSubExpression = Expression ( '+' | '-' ) Expression ;
    define_rule!(
        AddSubExpression,
        seq!(
            rule!(Expression),
            trie!(trieleaf!(Plus, "+"), trieleaf!(Minus, "-")),
            rule!(Expression)
        )
    );

    // AddressType = 'address' [ 'payable' ] ;
    define_rule!(
        AddressType,
        seq!(
            trie!(trieleaf!(Address, "address")),
            optional!(trie!(trieleaf!(Payable, "payable")))
        )
    );

    // AndExpression = Expression '&&' Expression ;
    define_rule!(
        AndExpression,
        seq!(
            rule!(Expression),
            trie!(trieleaf!(AmpersandAmpersand, "&&")),
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
                            scan_trie!(
                                scan_trieleaf!("\n"),
                                scan_trieleaf!("\r"),
                                scan_trieleaf!("\""),
                                scan_trieleaf!("'"),
                                scan_trieleaf!("\\"),
                                scan_trieleaf!("n"),
                                scan_trieleaf!("r"),
                                scan_trieleaf!("t")
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
                            scan_trie!(
                                scan_trieleaf!("\n"),
                                scan_trieleaf!("\r"),
                                scan_trieleaf!("\""),
                                scan_trieleaf!("'"),
                                scan_trieleaf!("\\"),
                                scan_trieleaf!("n"),
                                scan_trieleaf!("r"),
                                scan_trieleaf!("t")
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
            trie!(trieleaf!(Assembly, "assembly")),
            optional!(trie!(trieleaf!(DoubleQuoteEvmasmDoubleQuote, "\"evmasm\""))),
            optional!(rule!(AssemblyFlags)),
            rule!(YulBlock)
        )
    );

    // AssignmentExpression = Expression ( '=' | '|=' | '^=' | '&=' | '<<=' | '>>=' | '>>>=' | '+=' | '-=' | '*=' | '/=' | '%=' ) Expression ;
    define_rule!(
        AssignmentExpression,
        seq!(
            rule!(Expression),
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
            ),
            rule!(Expression)
        )
    );

    // BitAndExpression = Expression '&' Expression ;
    define_rule!(
        BitAndExpression,
        seq!(
            rule!(Expression),
            trie!(trieleaf!(Ampersand, "&")),
            rule!(Expression)
        )
    );

    // BitOrExpression = Expression '|' Expression ;
    define_rule!(
        BitOrExpression,
        seq!(
            rule!(Expression),
            trie!(trieleaf!(Pipe, "|")),
            rule!(Expression)
        )
    );

    // BitXOrExpression = Expression '^' Expression ;
    define_rule!(
        BitXOrExpression,
        seq!(
            rule!(Expression),
            trie!(trieleaf!(Caret, "^")),
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
        scan_make_node!(scan_trie!(scan_trieleaf!("false"), scan_trieleaf!("true")))
    );

    // BreakStatement = 'break' ';' ;
    define_rule!(
        BreakStatement,
        seq!(
            trie!(trieleaf!(Break, "break")),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // CatchClause = 'catch' [ [ «Identifier» ] ParameterList ] Block ;
    define_rule!(
        CatchClause,
        seq!(
            trie!(trieleaf!(Catch, "catch")),
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
                trie!(trieleaf!(Question, "?")),
                rule!(Expression),
                trie!(trieleaf!(Colon, ":")),
                rule!(Expression)
            )
        )
    );

    // ConstantDefinition = TypeName 'constant' «Identifier» '=' Expression ';' ;
    define_rule!(
        ConstantDefinition,
        seq!(
            rule!(TypeName),
            trie!(trieleaf!(Constant, "constant")),
            token!(Identifier),
            trie!(trieleaf!(Equal, "=")),
            rule!(Expression),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // ConstructorAttribute = ModifierInvocation | 'internal' | 'payable' | 'public' ;
    define_rule!(
        ConstructorAttribute,
        choice!(
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
            trie!(trieleaf!(Constructor, "constructor")),
            rule!(ParameterList),
            zero_or_more!(rule!(ConstructorAttribute)),
            rule!(Block)
        )
    );

    // ContinueStatement = 'continue' ';' ;
    define_rule!(
        ContinueStatement,
        seq!(
            trie!(trieleaf!(Continue, "continue")),
            trie!(trieleaf!(Semicolon, ";"))
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
            optional!(trie!(trieleaf!(Abstract, "abstract"))),
            trie!(trieleaf!(Contract, "contract")),
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
        trie!(
            trieleaf!(Calldata, "calldata"),
            trieleaf!(Memory, "memory"),
            trieleaf!(Storage, "storage")
        )
    );

    // «DecimalExponent» = ( 'e' | 'E' ) [ '-' ] «DecimalInteger» ;
    define_token!(
        DecimalExponent,
        scan_make_node!(scan_seq!(
            scan_terminal!(|&c: &char| c == 'e' || c == 'E'),
            scan_optional!(scan_terminal!('-')),
            scan_seq!(
                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')),
                scan_zero_or_more!(scan_seq!(
                    scan_optional!(scan_terminal!('_')),
                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))
                ))
            )
        ))
    );

    // «DecimalFloat» = [ «DecimalInteger» ] '.' «DecimalInteger» ;
    define_token!(
        DecimalFloat,
        scan_make_node!(scan_seq!(
            scan_optional!(scan_seq!(
                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')),
                scan_zero_or_more!(scan_seq!(
                    scan_optional!(scan_terminal!('_')),
                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))
                ))
            )),
            scan_terminal!('.'),
            scan_seq!(
                scan_terminal!(|&c: &char| ('0' <= c && c <= '9')),
                scan_zero_or_more!(scan_seq!(
                    scan_optional!(scan_terminal!('_')),
                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))
                ))
            )
        ))
    );

    // «DecimalInteger» = '0'…'9' { [ '_' ] '0'…'9' } ;
    define_token!(
        DecimalInteger,
        scan_make_node!(scan_seq!(
            scan_terminal!(|&c: &char| ('0' <= c && c <= '9')),
            scan_zero_or_more!(scan_seq!(
                scan_optional!(scan_terminal!('_')),
                scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))
            ))
        ))
    );

    // «DecimalNumber» = ( «DecimalInteger» | «DecimalFloat» ) [ «DecimalExponent» ] ;
    define_token!(
        DecimalNumber,
        scan_make_node!(scan_seq!(
            scan_choice!(
                scan_seq!(
                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')),
                    scan_zero_or_more!(scan_seq!(
                        scan_optional!(scan_terminal!('_')),
                        scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))
                    ))
                ),
                scan_seq!(
                    scan_optional!(scan_seq!(
                        scan_terminal!(|&c: &char| ('0' <= c && c <= '9')),
                        scan_zero_or_more!(scan_seq!(
                            scan_optional!(scan_terminal!('_')),
                            scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))
                        ))
                    )),
                    scan_terminal!('.'),
                    scan_seq!(
                        scan_terminal!(|&c: &char| ('0' <= c && c <= '9')),
                        scan_zero_or_more!(scan_seq!(
                            scan_optional!(scan_terminal!('_')),
                            scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))
                        ))
                    )
                )
            ),
            scan_optional!(scan_seq!(
                scan_terminal!(|&c: &char| c == 'e' || c == 'E'),
                scan_optional!(scan_terminal!('-')),
                scan_seq!(
                    scan_terminal!(|&c: &char| ('0' <= c && c <= '9')),
                    scan_zero_or_more!(scan_seq!(
                        scan_optional!(scan_terminal!('_')),
                        scan_terminal!(|&c: &char| ('0' <= c && c <= '9'))
                    ))
                )
            ))
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
            trie!(trieleaf!(Delete, "delete")),
            rule!(Expression),
            trie!(trieleaf!(Semicolon, ";"))
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
            trie!(trieleaf!(Do, "do")),
            rule!(Statement),
            trie!(trieleaf!(While, "while")),
            delimited_by!(
                terminal!(OpenParen, "("),
                rule!(Expression),
                terminal!(CloseParen, ")")
            ),
            trie!(trieleaf!(Semicolon, ";"))
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
                        scan_trie!(
                            scan_trieleaf!("\n"),
                            scan_trieleaf!("\r"),
                            scan_trieleaf!("\""),
                            scan_trieleaf!("'"),
                            scan_trieleaf!("\\"),
                            scan_trieleaf!("n"),
                            scan_trieleaf!("r"),
                            scan_trieleaf!("t")
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
                        scan_trie!(
                            scan_trieleaf!("\n"),
                            scan_trieleaf!("\r"),
                            scan_trieleaf!("\""),
                            scan_trieleaf!("'"),
                            scan_trieleaf!("\\"),
                            scan_trieleaf!("n"),
                            scan_trieleaf!("r"),
                            scan_trieleaf!("t")
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

    // ElementaryType = 'bool' | 'string' | AddressType | «FixedBytesType» | «SignedIntegerType» | «UnsignedIntegerType» | «SignedFixedType» | «UnsignedFixedType» ;
    define_rule!(
        ElementaryType,
        choice!(
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
            trie!(trieleaf!(Emit, "emit")),
            rule!(IdentifierPath),
            rule!(ArgumentList),
            trie!(trieleaf!(Semicolon, ";"))
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
            trie!(trieleaf!(Enum, "enum")),
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
            trie!(trieleaf!(BangEqual, "!="), trieleaf!(EqualEqual, "==")),
            rule!(Expression)
        )
    );

    // ErrorDefinition = 'error' «Identifier» '(' [ ErrorParameter  { ',' ErrorParameter } ] ')' ';' ;
    define_rule!(
        ErrorDefinition,
        seq!(
            trie!(trieleaf!(Error, "error")),
            token!(Identifier),
            delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(rule!(ErrorParameter), terminal!(Comma, ","))),
                terminal!(CloseParen, ")")
            ),
            trie!(trieleaf!(Semicolon, ";"))
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
                scan_trie!(
                    scan_trieleaf!("\n"),
                    scan_trieleaf!("\r"),
                    scan_trieleaf!("\""),
                    scan_trieleaf!("'"),
                    scan_trieleaf!("\\"),
                    scan_trieleaf!("n"),
                    scan_trieleaf!("r"),
                    scan_trieleaf!("t")
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
            trie!(trieleaf!(Event, "event")),
            token!(Identifier),
            delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(rule!(EventParameter), terminal!(Comma, ","))),
                terminal!(CloseParen, ")")
            ),
            optional!(trie!(trieleaf!(Anonymous, "anonymous"))),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // EventParameter = TypeName [ 'indexed' ] [ «Identifier» ] ;
    define_rule!(
        EventParameter,
        seq!(
            rule!(TypeName),
            optional!(trie!(trieleaf!(Indexed, "indexed"))),
            optional!(token!(Identifier))
        )
    );

    // ExperimentalPragma = 'experimental' «Identifier» ;
    define_rule!(
        ExperimentalPragma,
        seq!(
            trie!(trieleaf!(Experimental, "experimental")),
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
                trie!(trieleaf!(StarStar, "**")),
                rule!(Expression)
            )
            .boxed()
        } else {
            seq!(
                rule!(Expression),
                trie!(trieleaf!(StarStar, "**")),
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
                let prefix_operators = trie!(
                    trieleaf!(Bang, "!"),
                    trieleaf!(PlusPlus, "++"),
                    trieprefix!("-", [trieleaf!(MinusMinus, "-"), trieleaf!(Minus)]),
                    trieleaf!(Tilde, "~")
                )
                .map(|node| Pratt::Operator {
                    node,
                    kind: RuleKind::UnaryPrefixExpression,
                    left_binding_power: 255,
                    right_binding_power: 29u8,
                });
                let suffix_operators = choice!(
                    delimited_by!(
                        terminal!(OpenBracket, "["),
                        seq!(
                            optional!(rule!(Expression)),
                            optional!(seq!(
                                trie!(trieleaf!(Colon, ":")),
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
                    }),
                    seq!(
                        trie!(trieleaf!(Period, ".")),
                        choice!(token!(Identifier), trie!(trieleaf!(Address, "address")))
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::MemberAccessExpression,
                        left_binding_power: 33u8,
                        right_binding_power: 255
                    }),
                    seq!(
                        optional!(delimited_by!(
                            terminal!(OpenBrace, "{"),
                            separated_by!(rule!(NamedArgument), terminal!(Comma, ",")),
                            terminal!(CloseBrace, "}")
                        )),
                        rule!(ArgumentList)
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::FunctionCallExpression,
                        left_binding_power: 31u8,
                        right_binding_power: 255
                    }),
                    trie!(trieleaf!(PlusPlus, "++"), trieleaf!(MinusMinus, "--")).map(|node| {
                        Pratt::Operator {
                            node,
                            kind: RuleKind::UnarySuffixExpression,
                            left_binding_power: 27u8,
                            right_binding_power: 255,
                        }
                    }),
                    seq!(
                        trie!(trieleaf!(Question, "?")),
                        rule!(Expression),
                        trie!(trieleaf!(Colon, ":")),
                        rule!(Expression)
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::ConditionalExpression,
                        left_binding_power: 3u8,
                        right_binding_power: 255
                    })
                );
                let primary = rule!(PrimaryExpression);
                let prefixes_primary_suffixes = prefix_operators
                    .repeated()
                    .then(primary)
                    .then(suffix_operators.repeated());
                type PPS = ((Vec<Pratt>, Option<Rc<cst::Node>>), Vec<Pratt>);
                let binary_operator = choice!(
                    trie!(trieleaf!(StarStar, "**")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::ExponentiationExpression,
                        left_binding_power: 25u8 + 1,
                        right_binding_power: 25u8
                    }),
                    trie!(
                        trieleaf!(Percent, "%"),
                        trieleaf!(Star, "*"),
                        trieleaf!(Slash, "/")
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::MulDivModExpression,
                        left_binding_power: 23u8,
                        right_binding_power: 23u8 + 1
                    }),
                    trie!(trieleaf!(Plus, "+"), trieleaf!(Minus, "-")).map(|node| {
                        Pratt::Operator {
                            node,
                            kind: RuleKind::AddSubExpression,
                            left_binding_power: 21u8,
                            right_binding_power: 21u8 + 1,
                        }
                    }),
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
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::ShiftExpression,
                        left_binding_power: 19u8,
                        right_binding_power: 19u8 + 1
                    }),
                    trie!(trieleaf!(Ampersand, "&")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::BitAndExpression,
                        left_binding_power: 17u8,
                        right_binding_power: 17u8 + 1
                    }),
                    trie!(trieleaf!(Caret, "^")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::BitXOrExpression,
                        left_binding_power: 15u8,
                        right_binding_power: 15u8 + 1
                    }),
                    trie!(trieleaf!(Pipe, "|")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::BitOrExpression,
                        left_binding_power: 13u8,
                        right_binding_power: 13u8 + 1
                    }),
                    trie!(
                        trieprefix!("<", [trieleaf!(LessEqual, "="), trieleaf!(Less)]),
                        trieprefix!(">", [trieleaf!(GreaterEqual, "="), trieleaf!(Greater)])
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::OrderComparisonExpression,
                        left_binding_power: 11u8,
                        right_binding_power: 11u8 + 1
                    }),
                    trie!(trieleaf!(BangEqual, "!="), trieleaf!(EqualEqual, "==")).map(|node| {
                        Pratt::Operator {
                            node,
                            kind: RuleKind::EqualityComparisonExpression,
                            left_binding_power: 9u8,
                            right_binding_power: 9u8 + 1,
                        }
                    }),
                    trie!(trieleaf!(AmpersandAmpersand, "&&")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::AndExpression,
                        left_binding_power: 7u8,
                        right_binding_power: 7u8 + 1
                    }),
                    trie!(trieleaf!(PipePipe, "||")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::OrExpression,
                        left_binding_power: 5u8,
                        right_binding_power: 5u8 + 1
                    }),
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
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::AssignmentExpression,
                        left_binding_power: 1u8,
                        right_binding_power: 1u8 + 1
                    })
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
                        for (op, pps) in tail.into_iter() {
                            elements.push(op);
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
                                        i = 0;
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
                                        i = 0;
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
                                        i = 0;
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
                let prefix_operators = trie!(
                    trieleaf!(Bang, "!"),
                    trieleaf!(PlusPlus, "++"),
                    trieprefix!("-", [trieleaf!(MinusMinus, "-"), trieleaf!(Minus)]),
                    trieleaf!(Tilde, "~")
                )
                .map(|node| Pratt::Operator {
                    node,
                    kind: RuleKind::UnaryPrefixExpression,
                    left_binding_power: 255,
                    right_binding_power: 29u8,
                });
                let suffix_operators = choice!(
                    delimited_by!(
                        terminal!(OpenBracket, "["),
                        seq!(
                            optional!(rule!(Expression)),
                            optional!(seq!(
                                trie!(trieleaf!(Colon, ":")),
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
                    }),
                    seq!(
                        trie!(trieleaf!(Period, ".")),
                        choice!(token!(Identifier), trie!(trieleaf!(Address, "address")))
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::MemberAccessExpression,
                        left_binding_power: 33u8,
                        right_binding_power: 255
                    }),
                    seq!(
                        optional!(delimited_by!(
                            terminal!(OpenBrace, "{"),
                            separated_by!(rule!(NamedArgument), terminal!(Comma, ",")),
                            terminal!(CloseBrace, "}")
                        )),
                        rule!(ArgumentList)
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::FunctionCallExpression,
                        left_binding_power: 31u8,
                        right_binding_power: 255
                    }),
                    trie!(trieleaf!(PlusPlus, "++"), trieleaf!(MinusMinus, "--")).map(|node| {
                        Pratt::Operator {
                            node,
                            kind: RuleKind::UnarySuffixExpression,
                            left_binding_power: 27u8,
                            right_binding_power: 255,
                        }
                    }),
                    seq!(
                        trie!(trieleaf!(Question, "?")),
                        rule!(Expression),
                        trie!(trieleaf!(Colon, ":")),
                        rule!(Expression)
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::ConditionalExpression,
                        left_binding_power: 3u8,
                        right_binding_power: 255
                    })
                );
                let primary = rule!(PrimaryExpression);
                let prefixes_primary_suffixes = prefix_operators
                    .repeated()
                    .then(primary)
                    .then(suffix_operators.repeated());
                type PPS = ((Vec<Pratt>, Option<Rc<cst::Node>>), Vec<Pratt>);
                let binary_operator = choice!(
                    trie!(trieleaf!(StarStar, "**")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::ExponentiationExpression,
                        left_binding_power: 25u8,
                        right_binding_power: 25u8 + 1
                    }),
                    trie!(
                        trieleaf!(Percent, "%"),
                        trieleaf!(Star, "*"),
                        trieleaf!(Slash, "/")
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::MulDivModExpression,
                        left_binding_power: 23u8,
                        right_binding_power: 23u8 + 1
                    }),
                    trie!(trieleaf!(Plus, "+"), trieleaf!(Minus, "-")).map(|node| {
                        Pratt::Operator {
                            node,
                            kind: RuleKind::AddSubExpression,
                            left_binding_power: 21u8,
                            right_binding_power: 21u8 + 1,
                        }
                    }),
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
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::ShiftExpression,
                        left_binding_power: 19u8,
                        right_binding_power: 19u8 + 1
                    }),
                    trie!(trieleaf!(Ampersand, "&")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::BitAndExpression,
                        left_binding_power: 17u8,
                        right_binding_power: 17u8 + 1
                    }),
                    trie!(trieleaf!(Caret, "^")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::BitXOrExpression,
                        left_binding_power: 15u8,
                        right_binding_power: 15u8 + 1
                    }),
                    trie!(trieleaf!(Pipe, "|")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::BitOrExpression,
                        left_binding_power: 13u8,
                        right_binding_power: 13u8 + 1
                    }),
                    trie!(
                        trieprefix!("<", [trieleaf!(LessEqual, "="), trieleaf!(Less)]),
                        trieprefix!(">", [trieleaf!(GreaterEqual, "="), trieleaf!(Greater)])
                    )
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::OrderComparisonExpression,
                        left_binding_power: 11u8,
                        right_binding_power: 11u8 + 1
                    }),
                    trie!(trieleaf!(BangEqual, "!="), trieleaf!(EqualEqual, "==")).map(|node| {
                        Pratt::Operator {
                            node,
                            kind: RuleKind::EqualityComparisonExpression,
                            left_binding_power: 9u8,
                            right_binding_power: 9u8 + 1,
                        }
                    }),
                    trie!(trieleaf!(AmpersandAmpersand, "&&")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::AndExpression,
                        left_binding_power: 7u8,
                        right_binding_power: 7u8 + 1
                    }),
                    trie!(trieleaf!(PipePipe, "||")).map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::OrExpression,
                        left_binding_power: 5u8,
                        right_binding_power: 5u8 + 1
                    }),
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
                    .map(|node| Pratt::Operator {
                        node,
                        kind: RuleKind::AssignmentExpression,
                        left_binding_power: 1u8,
                        right_binding_power: 1u8 + 1
                    })
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
                        for (op, pps) in tail.into_iter() {
                            elements.push(op);
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
                                        i = 0;
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
                                        i = 0;
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
                                        i = 0;
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
        seq!(rule!(Expression), trie!(trieleaf!(Semicolon, ";")))
    );

    // FallbackFunctionAttribute = ModifierInvocation | OverrideSpecifier | 'external' | 'payable' | 'pure' | 'view' | 'virtual' ;
    define_rule!(
        FallbackFunctionAttribute,
        choice!(
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
            trie!(trieleaf!(Fallback, "fallback")),
            rule!(ParameterList),
            zero_or_more!(rule!(FallbackFunctionAttribute)),
            optional!(seq!(
                trie!(trieleaf!(Returns, "returns")),
                rule!(ParameterList)
            )),
            choice!(trie!(trieleaf!(Semicolon, ";")), rule!(Block))
        )
    );

    // «FixedBytesType» = 'bytes' ( '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '12' | '13' | '14' | '15' | '16' | '17' | '18' | '19' | '20' | '21' | '22' | '23' | '24' | '25' | '26' | '27' | '28' | '29' | '30' | '31' | '32' ) ;
    define_token!(
        FixedBytesType,
        scan_make_node!(scan_seq!(
            scan_trie!(scan_trieleaf!("bytes")),
            scan_trie!(
                scan_trieprefix!(
                    "1",
                    [
                        scan_trieleaf!("0"),
                        scan_trieleaf!("1"),
                        scan_trieleaf!("2"),
                        scan_trieleaf!("3"),
                        scan_trieleaf!("4"),
                        scan_trieleaf!("5"),
                        scan_trieleaf!("6"),
                        scan_trieleaf!("7"),
                        scan_trieleaf!("8"),
                        scan_trieleaf!("9"),
                        scan_trieleaf!()
                    ]
                ),
                scan_trieprefix!(
                    "2",
                    [
                        scan_trieleaf!("0"),
                        scan_trieleaf!("1"),
                        scan_trieleaf!("2"),
                        scan_trieleaf!("3"),
                        scan_trieleaf!("4"),
                        scan_trieleaf!("5"),
                        scan_trieleaf!("6"),
                        scan_trieleaf!("7"),
                        scan_trieleaf!("8"),
                        scan_trieleaf!("9"),
                        scan_trieleaf!()
                    ]
                ),
                scan_trieprefix!(
                    "3",
                    [
                        scan_trieleaf!("0"),
                        scan_trieleaf!("1"),
                        scan_trieleaf!("2"),
                        scan_trieleaf!()
                    ]
                ),
                scan_trieleaf!("4"),
                scan_trieleaf!("5"),
                scan_trieleaf!("6"),
                scan_trieleaf!("7"),
                scan_trieleaf!("8"),
                scan_trieleaf!("9")
            )
        ))
    );

    // ForStatement = 'for' '(' ( SimpleStatement | ';' ) ( ExpressionStatement | ';' ) [ Expression ] ')' Statement ;
    define_rule!(
        ForStatement,
        seq!(
            trie!(trieleaf!(For, "for")),
            delimited_by!(
                terminal!(OpenParen, "("),
                seq!(
                    choice!(rule!(SimpleStatement), trie!(trieleaf!(Semicolon, ";"))),
                    choice!(rule!(ExpressionStatement), trie!(trieleaf!(Semicolon, ";"))),
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
        seq!(
            rule!(Expression),
            optional!(delimited_by!(
                terminal!(OpenBrace, "{"),
                separated_by!(rule!(NamedArgument), terminal!(Comma, ",")),
                terminal!(CloseBrace, "}")
            )),
            rule!(ArgumentList)
        )
    );

    // FunctionDefinition = 'function' ( «Identifier» | 'fallback' | 'receive' ) ParameterList { FunctionAttribute } [ 'returns' ParameterList ] ( ';' | Block ) ;
    define_rule!(
        FunctionDefinition,
        seq!(
            trie!(trieleaf!(Function, "function")),
            choice!(
                token!(Identifier),
                trie!(
                    trieleaf!(Fallback, "fallback"),
                    trieleaf!(Receive, "receive")
                )
            ),
            rule!(ParameterList),
            zero_or_more!(rule!(FunctionAttribute)),
            optional!(seq!(
                trie!(trieleaf!(Returns, "returns")),
                rule!(ParameterList)
            )),
            choice!(trie!(trieleaf!(Semicolon, ";")), rule!(Block))
        )
    );

    // FunctionType = 'function' ParameterList { 'internal' | 'external' | 'private' | 'public' | 'pure' | 'view' | 'payable' } [ 'returns' ParameterList ] ;
    define_rule!(
        FunctionType,
        seq!(
            trie!(trieleaf!(Function, "function")),
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
            optional!(seq!(
                trie!(trieleaf!(Returns, "returns")),
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

    // «HexNumber» = '0x' 1…*{ «HexCharacter» }  { '_' 1…*{ «HexCharacter» } } ;
    define_token!(
        HexNumber,
        scan_make_node!(scan_seq!(
            scan_trie!(scan_trieleaf!("0x")),
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
            scan_trie!(scan_trieleaf!("hex")),
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
                scan_trie!(scan_trieleaf!("false"), scan_trieleaf!("true")),
                scan_seq!(
                    scan_trie!(scan_trieleaf!("bytes")),
                    scan_trie!(
                        scan_trieprefix!(
                            "1",
                            [
                                scan_trieleaf!("0"),
                                scan_trieleaf!("1"),
                                scan_trieleaf!("2"),
                                scan_trieleaf!("3"),
                                scan_trieleaf!("4"),
                                scan_trieleaf!("5"),
                                scan_trieleaf!("6"),
                                scan_trieleaf!("7"),
                                scan_trieleaf!("8"),
                                scan_trieleaf!("9"),
                                scan_trieleaf!()
                            ]
                        ),
                        scan_trieprefix!(
                            "2",
                            [
                                scan_trieleaf!("0"),
                                scan_trieleaf!("1"),
                                scan_trieleaf!("2"),
                                scan_trieleaf!("3"),
                                scan_trieleaf!("4"),
                                scan_trieleaf!("5"),
                                scan_trieleaf!("6"),
                                scan_trieleaf!("7"),
                                scan_trieleaf!("8"),
                                scan_trieleaf!("9"),
                                scan_trieleaf!()
                            ]
                        ),
                        scan_trieprefix!(
                            "3",
                            [
                                scan_trieleaf!("0"),
                                scan_trieleaf!("1"),
                                scan_trieleaf!("2"),
                                scan_trieleaf!()
                            ]
                        ),
                        scan_trieleaf!("4"),
                        scan_trieleaf!("5"),
                        scan_trieleaf!("6"),
                        scan_trieleaf!("7"),
                        scan_trieleaf!("8"),
                        scan_trieleaf!("9")
                    )
                ),
                scan_trie!(
                    scan_trieprefix!(
                        "a",
                        [
                            scan_trieleaf!("fter"),
                            scan_trieleaf!("lias"),
                            scan_trieleaf!("pply"),
                            scan_trieleaf!("uto")
                        ]
                    ),
                    scan_trieleaf!("byte"),
                    scan_trieprefix!("c", [scan_trieleaf!("ase"), scan_trieleaf!("opyof")]),
                    scan_trieprefix!(
                        "d",
                        [
                            scan_trieleaf!("ays"),
                            scan_trieprefix!("ef", [scan_trieleaf!("ault"), scan_trieleaf!("ine")])
                        ]
                    ),
                    scan_trieleaf!("ether"),
                    scan_trieprefix!("fin", [scan_trieleaf!("al"), scan_trieleaf!("ney")]),
                    scan_trieleaf!("gwei"),
                    scan_trieleaf!("hours"),
                    scan_trieprefix!(
                        "i",
                        [
                            scan_trieleaf!("mplements"),
                            scan_trieprefix!("n", [scan_trieleaf!("line"), scan_trieleaf!()])
                        ]
                    ),
                    scan_trieleaf!("let"),
                    scan_trieprefix!(
                        "m",
                        [
                            scan_trieprefix!("a", [scan_trieleaf!("cro"), scan_trieleaf!("tch")]),
                            scan_trieleaf!("inutes"),
                            scan_trieleaf!("utable")
                        ]
                    ),
                    scan_trieleaf!("null"),
                    scan_trieleaf!("of"),
                    scan_trieprefix!("p", [scan_trieleaf!("artial"), scan_trieleaf!("romise")]),
                    scan_trieprefix!(
                        "re",
                        [scan_trieleaf!("ference"), scan_trieleaf!("locatable")]
                    ),
                    scan_trieprefix!(
                        "s",
                        [
                            scan_trieprefix!(
                                "e",
                                [scan_trieleaf!("aled"), scan_trieleaf!("conds")]
                            ),
                            scan_trieleaf!("izeof"),
                            scan_trieleaf!("tatic"),
                            scan_trieleaf!("upports"),
                            scan_trieleaf!("witch"),
                            scan_trieleaf!("zabo")
                        ]
                    ),
                    scan_trieprefix!("type", [scan_trieleaf!("def"), scan_trieleaf!("of")]),
                    scan_trieleaf!("var"),
                    scan_trieprefix!("we", [scan_trieleaf!("eks"), scan_trieleaf!("i")]),
                    scan_trieleaf!("years")
                ),
                scan_seq!(
                    scan_trie!(scan_trieleaf!("int")),
                    scan_optional!(scan_trie!(
                        scan_trieprefix!(
                            "1",
                            [
                                scan_trieleaf!("04"),
                                scan_trieleaf!("12"),
                                scan_trieprefix!("2", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                                scan_trieleaf!("36"),
                                scan_trieleaf!("44"),
                                scan_trieleaf!("52"),
                                scan_trieprefix!(
                                    "6",
                                    [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                                ),
                                scan_trieleaf!("76"),
                                scan_trieleaf!("84"),
                                scan_trieleaf!("92")
                            ]
                        ),
                        scan_trieprefix!(
                            "2",
                            [
                                scan_trieprefix!("0", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                                scan_trieleaf!("16"),
                                scan_trieleaf!("24"),
                                scan_trieleaf!("32"),
                                scan_trieprefix!(
                                    "4",
                                    [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                                ),
                                scan_trieleaf!("56")
                            ]
                        ),
                        scan_trieleaf!("32"),
                        scan_trieprefix!("4", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                        scan_trieleaf!("56"),
                        scan_trieleaf!("64"),
                        scan_trieleaf!("72"),
                        scan_trieprefix!(
                            "8",
                            [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                        ),
                        scan_trieleaf!("96")
                    ))
                ),
                scan_seq!(
                    scan_terminal!('u'),
                    scan_seq!(
                        scan_trie!(scan_trieleaf!("int")),
                        scan_optional!(scan_trie!(
                            scan_trieprefix!(
                                "1",
                                [
                                    scan_trieleaf!("04"),
                                    scan_trieleaf!("12"),
                                    scan_trieprefix!(
                                        "2",
                                        [scan_trieleaf!("0"), scan_trieleaf!("8")]
                                    ),
                                    scan_trieleaf!("36"),
                                    scan_trieleaf!("44"),
                                    scan_trieleaf!("52"),
                                    scan_trieprefix!(
                                        "6",
                                        [
                                            scan_trieleaf!("0"),
                                            scan_trieleaf!("8"),
                                            scan_trieleaf!()
                                        ]
                                    ),
                                    scan_trieleaf!("76"),
                                    scan_trieleaf!("84"),
                                    scan_trieleaf!("92")
                                ]
                            ),
                            scan_trieprefix!(
                                "2",
                                [
                                    scan_trieprefix!(
                                        "0",
                                        [scan_trieleaf!("0"), scan_trieleaf!("8")]
                                    ),
                                    scan_trieleaf!("16"),
                                    scan_trieleaf!("24"),
                                    scan_trieleaf!("32"),
                                    scan_trieprefix!(
                                        "4",
                                        [
                                            scan_trieleaf!("0"),
                                            scan_trieleaf!("8"),
                                            scan_trieleaf!()
                                        ]
                                    ),
                                    scan_trieleaf!("56")
                                ]
                            ),
                            scan_trieleaf!("32"),
                            scan_trieprefix!("4", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                            scan_trieleaf!("56"),
                            scan_trieleaf!("64"),
                            scan_trieleaf!("72"),
                            scan_trieprefix!(
                                "8",
                                [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                            ),
                            scan_trieleaf!("96")
                        ))
                    )
                ),
                scan_trie!(
                    scan_trieprefix!(
                        "a",
                        [
                            scan_trieleaf!("bstract"),
                            scan_trieleaf!("ddress"),
                            scan_trieleaf!("nonymous"),
                            scan_trieprefix!("s", [scan_trieleaf!("sembly"), scan_trieleaf!()])
                        ]
                    ),
                    scan_trieprefix!("b", [scan_trieleaf!("ool"), scan_trieleaf!("reak")]),
                    scan_trieprefix!(
                        "c",
                        [
                            scan_trieprefix!(
                                "a",
                                [scan_trieleaf!("lldata"), scan_trieleaf!("tch")]
                            ),
                            scan_trieprefix!(
                                "on",
                                [
                                    scan_trieprefix!(
                                        "st",
                                        [scan_trieleaf!("ant"), scan_trieleaf!("ructor")]
                                    ),
                                    scan_trieprefix!(
                                        "t",
                                        [scan_trieleaf!("inue"), scan_trieleaf!("ract")]
                                    )
                                ]
                            )
                        ]
                    ),
                    scan_trieprefix!("d", [scan_trieleaf!("elete"), scan_trieleaf!("o")]),
                    scan_trieprefix!(
                        "e",
                        [
                            scan_trieleaf!("lse"),
                            scan_trieleaf!("mit"),
                            scan_trieleaf!("num"),
                            scan_trieleaf!("vent"),
                            scan_trieleaf!("xternal")
                        ]
                    ),
                    scan_trieprefix!(
                        "f",
                        [
                            scan_trieprefix!("al", [scan_trieleaf!("lback"), scan_trieleaf!("se")]),
                            scan_trieleaf!("ixed"),
                            scan_trieleaf!("or"),
                            scan_trieleaf!("unction")
                        ]
                    ),
                    scan_trieleaf!("hex"),
                    scan_trieprefix!(
                        "i",
                        [
                            scan_trieleaf!("f"),
                            scan_trieprefix!(
                                "m",
                                [scan_trieleaf!("mutable"), scan_trieleaf!("port")]
                            ),
                            scan_trieprefix!(
                                "n",
                                [
                                    scan_trieleaf!("dexed"),
                                    scan_trieprefix!(
                                        "ter",
                                        [scan_trieleaf!("face"), scan_trieleaf!("nal")]
                                    )
                                ]
                            ),
                            scan_trieleaf!("s")
                        ]
                    ),
                    scan_trieleaf!("library"),
                    scan_trieprefix!(
                        "m",
                        [
                            scan_trieleaf!("apping"),
                            scan_trieleaf!("emory"),
                            scan_trieleaf!("odifier")
                        ]
                    ),
                    scan_trieleaf!("new"),
                    scan_trieleaf!("override"),
                    scan_trieprefix!(
                        "p",
                        [
                            scan_trieleaf!("ayable"),
                            scan_trieprefix!(
                                "r",
                                [scan_trieleaf!("agma"), scan_trieleaf!("ivate")]
                            ),
                            scan_trieprefix!("u", [scan_trieleaf!("blic"), scan_trieleaf!("re")])
                        ]
                    ),
                    scan_trieprefix!(
                        "re",
                        [
                            scan_trieleaf!("ceive"),
                            scan_trieprefix!("turn", [scan_trieleaf!("s"), scan_trieleaf!()])
                        ]
                    ),
                    scan_trieprefix!(
                        "st",
                        [
                            scan_trieleaf!("orage"),
                            scan_trieprefix!("r", [scan_trieleaf!("ing"), scan_trieleaf!("uct")])
                        ]
                    ),
                    scan_trieprefix!(
                        "t",
                        [
                            scan_trieprefix!("r", [scan_trieleaf!("ue"), scan_trieleaf!("y")]),
                            scan_trieleaf!("ype")
                        ]
                    ),
                    scan_trieprefix!(
                        "u",
                        [
                            scan_trieleaf!("fixed"),
                            scan_trieleaf!("nchecked"),
                            scan_trieleaf!("sing")
                        ]
                    ),
                    scan_trieprefix!("vi", [scan_trieleaf!("ew"), scan_trieleaf!("rtual")]),
                    scan_trieleaf!("while")
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
            trie!(trieleaf!(If, "if")),
            delimited_by!(
                terminal!(OpenParen, "("),
                rule!(Expression),
                terminal!(CloseParen, ")")
            ),
            rule!(Statement),
            optional!(seq!(trie!(trieleaf!(Else, "else")), rule!(Statement)))
        )
    );

    // ImportDirective = 'import' ( SimpleImportDirective | StarImportDirective | SelectingImportDirective ) ';' ;
    define_rule!(
        ImportDirective,
        seq!(
            trie!(trieleaf!(Import, "import")),
            choice!(
                rule!(SimpleImportDirective),
                rule!(StarImportDirective),
                rule!(SelectingImportDirective)
            ),
            trie!(trieleaf!(Semicolon, ";"))
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
                        trie!(trieleaf!(Colon, ":")),
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
            trie!(trieleaf!(Is, "is")),
            separated_by!(rule!(InheritanceSpecifier), terminal!(Comma, ","))
        )
    );

    // InterfaceDefinition = 'interface' «Identifier» [ InheritanceSpecifierList ] '{' { ContractBodyElement } '}' ;
    define_rule!(
        InterfaceDefinition,
        seq!(
            trie!(trieleaf!(Interface, "interface")),
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
            scan_trie!(scan_trieleaf!("false"), scan_trieleaf!("true")),
            scan_seq!(
                scan_trie!(scan_trieleaf!("bytes")),
                scan_trie!(
                    scan_trieprefix!(
                        "1",
                        [
                            scan_trieleaf!("0"),
                            scan_trieleaf!("1"),
                            scan_trieleaf!("2"),
                            scan_trieleaf!("3"),
                            scan_trieleaf!("4"),
                            scan_trieleaf!("5"),
                            scan_trieleaf!("6"),
                            scan_trieleaf!("7"),
                            scan_trieleaf!("8"),
                            scan_trieleaf!("9"),
                            scan_trieleaf!()
                        ]
                    ),
                    scan_trieprefix!(
                        "2",
                        [
                            scan_trieleaf!("0"),
                            scan_trieleaf!("1"),
                            scan_trieleaf!("2"),
                            scan_trieleaf!("3"),
                            scan_trieleaf!("4"),
                            scan_trieleaf!("5"),
                            scan_trieleaf!("6"),
                            scan_trieleaf!("7"),
                            scan_trieleaf!("8"),
                            scan_trieleaf!("9"),
                            scan_trieleaf!()
                        ]
                    ),
                    scan_trieprefix!(
                        "3",
                        [
                            scan_trieleaf!("0"),
                            scan_trieleaf!("1"),
                            scan_trieleaf!("2"),
                            scan_trieleaf!()
                        ]
                    ),
                    scan_trieleaf!("4"),
                    scan_trieleaf!("5"),
                    scan_trieleaf!("6"),
                    scan_trieleaf!("7"),
                    scan_trieleaf!("8"),
                    scan_trieleaf!("9")
                )
            ),
            scan_trie!(
                scan_trieprefix!(
                    "a",
                    [
                        scan_trieleaf!("fter"),
                        scan_trieleaf!("lias"),
                        scan_trieleaf!("pply"),
                        scan_trieleaf!("uto")
                    ]
                ),
                scan_trieleaf!("byte"),
                scan_trieprefix!("c", [scan_trieleaf!("ase"), scan_trieleaf!("opyof")]),
                scan_trieprefix!(
                    "d",
                    [
                        scan_trieleaf!("ays"),
                        scan_trieprefix!("ef", [scan_trieleaf!("ault"), scan_trieleaf!("ine")])
                    ]
                ),
                scan_trieleaf!("ether"),
                scan_trieprefix!("fin", [scan_trieleaf!("al"), scan_trieleaf!("ney")]),
                scan_trieleaf!("gwei"),
                scan_trieleaf!("hours"),
                scan_trieprefix!(
                    "i",
                    [
                        scan_trieleaf!("mplements"),
                        scan_trieprefix!("n", [scan_trieleaf!("line"), scan_trieleaf!()])
                    ]
                ),
                scan_trieleaf!("let"),
                scan_trieprefix!(
                    "m",
                    [
                        scan_trieprefix!("a", [scan_trieleaf!("cro"), scan_trieleaf!("tch")]),
                        scan_trieleaf!("inutes"),
                        scan_trieleaf!("utable")
                    ]
                ),
                scan_trieleaf!("null"),
                scan_trieleaf!("of"),
                scan_trieprefix!("p", [scan_trieleaf!("artial"), scan_trieleaf!("romise")]),
                scan_trieprefix!(
                    "re",
                    [scan_trieleaf!("ference"), scan_trieleaf!("locatable")]
                ),
                scan_trieprefix!(
                    "s",
                    [
                        scan_trieprefix!("e", [scan_trieleaf!("aled"), scan_trieleaf!("conds")]),
                        scan_trieleaf!("izeof"),
                        scan_trieleaf!("tatic"),
                        scan_trieleaf!("upports"),
                        scan_trieleaf!("witch"),
                        scan_trieleaf!("zabo")
                    ]
                ),
                scan_trieprefix!("type", [scan_trieleaf!("def"), scan_trieleaf!("of")]),
                scan_trieleaf!("var"),
                scan_trieprefix!("we", [scan_trieleaf!("eks"), scan_trieleaf!("i")]),
                scan_trieleaf!("years")
            ),
            scan_seq!(
                scan_trie!(scan_trieleaf!("int")),
                scan_optional!(scan_trie!(
                    scan_trieprefix!(
                        "1",
                        [
                            scan_trieleaf!("04"),
                            scan_trieleaf!("12"),
                            scan_trieprefix!("2", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                            scan_trieleaf!("36"),
                            scan_trieleaf!("44"),
                            scan_trieleaf!("52"),
                            scan_trieprefix!(
                                "6",
                                [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                            ),
                            scan_trieleaf!("76"),
                            scan_trieleaf!("84"),
                            scan_trieleaf!("92")
                        ]
                    ),
                    scan_trieprefix!(
                        "2",
                        [
                            scan_trieprefix!("0", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                            scan_trieleaf!("16"),
                            scan_trieleaf!("24"),
                            scan_trieleaf!("32"),
                            scan_trieprefix!(
                                "4",
                                [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                            ),
                            scan_trieleaf!("56")
                        ]
                    ),
                    scan_trieleaf!("32"),
                    scan_trieprefix!("4", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                    scan_trieleaf!("56"),
                    scan_trieleaf!("64"),
                    scan_trieleaf!("72"),
                    scan_trieprefix!(
                        "8",
                        [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                    ),
                    scan_trieleaf!("96")
                ))
            ),
            scan_seq!(
                scan_terminal!('u'),
                scan_seq!(
                    scan_trie!(scan_trieleaf!("int")),
                    scan_optional!(scan_trie!(
                        scan_trieprefix!(
                            "1",
                            [
                                scan_trieleaf!("04"),
                                scan_trieleaf!("12"),
                                scan_trieprefix!("2", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                                scan_trieleaf!("36"),
                                scan_trieleaf!("44"),
                                scan_trieleaf!("52"),
                                scan_trieprefix!(
                                    "6",
                                    [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                                ),
                                scan_trieleaf!("76"),
                                scan_trieleaf!("84"),
                                scan_trieleaf!("92")
                            ]
                        ),
                        scan_trieprefix!(
                            "2",
                            [
                                scan_trieprefix!("0", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                                scan_trieleaf!("16"),
                                scan_trieleaf!("24"),
                                scan_trieleaf!("32"),
                                scan_trieprefix!(
                                    "4",
                                    [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                                ),
                                scan_trieleaf!("56")
                            ]
                        ),
                        scan_trieleaf!("32"),
                        scan_trieprefix!("4", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                        scan_trieleaf!("56"),
                        scan_trieleaf!("64"),
                        scan_trieleaf!("72"),
                        scan_trieprefix!(
                            "8",
                            [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                        ),
                        scan_trieleaf!("96")
                    ))
                )
            ),
            scan_trie!(
                scan_trieprefix!(
                    "a",
                    [
                        scan_trieleaf!("bstract"),
                        scan_trieleaf!("ddress"),
                        scan_trieleaf!("nonymous"),
                        scan_trieprefix!("s", [scan_trieleaf!("sembly"), scan_trieleaf!()])
                    ]
                ),
                scan_trieprefix!("b", [scan_trieleaf!("ool"), scan_trieleaf!("reak")]),
                scan_trieprefix!(
                    "c",
                    [
                        scan_trieprefix!("a", [scan_trieleaf!("lldata"), scan_trieleaf!("tch")]),
                        scan_trieprefix!(
                            "on",
                            [
                                scan_trieprefix!(
                                    "st",
                                    [scan_trieleaf!("ant"), scan_trieleaf!("ructor")]
                                ),
                                scan_trieprefix!(
                                    "t",
                                    [scan_trieleaf!("inue"), scan_trieleaf!("ract")]
                                )
                            ]
                        )
                    ]
                ),
                scan_trieprefix!("d", [scan_trieleaf!("elete"), scan_trieleaf!("o")]),
                scan_trieprefix!(
                    "e",
                    [
                        scan_trieleaf!("lse"),
                        scan_trieleaf!("mit"),
                        scan_trieleaf!("num"),
                        scan_trieleaf!("vent"),
                        scan_trieleaf!("xternal")
                    ]
                ),
                scan_trieprefix!(
                    "f",
                    [
                        scan_trieprefix!("al", [scan_trieleaf!("lback"), scan_trieleaf!("se")]),
                        scan_trieleaf!("ixed"),
                        scan_trieleaf!("or"),
                        scan_trieleaf!("unction")
                    ]
                ),
                scan_trieleaf!("hex"),
                scan_trieprefix!(
                    "i",
                    [
                        scan_trieleaf!("f"),
                        scan_trieprefix!("m", [scan_trieleaf!("mutable"), scan_trieleaf!("port")]),
                        scan_trieprefix!(
                            "n",
                            [
                                scan_trieleaf!("dexed"),
                                scan_trieprefix!(
                                    "ter",
                                    [scan_trieleaf!("face"), scan_trieleaf!("nal")]
                                )
                            ]
                        ),
                        scan_trieleaf!("s")
                    ]
                ),
                scan_trieleaf!("library"),
                scan_trieprefix!(
                    "m",
                    [
                        scan_trieleaf!("apping"),
                        scan_trieleaf!("emory"),
                        scan_trieleaf!("odifier")
                    ]
                ),
                scan_trieleaf!("new"),
                scan_trieleaf!("override"),
                scan_trieprefix!(
                    "p",
                    [
                        scan_trieleaf!("ayable"),
                        scan_trieprefix!("r", [scan_trieleaf!("agma"), scan_trieleaf!("ivate")]),
                        scan_trieprefix!("u", [scan_trieleaf!("blic"), scan_trieleaf!("re")])
                    ]
                ),
                scan_trieprefix!(
                    "re",
                    [
                        scan_trieleaf!("ceive"),
                        scan_trieprefix!("turn", [scan_trieleaf!("s"), scan_trieleaf!()])
                    ]
                ),
                scan_trieprefix!(
                    "st",
                    [
                        scan_trieleaf!("orage"),
                        scan_trieprefix!("r", [scan_trieleaf!("ing"), scan_trieleaf!("uct")])
                    ]
                ),
                scan_trieprefix!(
                    "t",
                    [
                        scan_trieprefix!("r", [scan_trieleaf!("ue"), scan_trieleaf!("y")]),
                        scan_trieleaf!("ype")
                    ]
                ),
                scan_trieprefix!(
                    "u",
                    [
                        scan_trieleaf!("fixed"),
                        scan_trieleaf!("nchecked"),
                        scan_trieleaf!("sing")
                    ]
                ),
                scan_trieprefix!("vi", [scan_trieleaf!("ew"), scan_trieleaf!("rtual")]),
                scan_trieleaf!("while")
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
            trie!(trieleaf!(Library, "library")),
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
            trie!(trieleaf!(Mapping, "mapping")),
            delimited_by!(
                terminal!(OpenParen, "("),
                seq!(
                    choice!(rule!(ElementaryType), rule!(IdentifierPath)),
                    trie!(trieleaf!(EqualGreater, "=>")),
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
            trie!(trieleaf!(Period, ".")),
            choice!(token!(Identifier), trie!(trieleaf!(Address, "address")))
        )
    );

    // ModifierAttribute = OverrideSpecifier | 'virtual' ;
    define_rule!(
        ModifierAttribute,
        choice!(
            rule!(OverrideSpecifier),
            trie!(trieleaf!(Virtual, "virtual"))
        )
    );

    // ModifierDefinition = 'modifier' «Identifier» [ ParameterList ] { ModifierAttribute } ( ';' | Block ) ;
    define_rule!(
        ModifierDefinition,
        seq!(
            trie!(trieleaf!(Modifier, "modifier")),
            token!(Identifier),
            optional!(rule!(ParameterList)),
            zero_or_more!(rule!(ModifierAttribute)),
            choice!(trie!(trieleaf!(Semicolon, ";")), rule!(Block))
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
            trie!(
                trieleaf!(Percent, "%"),
                trieleaf!(Star, "*"),
                trieleaf!(Slash, "/")
            ),
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
            trie!(trieleaf!(Colon, ":")),
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
            trie!(trieleaf!(New, "new")),
            rule!(IdentifierPath),
            rule!(ArgumentList)
        )
    );

    // «NumberUnit» = 'days' | 'ether' | 'finney' | 'gwei' | 'hours' | 'minutes' | 'seconds' | 'szabo' | 'weeks' | 'wei' | 'years' ;
    define_token!(
        NumberUnit,
        scan_make_node!(scan_trie!(
            scan_trieleaf!("days"),
            scan_trieleaf!("ether"),
            scan_trieleaf!("finney"),
            scan_trieleaf!("gwei"),
            scan_trieleaf!("hours"),
            scan_trieleaf!("minutes"),
            scan_trieprefix!("s", [scan_trieleaf!("econds"), scan_trieleaf!("zabo")]),
            scan_trieprefix!("we", [scan_trieleaf!("eks"), scan_trieleaf!("i")]),
            scan_trieleaf!("years")
        ))
    );

    // NumericLiteral = ( «HexNumber» | «DecimalNumber» ) [ «NumberUnit» ] ;
    define_rule!(
        NumericLiteral,
        seq!(
            choice!(token!(HexNumber), token!(DecimalNumber)),
            optional!(token!(NumberUnit))
        )
    );

    // OrExpression = Expression '||' Expression ;
    define_rule!(
        OrExpression,
        seq!(
            rule!(Expression),
            trie!(trieleaf!(PipePipe, "||")),
            rule!(Expression)
        )
    );

    // OrderComparisonExpression = Expression ( '<' | '>' | '<=' | '>=' ) Expression ;
    define_rule!(
        OrderComparisonExpression,
        seq!(
            rule!(Expression),
            trie!(
                trieprefix!("<", [trieleaf!(LessEqual, "="), trieleaf!(Less)]),
                trieprefix!(">", [trieleaf!(GreaterEqual, "="), trieleaf!(Greater)])
            ),
            rule!(Expression)
        )
    );

    // OverrideSpecifier = 'override' [ '(' IdentifierPath  { ',' IdentifierPath } ')' ] ;
    define_rule!(
        OverrideSpecifier,
        seq!(
            trie!(trieleaf!(Override, "override")),
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

    // PayableExpression = 'payable' ArgumentList ;
    define_rule!(
        PayableExpression,
        seq!(trie!(trieleaf!(Payable, "payable")), rule!(ArgumentList))
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
            trie!(trieleaf!(Pragma, "pragma")),
            choice!(
                rule!(VersionPragma),
                rule!(ABICoderPragma),
                rule!(ExperimentalPragma)
            ),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // PrimaryExpression = PayableExpression | TypeExpression | NewExpression | ParenthesisExpression | ArrayLiteral | «AsciiStringLiteral» | «UnicodeStringLiteral» | «HexStringLiteral» | NumericLiteral | «BooleanLiteral» | «Identifier» ;
    define_rule!(
        PrimaryExpression,
        choice!(
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
            trie!(trieleaf!(Receive, "receive")),
            rule!(ParameterList),
            zero_or_more!(rule!(ReceiveFunctionAttribute)),
            choice!(trie!(trieleaf!(Semicolon, ";")), rule!(Block))
        )
    );

    // «ReservedKeyword» = 'after' | 'alias' | 'apply' | 'auto' | 'byte' | 'case' | 'copyof' | 'default' | 'define' | 'final' | 'implements' | 'in' | 'inline' | 'let' | 'macro' | 'match' | 'mutable' | 'null' | 'of' | 'partial' | 'promise' | 'reference' | 'relocatable' | 'sealed' | 'sizeof' | 'static' | 'supports' | 'switch' | 'typedef' | 'typeof' | 'var' ;
    define_token!(
        ReservedKeyword,
        scan_make_node!(scan_trie!(
            scan_trieprefix!(
                "a",
                [
                    scan_trieleaf!("fter"),
                    scan_trieleaf!("lias"),
                    scan_trieleaf!("pply"),
                    scan_trieleaf!("uto")
                ]
            ),
            scan_trieleaf!("byte"),
            scan_trieprefix!("c", [scan_trieleaf!("ase"), scan_trieleaf!("opyof")]),
            scan_trieprefix!("def", [scan_trieleaf!("ault"), scan_trieleaf!("ine")]),
            scan_trieleaf!("final"),
            scan_trieprefix!(
                "i",
                [
                    scan_trieleaf!("mplements"),
                    scan_trieprefix!("n", [scan_trieleaf!("line"), scan_trieleaf!()])
                ]
            ),
            scan_trieleaf!("let"),
            scan_trieprefix!(
                "m",
                [
                    scan_trieprefix!("a", [scan_trieleaf!("cro"), scan_trieleaf!("tch")]),
                    scan_trieleaf!("utable")
                ]
            ),
            scan_trieleaf!("null"),
            scan_trieleaf!("of"),
            scan_trieprefix!("p", [scan_trieleaf!("artial"), scan_trieleaf!("romise")]),
            scan_trieprefix!(
                "re",
                [scan_trieleaf!("ference"), scan_trieleaf!("locatable")]
            ),
            scan_trieprefix!(
                "s",
                [
                    scan_trieleaf!("ealed"),
                    scan_trieleaf!("izeof"),
                    scan_trieleaf!("tatic"),
                    scan_trieleaf!("upports"),
                    scan_trieleaf!("witch")
                ]
            ),
            scan_trieprefix!("type", [scan_trieleaf!("def"), scan_trieleaf!("of")]),
            scan_trieleaf!("var")
        ))
    );

    // ReturnStatement = 'return' [ Expression ] ';' ;
    define_rule!(
        ReturnStatement,
        seq!(
            trie!(trieleaf!(Return, "return")),
            optional!(rule!(Expression)),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // RevertStatement = 'revert' [ IdentifierPath ] ArgumentList ';' ;
    define_rule!(
        RevertStatement,
        seq!(
            trie!(trieleaf!(Revert, "revert")),
            optional!(rule!(IdentifierPath)),
            rule!(ArgumentList),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // SelectedImport = «Identifier» [ 'as' «Identifier» ] ;
    define_rule!(
        SelectedImport,
        seq!(
            token!(Identifier),
            optional!(seq!(trie!(trieleaf!(As, "as")), token!(Identifier)))
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
            trie!(trieleaf!(From, "from")),
            rule!(ImportPath)
        )
    );

    // ShiftExpression = Expression ( '<<' | '>>' | '>>>' ) Expression ;
    define_rule!(
        ShiftExpression,
        seq!(
            rule!(Expression),
            trie!(
                trieleaf!(LessLess, "<<"),
                trieprefix!(
                    ">>",
                    [
                        trieleaf!(GreaterGreaterGreater, ">"),
                        trieleaf!(GreaterGreater)
                    ]
                )
            ),
            rule!(Expression)
        )
    );

    // «SignedFixedType» = 'fixed' [ 1…*{ '0'…'9' } 'x' 1…*{ '0'…'9' } ] ;
    define_token!(
        SignedFixedType,
        scan_make_node!(scan_seq!(
            scan_trie!(scan_trieleaf!("fixed")),
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
            scan_trie!(scan_trieleaf!("int")),
            scan_optional!(scan_trie!(
                scan_trieprefix!(
                    "1",
                    [
                        scan_trieleaf!("04"),
                        scan_trieleaf!("12"),
                        scan_trieprefix!("2", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                        scan_trieleaf!("36"),
                        scan_trieleaf!("44"),
                        scan_trieleaf!("52"),
                        scan_trieprefix!(
                            "6",
                            [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                        ),
                        scan_trieleaf!("76"),
                        scan_trieleaf!("84"),
                        scan_trieleaf!("92")
                    ]
                ),
                scan_trieprefix!(
                    "2",
                    [
                        scan_trieprefix!("0", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                        scan_trieleaf!("16"),
                        scan_trieleaf!("24"),
                        scan_trieleaf!("32"),
                        scan_trieprefix!(
                            "4",
                            [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                        ),
                        scan_trieleaf!("56")
                    ]
                ),
                scan_trieleaf!("32"),
                scan_trieprefix!("4", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                scan_trieleaf!("56"),
                scan_trieleaf!("64"),
                scan_trieleaf!("72"),
                scan_trieprefix!(
                    "8",
                    [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                ),
                scan_trieleaf!("96")
            ))
        ))
    );

    // SimpleImportDirective = ImportPath { 'as' «Identifier» } ;
    define_rule!(
        SimpleImportDirective,
        seq!(
            rule!(ImportPath),
            zero_or_more!(seq!(trie!(trieleaf!(As, "as")), token!(Identifier)))
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
            scan_trie!(scan_trieleaf!("//")),
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
                        scan_trie!(
                            scan_trieleaf!("\n"),
                            scan_trieleaf!("\r"),
                            scan_trieleaf!("\""),
                            scan_trieleaf!("'"),
                            scan_trieleaf!("\\"),
                            scan_trieleaf!("n"),
                            scan_trieleaf!("r"),
                            scan_trieleaf!("t")
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
                        scan_trie!(
                            scan_trieleaf!("\n"),
                            scan_trieleaf!("\r"),
                            scan_trieleaf!("\""),
                            scan_trieleaf!("'"),
                            scan_trieleaf!("\\"),
                            scan_trieleaf!("n"),
                            scan_trieleaf!("r"),
                            scan_trieleaf!("t")
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
            trie!(trieleaf!(Star, "*")),
            trie!(trieleaf!(As, "as")),
            token!(Identifier),
            trie!(trieleaf!(From, "from")),
            rule!(ImportPath)
        )
    );

    // StateVariableAttribute = OverrideSpecifier | 'constant' | 'immutable' | 'internal' | 'private' | 'public' ;
    define_rule!(
        StateVariableAttribute,
        choice!(
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
            rule!(TypeName),
            zero_or_more!(rule!(StateVariableAttribute)),
            token!(Identifier),
            optional!(seq!(trie!(trieleaf!(Equal, "=")), rule!(Expression))),
            trie!(trieleaf!(Semicolon, ";"))
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
            trie!(trieleaf!(Struct, "struct")),
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
            trie!(trieleaf!(Semicolon, ";"))
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
            trie!(trieleaf!(Try, "try")),
            rule!(Expression),
            optional!(seq!(
                trie!(trieleaf!(Returns, "returns")),
                rule!(ParameterList)
            )),
            rule!(Block),
            one_or_more!(rule!(CatchClause))
        )
    );

    // TupleDeconstructionStatement = '(' [ [ [ TypeName ] «Identifier» ]  { ',' [ [ TypeName ] «Identifier» ] } ] ')' '=' Expression ';' ;
    define_rule!(
        TupleDeconstructionStatement,
        seq!(
            delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(
                    optional!(seq!(optional!(rule!(TypeName)), token!(Identifier))),
                    terminal!(Comma, ",")
                )),
                terminal!(CloseParen, ")")
            ),
            trie!(trieleaf!(Equal, "=")),
            rule!(Expression),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // TypeExpression = 'type' '(' TypeName ')' ;
    define_rule!(
        TypeExpression,
        seq!(
            trie!(trieleaf!(Type, "type")),
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
            trie!(
                trieleaf!(Bang, "!"),
                trieleaf!(PlusPlus, "++"),
                trieprefix!("-", [trieleaf!(MinusMinus, "-"), trieleaf!(Minus)]),
                trieleaf!(Tilde, "~")
            ),
            rule!(Expression)
        )
    );

    // UnarySuffixExpression = Expression ( '++' | '--' ) ;
    define_rule!(
        UnarySuffixExpression,
        seq!(
            rule!(Expression),
            trie!(trieleaf!(PlusPlus, "++"), trieleaf!(MinusMinus, "--"))
        )
    );

    // UncheckedBlock = 'unchecked' Block ;
    define_rule!(
        UncheckedBlock,
        seq!(trie!(trieleaf!(Unchecked, "unchecked")), rule!(Block))
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
                            scan_trie!(
                                scan_trieleaf!("\n"),
                                scan_trieleaf!("\r"),
                                scan_trieleaf!("\""),
                                scan_trieleaf!("'"),
                                scan_trieleaf!("\\"),
                                scan_trieleaf!("n"),
                                scan_trieleaf!("r"),
                                scan_trieleaf!("t")
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
                            scan_trie!(
                                scan_trieleaf!("\n"),
                                scan_trieleaf!("\r"),
                                scan_trieleaf!("\""),
                                scan_trieleaf!("'"),
                                scan_trieleaf!("\\"),
                                scan_trieleaf!("n"),
                                scan_trieleaf!("r"),
                                scan_trieleaf!("t")
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
                scan_trie!(scan_trieleaf!("fixed")),
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
                scan_trie!(scan_trieleaf!("int")),
                scan_optional!(scan_trie!(
                    scan_trieprefix!(
                        "1",
                        [
                            scan_trieleaf!("04"),
                            scan_trieleaf!("12"),
                            scan_trieprefix!("2", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                            scan_trieleaf!("36"),
                            scan_trieleaf!("44"),
                            scan_trieleaf!("52"),
                            scan_trieprefix!(
                                "6",
                                [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                            ),
                            scan_trieleaf!("76"),
                            scan_trieleaf!("84"),
                            scan_trieleaf!("92")
                        ]
                    ),
                    scan_trieprefix!(
                        "2",
                        [
                            scan_trieprefix!("0", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                            scan_trieleaf!("16"),
                            scan_trieleaf!("24"),
                            scan_trieleaf!("32"),
                            scan_trieprefix!(
                                "4",
                                [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                            ),
                            scan_trieleaf!("56")
                        ]
                    ),
                    scan_trieleaf!("32"),
                    scan_trieprefix!("4", [scan_trieleaf!("0"), scan_trieleaf!("8")]),
                    scan_trieleaf!("56"),
                    scan_trieleaf!("64"),
                    scan_trieleaf!("72"),
                    scan_trieprefix!(
                        "8",
                        [scan_trieleaf!("0"), scan_trieleaf!("8"), scan_trieleaf!()]
                    ),
                    scan_trieleaf!("96")
                ))
            )
        ))
    );

    // UserDefinedValueTypeDefinition = 'type' «Identifier» 'is' ElementaryType ';' ;
    define_rule!(
        UserDefinedValueTypeDefinition,
        seq!(
            trie!(trieleaf!(Type, "type")),
            token!(Identifier),
            trie!(trieleaf!(Is, "is")),
            rule!(ElementaryType),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // UsingDirective = 'using' ( IdentifierPath | '{' IdentifierPath  { ',' IdentifierPath } '}' ) 'for' ( '*' | TypeName ) [ 'global' ] ';' ;
    define_rule!(
        UsingDirective,
        seq!(
            trie!(trieleaf!(Using, "using")),
            choice!(
                rule!(IdentifierPath),
                delimited_by!(
                    terminal!(OpenBrace, "{"),
                    separated_by!(rule!(IdentifierPath), terminal!(Comma, ",")),
                    terminal!(CloseBrace, "}")
                )
            ),
            trie!(trieleaf!(For, "for")),
            choice!(trie!(trieleaf!(Star, "*")), rule!(TypeName)),
            optional!(trie!(trieleaf!(Global, "global"))),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // VariableDeclarationStatement = TypeName [ DataLocation ] «Identifier» [ '=' Expression ] ';' ;
    define_rule!(
        VariableDeclarationStatement,
        seq!(
            rule!(TypeName),
            optional!(rule!(DataLocation)),
            token!(Identifier),
            optional!(seq!(trie!(trieleaf!(Equal, "=")), rule!(Expression))),
            trie!(trieleaf!(Semicolon, ";"))
        )
    );

    // VersionPragma = 'solidity' 1…*{ VersionPragmaSpecifier } ;
    define_rule!(
        VersionPragma,
        seq!(
            trie!(trieleaf!(Solidity, "solidity")),
            one_or_more!(rule!(VersionPragmaSpecifier))
        )
    );

    // «VersionPragmaOperator» = '^' | '~' | '=' | '<' | '>' | '<=' | '>=' ;
    define_token!(
        VersionPragmaOperator,
        scan_make_node!(scan_trie!(
            scan_trieprefix!("<", [scan_trieleaf!("="), scan_trieleaf!()]),
            scan_trieleaf!("="),
            scan_trieprefix!(">", [scan_trieleaf!("="), scan_trieleaf!()]),
            scan_trieleaf!("^"),
            scan_trieleaf!("~")
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
            trie!(trieleaf!(While, "while")),
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
            trie!(trieleaf!(ColonEqual, ":=")),
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
    define_rule!(YulBreakStatement, trie!(trieleaf!(Break, "break")));

    // YulContinueStatement = 'continue' ;
    define_rule!(YulContinueStatement, trie!(trieleaf!(Continue, "continue")));

    // «YulDecimalNumberLiteral» = '0' | '1'…'9' { '0'…'9' } ;
    define_token!(
        YulDecimalNumberLiteral,
        scan_make_node!(scan_choice!(
            scan_trie!(scan_trieleaf!("0")),
            scan_seq!(
                scan_terminal!(|&c: &char| ('1' <= c && c <= '9')),
                scan_zero_or_more!(scan_terminal!(|&c: &char| ('0' <= c && c <= '9')))
            )
        ))
    );

    // YulExpression = YulIdentifierPath | YulFunctionCall | YulLiteral ;
    define_rule!(
        YulExpression,
        choice!(
            rule!(YulIdentifierPath),
            rule!(YulFunctionCall),
            rule!(YulLiteral)
        )
    );

    // YulForStatement = 'for' YulBlock YulExpression YulBlock YulBlock ;
    define_rule!(
        YulForStatement,
        seq!(
            trie!(trieleaf!(For, "for")),
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
            token!(YulIdentifier),
            delimited_by!(
                terminal!(OpenParen, "("),
                optional!(separated_by!(rule!(YulExpression), terminal!(Comma, ","))),
                terminal!(CloseParen, ")")
            )
        )
    );

    // YulFunctionDefinition = 'function' «YulIdentifier» '(' [ «YulIdentifier»  { ',' «YulIdentifier» } ] ')' [ '->' «YulIdentifier»  { ',' «YulIdentifier» } ] YulBlock ;
    define_rule!(
        YulFunctionDefinition,
        seq!(
            trie!(trieleaf!(Function, "function")),
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
                trie!(trieleaf!(MinusGreater, "->")),
                separated_by!(Results, token!(YulIdentifier), terminal!(Comma, ","))
            )),
            rule!(YulBlock)
        )
    );

    // «YulHexLiteral» = '0x' 1…*{ «HexCharacter» } ;
    define_token!(
        YulHexLiteral,
        scan_make_node!(scan_seq!(
            scan_trie!(scan_trieleaf!("0x")),
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
            scan_trie!(
                scan_trieleaf!("break"),
                scan_trieprefix!("c", [scan_trieleaf!("ase"), scan_trieleaf!("ontinue")]),
                scan_trieleaf!("default"),
                scan_trieprefix!(
                    "f",
                    [
                        scan_trieleaf!("alse"),
                        scan_trieleaf!("or"),
                        scan_trieleaf!("unction")
                    ]
                ),
                scan_trieleaf!("hex"),
                scan_trieleaf!("if"),
                scan_trieprefix!("le", [scan_trieleaf!("ave"), scan_trieleaf!("t")]),
                scan_trieleaf!("switch"),
                scan_trieleaf!("true")
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
            trie!(trieleaf!(If, "if")),
            rule!(YulExpression),
            rule!(YulBlock)
        )
    );

    // «YulKeyword» = «BooleanLiteral» | 'break' | 'case' | 'continue' | 'default' | 'for' | 'function' | 'hex' | 'if' | 'leave' | 'let' | 'switch' ;
    define_token!(
        YulKeyword,
        scan_make_node!(scan_trie!(
            scan_trieleaf!("break"),
            scan_trieprefix!("c", [scan_trieleaf!("ase"), scan_trieleaf!("ontinue")]),
            scan_trieleaf!("default"),
            scan_trieprefix!(
                "f",
                [
                    scan_trieleaf!("alse"),
                    scan_trieleaf!("or"),
                    scan_trieleaf!("unction")
                ]
            ),
            scan_trieleaf!("hex"),
            scan_trieleaf!("if"),
            scan_trieprefix!("le", [scan_trieleaf!("ave"), scan_trieleaf!("t")]),
            scan_trieleaf!("switch"),
            scan_trieleaf!("true")
        ))
    );

    // YulLeaveStatement = 'leave' ;
    define_rule!(YulLeaveStatement, trie!(trieleaf!(Leave, "leave")));

    // YulLiteral = «YulDecimalNumberLiteral» | «YulHexLiteral» | «AsciiStringLiteral» | «BooleanLiteral» | «HexStringLiteral» ;
    define_rule!(
        YulLiteral,
        choice!(
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
            trie!(trieleaf!(Switch, "switch")),
            rule!(YulExpression),
            one_or_more!(seq!(
                choice!(
                    seq!(trie!(trieleaf!(Case, "case")), rule!(YulLiteral)),
                    trie!(trieleaf!(Default, "default"))
                ),
                rule!(YulBlock)
            ))
        )
    );

    // YulVariableDeclaration = 'let' YulIdentifierPath  { ',' YulIdentifierPath } [ ':=' YulExpression ] ;
    define_rule!(
        YulVariableDeclaration,
        seq!(
            trie!(trieleaf!(Let, "let")),
            separated_by!(rule!(YulIdentifierPath), terminal!(Comma, ",")),
            optional!(seq!(
                trie!(trieleaf!(ColonEqual, ":=")),
                rule!(YulExpression)
            ))
        )
    );

    // Return the Parsers object ------------------------

    parsers
}
