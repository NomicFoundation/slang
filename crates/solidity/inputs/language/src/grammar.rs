//! Defines [`GrammarConstructorDslV2`], which allows turning the DSL v2 model into [`Grammar`]
//! (used for generating the parser and the CST).

use std::cell::OnceCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

use codegen_grammar::{
    Grammar, GrammarElement, ParserDefinition, ParserDefinitionNode, PrecedenceOperatorModel,
    PrecedenceParserDefinition, PrecedenceParserDefinitionNode, ScannerDefinition,
    ScannerDefinitionNode, TriviaParserDefinition, VersionQuality, VersionQualityRange,
};
use codegen_language_definition::model;
use codegen_language_definition::model::{FieldsErrorRecovery, Identifier, Item};
use indexmap::IndexMap;

/// Materializes the DSL v2 model ([`model::Language`]) into [`Grammar`].
pub trait GrammarConstructorDslV2 {
    fn from_dsl_v2(lang: &model::Language) -> Grammar;
}

impl GrammarConstructorDslV2 for Grammar {
    #[allow(clippy::too_many_lines)] // TODO: Remove me once the hack below is removed
    fn from_dsl_v2(lang: &model::Language) -> Grammar {
        // Collect language items into a lookup table to speed up resolution
        let mut items: HashMap<_, _> = lang
            .items_with_section()
            .map(|(_, topic, item)| {
                (
                    item.name().clone(),
                    (topic.lexical_context.clone(), Rc::clone(item)),
                )
            })
            .collect();

        // TODO(#638): To minimize regression in the parser migration, we keep the existing DSL v1 model
        // of SourceUnit being followed by `EndOfFileTrivia`.
        items.insert(
            Identifier::from("SourceUnit"),
            (
                None,
                Rc::new(model::Item::Struct {
                    item: model::StructItem {
                        name: Identifier::from("SourceUnit"),
                        enabled: None,
                        error_recovery: None,
                        fields: IndexMap::from_iter([
                            (
                                Identifier::from("members"),
                                model::Field::Optional {
                                    reference: Identifier::from("SourceUnitMembers"),

                                    enabled: None,
                                },
                            ),
                            (
                                Identifier::from("eof_trivia"),
                                model::Field::Optional {
                                    reference: Identifier::from("EndOfFileTrivia"),
                                    enabled: None,
                                },
                            ),
                        ]),
                    },
                }),
            ),
        );

        let mut resolved = HashMap::new();
        let mut ctx = ResolveCtx {
            items: &items,
            resolved: &mut resolved,
        };

        let leading_trivia = Rc::new(NamedTriviaParser {
            name: "LeadingTrivia",
            def: resolve_trivia(lang.leading_trivia.clone(), &mut ctx),
        }) as Rc<dyn TriviaParserDefinition>;

        let trailing_trivia = Rc::new(NamedTriviaParser {
            name: "TrailingTrivia",
            def: resolve_trivia(lang.trailing_trivia.clone(), &mut ctx),
        }) as Rc<dyn TriviaParserDefinition>;

        let eof_trivia = Rc::new(NamedTriviaParser {
            name: "EndOfFileTrivia",
            def: resolve_trivia(lang.leading_trivia.clone(), &mut ctx),
        }) as Rc<dyn TriviaParserDefinition>;

        ctx.resolved.insert(
            Identifier::from("EndOfFileTrivia"),
            eof_trivia.clone().into(),
        );

        for (_lex_ctx, item) in items.values() {
            resolve_grammar_element(item.name(), &mut ctx);
        }

        // TODO(#638): To make sure the unused (not referred to) keywords are included in the scanner literal trie,
        // we replicate the DSL v1 behaviour of introducing a synthetic parser that is only meant to group
        // keywords by their lexical context.
        let mut keywords_per_ctxt = HashMap::new();
        for (ident, (lex_ctx, item)) in &items {
            let lex_ctx = lex_ctx.clone().unwrap_or(Identifier::from("Default"));
            if let Item::Keyword { .. } = item.as_ref() {
                keywords_per_ctxt
                    .entry(lex_ctx)
                    .or_insert_with(Vec::new)
                    .push(ident);
            }
        }
        for (lex_ctx, mut keywords) in keywords_per_ctxt {
            keywords.sort_unstable_by_key(|kw| kw.as_str());

            let parser_name = Identifier::from(format!("{lex_ctx}AllKeywords"));
            let all_keywords = model::EnumItem {
                name: parser_name.clone(),
                enabled: None,
                variants: keywords
                    .iter()
                    .map(|&ident| model::EnumVariant {
                        reference: ident.clone(),
                        enabled: None,
                    })
                    .collect(),
            };

            let def = resolve_choice(all_keywords, &mut ctx);
            ctx.resolved.insert(
                parser_name.clone(),
                GrammarElement::ParserDefinition(Rc::new(NamedParserThunk {
                    name: parser_name.to_string().leak(),
                    context: lex_ctx.to_string().leak(),
                    is_inline: true,
                    def: OnceCell::from(def),
                })),
            );
        }

        let resolved_items = ctx
            .resolved
            .iter()
            .map(|(name, elem)| (name.to_string().leak() as &_, elem.clone()));

        Grammar {
            name: lang.name.to_string(),
            versions: BTreeSet::from_iter(lang.versions.clone()),
            leading_trivia_parser: leading_trivia.clone(),
            trailing_trivia_parser: trailing_trivia.clone(),
            elements: resolved_items
                .chain(
                    [leading_trivia, trailing_trivia, eof_trivia]
                        .into_iter()
                        .map(|elem| (elem.name(), elem.into())),
                )
                .collect(),
        }
    }
}

