<!-- cSpell:disable -->
<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-duplicate-heading -->

# SLANG Code Generator

## Grammar

A grammar is an unordered set of named productions. All the productions in a
grammar must have distinct names.

A grammar requires two `Trivia` productions, with names `LeadingTrivia` and `TrailingTrivia`.

## Production

A production is an algebraic datatype with five cases.

`Rule`

: corresponds to typical non-recursive parsing rules.

`ExpressionRule`, and `ExpressionMemberRule` implement recursive expression
parsers with precedence and associativity.

`Trivia` is a special case of `Rule` that is used to handle whitespace, comments,
and other out-of-band information.

`Token` is strictly lexical.

### `Rule { expression: Expression }`

`Rule`s expose their internal substructure

### `ExpressionRule { references: [string] }`

A list of names of `ExpressionMemberRule` productions. `ExpressionRule`
implements a precedence mechanism over the referenced productions, each of which
can recursively reference this expression in either left, right, or both
positions. The `ExpressionMemberRule` productions are assigned a precedence
according to the order they appear in the `references`.

### `ExpressionMemberRule: { expression: Expression, associativity?: Left|Right = Left }`

Must be referenced from only a single `ExpressionRule` production. They can
themselves reference that `ExpressionRule` and specify that the recursion is
resolved with either left or right associativity.

!!! question

    Maybe we want to allow a builder on all productions so that the type and
    constructor can be selectively overridden

### `Trivia { expression: Expression }`

Any `reference`s in the `expression` must be to `Trivia` or `Token`
productions.

`Trivia` are are structurally similar to rules, but are handled specially by
the parser and the tree visitors.

Unlike a traditional parser that discards white space and comments (i.e.
_trivia_), slang preserves this, and even parses structure within it, such as
documentation embedded in comments. We do this for two reasons

- Comment content is often used for out-of-band information that is critical to
  the toolchain; and

- Slang wants to be useful for refactoring and formatting, both of which require
  preservation of such information.

### `Token { expression: Expression, builder?: bool = false }`

Any `reference`s in the `expression` must be to other `Token` productions.

In the AST you might want to create a domain object, such as a number, from
the substructure of a `Token`. For this case a builder can be used by setting
`builder: true` on the production. The name of the production is transformed
to a valid Rust module name, and that module should look something like the
following:

```rust title="parser_user.rs"
mod production_name_as_module_name {
    pub type ASTTokenType = … // or a struct etc
    pub from_node(node: slang::cst::Node) -> ASTType { … }
}
…
```

### Code Generation

There are two forms of code generation: parser and lexer. The same expression
structure is used for both, but the code generated is different.

#### Lexer

The lexer is just as powerful a recognizer as the parser, but doesn't include
error recovery, doesn't handle trivia, and produces only enough structure to
ease the writing of functions that produce a value for inclusion in the AST.
`Token` productions can specify whether such a function should be used by
setting the `builder` option to `true`. The structure created for this purpose
is not exposed in the parser.

Regardless of the vaue of `builder`, each reference to a `Token` production
results in the creation of a `cst::Node::Token` node in the CST. The trivia in that
node is parsed and created by the parser, not the lexer. The lexer doesn't know
about trivia.

If `builder` is `false` then the AST does not manifest the token. If `builder`
is true, then the AST manifests the token as as a `Token<T>` where `T` is the
return type of the builder.

```yaml
# grammar
```

```rust
// type of lexer rule
```

```rust
// example builder
```

```rust title="lex.rs"
  #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
  pub enum Node {
      None,
      Chars(Range<usize>),
      Choice(usize, NodeRef),
      Sequence(Vec<NodeRef>),
      Named(kinds::Token, NodeRef),
  }

  pub type NodeRef = Box<Node>;
```

| Expression        | Lexer Type  |
| ----------------- | ----------- |
| `terminal`        | `Chars`     |
| `character_range` | `Chars`     |
| `choice`          | `Choice`    |
| `sequence`        | `Sequence`  |
| `optional`        | `None \| T` |
| `repeated`        | `Sequence`  |
| `reference`       | `Reference` |
| `delimited_by`    | `Sequence`  |
| `separated_by`    | `Sequence`  |
| `zero_or_more`    | `Sequence`  |
| `one_or_more`     | `Sequence`  |
| `difference`      | `T`         |
| `lookahead`       | `T`         |

