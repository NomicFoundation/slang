//! Defines a translation of DSL v2 model into [`Grammar`], which is used for generating the parser and the CST.

use std::cell::OnceCell;
use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::LazyLock;

use indexmap::IndexMap;
use language_definition::model::{
    self, FieldsErrorRecovery, Identifier, Item, Language, PredefinedLabel,
};

use crate::parser::grammar::{
    DelimitedRecoveryTerminalThreshold, Grammar, GrammarElement, Labeled, ParserDefinition,
    ParserDefinitionNode, PrecedenceParserDefinition, PrecedenceParserDefinitionNode,
    TriviaParserDefinition,
};

static DEFAULT_LEX_CTXT: LazyLock<Identifier> = LazyLock::new(|| Identifier::from("Default"));

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
        &DEFAULT_LEX_CTXT
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

pub struct ResolveCtx {
    items: HashMap<Identifier, (Identifier, Item)>,
    resolved: HashMap<Identifier, GrammarElement>,
}

pub struct Resolution {
    /// Original items as defined by the DSL v2.
    items: HashMap<Identifier, (Identifier, Item)>,
    resolved: HashMap<Identifier, GrammarElement>,
}

impl ResolveCtx {
    pub fn resolve(lang: &Language) -> Resolution {
        // Collect language items into a lookup table to speed up resolution
        let items: HashMap<_, _> = lang
            .topics()
            .flat_map(|topic| {
                topic.items.iter().map(|item| {
                    let lex_ctxt = topic.lexical_context.as_ref().unwrap_or(&DEFAULT_LEX_CTXT);

                    (item.name().clone(), (lex_ctxt.clone(), item.clone()))
                })
            })
            .collect();

        let mut ctx = ResolveCtx {
            items,
            resolved: HashMap::new(),
        };

        for item in lang.items() {
            resolve_grammar_element(item.name(), &mut ctx);
        }

        // Trivia is defined separately from the main grammar
        let leading_trivia = Rc::new(NamedTriviaParser {
            name: Identifier::from("LeadingTrivia"),
            def: resolve_trivia(lang.leading_trivia.clone(), TriviaKind::Leading, &mut ctx),
        });

        let trailing_trivia = Rc::new(NamedTriviaParser {
            name: Identifier::from("TrailingTrivia"),
            def: resolve_trivia(lang.trailing_trivia.clone(), TriviaKind::Trailing, &mut ctx),
        });

        for trivia in [leading_trivia, trailing_trivia] {
            ctx.resolved.insert(
                trivia.name().clone(),
                GrammarElement::TriviaParserDefinition(trivia),
            );
        }

        Resolution {
            items: ctx.items,
            resolved: ctx.resolved,
        }
    }
}

impl Resolution {
    /// Returns the lexical context in which the item was defined.
    pub fn lex_ctx(&self, name: &Identifier) -> &Identifier {
        &self.items[name].0
    }

    /// Returns the resolved items.
    pub fn items(&self) -> impl Iterator<Item = (&Identifier, &GrammarElement)> {
        self.resolved.iter()
    }

    /// Collects the already resolved item into a [`Grammar`].
    pub fn to_grammar(&self) -> Grammar {
        Grammar {
            elements: self
                .resolved
                .iter()
                .map(|(name, elem)| (name.clone(), elem.clone()))
                .collect(),
        }
    }
}

/// Inserts a thunk for the given item in a context to be resolved later and returns it if it was inserted.
fn insert_parser_thunk(
    item: &Item,
    lex_ctx: &Identifier,
    ctx: &mut ResolveCtx,
) -> Option<ParserThunk> {
    match (item, ctx.resolved.contains_key(item.name())) {
        (Item::Precedence { .. }, false) => {
            let thunk = Rc::new(NamedPrecedenceParserThunk {
                name: item.name().clone(),
                context: lex_ctx.clone(),
                def: OnceCell::new(),
            });
            ctx.resolved.insert(
                item.name().clone(),
                (Rc::clone(&thunk) as Rc<dyn PrecedenceParserDefinition>).into(),
            );
            Some(ParserThunk::Precedence(thunk))
        }
        (item, false) if item.is_nonterminal() => {
            let thunk = Rc::new(NamedParserThunk {
                name: item.name().clone(),
                context: lex_ctx.clone(),
                is_inline: false,
                def: OnceCell::new(),
            });
            ctx.resolved.insert(
                item.name().clone(),
                (Rc::clone(&thunk) as Rc<dyn ParserDefinition>).into(),
            );
            Some(ParserThunk::Regular(thunk))
        }
        _ => None,
    }
}

