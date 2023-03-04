use codegen_schema::types::{precedence_parser::OperatorModel, production::Production};
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{code_generator::CodeGenerator, combinator_node::CombinatorNode};

impl<'context> CombinatorNode<'context> {
    pub fn to_parser_code(&self, is_trivia: bool, code: &mut CodeGenerator) -> TokenStream {
        match self {
            /**********************************************************************
             * Simple Reference
             */
            Self::Reference { tree } => match tree.production.as_ref() {
                Production::Scanner { name, .. } => {
                    let kind = format_ident!("{}", name);
                    let function_name = format_ident!("scan_{}", name.to_snake_case());
                    let scanner = quote! { self.#function_name(stream) };
                    let error_message = name;
                    scanner_code_to_parser_code(scanner, kind, &error_message, !is_trivia)
                }
                Production::TriviaParser { name, .. } => {
                    let function_name = format_ident!("parse_{}", name.to_snake_case());
                    quote! { self.#function_name(stream) }
                }
                Production::Parser { name, .. } | Production::PrecedenceParser { name, .. } => {
                    if !is_trivia {
                        let function_name = format_ident!("parse_{}", name.to_snake_case());
                        quote! { self.#function_name(stream) }
                    } else {
                        unreachable!(
                            "Trivia productions can only reference trivia or token productions"
                        )
                    }
                }
            },

            /**********************************************************************
             * Sequence and Choice
             */
            Self::Sequence { elements, name } => {
                let (vars, elements) = elements
                    .iter()
                    .enumerate()
                    .map(|(index, element)| {
                        (
                            format_ident!("result_{}", index),
                            element.to_parser_code(is_trivia, code),
                        )
                    })
                    .unzip::<_, _, Vec<_>, Vec<_>>();
                let kind =
                    code.add_rule_kind(name.clone().unwrap_or_else(|| "_SEQUENCE".to_string()));
                quote! {
                    loop {
                        let mut furthest_error = None;
                        #(
                            let #vars = match #elements{
                                Pass{ node, error } => { furthest_error = error.map(|error| error.maybe_merge_with(furthest_error)) ; node }
                                Fail{ error } => break Fail { error: error.maybe_merge_with(furthest_error) }
                            };
                        )*
                        break Pass{ node: cst::Node::rule(RuleKind::#kind, vec![#(#vars),*]), error: furthest_error }
                    }
                }
            }

            Self::Choice { elements, .. } => {
                let mut elements = elements
                    .iter()
                    .map(|element| element.to_parser_code(is_trivia, code));
                let first_element = elements.next().unwrap();
                quote! {
                    loop {
                        let start_position = stream.position();
                        let mut furthest_error;
                        match #first_element {
                            Fail{ error } => furthest_error = error,
                            pass => break pass,
                        }
                        #(
                            stream.set_position(start_position);
                            match #elements {
                                Fail{ error } => furthest_error.merge_with(error),
                                pass => break pass,
                            }
                        )*
                        break Fail{ error: furthest_error };
                    }
                }
            }

            /**********************************************************************
             * Numeric qualification
             */
            Self::Optional { expr } => {
                let expr = expr.to_parser_code(is_trivia, code);
                let kind = code.add_rule_kind("_OPTIONAL".to_string());
                quote! {
                    {
                        let start_position = stream.position();
                        match #expr {
                            Fail{ error } => {
                                stream.set_position(start_position);
                                Pass{ node: cst::Node::rule(RuleKind::#kind, vec![]), error: Some(error) }
                            }
                            pass => pass,
                        }
                    }
                }
            }

            Self::ZeroOrMore { expr, name } => {
                let expr = expr.to_parser_code(is_trivia, code);
                let kind =
                    code.add_rule_kind(name.clone().unwrap_or_else(|| "_REPEATED".to_string()));
                quote! {
                    {
                        let mut result = Vec::new();
                        loop {
                            let start_position = stream.position();
                            match #expr {
                                Fail{ error } => {
                                    stream.set_position(start_position);
                                    break Pass{ node: cst::Node::rule(RuleKind::#kind, result), error: Some(error) }
                                }
                                Pass{ node, .. } => result.push(node),
                            }
                        }
                    }
                }
            }

            Self::OneOrMore { expr, name } => {
                let expr = expr.to_parser_code(is_trivia, code);
                let kind =
                    code.add_rule_kind(name.clone().unwrap_or_else(|| "_REPEATED".to_string()));
                quote! {
                    {
                        let mut result = Vec::new();
                        loop {
                            let start_position = stream.position();
                            match #expr {
                                Fail{ error } => {
                                    if result.is_empty() {
                                        break Fail { error }
                                    }
                                    stream.set_position(start_position);
                                    break Pass{ node: cst::Node::rule(RuleKind::#kind, result), error: Some(error) }
                                }
                                Pass{ node, .. } => result.push(node),
                            }
                        }
                    }
                }
            }

            Self::Repeated {
                expr,
                min,
                max,
                name,
            } => {
                let expr = expr.to_parser_code(is_trivia, code);
                let kind =
                    code.add_rule_kind(name.clone().unwrap_or_else(|| "_REPEATED".to_string()));
                quote! {
                    {
                        let mut result = Vec::new();
                        loop {
                            let start_position = stream.position();
                            match #expr {
                                Fail{ error } => {
                                    if result.len() < #min {
                                        break error
                                    }
                                    stream.set_position(start_position);
                                    break Pass{ node: cst::Node::rule(RuleKind::#kind, result), error: Some(error) }
                                }
                                Pass{ node, .. } => result.push(node),
                            }
                            if result.len() == #max {
                                break Pass{ node: cst::Node::rule(RuleKind::#kind, result), error: None }
                            }
                        }
                    }
                }
            }

            /**********************************************************************
             * Special Structures
             */
            Self::DelimitedBy {
                open,
                expr,
                close,
                name,
            } => {
                let open = scanner_production_to_parser_code(open, is_trivia);
                let expr = expr.to_parser_code(is_trivia, code);
                let kind =
                    code.add_rule_kind(name.clone().unwrap_or_else(|| "_DELIMITEDBY".to_string()));
                let close = scanner_production_to_parser_code(close, is_trivia);
                quote! {
                    {
                        match #open {
                            err@Fail{ .. } => err,
                            Pass{ node: open_node, .. } => {
                                match #expr {
                                    err@Fail{ .. } => err,
                                    Pass{ node: expr_node, error: expr_error } => {
                                        match #close {
                                            Fail{ error } => Fail { error: error.maybe_merge_with(expr_error) },
                                            Pass{ node: close_node, .. } => {
                                                Pass{
                                                    node: cst::Node::rule(
                                                        RuleKind::#kind,
                                                        vec![
                                                            open_node,
                                                            expr_node,
                                                            close_node
                                                        ]
                                                    ),
                                                    error: None
                                                }
                                           }
                                        }
                                    }
                                }
                           }
                        }
                    }
                }
            }

            Self::SeparatedBy {
                expr,
                separator,
                name,
            } => {
                let kind =
                    code.add_rule_kind(name.clone().unwrap_or_else(|| "_SEPARATEDBY".to_string()));
                let expr = expr.to_parser_code(is_trivia, code);
                let separator = scanner_production_to_parser_code(separator, is_trivia);
                quote! {
                    {
                        let mut result = Vec::new();
                        loop {
                            match #expr {
                                err@Fail{ .. } => break err,
                                Pass{ node, .. } => {
                                    result.push(node);
                                    let save = stream.position();
                                    match #separator {
                                        Fail{ error } => {
                                            stream.set_position(save);
                                            break Pass{ node: cst::Node::rule(RuleKind::#kind, result), error: Some(error) }
                                        }
                                        Pass{ node, .. } => result.push(node),
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Self::TerminatedBy {
                expr,
                terminator,
                name,
            } => {
                let expr = expr.to_parser_code(is_trivia, code);
                let kind =
                    code.add_rule_kind(name.clone().unwrap_or_else(|| "_TERMINATEDBY".to_string()));
                let terminator = scanner_production_to_parser_code(terminator, is_trivia);
                quote! {
                    {
                        match #expr {
                            err@Fail{ .. } => err,
                            Pass{ node: expr_node, error: expr_error } => {
                                match #terminator {
                                    Fail{ error } => Fail { error: error.maybe_merge_with(expr_error) },
                                    Pass{ node: terminator_node, .. } => {
                                        Pass{
                                            node: cst::Node::rule(
                                                RuleKind::#kind,
                                                vec![
                                                    expr_node,
                                                    terminator_node
                                                ]
                                            ),
                                            error: None
                                        }
                                   }
                                }
                            }
                        }
                    }
                }
            }

            /**********************************************************************
             * Precedence parsing
             */
            Self::PrecedenceExpressionRule {
                primary_expressions,
                operators,
                ..
            } => {
                if is_trivia {
                    unreachable!("Precedence expressions cannot be used in trivia productions")
                }

                let mut prefix_operators = Vec::new();
                let mut postfix_operators = Vec::new();
                let mut binary_operators = Vec::new();
                for (index, operator) in operators.iter().enumerate() {
                    let binding_power = (1 + index * 2) as u8;
                    let rule_kind = code.add_rule_kind(operator.name.clone());
                    match operator.model {
                        OperatorModel::BinaryLeftAssociative => {
                            let operator_code = operator.operator.to_parser_code(is_trivia, code);
                            binary_operators.push(quote!{
                                match #operator_code {
                                    Pass{ node, .. } => Ok(Pratt::Operator {
                                        node, kind: RuleKind::#rule_kind, left_binding_power: #binding_power, right_binding_power: #binding_power + 1
                                    }),
                                    Fail{ error } => Err(error)
                                }
                             });
                        }
                        OperatorModel::BinaryRightAssociative => {
                            let operator_code = operator.operator.to_parser_code(is_trivia, code);
                            binary_operators.push(quote!{
                                match #operator_code {
                                    Pass{ node, .. } => Ok(Pratt::Operator {
                                        node, kind: RuleKind::#rule_kind, left_binding_power: #binding_power + 1, right_binding_power: #binding_power
                                    }),
                                    Fail{ error } => Err(error)
                                }
                             });
                        }
                        OperatorModel::UnaryPrefix => {
                            let operator_code = operator.operator.to_parser_code(is_trivia, code);
                            prefix_operators.push(quote!{
                                match #operator_code {
                                    Pass{ node, .. } => Ok(Pratt::Operator {
                                        node, kind: RuleKind::#rule_kind, left_binding_power: 255, right_binding_power: #binding_power
                                    }),
                                    Fail{ error } => Err(error)
                                }
                            });
                        }
                        OperatorModel::UnaryPostfix => {
                            let operator_code = operator.operator.to_parser_code(is_trivia, code);
                            postfix_operators.push(quote!{
                                match #operator_code {
                                    Pass{ node, .. } => Ok(Pratt::Operator {
                                        node, kind: RuleKind::#rule_kind, left_binding_power: #binding_power, right_binding_power: 255
                                    }),
                                    Fail{ error } => Err(error)
                                }
                             });
                        }
                    }
                }

                fn maybe_choice(mut elements: Vec<TokenStream>) -> Option<TokenStream> {
                    if elements.is_empty() {
                        None
                    } else if elements.len() == 1 {
                        elements.pop()
                    } else {
                        let mut elements = elements.into_iter();
                        let first_element = elements.next().unwrap();
                        Some(quote! {
                            loop {
                                let start_position = stream.position();
                                let mut furthest_error;
                                match #first_element {
                                    Err(error) => furthest_error = error,
                                    ok => break ok,
                                }
                                #(
                                    stream.set_position(start_position);
                                    match { #elements } {
                                        Err(error) =>
                                        if furthest_error.position < error.position {
                                            furthest_error = error
                                        } else if furthest_error.position == error.position {
                                            furthest_error.expected = format!("{}, or {}", furthest_error.expected, error.expected)
                                        },
                                        ok => break ok,
                                    }
                                )*
                                break Err(furthest_error);
                            }
                        })
                    }
                }

                let binary_operators = maybe_choice(binary_operators)
                    .map(|binary_operators| {
                        quote! {
                            let start_position = stream.position();
                            match #binary_operators {
                                Err(_) => {
                                    stream.set_position(start_position);
                                    break None
                                }
                                Ok(operator) => elements.push(operator),
                            }
                        }
                    })
                    .unwrap_or_else(|| {
                        quote! {
                            break None
                        }
                    });

                let prefix_operators = maybe_choice(prefix_operators).map(|prefix_operators| {
                    quote! {
                        loop {
                            let start_position = stream.position();
                            match #prefix_operators {
                                Err(_) => {
                                    stream.set_position(start_position);
                                    break;
                                }
                                Ok(operator) => elements.push(operator),
                            }
                        }
                    }
                });

                let postfix_operators = maybe_choice(postfix_operators).map(|postfix_operators| {
                    quote! {
                        loop {
                            let start_position = stream.position();
                            match #postfix_operators {
                                Err(_) => {
                                    stream.set_position(start_position);
                                    break;
                                }
                                Ok(operator) => elements.push(operator),
                            }
                        }
                    }
                });