#### Parser

Every production generates a parser, and a type declaration, which is the type
returned by the AST Parser. These are collected into a module which has roughly
the following interface and implementation:

```rust title="parser.rs"
// For each non-`Token` production
mod production_name_as_module_name {
   pub type ASTType = …
}
…

// For each `Token` production
mod production_name_as_module_name {
   // If it has a builder
   pub type ASTTokenType = parser_user::ASTTokenType; // i.e. re-export
   pub type ASTType = Token<ASTTokenType>
   // otherwise
   pub type ASTType = Token<()>
}
…

pub type ParserInput = … ;
pub type ParserResult<T> = (cst::Node, Option<T>);

pub type Parser<T> = Fn(ParserInput) -> ParserResult<T>;
struct Parsers {
   production_name: Parser<production_name_as_module_name::ASTType>,
   …
}
```

```rust title="parser_impl.rs"
use parser::*;

impl Parsers {
   pub fn new() -> Parsers {
      // Create a parser for each production, and return a Parsers struct
      …
      return Parsers {
         production_name: …,
         …
      }
   }
}
```

The parse result always includes a CST, but in the case of a parse error, the
AST is not produced, hence the `Option` type. The parse errors are available as
error nodes within the CST.

### Name inheritance

In all productions, the name of the production is applied recursively to its
expression tree. Named expressions, i.e. `choice` and `sequence`, set their name
from this inherited attribute (regardless of whether the author explicitly set
the name), and the recursion stops. Expressions that contain another expression
recursively apply the name to their primary child e.g. `minuend` in the case of
`difference:` expressions. Leaf expressions, i.e. those that don't contain other
expressions, do not recurse. Note that the only expressions with structural
multi-arity are `choice` and `sequence`, so this process doesn't branch.

Any `terminal` or `character_range` expression that remain anonymous after this
process are assigned names based on their content.

Any `sequence` expressions that remain anonymous after this process, or that
have any anonymous elements are generated as tuples rather than structs.

In all other cases, where a name is required by not provided, an error is
signaled. The most obvious case is a `choice` expression that contains an
anonymous element - the case must be named.

## Expression

A production is an algebraic datatype with the following cases:

- `reference { name: string, inline_in_ebnf?: boolean = false }`

  A reference to another production.

  In the CST this generates a single expression of type `Node`, which is the
  result of the referenced production.

  In the AST this generates a reference to the production result, named after the
  production if a name is required.

- `choice { name?: string, choices: [Expression] }`

  A choice between two or more expressions

  In the CST this generates a single expression of type `Node`.

  In the AST this generates an enum, which is always named because of the name
  inheritence process i.e. un-named `choice` expressions raise an error. Each of
  the components generates a case in the enum.

- `sequence { name?: string, expr₀: Expression, …, exprₖ: Expression }`

  A sequence of expressions.

  In the CST this generates a `Node::Rule` or a `Node::Group` with each of the
  expressions as children, depending on whether a name is supplied or inherited
  (or not).

  In the AST this generates a struct or a tuple, depending on whether a name is
  supplied or inherited (or not). Each of the elements generates a named field in
  the struct or an un-named element of the tuple.

- `terminal { mame?: string, text: string }`

  A string of one or more unicode characters.

  If the name is not supplied or inherited then a name is computed based on the
  text of the terminal.

  TODO - generates?

- `character_range { mame?: string, from: char, to: char }`

  A single character whose unicode codepoint falls between two given (inclusive)
  codepoints

  If the name is not supplied or inherited then a name is computed based on the
  character range.

  TODO - generates?

- `delimited_by { expression: Expression, start: string, close: string }`

  An expression delimited by a start string and an end string.

  In the CST this generates a `Node::Group` of `[ terminal, expression, terminal ]`.

  In the AST this generates a `expression_nme: DelimitedBy<T>`, linked to the CST `Node::Group`.