#[derive(Debug)]
struct NamedScanner {
    name: &'static str,
    def: ScannerDefinitionNode,
}

impl ScannerDefinition for NamedScanner {
    fn name(&self) -> &'static str {
        self.name
    }
    fn node(&self) -> &ScannerDefinitionNode {
        &self.def
    }
}

#[derive(Debug)]
struct NamedTriviaParser {
    name: &'static str,
    def: ParserDefinitionNode,
}

impl TriviaParserDefinition for NamedTriviaParser {
    fn name(&self) -> &'static str {
        self.name
    }

    fn context(&self) -> &'static str {
        // NOTE:
        "Default"
    }

    fn node(&self) -> &ParserDefinitionNode {
        &self.def
    }
}

#[derive(Debug)]
struct NamedParserThunk {
    name: &'static str,
    context: &'static str,
    is_inline: bool,
    def: OnceCell<ParserDefinitionNode>,
}

impl ParserDefinition for NamedParserThunk {
    fn name(&self) -> &'static str {
        self.name
    }

    fn context(&self) -> &'static str {
        self.context
    }

    fn is_inline(&self) -> bool {
        self.is_inline
    }

    fn node(&self) -> &ParserDefinitionNode {
        self.def.get().expect("Thunk to be resolved")
    }
}

#[derive(Debug)]
struct NamedPrecedenceParserThunk {
    name: &'static str,
    context: &'static str,
    def: OnceCell<PrecedenceParserDefinitionNode>,
}
impl PrecedenceParserDefinition for NamedPrecedenceParserThunk {
    fn name(&self) -> &'static str {
        self.name
    }

    fn context(&self) -> &'static str {
        self.context
    }

    fn node(&self) -> &PrecedenceParserDefinitionNode {
        self.def.get().expect("Thunk to be resolved")
    }
}

enum ParserThunk {
    Regular(Rc<NamedParserThunk>),
    Precedence(Rc<NamedPrecedenceParserThunk>),
}
impl ParserThunk {
    fn as_regular_def(&self) -> &OnceCell<ParserDefinitionNode> {
        match self {
            ParserThunk::Regular(thunk) => &thunk.def,
            ParserThunk::Precedence(..) => panic!("Expected a regular parser thunk"),
        }
    }

    fn as_precedence_def(&self) -> &OnceCell<PrecedenceParserDefinitionNode> {
        match self {
            ParserThunk::Precedence(thunk) => &thunk.def,
            ParserThunk::Regular(..) => panic!("Expected a precedence parser thunk"),
        }
    }
}

