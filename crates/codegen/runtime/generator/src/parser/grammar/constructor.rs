//! Defines a translation of DSL v2 model into [`Grammar`], which is used for generating the parser and the CST.

use std::cell::OnceCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ops::Deref;
use std::rc::Rc;

use codegen_language_definition::model::{self, FieldsErrorRecovery, Identifier, Item};
use indexmap::IndexMap;
use once_cell::sync::Lazy;

use crate::parser::grammar::{
    DelimitedRecoveryTerminalThreshold, Grammar, GrammarElement, Labeled, ParserDefinition,
    ParserDefinitionNode, PrecedenceParserDefinition, PrecedenceParserDefinitionNode,
    TriviaParserDefinition,
};

impl Grammar {
    /// Materializes the DSL v2 model ([`model::Language`]) into [`Grammar`].
    pub fn from_dsl_v2(lang: &model::Language) -> Grammar {
        // Collect language items into a lookup table to speed up resolution
        let items: HashMap<_, _> = lang
            .topics()
            .flat_map(|topic| {
                topic.items.iter().map(|item| {
                    (
                        item.name().clone(),
                        (topic.lexical_context.clone(), item.clone()),
                    )
                })
            })
            .collect();

        let mut resolved = HashMap::new();
        let mut ctx = ResolveCtx {
            items: &items,
            resolved: &mut resolved,
        };

        let leading_trivia = Rc::new(NamedTriviaParser {
            name: Identifier::from("LeadingTrivia"),
            def: resolve_trivia(lang.leading_trivia.clone(), TriviaKind::Leading, &mut ctx),
        }) as Rc<dyn TriviaParserDefinition>;

        let trailing_trivia = Rc::new(NamedTriviaParser {
            name: Identifier::from("TrailingTrivia"),
            def: resolve_trivia(lang.trailing_trivia.clone(), TriviaKind::Trailing, &mut ctx),
        }) as Rc<dyn TriviaParserDefinition>;

        for (_lex_ctx, item) in items.values() {
            resolve_grammar_element(item.name(), &mut ctx);
        }

        // TODO(#638): To make sure the unused (not referred to) keywords are included in the scanner literal trie,
        // we replicate the DSL v1 behaviour of introducing a synthetic parser that is only meant to group
        // keywords by their lexical context.
        let mut keywords_per_ctxt = HashMap::new();
        for (ident, (lex_ctx, item)) in &items {
            let lex_ctx = lex_ctx.clone().unwrap_or(Identifier::from("Default"));
            if let Item::Keyword { .. } = item {
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
                    name: parser_name,
                    context: lex_ctx,
                    is_inline: true,
                    def: OnceCell::from(def),
                })),
            );
        }

        let resolved_items = ctx
            .resolved
            .iter()
            .map(|(name, elem)| (name.clone(), elem.clone()));

        Grammar {
            name: lang.name.to_string(),
            versions: BTreeSet::from_iter(lang.versions.clone()),
            leading_trivia_parser: Rc::clone(&leading_trivia),
            trailing_trivia_parser: Rc::clone(&trailing_trivia),
            elements: resolved_items
                .chain(
                    [leading_trivia, trailing_trivia]
                        .into_iter()
                        .map(|elem| (elem.name().clone(), elem.into())),
                )
                .collect(),
        }
    }
}

#[derive(Debug)]
struct NamedTriviaParser {
    name: Identifier,
    def: ParserDefinitionNode,
}

impl TriviaParserDefinition for NamedTriviaParser {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn context(&self) -> &Identifier {
        static DEFAULT: Lazy<Identifier> = Lazy::new(|| Identifier::from("Default"));
        &DEFAULT
    }

    fn node(&self) -> &ParserDefinitionNode {
        &self.def
    }
}

#[derive(Debug)]
struct NamedParserThunk {
    name: Identifier,
    context: Identifier,
    is_inline: bool,
    def: OnceCell<ParserDefinitionNode>,
}

impl ParserDefinition for NamedParserThunk {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn context(&self) -> &Identifier {
        &self.context
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
    name: Identifier,
    context: Identifier,
    def: OnceCell<PrecedenceParserDefinitionNode>,
}
impl PrecedenceParserDefinition for NamedPrecedenceParserThunk {
    fn name(&self) -> &Identifier {
        &self.name
    }