- `separated_by { expression: Expression, separator: string }`

  A sequence of one or more instances of an expression, separated by a string.

  In the CST this generates a `Node::Group` of `[ expression, terminal, expression, … ]`.

  In the AST this generates a `expression_name: SeparatedBy<T>`, linked to the CST `Node::Group`.

- `optional { expression: Expression }`

  An expression that may be omitted.

  In the CST this generates the expression or `Node::None`. `Node:None` is
  required to ensure that indexes into the children are constant.

  In the AST this generates `expression_name: Option<expression>`.

- `zero_or_more { expression: Expression }`

  A sequence of zero of more instances of an expression

  In the CST this generates a sequence of children that are either lifted into a
  parent sequence or represented as `Node::Group`.

  In the AST this generates _`plural`_(`expression_name`)`: ZeroOrMore<expression>`.

- `one_or_more { expression: Expression }`

  A sequence of one or more instances of an expression

  In the CST this generates a sequence of children that are either lifted into a
  parent sequence or represented as `Node::Group`.

  In the AST this generates _`plural`_(`expression_name`)`: OneOrMore<expression>`.

- `repeated { expression: Expression, min?: int, max?: int }`

  A sequence of instances of an expression, with an optional lower bound and an
  optional upper bound on the number of such instances

  In the CST this generates a sequence of children that are either lifted into a
  parent sequence or represented as `Node::Group`.

  In the AST this generates _`plural`_(`expression_name`)`: Repeated<expression>`.

- `difference { minuend: Expression, subtrahend: Expression }`

  An instance of an expression that is not an instance of a second expression

  In both the CST and AST this generates whatever `minuend` generates.

- `lookahead { expression: Expression, followed_by: Expression }`

  An instance of an expression that must be followed by an instance of a second
  expression. The second expression is not consumed.

  In both the CST and AST this generates whatever `expression` generates.

The ADT cases do not form a minimal set, in the sense that e.g. `delimited_by`
could be represented by a `sequence` of three elements. However using a more
semantically meaningful construct improves many aspects of the tooling other
than happy-path parsing, such as error recovery and reporting, code folding etc.

## Concrete Syntax Tree

```rust title="cst.rs"
pub enum RuleKind {
   // i.e. ContractDefinition
}
pub enum TriviaKind {
   // i.e. LeadingTrivia
}
pub enum Node {
    None,
    Rule {
        range: Range<usize>,
        kind: RuleKind,
        children: Vec<NodeRef>,
    },
    // Distinct from Rule in order to use a different kind enum
    Trivia {
        range: Range<usize>,
        kind: TriviaKind,
        children: Vec<NodeRef>,
    },
    Token {
        range: Range<usize>, // Doesn't include the trivia
        kind: lexer::TokenKind,
        trivia: Vec<NodeRef>
    },
    // For anonymous groups referenced from AST nodes i.e. `delimited_by`
    Group {
        range: Range<usize>,
        children: Vec<NodeRef>,
    }
    // TODO: Error types
}

pub type NodeRef = Rc<Node>;
```

## Abstract Syntax Tree

The type of the AST tree is simply built from types that directly reflect the expression:

```rust title="ast.rs"
// The type of the`sequence` expression named N
pub struct N { pub node: NodeRef, pub F₀: T₀, …, pub Fₖ: Tₖ };

// The type of the `choice` expression named N
pub enum N { F₀(pub T₀), …, Fₖ(pub Tₖ) };

// The type of a `terminal` or `character_range` expression,
// or a `reference` to a `Token` production.
// T is either `()` or the type returned by the (optional) builder
pub struct Token<T> {
    pub node: Node,
    pub value: T
}

// the type of a `delimited_by` expression
pub struct DelimitedBy<T> {
    pub node: Node,
    pub value: T
}

// the type of a `separated_by` expression
pub struct SeparatedBy<T> {
    pub node: Node,
    pub values: Vec<T>
}

// The type of an `optional` expression
pub type Optional<T> = Option<T>

// the type of a `zero_or_more` expression
pub type ZeroOrMore<T> = Vec<T>

// The type of a `one_or_more` expression
pub type OneOrMore<T> = Vec<T>

// The type of a `repeated` expression
pub type Repeated<T> = Vec<T>
```
