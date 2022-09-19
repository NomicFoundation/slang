use std::{cell::Cell, collections::BTreeSet, rc::Rc};

use inflector::Inflector;
use mset::MultiSet;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use codegen_schema::*;

use super::{
    character_filter::{CharacterFilter, CharacterFilterRef},
    combinator_forest::CombinatorForest,
    combinator_tree::CombinatorTree,
    naming,
    terminal_trie::TerminalTrie,
};

/// All names are snake case

#[derive(Clone, Debug)]
pub enum CombinatorNode {
    CharacterFilter {
        name: String,
        filter: CharacterFilterRef,
    },
    Choice {
        name: String,
        elements: Vec<CombinatorNodeRef>,
    },
    DelimitedBy {
        name: String,
        open: String,
        expr: CombinatorNodeRef,
        close: String,
    },
    Difference {
        name: String,
        minuend: CombinatorNodeRef,
        subtrahend: CombinatorNodeRef,
    },
    Expression {
        name: String,
        members: Vec<ProductionRef>,
    },
    ExpressionMember {
        name: String,
        parent: ProductionRef,
        next_sibling: Option<ProductionRef>,
        operator: CombinatorNodeRef,
        operator_model: OperatorModel,
    },
    Lookahead {
        name: String,
        expr: CombinatorNodeRef,
        lookahead: CombinatorNodeRef,
    },
    OneOrMore {
        name: String,
        expr: CombinatorNodeRef,
    },
    Optional {
        name: String,
        expr: CombinatorNodeRef,
    },
    Reference {
        name: String,
        production: ProductionRef,
    },
    Repeated {
        name: String,
        expr: CombinatorNodeRef,
        min: usize,
        max: usize,
    },
    SeparatedBy {
        name: String,
        expr: CombinatorNodeRef,
        separator: String,
    },
    Sequence {
        name: String,
        elements: Vec<CombinatorNodeRef>,
    },
    TerminalTrie {
        name: String,
        trie: TerminalTrie,
    },
    ZeroOrMore {
        name: String,
        expr: CombinatorNodeRef,
    },
}

pub type CombinatorNodeRef = Rc<CombinatorNode>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorModel {
    None,
    BinaryLeftAssociative,
    BinaryRightAssociative,
    UnaryPrefix,
    UnarySuffix,
}

#[derive(Default)]
pub struct CodeForNode {
    pub cst_rule_kinds: BTreeSet<Ident>,
    pub cst_token_kinds: BTreeSet<Ident>,
    pub cst_token_part_kinds: BTreeSet<Ident>,
    pub cst_parser_impl_fragment: TokenStream,

    pub ast_fragment: Vec<TokenStream>,
    pub ast_impl_fragment: Vec<TokenStream>,
    pub ast_parser_type: TokenStream,
    pub ast_parser_impl_fragment: TokenStream,
}

