# Language Definition v2

<!--
cSpell:ignore paren
-->

This document describes the new language definition model (AKA DSL v2), and the features/possibilities it enables for
both syntax and semantic analysis. Each section describes a part of the definition model, and how it can affect the
scanner, parser, CST, and AST.

This is a collection of different discussions we had over the last few weeks, and can (and should) be broken down into
smaller work items if needed. It should be possible to map the current definition to the old one, so that we do
incremental progress, instead of rewriting everything at once.

## CST

We currently produce an untyped tree of nodes. It holds all parts of the input (byte for byte), even whitespace, comments,
and unrecognized (skipped) parts. We can reconstruct the original input back from the CST, just by iterating on nodes in
order. For memory/performance reasons, we don't hold positions/location information in the tree, but they are calculated
during iterating/visiting the tree.

The CST is useful for many use cases:

-   Tools that only want to deal with document contents, like formatters, and syntax-only linters.
-   For visitors/rewriters that want to run on certain nodes, regardless of their position/parent types.
-   Reconstructing the original input, including trivia/whitespace, and any skipped (unrecognized) parts.

Here is an example of the node type, similar to what we have now:

```rust
pub enum Node {
    Terminal { node: Rc<TerminalNode> },
    NonTerminal { node: Rc<NonTerminalNode> },
}

pub struct TerminalNode {
    pub kind: TerminalKind,
    pub text: String,
}

pub struct NonTerminalNode {
    pub kind: NonTerminalKind,
    pub text_length: TextIndex,
    pub children: Vec<Node>,
}
```

## AST

We intend to also produce a strongly typed tree (structs and enums). Having strong types provides safety/correctness
guarantees for users. It also allows us to generate visitor and rewriter APIs automatically.

Each AST node should provide an API to get the underlying CST node, where users can iterate over the actual terminals as
they appear in input, and get their position in the source code. However, this is a one-way operation. CST nodes don't
hold references to their AST nodes.

Note: some compilers drop syntactic elements that don't carry semantic information from their AST (like semicolons, or
commas). However, we don't make that distinction, as we intend to implement further analysis in the form of micro-passes,
that each can rewrite and pick parts of the tree that are relevant to them. So our initial tree (AST) should be complete.

## Versioning

The biggest benefit of the new language definition is that it allows scanners and parsers to attempt parsing input belonging
to any language version, and report errors afterwards if the input is not valid for the selected version. This is a huge
benefit over existing parsers, where they will either parse an inaccurate superset of all versions, or they parse a specific version,
and produce unhelpful errors like `Unrecognized 'abstract' keyword` when the current language version doesn't support it.

Not only we will be able to recover from such errors and continue parsing, producing an accurate/complete tree at the end,
but we will also be able to produce much better errors like:
`The 'abstract' keyword is not supported in the current version X. Please upgrade to version Y instead to be able to use it`.

## Terminals

### Token Items

Tokens consist of one or more `TokenDefinition`. Each definition is separate/unique, but produces the same `TerminalKind`.
This is useful for tokens like `DecimalLiteral` and `HexLiteral` who can have multiple forms, but each form is enabled
or disabled in certain versions of the language.

All definitions have a unique `Scanner`, and they can be combined in the same trie/FSM to produce a single token at each
position in the input. Afterwards, the scanner can compare the definition's `enabled` property
with the current language version, adding an error if they don't match, but continue parsing anyway.

### Keyword Items

Keywords also contribute a `TerminalKind`, and consist of one or more `KeywordDefinition`. But they have additional semantics:

First, instead of defining a regular `Scanner`, it defines a `KeywordValue` that produces a finite set of possibilities.
Most only produce one value (like `abstract` or `contract`), but some can produce multiple, like `bytesN` or `fixedMxN`,
that can have different values for `M` and `N`. This is important for us to build hash sets and quickly check for membership.

Second, because keywords can also overlap with identifiers, each keyword has an `identifier` property that refers to which
identifier token they can match. Instead of being part of same trie/FSM as tokens, whenever we want to scan a keyword,
we try to scan its identifier instead. Afterwards, we check if its contents match one of the values of the keyword.

Third, they have two additional `reserved` property. We should use these when we scan identifiers,
to make sure that the resulting identifier doesn't match a reserved keyword, and if so, we should report an error, but
continue parsing.

Unique to Solidity, keywords can be `reserved` in versions before or after the versions they are `enabled` in. They can
also be not `reserved` in versions before or after the versions they stop being `enabled` in. So we have to have these
additional checks, to be able to catch cases like when a certain input can both be a keyword and an identifier, or neither.

We should also be able to generate a public API `is_keyword(TerminalKind)` for users to conveniently detect them if needed.

### Trivia Items

Trivia items are similar tokens, contributing their own `TerminalKind`. They are referred to from the language's top-level
`leading_trivia` and `trailing_trivia` properties. Before and after each token, the scanner should try to scan these tokens,
collecting them in a flat list.