fn enabled_to_range(spec: model::VersionSpecifier) -> Vec<VersionQualityRange> {
    match spec {
        model::VersionSpecifier::Never => vec![VersionQualityRange {
            from: semver::Version::new(0, 0, 0),
            quality: VersionQuality::Removed,
        }],
        model::VersionSpecifier::From { from } => vec![VersionQualityRange {
            from,
            quality: VersionQuality::Introduced,
        }],
        model::VersionSpecifier::Till { till } => vec![VersionQualityRange {
            from: till,
            quality: VersionQuality::Removed,
        }],
        model::VersionSpecifier::Range { from, till } => vec![
            VersionQualityRange {
                from,
                quality: VersionQuality::Introduced,
            },
            VersionQualityRange {
                from: till,
                quality: VersionQuality::Removed,
            },
        ],
    }
}

struct ResolveCtx<'a> {
    items: &'a HashMap<Identifier, (Option<Identifier>, Rc<Item>)>,
    resolved: &'a mut HashMap<Identifier, GrammarElement>,
}

#[allow(clippy::too_many_lines)] // FIXME: Simplify me when we simplify the v2-to-v1 interface
fn resolve_grammar_element(ident: &Identifier, ctx: &mut ResolveCtx<'_>) -> GrammarElement {
    if ident.as_str() == "EndOfFileTrivia" {
        return ctx.resolved.get(ident).unwrap().clone();
    }

    let (lex_ctx, elem) = ctx.items.get(ident).expect("Missing item");

    // FIXME: Don't leak
    let lex_ctx = lex_ctx
        .as_ref()
        .map_or("Default", |l| l.to_string().leak() as &_);

    // The non-terminals are mutually recursive (so will be the resolution of their definitions),
    // so make sure to insert a thunk for non-terminals to resolve to break the cycle.
    let inserted_thunk = match (elem.as_ref(), ctx.resolved.contains_key(ident)) {
        (
            Item::Struct { .. }
            | Item::Enum { .. }
            | Item::Repeated { .. }
            | Item::Separated { .. },
            false,
        ) => {
            let thunk = Rc::new(NamedParserThunk {
                name: ident.to_string().leak(),
                context: lex_ctx,
                is_inline: false,
                def: OnceCell::new(),
            });
            ctx.resolved.insert(
                ident.clone(),
                (thunk.clone() as Rc<dyn ParserDefinition>).into(),
            );
            Some(ParserThunk::Regular(thunk))
        }
        (Item::Precedence { .. }, false) => {
            let thunk = Rc::new(NamedPrecedenceParserThunk {
                name: ident.to_string().leak(),
                context: lex_ctx,
                def: OnceCell::new(),
            });
            ctx.resolved.insert(
                ident.clone(),
                (thunk.clone() as Rc<dyn PrecedenceParserDefinition>).into(),
            );
            Some(ParserThunk::Precedence(thunk))
        }
        _ => None,
    };

    match (inserted_thunk, ctx.resolved.get(ident)) {
        // Already resolved
        (None, Some(resolved)) => resolved.clone(),
        (Some(..), None) => unreachable!("We just inserted a thunk!"),
        // First time resolving a non-terminal named `ident` (since we just inserted a thunk)
        // Any recursive resolution for this non-terminal will already use the thunk.
        // Once we're finished, we initialize the cell with the resolved definition.
        (Some(thunk), _) => {
            match elem.as_ref() {
                Item::Struct { item } => {
                    let item = item.clone();
                    thunk
                        .as_regular_def()
                        .set(resolve_sequence_like(
                            item.enabled,
                            item.fields,
                            item.error_recovery,
                            ctx,
                        ))
                        .unwrap();
                }
                Item::Enum { item } => {
                    thunk
                        .as_regular_def()
                        .set(resolve_choice(item.clone(), ctx))
                        .unwrap();
                }
                Item::Repeated { item } => {
                    thunk
                        .as_regular_def()
                        .set(resolve_repeated(item.clone(), ctx))
                        .unwrap();
                }
                Item::Separated { item } => {
                    thunk
                        .as_regular_def()
                        .set(resolve_separated(item.clone(), ctx))
                        .unwrap();
                }
                Item::Precedence { item } => {
                    thunk
                        .as_precedence_def()
                        .set(resolve_precedence(item.clone(), lex_ctx, ctx))
                        .unwrap();
                }
                _ => unreachable!("Only non-terminals can be resolved here"),
            };

            ctx.resolved.get(ident).cloned().unwrap()
        }
        // First time resolving a terminal named `ident`
        (None, None) => {
            let named_scanner = match elem.as_ref() {
                Item::Trivia { item } => NamedScanner {
                    name: ident.to_string().leak(),
                    def: resolve_scanner(item.scanner.clone(), ctx),
                },
                Item::Fragment { item } => NamedScanner {
                    name: ident.to_string().leak(),
                    def: resolve_fragment(item.clone(), ctx),
                },
                Item::Token { item } => NamedScanner {
                    name: ident.to_string().leak(),
                    def: resolve_token(item.clone(), ctx),
                },
                Item::Keyword { item } => NamedScanner {
                    name: ident.to_string().leak(),
                    def: resolve_keyword(item.clone()),
                },
                _ => unreachable!("Only terminals can be resolved here"),
            };

            let resolved = GrammarElement::ScannerDefinition(Rc::new(named_scanner));
            ctx.resolved.insert(ident.clone(), resolved.clone());

            resolved
        }
    }
}