impl CombinatorNode {
    pub fn from_expression(
        grammar: &GrammarRef,
        production: &ProductionRef,
        expression: &ExpressionRef,
        inherited_name: Option<String>,
        anonymous_counter: &Cell<usize>,
    ) -> CombinatorNodeRef {
        if let Some(filter) = CharacterFilter::from_expression(grammar, expression) {
            let name = expression
                .config
                .name
                .clone()
                .map(|s| s.to_snake_case())
                .or(inherited_name)
                .or(filter.default_name())
                .unwrap_or_else(|| naming::anonymous_filter(anonymous_counter));
            return Self::new_character_filter(name, filter);
        } else if let Some(terminal_trie) = TerminalTrie::from_expression(grammar, expression) {
            let name = expression
                .config
                .name
                .clone()
                .map(|s| s.to_snake_case())
                .or(inherited_name)
                .or(terminal_trie.default_name())
                .unwrap_or_else(|| naming::anonymous_terminals(anonymous_counter));
            return Self::new_terminal_trie(name, terminal_trie);
        } else {
            match &expression.ebnf {
                EBNF::Choice(exprs) => {
                    let name = expression
                        .config
                        .name
                        .clone()
                        .map(|s| s.to_snake_case())
                        .or(inherited_name)
                        .unwrap_or_else(|| naming::anonymous_choice(anonymous_counter));

                    if production.kind == Some(ProductionKind::ExpressionRule) {
                        let choices = exprs.iter().map(|e| {
                            if let EBNF::Reference(prod_name) = &e.ebnf {
                               grammar.get_production(prod_name)
                            } else {
                                unreachable!("Validation should have checked this: The Expression pattern is only applicable to a choice of references")
                            }
                        }).collect();
                        return Self::new_expression(name, choices);
                    }

                    if expression.config.merge != Some(true) {
                        let choices = exprs
                            .iter()
                            .map(|e| {
                                Self::from_expression(
                                    grammar,
                                    production,
                                    e,
                                    None,
                                    anonymous_counter,
                                )
                            })
                            .collect();
                        return Self::new_choice(name, choices);
                    }

                    // Merge runs of TerminalTrees and CharacterFilters

                    let mut choices: Vec<CombinatorNodeRef> = vec![];
                    {
                        let mut current_terminal_tree: Option<TerminalTrie> = None;
                        let mut current_character_filter: Option<CharacterFilterRef> = None;
                        for e in exprs {
                            // Sub-expressions with a user-specified name aren't merged
                            if e.config.name.is_none() {
                                if let Some(cf) = CharacterFilter::from_expression(grammar, e) {
                                    if let Some(ctt) = current_terminal_tree {
                                        choices.push({
                                            let name = ctt.default_name().unwrap_or_else(|| {
                                                naming::anonymous_terminals(anonymous_counter)
                                            });
                                            Self::new_terminal_trie(name, ctt)
                                        });
                                        current_terminal_tree = None
                                    };
                                    if let Some(ccf) = current_character_filter {
                                        current_character_filter = Some(ccf.merged_with(cf))
                                    } else {
                                        current_character_filter = Some(cf)
                                    }
                                    continue;
                                }
                                if let Some(tt) = TerminalTrie::from_expression(grammar, e) {
                                    if let Some(ccf) = current_character_filter {
                                        choices.push({
                                            let name = ccf.default_name().unwrap_or_else(|| {
                                                naming::anonymous_filter(anonymous_counter)
                                            });
                                            Self::new_character_filter(name, ccf)
                                        });
                                        current_character_filter = None
                                    };
                                    if let Some(ctt) = current_terminal_tree.as_mut() {
                                        ctt.merge_with(tt)
                                    } else {
                                        current_terminal_tree = Some(tt)
                                    }
                                    continue;
                                }
                            }

                            if let Some(ccf) = current_character_filter {
                                choices.push({
                                    let name = ccf.default_name().unwrap_or_else(|| {
                                        naming::anonymous_filter(anonymous_counter)
                                    });
                                    Self::new_character_filter(name, ccf)
                                });
                                current_character_filter = None
                            };

                            if let Some(ctt) = current_terminal_tree {
                                choices.push({
                                    let name = ctt.default_name().unwrap_or_else(|| {
                                        naming::anonymous_terminals(anonymous_counter)
                                    });
                                    Self::new_terminal_trie(name, ctt)
                                });
                                current_terminal_tree = None
                            };

                            choices.push(Self::from_expression(
                                grammar,
                                production,
                                e,
                                None,
                                anonymous_counter,
                            ))
                        }

                        if let Some(ccf) = current_character_filter {
                            choices.push({
                                let name = ccf
                                    .default_name()
                                    .unwrap_or_else(|| naming::anonymous_filter(anonymous_counter));
                                Self::new_character_filter(name, ccf)
                            });
                        };

                        if let Some(ctt) = current_terminal_tree {
                            choices.push({
                                let name = ctt.default_name().unwrap_or_else(|| {
                                    naming::anonymous_terminals(anonymous_counter)
                                });
                                Self::new_terminal_trie(name, ctt)
                            });
                        };
                    }

                    Self::new_choice(name, choices)
                }

                EBNF::DelimitedBy(EBNFDelimitedBy { open, expr, close }) => {
                    let expr =
                        Self::from_expression(grammar, production, expr, None, anonymous_counter);

                    let name = inherited_name.unwrap_or_else(|| {
                        format!(
                            "{}_and_{}_and_{}",
                            naming::name_of_terminal_string(open),
                            expr.name(),
                            naming::name_of_terminal_string(close)
                        )
                    });

                    {
                        let open = open.clone();
                        let close = close.clone();
                        Self::new_delimited_by(name, open, expr, close)
                    }
                }

                EBNF::Difference(EBNFDifference {
                    minuend,
                    subtrahend,
                }) => {
                    let minuend = Self::from_expression(
                        grammar,
                        production,
                        minuend,
                        None,
                        anonymous_counter,
                    );
                    let subtrahend = Self::from_expression(
                        grammar,
                        production,
                        subtrahend,
                        None,
                        anonymous_counter,
                    );
                    let name = expression
                        .config
                        .name
                        .clone()
                        .map(|s| s.to_snake_case())
                        .or(inherited_name)
                        .unwrap_or_else(|| minuend.name());
                    Self::new_difference(name, minuend, subtrahend)
                }

                EBNF::Not(_) => unimplemented!("¬ is only supported on characters or sets thereof"),

                EBNF::OneOrMore(expr) => {
                    let expr =
                        Self::from_expression(grammar, production, expr, None, anonymous_counter);
                    let name = expression
                        .config
                        .name
                        .clone()
                        .map(|s| s.to_snake_case())
                        .or(inherited_name)
                        .unwrap_or_else(|| naming::pluralize(&expr.name()));
                    Self::new_one_or_more(name, expr)
                }

                EBNF::Optional(expr) => {
                    let expr = Self::from_expression(
                        grammar,
                        production,
                        expr,
                        inherited_name,
                        anonymous_counter,
                    );
                    let name = expression
                        .config
                        .name
                        .clone()
                        .map(|s| s.to_snake_case())
                        .unwrap_or_else(|| expr.name());
                    Self::new_optional(name, expr)
                }

                EBNF::Range(_) => unreachable!("Ranges are always character filters"),

                EBNF::Reference(name) => {
                    Self::new_reference(name.to_snake_case(), grammar.get_production(&name))
                }

                EBNF::Repeat(EBNFRepeat { expr, min, max }) => {
                    let expr =
                        Self::from_expression(grammar, production, expr, None, anonymous_counter);
                    let name = expression
                        .config
                        .name
                        .clone()
                        .map(|s| s.to_snake_case())
                        .or(inherited_name)
                        .unwrap_or_else(|| naming::pluralize(&expr.name()));
                    {
                        let min = *min;
                        let max = *max;
                        Self::new_repeat(name, expr, min, max)
                    }
                }

                EBNF::SeparatedBy(EBNFSeparatedBy { expr, separator }) => {
                    let expr =
                        Self::from_expression(grammar, production, expr, None, anonymous_counter);
                    let name = expression
                        .config
                        .name
                        .clone()
                        .map(|s| s.to_snake_case())
                        .or(inherited_name)
                        .unwrap_or_else(|| {
                            format!(
                                "{}_and_{}",
                                naming::pluralize(&expr.name()),
                                naming::pluralize(&naming::name_of_terminal_string(separator))
                            )
                        });
                    {
                        let separator = separator.clone();
                        Self::new_separated_by(name, expr, separator)
                    }
                }

                EBNF::Sequence(exprs) => {
                    let name = expression
                        .config
                        .name
                        .clone()
                        .map(|s| s.to_snake_case())
                        .or(inherited_name)
                        .unwrap_or_else(|| naming::anonymous_sequence(anonymous_counter));
                    let mut exprs = exprs
                        .iter()
                        .map(|e| {
                            Self::from_expression(grammar, production, e, None, anonymous_counter)
                        })
                        .collect();
                    disambiguate_node_names(&mut exprs);
                    Self::new_sequence(name, exprs)
                }

                EBNF::Terminal(_) => {
                    unreachable!("Terminals are either character filters or terminal tries")
                }

                EBNF::ZeroOrMore(expr) => {
                    let expr =
                        Self::from_expression(grammar, production, expr, None, anonymous_counter);
                    let name = expression
                        .config
                        .name
                        .clone()
                        .map(|s| s.to_snake_case())
                        .or(inherited_name)
                        .unwrap_or_else(|| naming::pluralize(&expr.name()));
                    Self::new_zero_or_more(name, expr)
                }
            }
        }
    }

    pub fn new_character_filter(name: String, filter: CharacterFilterRef) -> CombinatorNodeRef {
        Rc::new(Self::CharacterFilter { name, filter })
    }

    pub fn new_choice(name: String, elements: Vec<CombinatorNodeRef>) -> CombinatorNodeRef {
        Rc::new(Self::Choice { name, elements })
    }

    pub fn new_delimited_by(
        name: String,
        open: String,
        expr: CombinatorNodeRef,
        close: String,
    ) -> CombinatorNodeRef {
        Rc::new(Self::DelimitedBy {
            name,
            open,
            expr,
            close,
        })
    }

    pub fn new_difference(
        name: String,
        minuend: CombinatorNodeRef,
        subtrahend: CombinatorNodeRef,
    ) -> CombinatorNodeRef {
        Rc::new(Self::Difference {
            name,
            minuend,
            subtrahend,
        })
    }

    pub fn new_expression(name: String, members: Vec<ProductionRef>) -> CombinatorNodeRef {
        Rc::new(Self::Expression { name, members })
    }

    pub fn new_expression_member(
        name: String,
        parent: ProductionRef,
        next_sibling: Option<ProductionRef>,
        operator: CombinatorNodeRef,
        operator_model: OperatorModel,
    ) -> CombinatorNodeRef {
        Rc::new(Self::ExpressionMember {
            name,
            parent,
            next_sibling,
            operator,
            operator_model,
        })
    }

    pub fn new_lookahead(
        name: String,
        expr: CombinatorNodeRef,
        lookahead: CombinatorNodeRef,
    ) -> CombinatorNodeRef {
        Rc::new(Self::Lookahead {
            name,
            expr,
            lookahead,
        })
    }

    pub fn new_one_or_more(name: String, expr: CombinatorNodeRef) -> CombinatorNodeRef {
        Rc::new(Self::OneOrMore { name, expr })
    }