    fn context(&self) -> &Identifier {
        &self.context
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

struct ResolveCtx<'a> {
    items: &'a HashMap<Identifier, (Option<Identifier>, Item)>,
    resolved: &'a mut HashMap<Identifier, GrammarElement>,
}

#[allow(clippy::too_many_lines)] // FIXME(#638): Simplify me when we simplify the v2-to-v1 interface
fn resolve_grammar_element(ident: &Identifier, ctx: &mut ResolveCtx<'_>) -> GrammarElement {
    let (lex_ctx, elem) = ctx.items.get(ident).expect("Missing item");

    let lex_ctx = lex_ctx
        .clone()
        .unwrap_or_else(|| Identifier::from("Default"));

    // The nonterminals are mutually recursive (so will be the resolution of their definitions),
    // so make sure to insert a thunk for nonterminals to resolve to break the cycle.
    let inserted_thunk = match (elem, ctx.resolved.contains_key(ident)) {
        (
            Item::Struct { .. }
            | Item::Enum { .. }
            | Item::Repeated { .. }
            | Item::Separated { .. },
            false,
        ) => {
            let thunk = Rc::new(NamedParserThunk {
                name: ident.clone(),
                context: lex_ctx.clone(),
                is_inline: false,
                def: OnceCell::new(),
            });
            ctx.resolved.insert(
                ident.clone(),
                (Rc::clone(&thunk) as Rc<dyn ParserDefinition>).into(),
            );
            Some(ParserThunk::Regular(thunk))
        }
        (Item::Precedence { .. }, false) => {
            let thunk = Rc::new(NamedPrecedenceParserThunk {
                name: ident.clone(),
                context: lex_ctx.clone(),
                def: OnceCell::new(),
            });
            ctx.resolved.insert(
                ident.clone(),
                (Rc::clone(&thunk) as Rc<dyn PrecedenceParserDefinition>).into(),
            );
            Some(ParserThunk::Precedence(thunk))
        }
        _ => None,
    };

    match (inserted_thunk, ctx.resolved.get(ident)) {
        // Already resolved
        (None, Some(resolved)) => resolved.clone(),
        (Some(..), None) => unreachable!("We just inserted a thunk!"),
        // First time resolving a nonterminal named `ident` (since we just inserted a thunk)
        // Any recursive resolution for this nonterminal will already use the thunk.
        // Once we're finished, we initialize the cell with the resolved definition.
        (Some(thunk), _) => {
            match elem {
                Item::Struct { item } => {
                    let item = item.deref().clone();
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
                        .set(resolve_choice(item.deref().clone(), ctx))
                        .unwrap();
                }
                Item::Repeated { item } => {
                    thunk
                        .as_regular_def()
                        .set(resolve_repeated(item.deref().clone(), ctx))
                        .unwrap();
                }
                Item::Separated { item } => {
                    thunk
                        .as_regular_def()
                        .set(resolve_separated(item.deref().clone(), ctx))
                        .unwrap();
                }
                Item::Precedence { item } => {
                    thunk
                        .as_precedence_def()
                        .set(resolve_precedence(item.deref().clone(), &lex_ctx, ctx))
                        .unwrap();
                }
                _ => unreachable!("Only nonterminals can be resolved here"),
            };

            ctx.resolved.get(ident).cloned().unwrap()
        }
        // First time resolving a terminal named `ident`
        (None, None) => {
            let named_scanner = match elem {
                Item::Trivia { item } => Rc::clone(item) as Rc<_>,
                Item::Fragment { item } => Rc::clone(item) as Rc<_>,
                Item::Token { item } => Rc::clone(item) as Rc<_>,
                Item::Keyword { item } => {
                    // Keywords are special scanners and are handled separately
                    let resolved =
                        GrammarElement::KeywordScannerDefinition(Rc::clone(item) as Rc<_>);
                    ctx.resolved.insert(ident.clone(), resolved.clone());
                    return resolved;
                }
                _ => unreachable!("Only terminals can be resolved here"),
            };

            let resolved = GrammarElement::ScannerDefinition(named_scanner);
            ctx.resolved.insert(ident.clone(), resolved.clone());

            resolved
        }
    }
}

fn resolve_trivia(
    parser: model::TriviaParser,
    kind: TriviaKind,
    ctx: &mut ResolveCtx<'_>,
) -> ParserDefinitionNode {
    match parser {
        model::TriviaParser::Optional { parser } => {
            ParserDefinitionNode::Optional(Box::new(resolve_trivia(*parser, kind, ctx)))
        }
        model::TriviaParser::OneOrMore { parser } => ParserDefinitionNode::OneOrMore(
            Labeled::anonymous(Box::new(resolve_trivia(*parser, kind, ctx))),
        ),
        model::TriviaParser::ZeroOrMore { parser } => ParserDefinitionNode::ZeroOrMore(
            Labeled::anonymous(Box::new(resolve_trivia(*parser, kind, ctx))),
        ),
        model::TriviaParser::Sequence { parsers } => ParserDefinitionNode::Sequence(
            parsers
                .into_iter()
                .map(|scanner| Labeled::anonymous(resolve_trivia(scanner, kind, ctx)))
                .collect(),
        ),
        model::TriviaParser::Choice { parsers } => {
            ParserDefinitionNode::Choice(Labeled::anonymous(
                parsers
                    .into_iter()
                    .map(|scanner| resolve_trivia(scanner, kind, ctx))
                    .collect(),
            ))
        }
        model::TriviaParser::Trivia { reference } => {
            match resolve_grammar_element(&reference, ctx) {
                GrammarElement::ScannerDefinition(parser) => {
                    // Hack: This is a sequence of a single scanner in order to emit the names
                    ParserDefinitionNode::Sequence(vec![Labeled::with_builtin_label(
                        kind.label(),
                        ParserDefinitionNode::ScannerDefinition(parser),
                    )])
                }
                _ => panic!("Expected {reference} to be a ScannerDefinition"),
            }
        }
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
        let open_idx = fields.iter().position(|(name, _)| name == &delimiters.open);
        let close_idx = fields
            .iter()
            .position(|(name, _)| name == &delimiters.close);
        let (open_idx, close_idx) = (open_idx.unwrap(), close_idx.unwrap());

        let delimited_body = ParserDefinitionNode::Sequence(
            fields
                .drain((open_idx + 1)..close_idx)
                .map(|(name, field)| Labeled::with_ident_name(name, field))
                .collect(),
        );

        // Replace the remaining delimiters with the new delimited body
        let delimited = {
            let mut delims = fields
                .drain(open_idx..=open_idx + 1)
                .map(|(name, field)| Labeled::with_ident_name(name, Box::new(field)));
            let open = delims.next().unwrap();
            let close = delims.next().unwrap();

            let threshold = DelimitedRecoveryTerminalThreshold::from(delimiters);
            ParserDefinitionNode::DelimitedBy(open, Box::new(delimited_body), close, threshold)
        };
        // Replace with a new delimited node
        fields.insert(
            open_idx,
            (
                // Inner nodes will be flattened by PG, let's use an empty name for clarity
                Identifier::from(""),
                delimited,
            ),
        );
    }

    let terminator = match terminator {
        Some(terminator) => {
            let (name, def) = fields.pop().unwrap();
            assert_eq!(name, terminator);

            Some(Labeled::with_ident_name(name, Box::new(def)))
        }
        None => None,
    };

    let body = ParserDefinitionNode::Sequence(
        fields
            .into_iter()
            .map(|(name, def)| Labeled::with_ident_name(name, def))
            .collect(),
    );

    if let Some(terminator) = terminator {
        ParserDefinitionNode::TerminatedBy(Box::new(body), terminator)
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

    ParserDefinitionNode::Choice(Labeled::with_builtin_label(BuiltInLabel::Variant, variants))
        .versioned(item.enabled)
}

fn resolve_repeated(item: model::RepeatedItem, ctx: &mut ResolveCtx<'_>) -> ParserDefinitionNode {
    let reference = Box::new(resolve_grammar_element(&item.reference, ctx).into_parser_def_node());

    let repeated = Labeled::with_builtin_label(BuiltInLabel::Item, reference);

    if item.allow_empty.unwrap_or_default() {
        ParserDefinitionNode::ZeroOrMore(repeated).versioned(item.enabled)
    } else {
        ParserDefinitionNode::OneOrMore(repeated).versioned(item.enabled)
    }
}

fn resolve_separated(item: model::SeparatedItem, ctx: &mut ResolveCtx<'_>) -> ParserDefinitionNode {
    let reference = resolve_grammar_element(&item.reference, ctx).into_parser_def_node();
    let separator = resolve_grammar_element(&item.separator, ctx).into_parser_def_node();

    let separated = ParserDefinitionNode::SeparatedBy(
        Labeled::with_builtin_label(BuiltInLabel::Item, Box::new(reference)),
        Labeled::with_builtin_label(BuiltInLabel::Separator, Box::new(separator)),
    );

    if item.allow_empty.unwrap_or_default() {
        ParserDefinitionNode::Optional(Box::new(separated)).versioned(item.enabled)
    } else {
        separated.versioned(item.enabled)
    }
}

fn resolve_precedence(
    item: model::PrecedenceItem,
    lex_ctx: &Identifier,
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
        _ => ParserDefinitionNode::Choice(Labeled::with_builtin_label(
            BuiltInLabel::Variant,
            primaries,
        )),
    });

