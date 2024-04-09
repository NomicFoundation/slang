# The Tree Query Language

## Query Syntax

A _query_ is a pattern that matches a
certain set of nodes in a tree. The expression to match a given node
consists of a pair of brackets (`[]`) containing two things: the node's kind, and
optionally, a series of other patterns that match the node's children. For
example, this pattern would match any `MultiplicativeExpression` node whose children
are exactly both `Expression` nodes with an `Asterisk` node in between (no whitespace):

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:query-syntax-1"
```

The children of a node can optionally be named. The name is a property of the edge from
the node to the child, and is not a property of the child. For example, this pattern will match
a `MultiplicativeExpression` node with the two `Expression` children named `left` and `right`:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:query-syntax-2"
```

You can also match a node's textual content using a string literal. For example, this pattern would match a
`MultiplicativeExpression` with a `*` operator (for clarity):

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:query-syntax-3"
```

If you don't care about the kind of a node, you can use an underscore `_`, which matches any kind.
For example, this pattern will match a `MultiplicativeExpression`
node with two children, one of any kind named `left` and one of any kind:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:query-syntax-4"
```

Children can also be elided. For example, this would produce multiple matches for a
`MultiplicativeExpression` where at least _one_ of the children is an expression of a `StringExpression` variant, where each match
is associated with each of the `StringExpression` children:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:query-syntax-5"
```

### Capturing Nodes

When matching patterns, you may want to process specific nodes within the
pattern. Captures allow you to associate names with specific nodes in a pattern,
so that you can later refer to those nodes by those names. Capture names are
written _before_ the nodes that they refer to, and start with an `@` character.

For example, this pattern would match any struct definition and it would associate
the name `struct_name` with the identifier:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:capturing-nodes-1"
```

And this pattern would match all event definitions for a contract, associating the name
`event_name` with the event name, `contract_name` with the containing contract name:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:capturing-nodes-2"
```

### Quantification

You can surround a sequence of patterns in parenthesis (`()`), followed
by a `?`, `*` or `+` operator. The `?` operator matches _zero or one_ repetitions
of a pattern, the `*` operator matches _zero or more_, and the `+` operator
matches _one or more_.

For example, this pattern would match a sequence of one or more comments at the top of the file:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:quantification-1"
```

This pattern would match a contract definition with at least one doc comment, capturing them:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:quantification-2"
```

This pattern would match all function calls, capturing a string argument if one was
present:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:quantification-3"
```

### Alternations

An alternation is written as a sequence of patterns separated by '|' and surrounded by parentheses.

For example, this pattern would match a call to either a variable or an object property.
In the case of a variable, capture it as `@function`, and in the case of a property, capture it as `@method`:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:alternations-1"
```

This pattern would match a set of possible keyword tokens, capturing them as `@keyword`:

```{ .scheme }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/query_language.rs:alternations-2"
```
