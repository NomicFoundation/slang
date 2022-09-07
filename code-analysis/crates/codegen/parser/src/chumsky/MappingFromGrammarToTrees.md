<!-- cSpell:disable -->
<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-code -->

# Mapping from grammar to trees

## Productions

All productions generate a type:

`type `_`production`_` = T`

where `T` is the type in the table below. Note that in the case of structured
types, `T` is the name of the type e.g. _`name`_ in the table below. In that
case type of the production is a type alias.

All types that are declared as part of the internal expressions in the
production are namespaced in a module named after the production.

Future work _may_ involve

1. Inlining type aliases, which will eliminate many anonymous type names. Note
   however that in the case of aliased types, the alias is usable in all cases
   where the aliased form is expected, so the anonymous names are not required e.g.

   ```rust
   type MyProduction = my_production::Anon_1;
   mod my_production {
       struct Anon_1 { a:A, b:B, … }
       …
   }
   ```

   becomes

   ```rust
   struct MyProduction { a:A, b:B, … }
   mod my_production {
       …
   }
   ```

2. Inlining enum members where the member type isn't otherwise required e.g.

   ```rust
   struct Anon_1 { a:A, b:B, … }
   enum E { N(Anon_1), … }
   ```

   becomes

   ```rust
   enum E { N{ a:A, b:B, … }, … }
   ```

## Expressions

The name of an expression is, in order or precedence, either:

1. The name of the production in the case that it is the top expression in the production; or
2. The name give as `config.name` in the expression in the `YAML`; or
3. The default name given in the table below. In the case of anonymous names,
   the index _`n`_ is strictly monotonic increasing, starting at `0` in each production.

This name is used in two places:

1. As the name of the type in the case of structured types; and
2. As the name of the field in the case where the type is used as member of a structured type.

When used as field names in structured types, duplicate names are disambiguated
by adding a strictly monotonic increasing suffix `_`_`n`_ to the name, starting
at `0` in each structured type.

The canonical form of names `snake_case`. They are converted to `PascalCase` as required e.g. when used as type names.

| CombinatorTree Node Type            | Type                                                                                                                                                                                                                                                                                                                                       | Default Name                                                                  |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------- |
| `CharacterFilter`                   | Inside a lexical rule: `FixedSizeTerminal<1>`<br/>Inside a syntactic rule: `FixedSizeTerminalWithTrivia<1>`                                                                                                                                                                                                                                | if a single character then the name of that character, otherwise `anon_`_`n`_ |
| `TerminalTrie` - _fixed length_     | Inside a lexical rule: `FixedSizeTerminal<...>`<br/>Inside a syntactic rule: `FixedSizeTerminalWithTrivia<...>`                                                                                                                                                                                                                            | if a single terminal, then the terminal itself, otherwise `anon_`_`n`_        |
| `TerminalTrie` - _variable length_  | Inside a lexical rule: `VariableSizeTerminal`<br/>Inside a syntactic rule: `VariableSizeTerminalWithTrivia`                                                                                                                                                                                                                                | `anon_`_`n`_                                                                  |
| `Optional`                          | `Option<T>`                                                                                                                                                                                                                                                                                                                                | the name of `T`                                                               |
| `Repeated`                          | `Vec<T>`                                                                                                                                                                                                                                                                                                                                   | the pluralized name of `T`                                                    |
| `OneOrMore`                         | `Vec<T>`                                                                                                                                                                                                                                                                                                                                   | the pluralized name of `T`                                                    |
| `ZeroOrMore`                        | `Vec<T>`                                                                                                                                                                                                                                                                                                                                   | the pluralized name of `T`                                                    |
| `End`                               | `()`                                                                                                                                                                                                                                                                                                                                       | `end_of_input`                                                                |
| `SeparatedBy`                       | _`name`_<br/>**where** `struct `_`name`_` { N`<sub>0</sub>`:Vec`<sub>0</sub>`<T>, N`<sub>1</sub>`:Vec`<sub>1</sub>`<FST<…>> }`<br/>and `Vec`<sub>0</sub>`.size() == Vec`<sub>1</sub>`.size() + 1`                                                                                                                                          | `{N`<sub>0</sub>`}_and_{N`<sub>1</sub>`}`                                     |
| `DelimitedBy`                       | _`name`_<br/>**where** `struct `_`name`_` { N`<sub>0</sub>`:FST<…>, N`<sub>1</sub>`:T, N`<sub>2</sub>`:FST<…> }`                                                                                                                                                                                                                           | `{N`<sub>0</sub>`}_and_{N`<sub>1</sub>`}_and_{N`<sub>2</sub>`}`               |
| `Expression`                        | _`name`_<br/>**where** `enum `_`name`_` { N`<sub>0</sub>`(T`<sub>0</sub>`), …, N`<sub>k</sub>`(T`<sub>k</sub>`) }`<br/>and `T`<sub>_i_</sub> is the type of the `ExpressionMember`s listed below<br/>and `N`<sub>_i_</sub> is the name of `T`<sub>_i_</sub><br/>All `N`<sub>_i_</sub> are already unique because they are production names | Only occurs as the top expression in a production                             |
| `ExpressionMember` - _unary suffix_ | _`name`_<br/>**where** `struct `_`name`_` { left_operand:T, N:O }`<br/>and `N` is the name of `O`<br/>and `T` is the type of the owning `Expression`                                                                                                                                                                                       | Only occurs as the top expression in a production                             |
| `ExpressionMember` - _unary prefix_ | _`name`_<br/>**where** `struct `_`name`_` { N:O, right_operand:T }` <br/>and `N` is the name of `O`<br/>and `T` is the type of the owning `Expression`                                                                                                                                                                                     | Only occurs as the top expression in a production                             |
| `ExpressionMember` - _binary_       | _`name`_<br/>**where** `struct `_`name`_` { left_operand:T, N:O, right_operand:T }` <br/>and `N` is the name of `O`<br/>and `T` is the type of the owning `Expression`                                                                                                                                                                     | Only occurs as the top expression in a production                             |
| `Sequence`                          | _`name`_<br/>**where** `struct `_`name`_` { N`<sub>0</sub>`:T`<sub>0</sub>`, …, N`<sub>k</sub>`:T`<sub>k</sub>` }`<br/>and `N`<sub>_i_</sub> is the name of `T`<sub>_i_</sub><br/>and all `N`<sub>_i_</sub> are made unique by adding numeric suffix if necessary                                                                          | `anon_`_`n`_                                                                  |
| `Choice`                            | _`name`_<br/>**where** `enum `_`name`_` { N`<sub>0</sub>`(T`<sub>0</sub>`), …, N`<sub>k</sub>`(T`<sub>k</sub>`) }`<br/>and `N`<sub>_i_</sub> is the name of `T`<sub>_i_</sub><br/>and all `N`<sub>_i_</sub> are made unique by adding numeric suffix if necessary                                                                          | `anon_`_`n`_                                                                  |
| `Reference`                         | `T`                                                                                                                                                                                                                                                                                                                                        | the name of `T`                                                               |

## Transparent Expressions

These constructs reflect the properties of their primary expression.

| CombinatorTree Node Type       |
| ------------------------------ |
| `Difference`                   |
| `Lookahead`                    |
| `ExpressionMember` - _default_ |