fn resolve_grammar_element(ident: &Identifier, ctx: &mut ResolveCtx) -> GrammarElement {
    let (lex_ctx, elem) = ctx.items.get(ident).cloned().expect("Missing item");

    // The nonterminals are mutually recursive (so will be the resolution of their definitions),
    // so make sure to insert a thunk for nonterminals to resolve to break the cycle.
    let inserted_thunk = insert_parser_thunk(&elem, &lex_ctx, ctx);

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
                _ => unreachable!("{ident}: Only nonterminals can be resolved here"),
            }

            ctx.resolved.get(ident).cloned().unwrap()
        }
        // First time resolving a terminal named `ident`
        (None, None) => {
            let named_scanner = match elem {
                Item::Keyword { item } => {
                    // Keywords are special scanners and are handled separately
                    let resolved = GrammarElement::KeywordScannerDefinition(item as Rc<_>);
                    ctx.resolved.insert(ident.clone(), resolved.clone());
                    return resolved;
                }
                Item::Token { item } => item as Rc<_>,
                Item::Trivia { item } => item as Rc<_>,
                Item::Fragment { item } => item as Rc<_>,
                _ => unreachable!("{ident}: Only terminals can be resolved here"),
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
    ctx: &mut ResolveCtx,
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
                    ParserDefinitionNode::Sequence(vec![Labeled::with_predefined_label(
                        kind.label(),
                        ParserDefinitionNode::ScannerDefinition(parser),
                    )])
                }
                _ => panic!("Expected {reference} to be a ScannerDefinition"),
            }
        }
    }
}

fn resolve_field(field: model::Field, ctx: &mut ResolveCtx) -> ParserDefinitionNode {
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
    ctx: &mut ResolveCtx,
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

fn resolve_choice(item: model::EnumItem, ctx: &mut ResolveCtx) -> ParserDefinitionNode {
    let variants = item
        .variants
        .into_iter()
        .map(|variant| {
            resolve_grammar_element(&variant.reference, ctx)
                .into_parser_def_node()
                .versioned(variant.enabled)
        })
        .collect();

    ParserDefinitionNode::Choice(Labeled::with_predefined_label(
        PredefinedLabel::Variant,
        variants,
    ))
    .versioned(item.enabled)
}

fn resolve_repeated(item: model::RepeatedItem, ctx: &mut ResolveCtx) -> ParserDefinitionNode {
    let reference = Box::new(resolve_grammar_element(&item.reference, ctx).into_parser_def_node());

    let repeated = Labeled::with_predefined_label(PredefinedLabel::Item, reference);

    if item.allow_empty.unwrap_or_default() {
        ParserDefinitionNode::ZeroOrMore(repeated).versioned(item.enabled)
    } else {
        ParserDefinitionNode::OneOrMore(repeated).versioned(item.enabled)
    }
}

fn resolve_separated(item: model::SeparatedItem, ctx: &mut ResolveCtx) -> ParserDefinitionNode {
    let reference = resolve_grammar_element(&item.reference, ctx).into_parser_def_node();
    let separator = resolve_grammar_element(&item.separator, ctx).into_parser_def_node();

    let separated = ParserDefinitionNode::SeparatedBy(
        Labeled::with_predefined_label(PredefinedLabel::Item, Box::new(reference)),
        Labeled::with_predefined_label(PredefinedLabel::Separator, Box::new(separator)),
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
    ctx: &mut ResolveCtx,
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
        0 => panic!(
            "Precedence operator {item} has no primary expressions",
            item = item.name
        ),
        _ => ParserDefinitionNode::Choice(Labeled::with_predefined_label(
            PredefinedLabel::Variant,
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
                // NOTE: We give empty names to not overwrite the names of the flattened fields of the operators
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
    fn label(self) -> PredefinedLabel {
        match self {
            TriviaKind::Leading => PredefinedLabel::LeadingTrivia,
            TriviaKind::Trailing => PredefinedLabel::TrailingTrivia,
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
    fn with_predefined_label(name: PredefinedLabel, node: T) -> Self;
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

    fn with_predefined_label(label: PredefinedLabel, value: T) -> Self {
        Self {
            label: label.as_ref().to_owned(),
            value,
        }
    }
}
