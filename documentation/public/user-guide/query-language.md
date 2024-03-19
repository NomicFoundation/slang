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

You can also match a node's textual content using a string literal. For example, this pattern would match a
`binary expression` with a `+` operator:

```scheme
[binary_expression [operator:"+"] [left:_] [right:_]]
```

If you don't care about the kind of a node, you can use an underscore '\_', which matches any kind.
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

### Quantification

You can surround a sequence of patterns in parenthesis (`()`), followed
by a `?`, `*` or `+` operator. The `?` operator matches _zero or one_ repetitions
of a pattern, the`*` operator matches _zero or more_, and the `+` operator
matches _one or more_.

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

This pattern would match all function calls, capturing a string argument if one was
present:

```scheme
[call_expression
  @the-function [function:identifier]
  [arguments:arguments (@the-string-arg [string])?]]
```

### Alternations

An alternation is written as a sequence of patterns separated by '|' and surrounded by parentheses.

For example, this pattern would match a call to either a variable or an object property.
In the case of a variable, capture it as `@function`, and in the case of a property, capture it as `@method`:

```scheme
[call_expression
  function: (
      @function [identifier]
    | [member_expression @method [property:property_identifier]]
  )
]
```

This pattern would match a set of possible keyword tokens, capturing them as `@keyword`:

```scheme
@keyword (
    "break"
  | "delete"
  | "else"
  | "for"
  | "function"
  | "if"
  | "return"
  | "try"
  | "while"
)
```