                let primary_expressions = maybe_choice(
                    primary_expressions
                        .iter()
                        .map(|tree| {
                            let function_name =
                                format_ident!("parse_{}", tree.production.name().to_snake_case());
                            quote! { self.#function_name(stream) }
                        })
                        .collect::<Vec<_>>(),
                )
                .map(|primary_expressions| {
                    quote! {
                        match #primary_expressions {
                            Fail{ error } => break Some(error),
                            Pass{ node, .. } => elements.push(Pratt::Node(node)),
                        }
                    }
                })
                .expect(
                    "Validation should have ensured that we have at least one primary expression",
                );

                quote! {
                    {
                        enum Pratt {
                            Operator {
                                kind: RuleKind,
                                node: Rc<cst::Node>,
                                left_binding_power: u8,
                                right_binding_power: u8,
                            },
                            Node(Rc<cst::Node>),
                        }
                        let mut elements = Vec::new();
                        if let Some(error) = loop {
                            #prefix_operators
                            #primary_expressions
                            #postfix_operators
                            #binary_operators
                        } {
                            Fail{ error }
                        } else {
                            let mut i = 0;
                            while elements.len() > 1 {
                                if let Pratt::Operator { right_binding_power, left_binding_power, .. } = &elements[i] {
                                    let next_left_binding_power = if elements.len() == i + 1 {
                                        0
                                    } else if let Pratt::Operator { left_binding_power, .. } = &elements[i + 1] {
                                        *left_binding_power
                                    } else if elements.len() == i + 2 {
                                        0
                                    } else if let Pratt::Operator { left_binding_power, .. } = &elements[i + 2] {
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
                                        if let (Pratt::Node(left), Pratt::Operator { node: op, kind, .. }) = (left, op) {
                                            let node = cst::Node::rule(
                                                kind,
                                                vec![left, op],
                                            );
                                            elements.insert(i - 1, Pratt::Node(node));
                                            i = i.saturating_sub(2);
                                        } else {
                                            unreachable!()
                                        }
                                    } else  if *left_binding_power == 255 {
                                        let op = elements.remove(i);
                                        let right = elements.remove(i);
                                        if let (Pratt::Operator { node: op, kind, .. }, Pratt::Node(right)) = (op, right) {
                                            let node = cst::Node::rule(
                                                kind,
                                                vec![op, right],
                                            );
                                            elements.insert(i, Pratt::Node(node));
                                            i = i.saturating_sub(1);
                                        } else {
                                            unreachable!()
                                        }
                                    } else {
                                        let left = elements.remove(i - 1);
                                        let op = elements.remove(i - 1);
                                        let right = elements.remove(i - 1);
                                        if let (Pratt::Node(left), Pratt::Operator { node: op, kind, .. }, Pratt::Node(right)) = (left, op, right) {
                                            let node = cst::Node::rule(
                                                kind,
                                                vec![left, op, right],
                                            );
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
                                Pass{ node, error: None }
                            } else {
                                unreachable!()
                            }
                        }
                    }
                }
            }

            /**********************************************************************
             * Terminals and their utilities
             */
            Self::CharacterFilter { .. } => {
                unreachable!("CharacterFilter cannot be generated from a parser")
            }

            Self::TerminalTrie { .. } => {
                unreachable!("TerminalTrie cannot be generated from a parser")
            }

            Self::TrailingContext { .. } => {
                unreachable!("TrailingContext cannot be generated from a parser")
            }

            Self::Difference { .. } => unreachable!("Difference cannot be generated from a parser"),
        }
    }
}

