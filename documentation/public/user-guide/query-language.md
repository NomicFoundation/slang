# The Tree Query Language

## Query Syntax

A _query_ is a pattern that matches a
certain set of nodes in a tree. The expression to match a given node
consists of a pair of brackets (`[]`) containing two things: the node's kind, and
optionally, a series of other patterns that match the node's children. For
example, this pattern would match any `binary_expression` node whose children
are both `number_literal` nodes:

```scheme
[binary_expression [number_literal] [number_literal]]
```

The children of a node can optionally be named. The name is a property of the edge from
the node to the child, and is not a property of the child. For example, this pattern will match a `binary_expression`
node with two `number literal` children, named `left` and `right`:

```scheme
[binary_expression [left:number_literal] [right:number_literal]]
```

If you don't care about the kind of a node, you can use a underscore '\_', which matches any kind.
For example, this pattern will match a `binary_expression`
node with two children, one of any kind named`left` and one of any kind:

```scheme
[binary_expression [left:_] [_]]
```

Children can also be elided. For example, this would produce multiple matches for a
`binary_expression` where at least _one_ of the children is a `string_literal` node, where each match
is associated with each of the `string_literal` children:

```scheme
[binary_expression ... [string_literal] ...]
```

### Capturing Nodes

When matching patterns, you may want to process specific nodes within the
pattern. Captures allow you to associate names with specific nodes in a pattern,
so that you can later refer to those nodes by those names. Capture names are
written _after_ the nodes that they refer to, and start with an `@` character.

For example, this pattern would match any assignment of a `function` to an
`identifier`, and it would associate the name `the-function-name` with the
identifier:

```scheme
[assignment_expression
  @the-function-name [left:identifier]
  [right:function]]
```

And this pattern would match all method definitions, associating the name
`the-method-name` with the method name, `the-class-name` with the containing
class name:

```scheme
[class_declaration
  @the-class-name [name:identifier]
  [body:class_body
    [method_definition
      @the-method-name [name:property_identifier]]]]
```

### Quantification Operators

You can match a repeating sequence of sibling nodes using the postfix `+` and
`*` _repetition_ operators, which work analogously to the `+` and `*` operators
[in regular
expressions](https://en.wikipedia.org/wiki/Regular_expression#Basic_concepts).
The `+` operator matches _one or more_ repetitions of a pattern, and the `*`
operator matches _zero or more_.

For example, this pattern would match a sequence of one or more comments:

```scheme
([comment])+
```

This pattern would match a class declaration, capturing all of the decorators if
any were present:

```scheme
[class_declaration
  (@the-decorator [decorator])*
  @the-name [name:identifier]]
```

You can also mark a node as optional using the `?` operator. For example, this
pattern would match all function calls, capturing a string argument if one was
present:

```scheme
[call_expression
  @the-function [function:identifier]
  [arguments:arguments (@the-string-arg [string])?]]
```

### Grouping Sibling Nodes

You can also use parentheses for grouping a sequence of _sibling_ nodes. For
example, this pattern would match a comment followed by a function declaration:

```scheme
(
  [comment]
  [function_declaration]
)
```

Any of the quantification operators mentioned above (`+`, `*`, and `?`) can also
be applied to groups. For example, this pattern would match a comma-separated
series of numbers:

```scheme
(
  [number]
  ("," [number])*
)
```

### Alternations

An alternation is written as a pair of square brackets (`[]`) containing a list of alternative patterns.
This is similar to _character classes_ from regular expressions (`[abc]` matches either a, b, or c).

For example, this pattern would match a call to either a variable or an object property.
In the case of a variable, capture it as `@function`, and in the case of a property, capture it as `@method`:

```scheme
[call_expression
  function: (
      [identifier] @function
    | [member_expression property: [property_identifier] @method]
  )
]
```

This pattern would match a set of possible keyword tokens, capturing them as `@keyword`:

```scheme
(
    "break"
  | "delete"
  | "else"
  | "for"
  | "function"
  | "if"
  | "return"
  | "try"
  | "while"
) @keyword
```

### Wildcard Node

A wildcard node is represented with an underscore (`_`), it matches any node.
This is similar to `.` in regular expressions.
There are two types, `(_)` will match any named node,
and `_` will match any named or anonymous node.

For example, this pattern would match any node inside a call:

```scheme
[call [_] @call.inner]
```

### Anchors

The anchor operator, `.`, is used to constrain the ways in which child patterns
are matched. It has different behaviors depending on where it's placed inside a
query.

When `.` is placed before the _first_ child within a parent pattern, the child
will only match when it is the first named node in the parent. For example, the
below pattern matches a given `array` node at most once, assigning the
`@the-element` capture to the first `identifier` node in the parent `array`:

```scheme
(array . (identifier) @the-element)
```

Without this anchor, the pattern would match once for every identifier in the
array, with `@the-element` bound to each matched identifier.

Similarly, an anchor placed after a pattern's _last_ child will cause that child
pattern to only match nodes that are the last named child of their parent. The
below pattern matches only nodes that are the last named child within a `block`.

```scheme
(block (_) @last-expression .)
```

Finally, an anchor _between_ two child patterns will cause the patterns to only
match nodes that are immediate siblings. The pattern below, given a long dotted
name like `a.b.c.d`, will only match pairs of consecutive identifiers: `a, b`,
`b, c`, and `c, d`.

```scheme
(dotted_name
  (identifier) @prev-id
  .
  (identifier) @next-id)
```

Without the anchor, non-consecutive pairs like `a, c` and `b, d` would also be matched.

The restrictions placed on a pattern by an anchor operator ignore anonymous nodes.