fn resolve_scanner(scanner: model::Scanner, ctx: &mut ResolveCtx<'_>) -> ScannerDefinitionNode {
    match scanner {
        model::Scanner::Optional { scanner } => {
            ScannerDefinitionNode::Optional(Box::new(resolve_scanner(*scanner, ctx)))
        }
        model::Scanner::ZeroOrMore { scanner } => {
            ScannerDefinitionNode::ZeroOrMore(Box::new(resolve_scanner(*scanner, ctx)))
        }
        model::Scanner::OneOrMore { scanner } => {
            ScannerDefinitionNode::OneOrMore(Box::new(resolve_scanner(*scanner, ctx)))
        }
        model::Scanner::Sequence { scanners } => ScannerDefinitionNode::Sequence(
            scanners
                .into_iter()
                .map(|scanner| resolve_scanner(scanner, ctx))
                .collect(),
        ),
        model::Scanner::Choice { scanners } => ScannerDefinitionNode::Choice(
            scanners
                .into_iter()
                .map(|scanner| resolve_scanner(scanner, ctx))
                .collect(),
        ),
        model::Scanner::Not { chars } => ScannerDefinitionNode::NoneOf(chars.into_iter().collect()),
        model::Scanner::TrailingContext {
            scanner,
            not_followed_by,
        } => ScannerDefinitionNode::NotFollowedBy(
            Box::new(resolve_scanner(*scanner, ctx)),
            Box::new(resolve_scanner(*not_followed_by, ctx)),
        ),
        model::Scanner::Range {
            inclusive_start,
            inclusive_end,
        } => ScannerDefinitionNode::CharRange(inclusive_start, inclusive_end),
        model::Scanner::Atom { atom } => ScannerDefinitionNode::Literal(atom),
        model::Scanner::Fragment { reference } => match resolve_grammar_element(&reference, ctx) {
            GrammarElement::ScannerDefinition(parser) => {
                ScannerDefinitionNode::ScannerDefinition(parser)
            }
            _ => panic!("Expected {reference} to be a ScannerDefinition"),
        },
    }
}

fn resolve_fragment(
    fragment: model::FragmentItem,
    ctx: &mut ResolveCtx<'_>,
) -> ScannerDefinitionNode {
    resolve_scanner(fragment.scanner, ctx).versioned(fragment.enabled)
}

fn resolve_token(token: model::TokenItem, ctx: &mut ResolveCtx<'_>) -> ScannerDefinitionNode {
    let resolved_defs: Vec<_> = token
        .definitions
        .into_iter()
        .map(|def| resolve_scanner(def.scanner, ctx).versioned(def.enabled))
        .collect();

    match resolved_defs.len() {
        0 => panic!("Token {} has no definitions", token.name),
        1 => resolved_defs.into_iter().next().unwrap(),
        _ => ScannerDefinitionNode::Choice(resolved_defs),
    }
}