fn scanner_production_to_parser_code(
    open: &&crate::combinator_tree::CombinatorTree,
    is_trivia: bool,
) -> TokenStream {
    if let Production::Scanner { name, .. } = open.production.as_ref() {
        let kind = format_ident!("{}", name);
        let function_name = format_ident!("scan_{}", name.to_snake_case());
        let scanner = quote! { self.#function_name(stream) };
        let error_message = name;
        scanner_code_to_parser_code(scanner, kind, &error_message, !is_trivia)
    } else {
        unreachable!("This reference should be to a scanner")
    }
}

fn scanner_code_to_parser_code(
    scanner_code: TokenStream,
    kind: Ident,
    error_message: &str,
    with_trivia: bool,
) -> TokenStream {
    if with_trivia {
        quote! {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if #scanner_code {
                    let end = stream.position();
                    let trailing_trivia = self.optional_trailing_trivia(stream);
                    Pass{ node: cst::Node::token(TokenKind::#kind, Range { start, end }, leading_trivia, trailing_trivia), error: None }
                } else {
                    Fail{ error: ParseError::new(start, #error_message) }
                }
            }
        }
    } else {
        quote! {
            {
                let start = stream.position();
                if #scanner_code {
                    let end = stream.position();
                    Pass{ node: cst::Node::token(TokenKind::#kind, Range { start, end }, None, None), error: None }
                } else {
                    Fail{ error: ParseError::new(start, #error_message) }
                }
            }
        }
    }
}