    let mut operators = vec![];
    let mut precedence_expression_names = Vec::with_capacity(item.precedence_expressions.len());
    for expr in item.precedence_expressions {
        let name = &expr.name;
        let name = name.clone();

        precedence_expression_names.push(name.clone());
        // Register it as a regular parser with a given name, however we need to
        // define it as a choice over the "operator" sequences
        // Then, when returning, we should actually return a node ref pointing to that combined parser
        // And ideally, we shouldn't even use the "enabled" mode of the original DSL
        let thunk = Rc::new(NamedParserThunk {
            name: name.clone(),
            context: lex_ctx.clone(),
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
                [ParserDefinitionNode::Versioned(..)] => {
                    ParserDefinitionNode::Choice(Labeled::anonymous(defs))
                }
                [_] => defs.into_iter().next().unwrap(),
                // NOTE: We give empty names to not ovewrite the names of the flattened fields of the operators
                _ => ParserDefinitionNode::Choice(Labeled::anonymous(defs)),
            };

            all_operators.push(def.clone());
            operators.push((model, name.clone(), def));
        }

        // Register the combined parser definition to appease the codegen and to mark terminals
        // as reachable and ensure we emit a token kind for each
        thunk
            .def
            .set(ParserDefinitionNode::Choice(Labeled::anonymous(
                all_operators,
            )))
            .unwrap();
        assert!(
            !ctx.resolved.contains_key(&name),
            "Encountered a duplicate Precedence Expression named {name} when resolving"
        );
        ctx.resolved.insert(
            name.clone(),
            GrammarElement::ParserDefinition(Rc::clone(&thunk) as Rc<dyn ParserDefinition>),
        );
    }