fn resolve_keyword(keyword: model::KeywordItem) -> ScannerDefinitionNode {
    // TODO(#568): Handle reserved keywords using the given "Identifier" parser
    let _ = keyword.identifier;

    let defs: Vec<_> = keyword
        .definitions
        .into_iter()
        .map(|def| {
            let value = resolve_keyword_value(def.value);
            // If missing, the default is "Always"
            match (def.enabled, def.reserved) {
                // Contextual keywords (never reserved)
                // TODO(#568): Properly support contextual keywords.
                // Currently, to minimize the diff and ease the transition to the DSL v2, we treat them as normal keywords.
                // Moreover, since the DSL v1 only treats "enablement" as being reserved, we try to preserve that for now.
                (enabled, Some(model::VersionSpecifier::Never)) => value.versioned(enabled),
                // TODO(#568): If a contextual keyword was enabled at some point and then reserved, for now we treat it
                // as a reserved keyword starting from when it was being used, to preserve the DSL v1 behaviour.
                (
                    Some(model::VersionSpecifier::From { from: enabled }),
                    Some(model::VersionSpecifier::From { from: reserved }),
                ) if enabled < reserved => ScannerDefinitionNode::Versioned(
                    Box::new(value),
                    enabled_to_range(model::VersionSpecifier::From { from: enabled }),
                ),
                (_, Some(reserved)) => {
                    ScannerDefinitionNode::Versioned(Box::new(value), enabled_to_range(reserved))
                }
                // The keyword is always reserved
                (_, None) => value,
            }
        })
        .collect();

    match defs.len() {
        0 => panic!("Keyword {} has no definitions", keyword.name),
        1 => defs.into_iter().next().unwrap(),
        _ => ScannerDefinitionNode::Choice(defs),
    }
}

fn resolve_keyword_value(value: model::KeywordValue) -> ScannerDefinitionNode {
    match value {
        model::KeywordValue::Sequence { values } => {
            ScannerDefinitionNode::Sequence(values.into_iter().map(resolve_keyword_value).collect())
        }
        model::KeywordValue::Choice { values } => {
            ScannerDefinitionNode::Choice(values.into_iter().map(resolve_keyword_value).collect())
        }
        model::KeywordValue::Optional { value } => {
            ScannerDefinitionNode::Optional(Box::new(resolve_keyword_value(*value)))
        }
        model::KeywordValue::Atom { atom } => ScannerDefinitionNode::Literal(atom),
    }
}

fn resolve_trivia(parser: model::TriviaParser, ctx: &mut ResolveCtx<'_>) -> ParserDefinitionNode {
    match parser {
        model::TriviaParser::Optional { parser } => {
            ParserDefinitionNode::Optional(Box::new(resolve_trivia(*parser, ctx)))
        }
        model::TriviaParser::OneOrMore { parser } => {
            ParserDefinitionNode::OneOrMore(Box::new(resolve_trivia(*parser, ctx)))
        }
        model::TriviaParser::ZeroOrMore { parser } => {
            ParserDefinitionNode::ZeroOrMore(Box::new(resolve_trivia(*parser, ctx)))
        }
        model::TriviaParser::Sequence { parsers } => ParserDefinitionNode::Sequence(
            parsers
                .into_iter()
                .map(|scanner| resolve_trivia(scanner, ctx))
                .collect(),
        ),
        model::TriviaParser::Choice { parsers } => ParserDefinitionNode::Choice(
            parsers
                .into_iter()
                .map(|scanner| resolve_trivia(scanner, ctx))
                .collect(),
        ),
        model::TriviaParser::Trivia { trivia } => match resolve_grammar_element(&trivia, ctx) {
            GrammarElement::ScannerDefinition(parser) => {
                ParserDefinitionNode::ScannerDefinition(parser)
            }
            _ => panic!("Expected {trivia} to be a ScannerDefinition"),
        },
    }
}

fn resolve_field(field: model::Field, ctx: &mut ResolveCtx<'_>) -> ParserDefinitionNode {
    match field {
        model::Field::Required { reference } => {
            resolve_grammar_element(&reference, ctx).into_parser_def_node()
        }
        model::Field::Optional { reference, enabled } => ParserDefinitionNode::Optional(Box::new(
            resolve_grammar_element(&reference, ctx)
                .into_parser_def_node()
                .versioned(enabled),
        )),
    }
}