Previously, we used to create many `LeadingTrivia` and `TrailingTrivia` nodes that hold whitespace/comments. Not only this
is wasteful memory-wise, it is also unnatural/unexpected to wrap whitespace in non-terminal nodes. Instead, I propose
treating them like any other token, and storing them as siblings to the tokens they belong to (in-order). Not only this
is simpler, it is also more efficient, and is natural to how input is consumed and produced.

### Fragment Items

Fragments are not visible to users, and don't contribute a `TerminalKind`. They are just a utility used to refactor
common parts of the grammar, and avoid duplication. During processing the language definition, they are inlined wherever
they are referenced.

## NonTerminals

### Struct Items

Structs represent a flat list (sequence) of typed fields. They are the simplest non-terminal, and generate a `struct` AST type.
Their fields match 1-1 with the item fields. The struct name contributes a `NonTerminalKind`.

Each field can be either `Required(T)` or `Optional(T)`. Required fields are always present and parsed. Optional fields
can be omitted if they don't exist, and are represented with Rust's `Option<T>` type (or TypeScript `T | undefined`).
However, optional fields have an additional `enabled` property. After parsing optional fields, we should compare them
with the current language version, and produce errors if they don't match, but continue parsing normally.

The type of each field can be a `NonTerminal(T)` or `Terminal(Set<T>)`. A non-terminal field refers to another item, and
holds its type. A terminal field refers to one or more terminal items (all valid in this position), and is of type `TerminalNode`.

Additionally, the struct also stores the CST node that holds its contents:

```rust title="Definition"
Struct(
    name = ParametersDeclaration,
    fields = (
        open_paren = Required(Terminal([OpenParen])),
        parameters = Required(NonTerminal(Parameters)),
        close_paren = Required(Terminal([CloseParen]))
    )
)
```

```rust title="AST Type"
pub struct ParametersDeclaration {
    pub open_paren: Rc<TerminalNode>,
    pub parameters: Rc<Parameters>,
    pub close_paren: Rc<TerminalNode>,

    pub cst: Rc<NonTerminalNode>,
}
```

### Enum Items

