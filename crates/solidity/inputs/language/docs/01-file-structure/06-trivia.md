## Single Line Comments

A single-line comment is terminated by any unicode line terminator (`LF`, `VF`, `FF`, `CR`, `NEL`, `LS` or `PS`) in UTF-8 encoding. The terminator is still part of the source code after the comment, so if it is not an ASCII symbol (these are `NEL`, `LS` and `PS`), it will lead to a parser error.

```solidity
// This is a single-line comment.
```

## Multi-line Comments

Comments starting with `/*` and ending with `*/` are allowed to range multiple lines:

```solidity
/*
This is a
multi-line comment.
*/
```

## NatSpec Comments

Additionally, there is another type of comment called a NatSpec comment. They are written with a triple slash `///` or a double asterisk block `/**...*/` and they should be used directly above function declarations or statements. It is recommended that Solidity contracts are fully annotated using NatSpec for all public interfaces (everything in the ABI).

```solidity
/// @author My Team Name
/// @title A simple contract example
contract MyContract {}
```

Please see the [NatSpec Format](./07-nat-spec-format.md) section for further information.

--8<-- "crates/solidity/inputs/language/snippets/under-construction.md"