fn resolve_sequence_like(
    enabled: Option<model::VersionSpecifier>,
    fields: IndexMap<Identifier, model::Field>,
    error_recovery: Option<FieldsErrorRecovery>,
    ctx: &mut ResolveCtx<'_>,
) -> ParserDefinitionNode {
    let (terminator, delimiters) = match error_recovery {
        Some(FieldsErrorRecovery {
            terminator: None,
            delimiters: None,
        }) => panic!("Empty error_recovery"),
        None => (None, None),
        Some(FieldsErrorRecovery {
            terminator,
            delimiters,
        }) => (terminator, delimiters),
    };

    let mut fields: Vec<_> = fields
        .into_iter()
        .map(|(name, field)| (name, resolve_field(field, ctx)))
        .collect();

    // Transform inline [.., open, body, close, ..] sequence into single DelimitedBy(open, body, close) node
    if let Some(delimiters) = delimiters {
        let open_idx = fields.iter().position(|(nam, _)| nam == &delimiters.open);
        let close_idx = fields.iter().position(|(nam, _)| nam == &delimiters.close);
        let (open_idx, close_idx) = (open_idx.unwrap(), close_idx.unwrap());

        let delimited_body: Vec<_> = fields
            .drain((open_idx + 1)..close_idx)
            .map(|(_, field)| field)
            .collect();

        let delimited_body = match delimited_body.len() {
            1 => delimited_body.into_iter().next().unwrap(),
            0 => ParserDefinitionNode::Sequence(vec![]),
            _ => ParserDefinitionNode::Sequence(delimited_body),
        };
        // Replace the remaining delimiters with the new delimited body
        let delimited = {
            let mut delims = fields
                .drain(open_idx..=open_idx + 1)
                .map(|(_, field)| field);
            let open = delims.next().unwrap();
            let close = delims.next().unwrap();

            ParserDefinitionNode::DelimitedBy(
                Box::new(open),
                Box::new(delimited_body),
                Box::new(close),
            )
        };
        fields.insert(
            open_idx,
            (
                delimiters.open, // dummy, identifiers are stripped here anyway
                delimited,
            ),
        );
    }

    let terminator = match terminator {
        Some(terminator) => {
            let (name, def) = fields.pop().unwrap();
            assert_eq!(name, terminator);

            Some(def)
        }
        None => None,
    };

    let body = ParserDefinitionNode::Sequence(fields.into_iter().map(|(_, def)| def).collect());

    if let Some(terminator) = terminator {
        ParserDefinitionNode::TerminatedBy(Box::new(body), Box::new(terminator))
    } else {
        body
    }
    .versioned(enabled)
}

fn resolve_choice(item: model::EnumItem, ctx: &mut ResolveCtx<'_>) -> ParserDefinitionNode {
    let variants = item
        .variants
        .into_iter()
        .map(|variant| {
            resolve_grammar_element(&variant.reference, ctx)
                .into_parser_def_node()
                .versioned(variant.enabled)
        })
        .collect();

    ParserDefinitionNode::Choice(variants).versioned(item.enabled)
}

fn resolve_repeated(item: model::RepeatedItem, ctx: &mut ResolveCtx<'_>) -> ParserDefinitionNode {
    let body = Box::new(resolve_grammar_element(&item.repeated, ctx).into_parser_def_node());

    ParserDefinitionNode::OneOrMore(body).versioned(item.enabled)
}

fn resolve_separated(item: model::SeparatedItem, ctx: &mut ResolveCtx<'_>) -> ParserDefinitionNode {
    let body = resolve_grammar_element(&item.separated, ctx).into_parser_def_node();
    let separator = resolve_grammar_element(&item.separator, ctx).into_parser_def_node();

    ParserDefinitionNode::SeparatedBy(Box::new(body), Box::new(separator)).versioned(item.enabled)
}

