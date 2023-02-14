use codegen_schema::types::productions::ProductionKind;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{
    char_set::CharSet,
    code_generator::CodeGenerator,
    combinator_node::CombinatorNode,
    combinator_node::OperatorModel,
    naming::name_of_terminal_char,
    trie::{TerminalTrie, Trie},
};

fn create_token_parser_from_scanner(
    scanner: TokenStream,
    kind: Ident,
    error_message: &str,
    with_trivia: bool,
) -> TokenStream {
    if with_trivia {
        quote! {
            {
                let leading_trivia = self.optional_leading_trivia(stream);
                let start = stream.position();
                if #scanner {
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
                if #scanner {
                    let end = stream.position();
                    Pass{ node: cst::Node::token(TokenKind::#kind, Range { start, end }, None, None), error: None }
                } else {
                    Fail{ error: ParseError::new(start, #error_message) }
                }
            }
        }
    }
}

impl<'context> CombinatorNode<'context> {
    pub fn to_parser_code(&self, is_trivia: bool, code: &mut CodeGenerator) -> TokenStream {
        fn create_kind(name: &Option<String>, code: &mut CodeGenerator) -> Ident {
            name.as_ref()
                .map(|name| code.add_rule_kind(name.clone()))
                .unwrap_or_else(|| format_ident!("_ANON").into())
        }

        match self {
            /**********************************************************************
             * Simple Reference
             */
            Self::Reference { tree } => match tree.production.kind {
                ProductionKind::Rule if is_trivia => unreachable!(
                    "Trivia productions can only reference trivia or token productions"
                ),
                ProductionKind::Rule | ProductionKind::Trivia => {
                    let function_name =
                        format_ident!("parse_{}", tree.production.name.to_snake_case());
                    quote! { self.#function_name(stream) }
                }
                ProductionKind::Token => {
                    let kind = format_ident!("{}", tree.production.name.to_pascal_case());
                    let function_name =
                        format_ident!("scan_{}", tree.production.name.to_snake_case());
                    let scanner = quote! { self.#function_name(stream) };
                    let error_message = tree.production.name.to_pascal_case();
                    create_token_parser_from_scanner(scanner, kind, &error_message, !is_trivia)
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
                let kind = create_kind(name, code);
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
                quote! {
                    {
                        let start_position = stream.position();
                        match #expr {
                            Fail{ error } => {
                                stream.set_position(start_position);
                                Pass{ node: cst::Node::rule(RuleKind::_ANON, vec![]), error: Some(error) }
                            }
                            pass => pass,
                        }
                    }
                }
            }

            Self::ZeroOrMore { expr, name } => {
                let expr = expr.to_parser_code(is_trivia, code);
                let kind = create_kind(name, code);
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
                let kind = create_kind(name, code);
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
                let kind = create_kind(name, code);
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
                let open_kind = code.add_terminal_kind(open.clone());
                let open_error_message = format!("'{}'", open);
                let open_chars = open.chars();
                let open = create_token_parser_from_scanner(
                    quote! { scan_chars!(stream, #(#open_chars),*) },
                    open_kind,
                    &open_error_message,
                    !is_trivia,
                );

                let expr = expr.to_parser_code(is_trivia, code);
                let kind = create_kind(name, code);

                let close_kind = code.add_terminal_kind(close.clone());
                let close_error_message = format!("'{}'", close);
                let close_chars = close.chars();
                let close = create_token_parser_from_scanner(
                    quote! { scan_chars!(stream, #(#close_chars),*) },
                    close_kind,
                    &close_error_message,
                    !is_trivia,
                );

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
                let kind = create_kind(name, code);
                let expr = expr.to_parser_code(is_trivia, code);

                let separator_kind = code.add_terminal_kind(separator.clone());
                let separator_chars = separator.chars();
                let separator = create_token_parser_from_scanner(
                    quote! { scan_chars!(stream, #(#separator_chars),*) },
                    separator_kind,
                    format!("'{}'", separator).as_str(),
                    !is_trivia,
                );

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
                let mut prefix_operator_trie = OperatorTrie::new();
                let mut suffix_operators = Vec::new();
                let mut suffix_operator_trie = OperatorTrie::new();
                let mut binary_operators = Vec::new();
                let mut binary_operator_trie = OperatorTrie::new();
                for (index, operator) in operators.iter().enumerate() {
                    let binding_power = (1 + index * 2) as u8;
                    let rule_kind = code.add_rule_kind(operator.name.clone());
                    match operator.model {
                        OperatorModel::BinaryLeftAssociative => {
                            if let Self::TerminalTrie { trie, .. } = &operator.operator {
                                for (value, payload) in trie.iter() {
                                    binary_operator_trie.insert(
                                        &value,
                                        OperatorTriePayload {
                                            token_kind: code
                                                .add_token_kind(payload.clone().unwrap()),
                                            rule_kind: rule_kind.clone(),
                                            left_binding_power: binding_power,
                                            right_binding_power: binding_power + 1,
                                        },
                                    );
                                }
                            } else {
                                let operator_code =
                                    operator.operator.to_parser_code(is_trivia, code);
                                binary_operators.push(quote!{
                                    match #operator_code {
                                        Pass{ node, .. } => Ok(Pratt::Operator {
                                            node, kind: RuleKind::#rule_kind, left_binding_power: #binding_power, right_binding_power: #binding_power + 1
                                        }),
                                        Fail{ error } => Err(error)
                                    }
                                 })
                            }
                        }
                        OperatorModel::BinaryRightAssociative => {
                            if let Self::TerminalTrie { trie, .. } = &operator.operator {
                                for (value, payload) in trie.iter() {
                                    binary_operator_trie.insert(
                                        &value,
                                        OperatorTriePayload {
                                            token_kind: code
                                                .add_token_kind(payload.clone().unwrap()),
                                            rule_kind: rule_kind.clone(),
                                            left_binding_power: binding_power + 1,
                                            right_binding_power: binding_power,
                                        },
                                    );
                                }
                            } else {
                                let operator_code =
                                    operator.operator.to_parser_code(is_trivia, code);
                                binary_operators.push(quote!{
                                    match #operator_code {
                                        Pass{ node, .. } => Ok(Pratt::Operator {
                                            node, kind: RuleKind::#rule_kind, left_binding_power: #binding_power + 1, right_binding_power: #binding_power
                                        }),
                                        Fail{ error } => Err(error)
                                    }
                                 })
                            }
                        }
                        OperatorModel::UnaryPrefix => {
                            if let Self::TerminalTrie { trie, .. } = &operator.operator {
                                for (value, payload) in trie.iter() {
                                    prefix_operator_trie.insert(
                                        &value,
                                        OperatorTriePayload {
                                            token_kind: code
                                                .add_token_kind(payload.clone().unwrap()),
                                            rule_kind: rule_kind.clone(),
                                            left_binding_power: 255,
                                            right_binding_power: binding_power,
                                        },
                                    );
                                }
                            } else {
                                let operator_code =
                                    operator.operator.to_parser_code(is_trivia, code);
                                prefix_operators.push(quote!{
                                    match #operator_code {
                                        Pass{ node, .. } => Ok(Pratt::Operator {
                                            node, kind: RuleKind::#rule_kind, left_binding_power: 255, right_binding_power: #binding_power
                                        }),
                                        Fail{ error } => Err(error)
                                    }
                                })
                            }
                        }
                        OperatorModel::UnarySuffix => {
                            if let Self::TerminalTrie { trie, .. } = &operator.operator {
                                for (value, payload) in trie.iter() {
                                    suffix_operator_trie.insert(
                                        &value,
                                        OperatorTriePayload {
                                            token_kind: code
                                                .add_token_kind(payload.clone().unwrap()),
                                            rule_kind: rule_kind.clone(),
                                            left_binding_power: binding_power,
                                            right_binding_power: 255,
                                        },
                                    );
                                }
                            } else {
                                let operator_code =
                                    operator.operator.to_parser_code(is_trivia, code);
                                suffix_operators.push(quote!{
                                    match #operator_code {
                                        Pass{ node, .. } => Ok(Pratt::Operator {
                                            node, kind: RuleKind::#rule_kind, left_binding_power: #binding_power, right_binding_power: 255
                                        }),
                                        Fail{ error } => Err(error)
                                    }
                                 })
                            }
                        }
                    }
                }

                binary_operators.insert(0, binary_operator_trie.to_parser_code());
                prefix_operators.insert(0, prefix_operator_trie.to_parser_code());
                suffix_operators.insert(0, suffix_operator_trie.to_parser_code());

                fn maybe_choice(mut elements: Vec<TokenStream>) -> TokenStream {
                    if elements.len() == 1 {
                        elements.pop().unwrap()
                    } else {
                        let mut elements = elements.into_iter();
                        let first_element = elements.next().unwrap();
                        quote! {
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
                        }
                    }
                }
                let binary_operators = maybe_choice(binary_operators);
                let prefix_operators = maybe_choice(prefix_operators);
                let suffix_operators = maybe_choice(suffix_operators);

                let primary_expressions = maybe_choice(
                    primary_expressions
                        .iter()
                        .map(|tree| {
                            let function_name =
                                format_ident!("parse_{}", tree.production.name.to_snake_case());
                            quote! { self.#function_name(stream) }
                        })
                        .collect::<Vec<_>>(),
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
                            match #primary_expressions {
                                Fail{ error } => break Some(error),
                                Pass{ node, .. } => elements.push(Pratt::Node(node)),
                            }
                            loop {
                                let start_position = stream.position();
                                match #suffix_operators {
                                    Err(_) => {
                                        stream.set_position(start_position);
                                        break;
                                    }
                                    Ok(operator) => elements.push(operator),
                                }
                            }
                            let start_position = stream.position();
                            match #binary_operators {
                                Err(_) => {
                                    stream.set_position(start_position);
                                    break None
                                }
                                Ok(operator) => elements.push(operator),
                            }
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
            Self::CharacterFilter { filter, name } => {
                filter.to_parser_code(name.clone(), is_trivia, code)
            }

            Self::TerminalTrie { trie, .. } => trie.to_parser_code(is_trivia, code),

            Self::Difference { .. } => todo!("Handle difference in rules"),

            Self::Lookahead { .. } => todo!("Handle lookahead in rules"),
        }
    }
}

impl CharSet {
    pub fn to_parser_code(
        &self,
        name: Option<String>,
        is_trivia: bool,
        code: &mut CodeGenerator,
    ) -> TokenStream {
        if let Some(char) = self.single_char() {
            let name = name.unwrap_or_else(|| name_of_terminal_char(char));
            let kind = code.add_token_kind(name);
            let scanner = quote! { scan_chars!(stream, #char)};
            let error_message = kind.to_string();
            create_token_parser_from_scanner(scanner, kind, &error_message, !is_trivia)
        } else {
            unreachable!("Validation should have ensured that rules only contain simple character references.");
        }
    }
}

impl TerminalTrie {
    pub fn to_parser_code(&self, is_trivia: bool, code: &mut CodeGenerator) -> TokenStream {
        fn generate_code(trie: &TerminalTrie, code: &mut CodeGenerator) -> TokenStream {
            let (path, trie) = trie.next_interesting_node(None);

            let subtries = trie
                .subtries
                .iter()
                .map(|(c, t)| {
                    if t.subtries.is_empty() {
                        let kind = code.add_token_kind(t.payload.clone().unwrap().unwrap());
                        quote! { Some(#c) => Ok(TokenKind::#kind) }
                    } else {
                        let child_code = generate_code(t, code);
                        quote! { Some(#c) => #child_code }
                    }
                })
                .collect::<Vec<_>>();

            let error_message = format!("'{}'", trie.keys().join("', or '"));

            let chars = path.iter();
            let prefix = quote! { scan_chars!(stream, #(#chars),*) };
            if subtries.is_empty() {
                let kind = code.add_token_kind(trie.payload.clone().unwrap().unwrap());
                quote! {
                    if #prefix {
                        Ok(TokenKind::#kind)
                    } else {
                        Err(ParseError::new(stream.position(), #error_message))
                    }
                }
            } else {
                let (catchall_prep, catchall) = trie.payload.as_ref().map(|payload| {
                    let kind = format_ident!("{}", payload.as_ref().unwrap());
                    (
                        Some(quote! { let start_position = stream.position(); }),
                        quote! {
                            Some(_) => { stream.set_position(start_position); Ok(TokenKind::#kind) }
                            None => Ok(TokenKind::#kind)
                        }
                    )
                }).unwrap_or_else(|| (
                    None,
                    quote! { _ => Err(ParseError::new(stream.position(), #error_message)) }
                ));
                let trie_code = quote! {
                    {
                        #catchall_prep
                        match stream.next() {
                            #(#subtries,)*
                            #catchall
                        }
                    }
                };
                if path.is_empty() {
                    trie_code
                } else {
                    quote! {
                        {
                            if #prefix
                                #trie_code
                            else {
                                Err(ParseError::new(stream.position(), #error_message))
                            }
                        }
                    }
                }
            }
        }

        let (path, trie) = self.next_interesting_node(None);
        if trie.subtries.is_empty() {
            let chars = path.iter();
            let kind = code.add_token_kind(trie.payload.clone().unwrap().unwrap());
            let error_message = format!("'{}'", trie.keys()[0]);
            create_token_parser_from_scanner(
                quote! { scan_chars!(stream, #(#chars),*) },
                kind,
                &error_message,
                !is_trivia,
            )
        } else {
            let trie = generate_code(self, code);
            if !is_trivia {
                quote! {
                    {
                        let leading_trivia = self.optional_leading_trivia(stream);
                        let start = stream.position();
                        match #trie {
                            Err(mut error) => { error.position = start; Fail{ error } }
                            Ok(kind) => {
                                let end = stream.position();
                                let trailing_trivia = self.optional_trailing_trivia(stream);
                                Pass{ node: cst::Node::token(kind, Range { start, end }, leading_trivia, trailing_trivia), error: None }
                            }
                        }
                    }
                }
            } else {
                quote! {
                    {
                        let start = stream.position();
                        match #trie {
                            Err(mut error) => { error.position = start; Fail{ error } }
                            Ok(kind) => {
                                let end = stream.position();
                                Pass{ node: cst::Node::token(kind, Range { start, end }, None, None), error: None }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct OperatorTriePayload {
    pub token_kind: Ident,
    pub rule_kind: Ident,
    pub left_binding_power: u8,
    pub right_binding_power: u8,
}

type OperatorTrie = Trie<OperatorTriePayload>;

impl OperatorTrie {
    pub fn to_parser_code(&self) -> TokenStream {
        fn generate_code(trie: &OperatorTrie) -> TokenStream {
            let (path, trie) = trie.next_interesting_node(None);

            let subtries = trie
                .subtries
                .iter()
                .map(|(c, t)| {
                    if t.subtries.is_empty() {
                        let OperatorTriePayload {
                            token_kind,
                            rule_kind,
                            left_binding_power,
                            right_binding_power,
                        } = t.payload.as_ref().unwrap();
                        quote! { Some(#c) => Ok((TokenKind::#token_kind, RuleKind::#rule_kind, #left_binding_power, #right_binding_power)) }
                    } else {
                        let child_code = generate_code(t);
                        quote! { Some(#c) => #child_code }
                    }
                })
                .collect::<Vec<_>>();

            let error_message = format!("'{}'", trie.keys().join("', or '"));

            let chars = path.iter();
            let prefix = quote! { scan_chars!(stream, #(#chars),*) };
            if subtries.is_empty() {
                let OperatorTriePayload {
                    token_kind,
                    rule_kind,
                    left_binding_power,
                    right_binding_power,
                } = trie.payload.as_ref().unwrap();
                quote! {
                    if #prefix {
                        Ok((TokenKind::#token_kind, RuleKind::#rule_kind, #left_binding_power, #right_binding_power))
                    } else {
                        Err(ParseError::new(stream.position(), #error_message))
                    }
                }
            } else {
                let (catchall_prep, catchall) = trie.payload.as_ref().map(|payload| {
                    let OperatorTriePayload {
                        token_kind,
                        rule_kind,
                        left_binding_power,
                        right_binding_power,
                    } = payload;
                    (
                        Some(quote! { let start_position = stream.position(); }),
                        quote! {
                            Some(_) => {
                                stream.set_position(start_position);
                                Ok((TokenKind::#token_kind, RuleKind::#rule_kind, #left_binding_power, #right_binding_power))
                            }
                            None => Ok((TokenKind::#token_kind, RuleKind::#rule_kind, #left_binding_power, #right_binding_power))
                        }
                    )
                }).unwrap_or_else(|| (
                    None,
                    quote! { _ => Err(ParseError::new(stream.position(), #error_message)) }
                ));
                let trie_code = quote! {
                    {
                        #catchall_prep
                        match stream.next() {
                            #(#subtries,)*
                            #catchall
                        }
                    }
                };
                if path.is_empty() {
                    trie_code
                } else {
                    quote! {
                        {
                            if #prefix
                                #trie_code
                            else {
                                Err(ParseError::new(stream.position(), #error_message))
                            }
                        }
                    }
                }
            }
        }

        let (path, trie) = self.next_interesting_node(None);
        if trie.subtries.is_empty() {
            let OperatorTriePayload {
                token_kind,
                rule_kind,
                left_binding_power,
                right_binding_power,
            } = trie.payload.as_ref().unwrap();
            let error_message = trie.keys().join(", or ");
            let chars = path.iter();
            let operator = create_token_parser_from_scanner(
                quote! { scan_chars!(stream, #(#chars),*) },
                token_kind.clone(),
                &error_message,
                true,
            );
            quote! {
                {
                    let start = stream.position();
                    match #operator {
                        Err(error) => Err(error),
                        Ok(node) => Ok(Pratt::Operator {
                            node,
                            kind: #rule_kind,
                            left_binding_power: #left_binding_power,
                            right_binding_power: #right_binding_power
                        })
                    }
                }
            }
        } else {
            let trie = generate_code(self);
            quote! {
                {
                    let leading_trivia = self.optional_leading_trivia(stream);
                    let start = stream.position();
                    match #trie {
                        Ok((token_kind, rule_kind, left_binding_power, right_binding_power)) => {
                            let end = stream.position();
                            let trailing_trivia = self.optional_trailing_trivia(stream);
                            Ok(Pratt::Operator {
                                node: cst::Node::token(
                                    token_kind,
                                    Range { start, end },
                                    leading_trivia,
                                    trailing_trivia
                                ),
                                kind: rule_kind,
                                left_binding_power,
                                right_binding_power
                            })
                        }
                        Err(error) => Err(error)
                    }
                }
            }
        }
    }
}