    PrecedenceParserDefinitionNode {
        primary_expression,
        operators,
        precedence_expression_names,
    }
}

#[derive(Clone, Copy)]
enum TriviaKind {
    Leading,
    Trailing,
}

impl TriviaKind {
    fn label(self) -> BuiltInLabel {
        match self {
            TriviaKind::Leading => BuiltInLabel::LeadingTrivia,
            TriviaKind::Trailing => BuiltInLabel::TrailingTrivia,
        }
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
            GrammarElement::KeywordScannerDefinition(scanner) => {
                ParserDefinitionNode::KeywordScannerDefinition(scanner)
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
            Self::Versioned(Box::new(self), enabled)
        } else {
            self
        }
    }
}

trait LabeledExt<T> {
    fn anonymous(node: T) -> Self;
    fn with_ident_name(name: Identifier, node: T) -> Self;
    fn with_builtin_label(name: BuiltInLabel, node: T) -> Self;
}

#[allow(dead_code)]
#[derive(strum_macros::AsRefStr)]
#[strum(serialize_all = "snake_case")]
enum BuiltInLabel {
    // _SLANG_INTERNAL_RESERVED_NODE_LABELS_ (keep in sync)
    Item,
    Variant,
    Separator,
    Operand,
    LeftOperand,
    RightOperand,
    LeadingTrivia,
    TrailingTrivia,
}

impl<T> LabeledExt<T> for Labeled<T> {
    fn anonymous(value: T) -> Self {
        Self {
            label: String::new(),
            value,
        }
    }

    fn with_ident_name(name: Identifier, value: T) -> Self {
        Self {
            label: name.to_string(),
            value,
        }
    }

    fn with_builtin_label(label: BuiltInLabel, value: T) -> Self {
        Self {
            label: label.as_ref().to_owned(),
            value,
        }
    }
}