fn resolve_precedence(
    item: model::PrecedenceItem,
    lex_ctx: &'static str,
    ctx: &mut ResolveCtx<'_>,
) -> PrecedenceParserDefinitionNode {
    let primaries: Vec<_> = item
        .primary_expressions
        .into_iter()
        .map(|prim| {
            resolve_grammar_element(&prim.reference, ctx)
                .into_parser_def_node()
                .versioned(prim.enabled)
        })
        .collect();
    let primary_expression = Box::new(match primaries.len() {
        0 => panic!("Precedence operator has no primary expressions"),
        1 => primaries.into_iter().next().unwrap(),
        _ => ParserDefinitionNode::Choice(primaries),
    });

    #[allow(clippy::items_after_statements)] // simple and specific to this site
    fn model_to_enum(model: model::OperatorModel) -> PrecedenceOperatorModel {
        match model {
            model::OperatorModel::BinaryLeftAssociative => {
                PrecedenceOperatorModel::BinaryLeftAssociative
            }
            model::OperatorModel::BinaryRightAssociative => {
                PrecedenceOperatorModel::BinaryRightAssociative
            }
            model::OperatorModel::Prefix => PrecedenceOperatorModel::Prefix,
            model::OperatorModel::Postfix => PrecedenceOperatorModel::Postfix,
        }
    }

    let mut operators = vec![];
    let mut precedence_expression_names = Vec::with_capacity(item.precedence_expressions.len());
    for expr in item.precedence_expressions {
        let name = expr.name;
        // TODO: Don't leak
        let leaked_name = name.to_string().leak() as &_;

        precedence_expression_names.push(leaked_name);
        // Register it as a regular parser with a given name, however we need to
        // define it as a choice over the "operator" sequences
        // Then, when returning, we should actually return a node ref pointing to that combined parser
        // And ideally, we shouldn't even use the "enabled" mode of the original DSL
        let thunk = Rc::new(NamedParserThunk {
            name: leaked_name,
            context: lex_ctx,
            is_inline: true,
            def: OnceCell::new(),
        });

        // Each precedence expression can have multiple operators with different modes and versions
        // We partition them by model and then resolve each group separately
        let mut operators_per_model = BTreeMap::<_, Vec<_>>::new();
        for op in &expr.operators {
            operators_per_model.entry(op.model).or_default().push(op);
        }

        let mut all_operators = vec![];
        for (model, model_operators) in operators_per_model {
            let defs: Vec<_> = model_operators
                .into_iter()
                .map(ToOwned::to_owned)
                .map(|op| resolve_sequence_like(op.enabled, op.fields, op.error_recovery, ctx))
                .collect();

            let def = match &defs[..] {
                // HACK: Despite it being a single definition, we still need to wrap a versioned
                // node around the choice for it to emit the version checks for the node.
                [ParserDefinitionNode::Versioned(..)] => ParserDefinitionNode::Choice(defs),
                [_] => defs.into_iter().next().unwrap(),
                _ => ParserDefinitionNode::Choice(defs),
            };

            all_operators.push(def.clone());
            operators.push((model_to_enum(model), leaked_name, def));
        }

        // Register the combined parser definition to appease the codegen and to mark terminals
        // as reachable and ensure we emit a token kind for each
        thunk
            .def
            .set(ParserDefinitionNode::Choice(all_operators))
            .unwrap();
        assert!(
            !ctx.resolved.contains_key(&name),
            "Encountered a duplicate Precedence Expression named {name} when resolving"
        );
        ctx.resolved.insert(
            name.clone(),
            GrammarElement::ParserDefinition(thunk.clone()),
        );
    }

    PrecedenceParserDefinitionNode {
        primary_expression,
        operators,
        precedence_expression_names,
    }
}

trait IntoParserDefNode {
    fn into_parser_def_node(self) -> ParserDefinitionNode;
}

impl IntoParserDefNode for GrammarElement {
    fn into_parser_def_node(self) -> ParserDefinitionNode {
        match self {
            GrammarElement::ParserDefinition(parser) => {
                ParserDefinitionNode::ParserDefinition(parser)
            }
            GrammarElement::ScannerDefinition(parser) => {
                ParserDefinitionNode::ScannerDefinition(parser)
            }
            GrammarElement::TriviaParserDefinition(parser) => {
                ParserDefinitionNode::TriviaParserDefinition(parser)
            }
            GrammarElement::PrecedenceParserDefinition(parser) => {
                ParserDefinitionNode::PrecedenceParserDefinition(parser)
            }
        }
    }
}

/// Helper trait to wrap a definition node with a version specifier.
trait VersionWrapped {
    fn versioned(self, enabled: Option<model::VersionSpecifier>) -> Self;
}

impl VersionWrapped for ParserDefinitionNode {
    fn versioned(self, enabled: Option<model::VersionSpecifier>) -> Self {
        if let Some(enabled) = enabled {
            Self::Versioned(Box::new(self), enabled_to_range(enabled))
        } else {
            self
        }
    }
}

impl VersionWrapped for ScannerDefinitionNode {
    fn versioned(self, enabled: Option<model::VersionSpecifier>) -> Self {
        if let Some(enabled) = enabled {
            Self::Versioned(Box::new(self), enabled_to_range(enabled))
        } else {
            self
        }
    }
}