    pub fn new_optional(name: String, expr: CombinatorNodeRef) -> CombinatorNodeRef {
        Rc::new(Self::Optional { name, expr })
    }

    pub fn new_reference(name: String, production: ProductionRef) -> CombinatorNodeRef {
        Rc::new(Self::Reference { name, production })
    }

    pub fn new_repeat(
        name: String,
        expr: CombinatorNodeRef,
        min: usize,
        max: usize,
    ) -> CombinatorNodeRef {
        Rc::new(Self::Repeated {
            name,
            expr,
            min,
            max,
        })
    }

    pub fn new_separated_by(
        name: String,
        expr: CombinatorNodeRef,
        separator: String,
    ) -> CombinatorNodeRef {
        Rc::new(Self::SeparatedBy {
            name,
            expr,
            separator,
        })
    }

    pub fn new_sequence(name: String, elements: Vec<CombinatorNodeRef>) -> CombinatorNodeRef {
        Rc::new(Self::Sequence { name, elements })
    }

    pub fn new_terminal_trie(name: String, trie: TerminalTrie) -> CombinatorNodeRef {
        Rc::new(Self::TerminalTrie { name, trie })
    }

    pub fn new_zero_or_more(name: String, expr: CombinatorNodeRef) -> CombinatorNodeRef {
        Rc::new(Self::ZeroOrMore { name, expr })
    }

    pub fn name(&self) -> String {
        match self {
            Self::CharacterFilter { name, .. }
            | Self::Choice { name, .. }
            | Self::DelimitedBy { name, .. }
            | Self::Difference { name, .. }
            | Self::Expression { name, .. }
            | Self::ExpressionMember { name, .. }
            | Self::Lookahead { name, .. }
            | Self::OneOrMore { name, .. }
            | Self::Optional { name, .. }
            | Self::Reference { name, .. }
            | Self::Repeated { name, .. }
            | Self::SeparatedBy { name, .. }
            | Self::Sequence { name, .. }
            | Self::TerminalTrie { name, .. }
            | Self::ZeroOrMore { name, .. } => name.clone(),
        }
    }

    pub fn has_default(&self, forest: &CombinatorForest) -> bool {
        match self {
            Self::CharacterFilter { .. }
            | Self::OneOrMore { .. }
            | Self::Optional { .. }
            | Self::Repeated { .. }
            | Self::SeparatedBy { .. }
            | Self::TerminalTrie { .. }
            | Self::ZeroOrMore { .. } => true,

            Self::Choice { .. } | Self::Expression { .. } => false,

            Self::DelimitedBy { expr, .. }
            | Self::Difference { minuend: expr, .. }
            | Self::Lookahead { expr, .. } => expr.has_default(forest),

            Self::ExpressionMember {
                operator,
                operator_model,
                ..
            } => *operator_model == OperatorModel::None && operator.has_default(forest),

            Self::Reference { production, .. } => forest
                .trees_by_production_name
                .get(&production.name)
                .expect("Production does not exist - should have been caught by the validator")
                .root_node
                .has_default(forest),

            Self::Sequence { elements, .. } => elements.iter().all(|e| e.has_default(forest)),
        }
    }

