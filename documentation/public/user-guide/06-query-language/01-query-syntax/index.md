# 6.1. Query Syntax

It's often more convenient to use the declarative `Query` API to traverse the CST, as they allow you to express your
intent more concisely and can largely replace the need for both internal (cursor), and external (visitor) iterator patterns.

The query engine performs pattern matching, and the execution semantics are closer to unification than to regular expression matching.
A query returns all possible matches, not just the longest/shortest/first/last match.

## Matching

A _query_ is a pattern that matches a
certain set of nodes in a tree. The expression to match a given node
consists of a pair of brackets (`[]`) containing two things: the node's kind, and
optionally, a series of other patterns that match the node's children. For
example, this pattern would match any `MultiplicativeExpression` node that has
two children `Expression` nodes, with an `Asterisk` node in between:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:query-syntax-1"
```

The children of a node can optionally be labeled. The label is a property of the edge from
the node to the child, and is not a property of the child. For example, this pattern will match
a `MultiplicativeExpression` node with the two `Expression` children labeled `left_operand` and `right_operand`:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:query-syntax-2"
```

You can also match a node's textual content using a string literal. For example, this pattern would match a
`MultiplicativeExpression` with a `*` operator (for clarity):

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:query-syntax-3"
```

If you don't care about the kind of a node, you can use an underscore `_`, which matches any kind.
For example, this pattern will match a `MultiplicativeExpression`
node with any two children with any kind, as long as one of them is labeled `left_operand`:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:query-syntax-4"
```

Children can be elided. For example, this would produce multiple matches for a
`MultiplicativeExpression` where at least _one_ of the children is an expression of a `StringExpression` variant, where each match
is associated with each of the `StringExpression` children:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:query-syntax-5"
```

Trivia nodes (whitespace, comments, etc.) will be skipped over when running a
query. Furthermore, trivia nodes cannot be explicitly (or implicitly with `_`)
matched by queries.

## Capturing

When matching patterns, you may want to process specific nodes within the
pattern. Captures allow you to associate names with specific nodes in a pattern,
so that you can later refer to those nodes by those names. Capture names are
written _before_ the nodes that they refer to, and start with an `@` character.

For example, this pattern would match any struct definition and it would associate
the name `struct_name` with the identifier:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:capturing-nodes-1"
```

And this pattern would match all event definitions for a contract, associating the name
`event_name` with the event name, `contract_name` with the containing contract name:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:capturing-nodes-2"
```

## Quantification

You can surround a sequence of patterns in parenthesis (`()`), followed
by a `?`, `*` or `+` operator. The `?` operator matches _zero or one_ repetitions
of a pattern, the `*` operator matches _zero or more_, and the `+` operator
matches _one or more_.

For example, this pattern would match a sequence of one or more import directives at the top of the file:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:quantification-1"
```

This pattern would match a structure definition with one or more members, capturing their names:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:quantification-2"
```

This pattern would match all function calls, capturing a string argument if one was
present:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:quantification-3"
```

## Alternation

An alternation is written as a sequence of patterns separated by `|` and surrounded by parentheses.

For example, this pattern would match a call to either a variable or an object property.
In the case of a variable, capture it as `@function`, and in the case of a property, capture it as `@method`:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:alternations-1"
```

This pattern would match a set of possible keyword terminals, capturing them as `@keyword`:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:alternations-2"
```

## Adjacency

By using the adjacency operator `.` you can constrain a pattern to only match
the first or the last child nodes.

For example, the following pattern would match only the first parameter
declaration in a function definition:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:adjacency-1"
```

And conversely the following will match only the last parameter:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:adjacency-2"
```

If the adjacency operator is used in between two patterns it constrains matches
on both patterns to occur consecutively, ie. without any other sibling node in
between. For example, this pattern matches pairs of consecutive statements:

```.scheme
--8<-- "documentation/public/user-guide/06-query-language/01-query-syntax/examples/01-queries.test.mts:adjacency-3"
```