Enums represent an ordered choice operator of multiple variants (possibilities). The enum name itself does NOT contribute
a `NonTerminalKind`, since it will always result in one of its variants (each with a unique `TerminalKind` or a `NonTerminalKind`.
They only exist in the AST, and don't affect the CST at all.

We attempt to parse each variant (in-order), and choose the first one that succeeds. However, each variant can have
an additional `enabled` property. We should always try to parse the variants that are valid in the current version first,
and if not, still parse the rest, but produce an error afterwards. The fields of each variant are parsed similar to a
struct fields (example above).

```rust title="Definition"
Enum(
    name = FunctionBody,
    variants = [
        EnumVariant(name = Block, reference = Block),
        EnumVariant(name = Semicolon, reference = Semicolon)
    ]
)
```

```rust title="AST Type"
pub enum FunctionBody {
    Block {
        block: Rc<Block>,

        cst: Rc<NonTerminalNode>,
    },
    Semicolon {
        semicolon: Rc<TerminalNode>,

        cst: Rc<NonTerminalNode>,
    },
}
```

### Repeated Items

Repeated items represent a list of items of the same kind. The item name contributes a `NonTerminalKind`.
The AST type is a wrapper around a `Vec<T>`, with any utilities we need to add for convenience.

It has an `allow_empty` boolean property, which allows parsing zero items. If it is `false`, we should still allow parsing
zero items, but produce an error afterwards.

```rust title="Definition"
Repeated(
    name = FunctionAttributes,
    repeated = FunctionAttribute,
    allow_empty = true
)
```

```rust title="AST Type"
pub struct FunctionAttributes {
    pub items: Vec<Rc<FunctionAttribute>>

    pub cst: Rc<NonTerminalNode>,
}
```

### Separated Items

Separated items represent a list of items of the same kind, separated by a delimiter.
The item name contributes a `NonTerminalKind`. The AST type is a wrapper around two `Vec<T>` for items and their delimiters,
with any utilities we need to add for convenience. For example, we should add APIs to create iterators for only the
separated items, the separators, or both (in-order).

It has an `allow_empty` boolean property, which allows parsing zero items. If it is `false`, we should still allow parsing
zero items, but produce an error afterwards. We should also allow parsing a trailing separator at the end, but still produce
an error afterwards.

```rust title="Definition"
Separated(
    name = EventParameters,
    separated = EventParameter,
    separator = Comma,
    allow_empty = true
)
```

```rust title="AST Type"
pub struct EventParameters {
    pub items: Vec<Rc<EventParameter>>
    pub separators: Vec<Rc<TerminalNode>>

    pub cst: Rc<NonTerminalNode>,
}
```

### Precedence Items

This is perhaps the most complex non-terminal. It still uses the same PRATT algorithm from the previous implementation (no changes there), but
adapted for the new AST types. It has two lists:

First, a list of `precedence_expressions`, with each expression having a list of operators. Each operator has its own
versioning (`enabled` property), a list of fields, and a model (prefix/postfix/binary).

The operators from all expressions are flattened and combined in the parent PRATT parser. That grouping is only used to
indicate that some operators can produce the same `PrecedenceExpression` name. However, we should exclude operators that
don't match the current language version. This is useful for things like `ExponentiationExpression` where it has two
operators with different associativity, but defined in enabled/disabled in different versions.

Second, a list of `primary_expressions`, with their own versioning (`enabled` property) as well.
We should try to parse them as an operator (similar to `EnumItem`), and produce an error if the version doesn't match afterwards.

It is important to note that the item name doesn't contribute a `NonTerminalKind`, but each `PrecedenceExpression` under it contributes one.

```rust title="Definition"
Precedence(
    name = Expression,
    precedence_expressions = [
        PrecedenceExpression(
            name = AdditionExpression,
            operators = [PrecedenceOperator(
                model = BinaryLeftAssociative,
                fields = (operator = Required(Terminal([Plus])))
            )]
        ),
        PrecedenceExpression(
            name = FunctionCallExpression,
            operators = [PrecedenceOperator(
                model = Postfix,
                fields = (
                    open_paren = Required(Terminal([OpenParen])),
                    arguments = Required(NonTerminal(Arguments)),
                    close_paren = Required(Terminal([CloseParen]))
                )
            )]
        ),
        PrecedenceExpression(
            name = NegationExpression,
            operators = [PrecedenceOperator(
                model = Prefix,
                fields = (operator = Required(Terminal([Not])))
            )]
        )
    )],
    primary_expressions = [
        PrimaryExpression(expression = Identifier),
        PrimaryExpression(expression = NumberLiteral),
        PrimaryExpression(expression = StringLiteral)
    ]
)
```

```rust title="AST Type"
pub enum Expression {
    AdditionExpression { expression: Rc<AdditionExpression> },
    FunctionCallExpression { expression: Rc<FunctionCallExpression> },
    NegationExpression { expression: Rc<NegationExpression> },

    Identifier { expression: Rc<TerminalNode> },
    NumberLiteral { expression: Rc<TerminalNode> },
    StringLiteral { expression: Rc<TerminalNode> },
}

pub struct AdditionExpression {
    // 'left_operand' auto-generated (before) because it is a binary expression, and same type as parent
    pub left_operand: Rc<Expression>,
    // operator 'fields' are flattened into the expression node here
    pub operator: Rc<TerminalNode>,
    // 'right_operand' auto-generated (after) because it is a binary expression, and same type as parent
    pub right_operand: Rc<Expression>,

    pub cst: Rc<NonTerminalNode>,
}

pub struct FunctionCallExpression {
    // 'operand' auto-generated (before) because it is a postfix expression, and same type as parent
    pub operand: Rc<Expression>,
    // operator 'fields' are flattened into the expression node here
    pub open_paren: Rc<TerminalNode>,
    pub arguments: Rc<Arguments>,
    pub close_paren: Rc<TerminalNode>,

    pub cst: Rc<NonTerminalNode>,
}

pub struct NegationExpression {
    // operator 'fields' are flattened into the expression node here
    pub operator: Rc<TerminalNode>,
    // 'operand' auto-generated (after) because it is a prefix expression, and same type as parent
    pub operand: Rc<Expression>,

    pub cst: Rc<NonTerminalNode>,
}
```

## Error Recovery

For the CST, I think the current algorithms work well, and we should be able to keep them. Unrecognized (skipped) input
is grouped into one token, and we can just add it as-is to the `cst` node under its AST node.

During AST construction, we will simply check for `TerminalKind::UNRECOGNIZED` nodes, and skip construction if there are any.

## Public API Changes

Based on the above, I propose the following changes to the current public API:

-   Rename `TokenKind` to `TerminalKind`, since it will also refer to trivia.
-   Rename `RuleKind` to `NonTerminalKind`, since "rule" is ambiguous.
-   Rename `TerminalKind::SKIPPED`to `UNRECOGNIZED` for clarity.
-   Hide `LexicalContext` and `fn scan(TokenKind)` from the public API, as it is a short-term workaround,
    and will be replaced later when we have language embedding.
-   Remove `ProductionKind` completely, since it is no longer needed. We only need to expose `fn parse(NonTerminalKind)`.
-   Since `EndOFFileTrivia` no longer exists, `ParseResult` should collect any remaining trivia at the end of the input,
    and include it in the `ParseResult` returned, for any kind of node, not just `SourceUnit`.

## Visitors and Cursors

The current CST visitors/cursors should still work as-is, since the CST tree will be unchanged. However, the new AST types
allow us in the future to produce typed visitors and traits with named functions for every node type, similar to a lot of
other AST processing libraries. I want to at least produce an immutable `Visitor` and a mutable `Rewriter`.