    pub fn to_generated_code(
        &self,
        forest: &CombinatorForest,
        tree: &CombinatorTree,
    ) -> CodeForNode {
        match self {
            Self::CharacterFilter { filter, name } => filter.to_generated_code(
                if naming::is_anonymous(name) {
                    None
                } else {
                    Some(naming::to_kind_ident(&tree.name, name))
                },
                !tree.production.is_token(),
            ),

            Self::Choice { name, elements } => {
                let mut result: CodeForNode = Default::default();

                let module_name = naming::to_module_name_ident(&tree.name);
                let type_name = naming::to_type_name_ident(&name);

                let mut cst_parsers = vec![];
                let mut ast_fields = vec![];
                let mut ast_parsers = vec![];
                for element in elements {
                    result.merge(element.to_generated_code(forest, tree));

                    let name = naming::to_enum_tag_ident(&element.name());
                    let ast_parser_type = result.ast_parser_type.clone();
                    ast_fields.push(quote!( #name(#ast_parser_type) ));

                    let cst_parser = result.cst_parser_impl_fragment.clone();
                    cst_parsers.push(cst_parser);

                    let ast_parser = result.ast_parser_impl_fragment.clone();
                    ast_parsers.push(
                        quote!( #ast_parser.map(|v| Box::new(#module_name::#type_name::#name(v))) ),
                    );
                }

                result.ast_fragment.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub enum #type_name { #(#ast_fields),* }
                ));

                if naming::is_anonymous(&name) {
                    result.cst_parser_impl_fragment = quote!( choice(( #(#cst_parsers),* )) );
                } else {
                    let kind = naming::to_kind_ident(&tree.name, &name);
                    result.cst_rule_kinds.insert(kind.clone());
                    result.cst_parser_impl_fragment = quote!( choice(( #(#cst_parsers),* )).map(|v| Node::new_rule(RuleKind::#kind, vec![v])) );
                }

                result.ast_parser_impl_fragment = quote!( choice(( #(#ast_parsers),* )) );
                result.ast_parser_type = quote!( Box<#module_name::#type_name> );

                result
            }

            Self::DelimitedBy {
                name,
                open,
                expr,
                close,
            } => {
                let module_name = naming::to_module_name_ident(&tree.name);
                let type_name = naming::to_type_name_ident(&name);

                let (open_ast_parser, open_ast_type, mut open_name) =
                    generated_ast_code_for_string(open, !tree.production.is_token());
                let (close_ast_parser, close_ast_type, mut close_name) =
                    generated_ast_code_for_string(close, !tree.production.is_token());

                if open_name == close_name {
                    open_name = format!("opening_{}", open_name);
                    close_name = format!("closing_{}", close_name);
                }

                let open_name = naming::to_field_name_ident(&open_name);
                let expr_name = naming::to_field_name_ident(&expr.name());
                let close_name = naming::to_field_name_ident(&close_name);

                let mut result = expr.to_generated_code(forest, tree);
                let expr_annotation = if self.has_default(forest) {
                    result.ast_impl_fragment.push(quote!(
                        impl Default for #module_name::#type_name {
                            fn default() -> Self {
                                Self {
                                    #open_name: Default::default(),
                                    #expr_name: Default::default(),
                                    #close_name: Default::default(),
                                }
                            }
                        }
                        impl DefaultTest for #module_name::#type_name {
                            fn is_default(&self) -> bool {
                                self.#open_name.is_default()
                                && self.#expr_name.is_default()
                                && self.#close_name.is_default()
                            }
                        }
                    ));

                    quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                } else {
                    quote!()
                };

                let expr_type = result.ast_parser_type;

                result.ast_fragment.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub struct #type_name {
                        #[serde(default, skip_serializing_if="DefaultTest::is_default")]
                        pub #open_name: #open_ast_type,
                        #expr_annotation
                        pub #expr_name: #expr_type,
                        #[serde(default, skip_serializing_if="DefaultTest::is_default")]
                        pub #close_name: #close_ast_type,
                    }
                ));

                result.ast_impl_fragment.push(quote!(
                    impl #module_name::#type_name {
                        pub fn from_parse(((#open_name, #expr_name), #close_name): ((#open_ast_type, #expr_type), #close_ast_type)) -> Self {
                            Self { #open_name, #expr_name, #close_name }
                        }
                    }
                ));

                let (open_cst_parser, open_kind) =
                    generated_cst_code_for_string(open, !tree.production.is_token());
                result.cst_token_part_kinds.insert(open_kind.clone());
                let (close_cst_parser, close_kind) =
                    generated_cst_code_for_string(close, !tree.production.is_token());
                result.cst_token_part_kinds.insert(close_kind.clone());

                let expr_parser = result.cst_parser_impl_fragment;
                if naming::is_anonymous(&name) {
                    result.cst_parser_impl_fragment = quote!( #open_cst_parser.then(#expr_parser).then(#close_cst_parser).map(|((a, b), c)| Node::new_anonymous_rule(vec![a, b, c])) );
                } else {
                    let kind = naming::to_kind_ident(&tree.name, &name);
                    result.cst_rule_kinds.insert(kind.clone());
                    result.cst_parser_impl_fragment = quote!( #open_cst_parser.then(#expr_parser).then(#close_cst_parser).map(|((a, b), c)| Node::new_rule(RuleKind::#kind, vec![a, b, c])) );
                }

                let expr_parser = result.ast_parser_impl_fragment;
                result.ast_parser_impl_fragment = quote!( #open_ast_parser.then(#expr_parser).then(#close_ast_parser).map(|v| #module_name::#type_name::from_parse(v)) );
                result.ast_parser_type = quote!( #module_name::#type_name );

                result
            }

            Self::Difference {
                minuend,
                subtrahend,
                ..
            } => {
                let mut minuend = minuend.to_generated_code(forest, tree);
                let subtrahend = subtrahend.to_generated_code(forest, tree);

                let minuend_parser = minuend.cst_parser_impl_fragment;
                let subtrahend_parser = subtrahend.cst_parser_impl_fragment;
                minuend.cst_parser_impl_fragment =
                    quote! ( difference(#minuend_parser, #subtrahend_parser) );

                let minuend_parser = minuend.ast_parser_impl_fragment;
                let subtrahend_parser = subtrahend.ast_parser_impl_fragment;
                minuend.ast_parser_impl_fragment =
                    quote! ( difference(#minuend_parser, #subtrahend_parser) );

                minuend
            }

            Self::Expression { name, members } => {
                let mut result: CodeForNode = Default::default();

                let module_name = naming::to_module_name_ident(&tree.name);
                let type_name = naming::to_type_name_ident(&name);

                let names = members
                    .iter()
                    .map(|p| p.name.to_snake_case())
                    .collect::<Vec<_>>();

                let fields = names
                    .iter()
                    .map(|name| {
                        let tag_name = naming::to_enum_tag_ident(&name);
                        let module_name = naming::to_module_name_ident(&name);
                        quote!( #tag_name(#module_name::E) )
                    })
                    .collect::<Vec<_>>();
                result.ast_fragment.push(quote!(
                   #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                   pub enum #type_name { #(#fields),* }
                ));

                let first_parser_name = naming::to_parser_name_ident(&names[0]);
                let kind = naming::to_kind_ident(&tree.name, &name);
                result.cst_rule_kinds.insert(kind.clone());
                result.cst_parser_impl_fragment = quote!( #first_parser_name.clone().map(|v| Node::new_rule(RuleKind::#kind, vec![v])) );
                result.ast_parser_impl_fragment = quote!( #first_parser_name.clone() );
                result.ast_parser_type = quote!( Box<#module_name::#type_name> );

                result
            }

            Self::ExpressionMember {
                name,
                parent,
                next_sibling,
                operator,
                operator_model,
            } => {
                let mut result = operator.to_generated_code(forest, tree);

                let operator_cst_parser = result.cst_parser_impl_fragment;
                let operator_ast_type = result.ast_parser_type;
                let operator_ast_parser = result.ast_parser_impl_fragment;

                let tag_name = naming::to_enum_tag_ident(&name);
                let module_name = naming::to_module_name_ident(&name);
                let parent_name = parent.name.to_snake_case();
                let parent_type_name = naming::to_type_name_ident(&parent_name);
                let parent_module_name = naming::to_module_name_ident(&parent_name);

                match operator_model {
                    OperatorModel::None => {
                        result.cst_parser_impl_fragment = operator_cst_parser;
                        result.ast_parser_impl_fragment = match next_sibling {
                            Some(next_sibling) => {
                                let next_sibling_parser_name = naming::to_parser_name_ident(
                                    &next_sibling.name.to_snake_case(),
                                );
                                quote!(
                                    choice((
                                        #operator_ast_parser.map(|v| Box::new(#parent_module_name::#parent_type_name::#tag_name(v))),
                                        #next_sibling_parser_name.clone()
                                    ))
                                )
                            }
                            None => {
                                quote!( #operator_ast_parser.map(|v| Box::new(#parent_module_name::#parent_type_name::#tag_name(v))) )
                            }
                        };
                        result
                            .ast_fragment
                            .push(quote!( pub type E = #operator_ast_type; ));
                    }

                    OperatorModel::BinaryLeftAssociative => {
                        let next_sibling_parser_name = naming::to_parser_name_ident(
                            &next_sibling
                                .clone()
                                .expect("Cannot have binary operator as last expression member")
                                .name
                                .to_snake_case(),
                        );
                        let annotation = if operator.has_default(forest) {
                            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                        } else {
                            quote!()
                        };
                        result.ast_fragment.push(quote!(
                            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                            pub struct E {
                                pub left_operand: #parent_type_name,
                                #annotation
                                pub operator: #operator_ast_type,
                                pub right_operand: #parent_type_name,
                            }
                        ));

                        let kind = naming::to_kind_ident(&tree.name, &name);
                        result.cst_rule_kinds.insert(kind.clone());
                        result.cst_parser_impl_fragment = quote!(
                           #next_sibling_parser_name.clone()
                            .then(#operator_cst_parser.then(#next_sibling_parser_name.clone()).repeated())
                            .map(|(first_operand, operator_operand_pairs)|
                                if operator_operand_pairs.is_empty() {
                                    first_operand
                                } else {
                                    // a [ (X b) (Y c) (Z d) ] => { { { a X b } Y c } Z d }
                                    operator_operand_pairs.into_iter().fold(first_operand, |left_operand, (operator, right_operand)|
                                        Node::new_rule(RuleKind::#kind, vec![
                                            left_operand,
                                            operator,
                                            right_operand,
                                        ]))
                                }
                            )
                        );
                        result.ast_parser_impl_fragment = quote!(
                           #next_sibling_parser_name.clone()
                            .then(#operator_ast_parser.then(#next_sibling_parser_name.clone()).repeated())
                            .map(|(first_operand, operator_operand_pairs)|
                                if operator_operand_pairs.is_empty() {
                                    first_operand
                                } else {
                                    // a [ (X b) (Y c) (Z d) ] => { { { a X b } Y c } Z d }
                                    operator_operand_pairs.into_iter().fold(first_operand, |left_operand, (operator, right_operand)|
                                        Box::new(#parent_module_name::#parent_type_name::#tag_name(#module_name::E { left_operand, operator, right_operand }))
                                    )
                                }
                            )
                        );
                    }

                    OperatorModel::BinaryRightAssociative => {
                        let next_sibling_parser_name = naming::to_parser_name_ident(
                            &next_sibling
                                .clone()
                                .expect("Cannot have binary operator as last expression member")
                                .name
                                .to_snake_case(),
                        );
                        let annotation = if operator.has_default(forest) {
                            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                        } else {
                            quote!()
                        };
                        result.ast_fragment.push(quote!(
                            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                            pub struct E {
                                pub left_operand: #parent_type_name,
                                #annotation
                                pub operator: #operator_ast_type,
                                pub right_operand: #parent_type_name,
                            }
                        ));

                        let kind = naming::to_kind_ident(&tree.name, &name);
                        result.cst_rule_kinds.insert(kind.clone());
                        result.cst_parser_impl_fragment = quote!(
                           #next_sibling_parser_name.clone()
                            .then(#operator_cst_parser.then(#next_sibling_parser_name.clone()).repeated())
                            .map(|(first_operand, operator_operand_pairs)|
                                if operator_operand_pairs.is_empty() {
                                    first_operand
                                } else {
                                    // a [ (X b) (Y c) (Z d) ] => [ (a X) (b Y) (c Z) ] d
                                    let mut last_operand = first_operand;
                                    let mut operand_operator_pairs = vec![];
                                    for (operator, right_operand) in operator_operand_pairs.into_iter() {
                                        let left_operand = std::mem::replace(&mut last_operand, right_operand);
                                        operand_operator_pairs.push((left_operand, operator))
                                    }
                                    // [ (a X) (b Y) (c Z) ] d => { a X { b Y { c Z d } } }
                                    operand_operator_pairs.into_iter().rfold(last_operand, |right_operand, (left_operand, operator)|
                                        Node::new_rule(RuleKind::#kind, vec![
                                            left_operand,
                                            operator,
                                            right_operand,
                                        ])
                                    )
                                }
                            )
                        );
                        result.ast_parser_impl_fragment = quote!(
                           #next_sibling_parser_name.clone()
                            .then(#operator_ast_parser.then(#next_sibling_parser_name.clone()).repeated())
                            .map(|(first_operand, operator_operand_pairs)|
                                if operator_operand_pairs.is_empty() {
                                    first_operand
                                } else {
                                    // a [ (X b) (Y c) (Z d) ] => [ (a X) (b Y) (c Z) ] d
                                    let mut last_operand = first_operand;
                                    let mut operand_operator_pairs = vec![];
                                    for (operator, right_operand) in operator_operand_pairs.into_iter() {
                                        let left_operand = std::mem::replace(&mut last_operand, right_operand);
                                        operand_operator_pairs.push((left_operand, operator))
                                    }
                                    // [ (a X) (b Y) (c Z) ] d => { a X { b Y { c Z d } } }
                                    operand_operator_pairs.into_iter().rfold(last_operand, |right_operand, (left_operand, operator)|
                                        Box::new(#parent_module_name::#parent_type_name::#tag_name(#module_name::E { left_operand, operator, right_operand }))
                                    )
                                }
                            )
                        );
                    }

                    OperatorModel::UnaryPrefix => {
                        let next_sibling_parser_name = naming::to_parser_name_ident(
                            &next_sibling
                                .clone()
                                .expect(
                                    "Cannot have unary prefix operator as last expression member",
                                )
                                .name
                                .to_snake_case(),
                        );
                        let annotation = if operator.has_default(forest) {
                            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                        } else {
                            quote!()
                        };
                        result.ast_fragment.push(quote!(
                            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                            pub struct E {
                                #annotation
                                pub operator: #operator_ast_type,
                                pub right_operand: #parent_type_name,
                            }
                        ));

                        let kind = naming::to_kind_ident(&tree.name, &name);
                        result.cst_rule_kinds.insert(kind.clone());
                        result.cst_parser_impl_fragment = quote!(
                            #operator_cst_parser.repeated()
                            .then(#next_sibling_parser_name.clone())
                            .map(|(mut operators, operand)|
                                if operators.is_empty() {
                                    operand
                                } else {
                                    operators.reverse();
                                    operators.into_iter().fold(operand, |right_operand, operator|
                                        Node::new_rule(RuleKind::#kind, vec![
                                            operator,
                                            right_operand,
                                        ])
                                    )
                                }
                            )
                        );
                        result.ast_parser_impl_fragment = quote!(
                            #operator_ast_parser.repeated()
                            .then(#next_sibling_parser_name.clone())
                            .map(|(mut operators, operand)|
                                if operators.is_empty() {
                                    operand
                                } else {
                                    operators.reverse();
                                    operators.into_iter().fold(operand, |right_operand, operator|
                                        Box::new(#parent_module_name::#parent_type_name::#tag_name(#module_name::E { operator, right_operand }))
                                    )
                                }
                            )
                        )
                    }

                    OperatorModel::UnarySuffix => {
                        let next_sibling_parser_name = naming::to_parser_name_ident(
                            &next_sibling
                                .clone()
                                .expect(
                                    "Cannot have unary suffix operator as last expression member",
                                )
                                .name
                                .to_snake_case(),
                        );
                        let annotation = if operator.has_default(forest) {
                            quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                        } else {
                            quote!()
                        };
                        result.ast_fragment.push(quote!(
                            #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                            pub struct E {
                                pub left_operand: #parent_type_name,
                                #annotation
                                pub operator: #operator_ast_type,
                            }
                        ));

                        let kind = naming::to_kind_ident(&tree.name, &name);
                        result.cst_rule_kinds.insert(kind.clone());
                        result.cst_parser_impl_fragment = quote!(
                            #next_sibling_parser_name.clone()
                            .then(#operator_cst_parser.repeated())
                            .map(|(operand, operators)|
                                if operators.is_empty() {
                                    operand
                                } else {
                                    operators.into_iter().fold(operand, |left_operand, operator|
                                        Node::new_rule(RuleKind::#kind, vec![
                                            left_operand,
                                            operator,
                                        ])
                                    )
                                }
                            )
                        );
                        result.ast_parser_impl_fragment = quote!(
                            #next_sibling_parser_name.clone()
                            .then(#operator_ast_parser.repeated())
                            .map(|(operand, operators)|
                                if operators.is_empty() {
                                    operand
                                } else {
                                    operators.into_iter().fold(operand, |left_operand, operator|
                                        Box::new(#parent_module_name::#parent_type_name::#tag_name(#module_name::E { left_operand, operator }))
                                    )
                                }
                            )
                        );
                    }
                };

                result.ast_parser_type = quote!( #parent_type_name );

                result
            }

            Self::Lookahead {
                expr, lookahead, ..
            } => {
                let mut expr = expr.to_generated_code(forest, tree);
                let lookahead = lookahead.to_generated_code(forest, tree);

                let expr_parser = expr.cst_parser_impl_fragment;
                let lookahead_parser = lookahead.cst_parser_impl_fragment;
                expr.cst_parser_impl_fragment =
                    quote!( #expr_parser.then_ignore( #lookahead_parser.rewind() ));

                let expr_parser = expr.ast_parser_impl_fragment;
                let lookahead_parser = lookahead.ast_parser_impl_fragment;
                expr.ast_parser_impl_fragment =
                    quote!( #expr_parser.then_ignore( #lookahead_parser.rewind() ));

                expr
            }

            Self::OneOrMore { expr, name } => {
                let mut result = expr.to_generated_code(forest, tree);

                let mut cst_parser = result.cst_parser_impl_fragment;
                cst_parser = quote!( #cst_parser.repeated().at_least(1usize) );
                let mut ast_parser = result.ast_parser_impl_fragment;
                ast_parser = quote!( #ast_parser.repeated().at_least(1usize) );

                if matches!(**expr, Self::CharacterFilter { .. }) {
                    // Vec<()> -> VeriableSizeTerminal
                    if naming::is_anonymous(name) {
                        cst_parser =
                            quote!( #cst_parser.map(|v| Node::new_anonymous_token(v.len())) );
                    } else {
                        let kind = naming::to_kind_ident(&tree.name, &name);
                        result.cst_token_kinds.insert(kind.clone());
                        cst_parser =
                            quote!( #cst_parser.map(|v| Node::new_token(TokenKind::#kind, v)) );
                    }
                    ast_parser = quote!( #ast_parser.map(|v| VariableSizeTerminal(v.len())) );
                    result.ast_parser_type = quote!(VariableSizeTerminal);
                } else {
                    if naming::is_anonymous(&name) {
                        cst_parser = quote!( #cst_parser.map(Node::new_anonymous_rule) );
                    } else {
                        let kind = naming::to_kind_ident(&tree.name, &name);
                        result.cst_rule_kinds.insert(kind.clone());
                        cst_parser =
                            quote!( #cst_parser.map(|v| Node::new_rule(RuleKind::#kind, v)) );
                    }
                    let parser_type = result.ast_parser_type;
                    result.ast_parser_type = quote!( Vec<#parser_type> );
                };

                result.cst_parser_impl_fragment = cst_parser;
                result.ast_parser_impl_fragment = ast_parser;

                result
            }

            Self::Optional { expr, .. } => {
                let mut result = expr.to_generated_code(forest, tree);

                let cst_parser = result.cst_parser_impl_fragment;
                result.cst_parser_impl_fragment =
                    quote!( #cst_parser.or_not().map(|v| v.unwrap_or_else(|| Node::new_none())) );
                let ast_parser = result.ast_parser_impl_fragment;
                result.ast_parser_impl_fragment = quote!( #ast_parser.or_not() );

                let parser_type = result.ast_parser_type;
                result.ast_parser_type = quote!( Option<#parser_type> );

                result
            }

            Self::Reference { name, production } => {
                let mut result: CodeForNode = Default::default();

                let production_parser_name = naming::to_parser_name_ident(&name);
                let production_type_name = naming::to_type_name_ident(&name);

                if !tree.production.is_token()
                    && production.is_token()
                    && production.name != "LeadingTrivia"
                    && production.name != "TrailingTrivia"
                    && production.name != "EndOfFileTrivia"
                {
                    let production_module_name = naming::to_module_name_ident(&name);
                    result.cst_parser_impl_fragment = quote!(
                        leading_trivia_parser.clone()
                        .then(#production_parser_name.clone())
                        .then(trailing_trivia_parser.clone())
                        .map(Node::new_with_trivia)
                    );
                    result.ast_parser_impl_fragment = quote!(
                        leading_trivia_parser.clone()
                        .then(#production_parser_name.clone())
                        .then(trailing_trivia_parser.clone())
                        .map(|((leading_trivia, terminal), trailing_trivia)|
                            #production_module_name::WithTrivia { leading_trivia, terminal, trailing_trivia })
                    );
                    result.ast_parser_type = quote!( #production_module_name::WithTrivia );
                } else {
                    result.cst_parser_impl_fragment = quote!( #production_parser_name.clone() );
                    result.ast_parser_impl_fragment = quote!( #production_parser_name.clone() );
                    result.ast_parser_type = quote!( #production_type_name );
                }

                result
            }

            Self::Repeated {
                expr,
                min,
                max,
                name,
            } => {
                let mut result = expr.to_generated_code(forest, tree);

                let mut cst_parser = result.cst_parser_impl_fragment;
                let mut ast_parser = result.ast_parser_impl_fragment;
                if min == max {
                    cst_parser = quote!( #cst_parser.repeated().exactly(#min) );
                    ast_parser = quote!( #ast_parser.repeated().exactly(#min) );
                } else {
                    cst_parser = quote!( #cst_parser.repeated().at_least(#min).at_most(#max) );
                    ast_parser = quote!( #ast_parser.repeated().at_least(#min).at_most(#max) );
                };

                if matches!(**expr, Self::CharacterFilter { .. }) {
                    // Vec<()> -> VeriableSizeTerminal
                    if naming::is_anonymous(name) {
                        cst_parser =
                            quote!( #cst_parser.map(|v| Node::new_anonymous_token(v.len())) );
                    } else {
                        let kind = naming::to_kind_ident(&tree.name, &name);
                        result.cst_token_kinds.insert(kind.clone());
                        if *min == 0 {
                            cst_parser = quote!( #cst_parser.map(|v| if v.is_empty() { Node::new_none() } else { Node::new_token(TokenKind::#kind, v) }) );
                        } else {
                            cst_parser =
                                quote!( #cst_parser.map(|v| Node::new_token(TokenKind::#kind, v)) );
                        }
                    }
                    ast_parser = quote!( #ast_parser.map(|v| VariableSizeTerminal(v.len())) );
                    result.ast_parser_type = quote!(VariableSizeTerminal);
                } else {
                    if naming::is_anonymous(name) {
                        cst_parser = quote!( #cst_parser.map(Node::new_anonymous_rule) );
                    } else {
                        let kind = naming::to_kind_ident(&tree.name, &name);
                        result.cst_rule_kinds.insert(kind.clone());
                        if *min == 0 {
                            cst_parser = quote!( #cst_parser.map(|v| if v.is_empty() { Node::new_none() } else { Node::new_rule(RuleKind::#kind, v) }) );
                        } else {
                            cst_parser =
                                quote!( #cst_parser.map(|v| Node::new_rule(RuleKind::#kind, v)) );
                        }
                    }
                    let parser_type = result.ast_parser_type;
                    result.ast_parser_type = quote!( Vec<#parser_type> );
                };

                result.cst_parser_impl_fragment = cst_parser;
                result.ast_parser_impl_fragment = ast_parser;

                result
            }

            Self::SeparatedBy {
                name,
                expr,
                separator,
            } => {
                let mut result: CodeForNode = Default::default();

                let module_name = naming::to_module_name_ident(&tree.name);
                let type_name = naming::to_type_name_ident(&name);

                let expr_name = naming::pluralize(&expr.name());
                let expr_name = naming::to_field_name_ident(&expr_name);

                let expr = expr.to_generated_code(forest, tree);
                let (separator_cst_parser, separator_token_kind) =
                    generated_cst_code_for_string(separator, !tree.production.is_token());
                let (separator_ast_parser, separator_ast_type, separator_name) =
                    generated_ast_code_for_string(separator, !tree.production.is_token());

                let separator_name = naming::pluralize(&separator_name);
                let separator_name = naming::to_field_name_ident(&separator_name);

                let expr_cst_parser = expr.cst_parser_impl_fragment.clone();
                let separator_cst_parser = separator_cst_parser.clone();

                let expr_ast_parser_type = expr.ast_parser_type.clone();
                let separator_ast_parser_type = separator_ast_type.clone();

                let expr_ast_parser = expr.ast_parser_impl_fragment.clone();
                let separator_ast_parser = separator_ast_parser.clone();

                result.merge(expr);

                result.cst_token_part_kinds.insert(separator_token_kind);

                if naming::is_anonymous(&name) {
                    result.cst_parser_impl_fragment = quote!(
                        #expr_cst_parser.then(#separator_cst_parser.then(#expr_cst_parser).repeated())
                            .map(repetition_mapper)
                            .map(Node::new_anonymous_rule)
                    );
                } else {
                    let kind = naming::to_kind_ident(&tree.name, &name);
                    result.cst_rule_kinds.insert(kind.clone());
                    result.cst_parser_impl_fragment = quote!(
                        #expr_cst_parser.then(#separator_cst_parser.then(#expr_cst_parser).repeated())
                            .map(repetition_mapper)
                            .map(|v| Node::new_rule(RuleKind::#kind, v))
                    );
                }

                result.ast_parser_impl_fragment = quote!(
                    #expr_ast_parser.then(#separator_ast_parser.then(#expr_ast_parser).repeated())
                        .map(repetition_mapper)
                        .map(|(#expr_name, #separator_name)| #module_name::#type_name { #expr_name, #separator_name })
                );

                result.ast_fragment.push({
                    quote!(
                        #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                        pub struct #type_name {
                            #[serde(default, skip_serializing_if="DefaultTest::is_default")]
                            pub #expr_name: Vec<#expr_ast_parser_type>,
                            #[serde(default, skip_serializing_if="DefaultTest::is_default")]
                            pub #separator_name: Vec<#separator_ast_parser_type>
                        }
                    )
                });

                result.ast_impl_fragment.push(quote!(
                    impl Default for #module_name::#type_name {
                        fn default() -> Self {
                            Self {
                                #expr_name: Default::default(),
                                #separator_name: Default::default(),
                            }
                        }
                    }
                    impl DefaultTest for #module_name::#type_name {
                        fn is_default(&self) -> bool {
                            self.#expr_name.is_default() && self.#separator_name.is_default()
                        }
                    }
                ));

                result.ast_parser_type = quote!( #module_name::#type_name );

                result
            }

            Self::Sequence { name, elements } => {
                let mut result: CodeForNode = Default::default();

                let module_name = naming::to_module_name_ident(&tree.name);
                let type_name = naming::to_type_name_ident(&name);

                let mut field_names = vec![];
                let mut cst_parser_chain = None;
                let mut ast_field_annotations = vec![];
                let mut ast_field_types = vec![];
                let mut ast_parser_chain = None;

                for element in elements {
                    result.merge(element.to_generated_code(forest, tree));

                    let name = naming::to_field_name_ident(&element.name());
                    field_names.push(quote!( #name ));

                    let next_parser = result.cst_parser_impl_fragment.clone();
                    cst_parser_chain = cst_parser_chain
                        .and_then(|p| Some(quote!( #p.then(#next_parser) )))
                        .or_else(|| Some(next_parser));

                    ast_field_annotations.push(if element.has_default(forest) {
                        quote!( #[serde(default, skip_serializing_if="DefaultTest::is_default")] )
                    } else {
                        quote!()
                    });

                    let ast_parser_type = result.ast_parser_type.clone();
                    ast_field_types.push(quote!( #ast_parser_type ));

                    let next_parser = result.ast_parser_impl_fragment.clone();
                    ast_parser_chain = ast_parser_chain
                        .and_then(|p| Some(quote!( #p.then(#next_parser) )))
                        .or_else(|| Some(next_parser));
                }

                result.ast_fragment.push(quote!(
                    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
                    pub struct #type_name {
                        #(
                            #ast_field_annotations
                            pub #field_names: #ast_field_types
                        ),*
                    }
                ));

                let folded_field_names = field_names
                    .clone()
                    .into_iter()
                    .reduce(|accum, next| quote!( (#accum, #next) ));
                let folded_field_types = ast_field_types
                    .clone()
                    .into_iter()
                    .reduce(|accum, next| quote!( (#accum, #next) ));
                result.ast_impl_fragment.push(quote!(
                    impl #module_name::#type_name {
                        pub fn from_parse(#folded_field_names: #folded_field_types) -> Self {
                            Self { #(#field_names),* }
                        }
                    }
                ));

                if self.has_default(forest) {
                    result.ast_impl_fragment.push(quote!(
                        impl Default for #module_name::#type_name {
                            fn default() -> Self {
                                Self {
                                    #(#field_names: Default::default()),*
                                }
                            }
                        }
                        impl DefaultTest for #module_name::#type_name {
                            fn is_default(&self) -> bool {
                                #(self. #field_names .is_default())&&*
                            }
                        }
                    ));
                }

                let cst_parser = cst_parser_chain.unwrap();
                if naming::is_anonymous(&name) {
                    result.cst_parser_impl_fragment = quote!(
                        #cst_parser.map(|#folded_field_names| Node::new_anonymous_rule(vec![#(#field_names),*]))
                    );
                } else {
                    let kind = naming::to_kind_ident(&tree.name, &name);
                    result.cst_rule_kinds.insert(kind.clone());
                    result.cst_parser_impl_fragment = quote!(
                        #cst_parser.map(|#folded_field_names| Node::new_rule(RuleKind::#kind, vec![#(#field_names),*]))
                    );
                }
                let ast_parser = ast_parser_chain.unwrap();
                result.ast_parser_impl_fragment =
                    quote!( #ast_parser.map(|v| #module_name::#type_name::from_parse(v)) );
                result.ast_parser_type = quote!( #module_name::#type_name );

                result
            }

            Self::TerminalTrie { trie, .. } => trie.to_generated_code(!tree.production.is_token()),

            Self::ZeroOrMore { expr, name } => {
                let mut result = expr.to_generated_code(forest, tree);

                let mut cst_parser = result.cst_parser_impl_fragment;
                cst_parser = quote!( #cst_parser.repeated() );

                let mut ast_parser = result.ast_parser_impl_fragment;
                ast_parser = quote!( #ast_parser.repeated() );

                if matches!(**expr, Self::CharacterFilter { .. }) {
                    // Vec<()> -> VeriableSizeTerminal
                    if naming::is_anonymous(&name) {
                        cst_parser =
                            quote!( #cst_parser.map(|v| Node::new_anonymous_token(v.len())) );
                    } else {
                        let kind = naming::to_kind_ident(&tree.name, name);
                        result.cst_token_kinds.insert(kind.clone());
                        cst_parser = quote!( #cst_parser.map(|v| if v.is_empty() { Node::new_none() } else { Node::new_token(TokenKind::#kind, v) }) );
                    }
                    ast_parser = quote!( #ast_parser.map(|v| VariableSizeTerminal(v.len())) );
                    result.ast_parser_type = quote!(VariableSizeTerminal);
                } else {
                    if naming::is_anonymous(name) {
                        cst_parser = quote!( #cst_parser.map(Node::new_anonymous_rule) );
                    } else {
                        let kind = naming::to_kind_ident(&tree.name, name);
                        result.cst_rule_kinds.insert(kind.clone());
                        cst_parser = quote!( #cst_parser.map(|v| if v.is_empty() { Node::new_none() } else { Node::new_rule(RuleKind::#kind, v) }) );
                    }
                    let parser_type = result.ast_parser_type;
                    result.ast_parser_type = quote!( Vec<#parser_type> );
                };

                result.cst_parser_impl_fragment = cst_parser;
                result.ast_parser_impl_fragment = ast_parser;

                result
            }
        }
    }

    pub fn collect_identifiers(&self, accum: &mut BTreeSet<String>) {
        match self {
            Self::CharacterFilter { .. } | Self::TerminalTrie { .. } => {}

            Self::Choice { elements, .. } | Self::Sequence { elements, .. } => {
                for member in elements {
                    member.collect_identifiers(accum);
                }
            }

            Self::DelimitedBy { expr, .. }
            | Self::OneOrMore { expr, .. }
            | Self::Optional { expr, .. }
            | Self::Repeated { expr, .. }
            | Self::SeparatedBy { expr, .. }
            | Self::ZeroOrMore { expr, .. } => expr.collect_identifiers(accum),

            Self::Difference {
                minuend,
                subtrahend,
                ..
            } => {
                minuend.collect_identifiers(accum);
                subtrahend.collect_identifiers(accum)
            }

            Self::Expression { members, .. } => {
                for pr in members {
                    accum.insert(pr.name.clone());
                }
            }

            Self::ExpressionMember {
                parent,
                next_sibling,
                operator,
                ..
            } => {
                accum.insert(parent.name.clone());
                if let Some(n) = next_sibling {
                    accum.insert(n.name.clone());
                }
                operator.collect_identifiers(accum);
            }

            Self::Lookahead {
                expr, lookahead, ..
            } => {
                expr.collect_identifiers(accum);
                lookahead.collect_identifiers(accum);
            }

            Self::Reference { production, .. } => {
                accum.insert(production.name.clone());
            }
        }
    }

    fn rename(&self, name: String) -> CombinatorNodeRef {
        match self.clone() {
            Self::Sequence { elements, .. } => Self::new_sequence(name, elements),
            Self::CharacterFilter { filter, .. } => Self::new_character_filter(name, filter),
            Self::Choice { elements, .. } => Self::new_choice(name, elements),
            Self::DelimitedBy {
                open, expr, close, ..
            } => Self::new_delimited_by(name, open, expr, close),
            Self::Difference {
                minuend,
                subtrahend,
                ..
            } => Self::new_difference(name, minuend, subtrahend),
            Self::Expression { members, .. } => Self::new_expression(name, members),
            Self::ExpressionMember {
                parent,
                next_sibling,
                operator,
                operator_model,
                ..
            } => Self::new_expression_member(name, parent, next_sibling, operator, operator_model),
            Self::Lookahead {
                expr, lookahead, ..
            } => Self::new_lookahead(name, expr, lookahead),
            Self::OneOrMore { expr, .. } => Self::new_one_or_more(name, expr),
            Self::Optional { expr, .. } => Self::new_optional(name, expr),
            Self::Reference { production, .. } => Self::new_reference(name, production),
            Self::Repeated { expr, min, max, .. } => Self::new_repeat(name, expr, min, max),
            Self::SeparatedBy {
                expr, separator, ..
            } => Self::new_separated_by(name, expr, separator),
            Self::TerminalTrie { trie, .. } => Self::new_terminal_trie(name, trie),
            Self::ZeroOrMore { expr, .. } => Self::new_zero_or_more(name, expr),
        }
    }
}

impl CodeForNode {
    pub fn merge(&mut self, other: Self) {
        self.cst_rule_kinds.extend(other.cst_rule_kinds);
        self.cst_token_kinds.extend(other.cst_token_kinds);
        self.cst_token_part_kinds.extend(other.cst_token_part_kinds);
        self.cst_parser_impl_fragment = other.cst_parser_impl_fragment;

        self.ast_fragment.extend(other.ast_fragment);
        self.ast_impl_fragment.extend(other.ast_impl_fragment);
        self.ast_parser_type = other.ast_parser_type;
        self.ast_parser_impl_fragment = other.ast_parser_impl_fragment;
    }
}

fn generated_cst_code_for_string(str: &str, with_trivia: bool) -> (TokenStream, Ident) {
    let name = naming::name_of_terminal_string(str);
    let kind = naming::to_kind_ident(&name, &name);
    let size = str.chars().count();
    let cst_parser = if size == 1 {
        let char = str.chars().next().unwrap();
        quote!( just(#char).to(Node::new_token_part(TokenPartKind::#kind, #size)) )
    } else {
        quote!( just(#str).to(Node::new_token_part(TokenPartKind::#kind, #size)) )
    };

    if with_trivia {
        (
            quote!( leading_trivia_parser.clone().then(#cst_parser).then(trailing_trivia_parser.clone()).map(Node::new_with_trivia) ),
            kind,
        )
    } else {
        (cst_parser, kind)
    }
}

fn generated_ast_code_for_string(
    str: &str,
    with_trivia: bool,
) -> (TokenStream, TokenStream, String) {
    let name = naming::name_of_terminal_string(str);
    let size = str.chars().count();
    let ast_parser = if size == 1 {
        let char = str.chars().next().unwrap();
        quote!( just(#char).to(FixedSizeTerminal::<#size>()) )
    } else {
        quote!( just(#str).to(FixedSizeTerminal::<#size>()) )
    };

    if with_trivia {
        (
            quote!(
                leading_trivia_parser.clone().then(#ast_parser).then(trailing_trivia_parser.clone())
                .map(|((leading_trivia, terminal), trailing_trivia)| FixedSizeTerminalWithTrivia { leading_trivia, terminal, trailing_trivia })
            ),
            quote!(FixedSizeTerminalWithTrivia<#size>),
            name,
        )
    } else {
        (ast_parser, quote!(FixedSizeTerminal<#size>), name)
    }
}

// TODO: this should remove disambiguation suffixes *before* checking
// for repeated identifiers.
fn disambiguate_node_names(nodes: &mut Vec<CombinatorNodeRef>) {
    // Find all the duplicated names, with the count of their occurance
    let mut names = MultiSet::<String>::from_iter(nodes.iter().map(|n| n.name()));
    names.retain(|_, count| count > 1);
    nodes.reverse();
    nodes.iter_mut().for_each(|n| {
        let name = n.name();
        if let Some(count) = names.get(&name) {
            *n = n.rename(naming::with_disambiguating_suffix(&name, count - 1));
            names.remove(&name);
        }
    });
    nodes.reverse();
}
