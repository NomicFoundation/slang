<!-- markdownlint-configure-file { "first-line-heading": { "level": 2 } } -->

<!--
cSpell:ignore Doxygen
cSpell:ignore inheritdoc
cSpell:ignore NatSpec
-->

## What is NatSpec?

Solidity contracts can use a special form of comments to provide rich documentation for functions, return variables and more. This special form is named the Ethereum Natural Language Specification Format (NatSpec). It was inspired by [Doxygen](https://en.wikipedia.org/wiki/Doxygen), and while it uses Doxygen-style comments and tags, there is no intention to keep strict compatibility with Doxygen.

It is recommended that Solidity contracts are fully annotated using NatSpec for all public interfaces (everything in the ABI). It is used in:

- Developer-focused output, for documenting, sharing, and reusing the source code.
- User-facing output, at the time that they will interact with the contract (i.e. sign a transaction).
- Machine-readable output, to be used by downstream tools.

## Syntax

Documentation can be inserted above each `contract`, `interface`, `library`,
`function`, `event` and `state variable`.

They can either exist in a single line format, starting with `///`:

```solidity
/// @title An example contract
contract MyContract {}
```

And also in multi-line format, starting with `/**` and ending with `*/`:

```solidity
/**
 * @title An example contract
 */
contract MyContract {}
```

## Tags

Tags categorize different comments according to their purpose. The table below shows the different tags supported. Please note that they are optional, and without one, the entire comment will be interpreted as it had a `@notice` tag.

| Tag | Description | Context |
| ----------- | ------------------------------------ | |
| `@title` | A title that should describe the contract/interface |`contract`, `library`, `interface` |
| `@author` | The name of the author | `contract`, `library`, `interface` |
| `@notice` | Explain to an end user what this does | `contract`, `library`, `interface`, `function`, `event`, `state variable` |
| `@dev`| Explain to a developer any extra details | `contract`, `library`, `interface`, `function`, `event`, `state variable` |
| `@param`| Documents a parameter just like in Doxygen (must be followed by parameter name) | `function`, `event` |
| `@return` | Documents the return variables of a contract's function | `function`, `state variable` |
| `@inheritdoc` | Copies all missing tags from the base function (must be followed by the contract name) | `function`, `state variable` |
| `@custom:FOO` | Custom tag, semantics is application-defined | can be used everywhere |

### Function Return Types

If your function returns multiple values, like `(int quotient, int remainder)`
then use multiple `@return` statements in the same format as the `@param` statements.

### Custom Tags

Custom tags start with `@custom:` and must be followed by one or more lowercase letters or hyphens. It cannot start with a hyphen however. They can be used everywhere and are part of the developer documentation. For example, `@custom:foo` or `@custom:foo-bar`. A good use case is analysis and verification tools.

## Dynamic expressions

The Solidity compiler will pass through NatSpec documentation from your Solidity source code to the JSON output as described in this guide. The consumer of this JSON output may present this to the end-user directly or it may apply some pre-processing.

Specifying these dynamic expressions is outside the scope of the Solidity documentation. However, you can find one useful example in the [RadSpec Project](https://github.com/aragon/radspec), where it evaluates references to function inputs to its values. For example, this line:

```solidity
/// @notice This function will multiply `a` by 7
```

Can be evaluated as the following, where the value of `a` is `10`:

```txt
This function will multiply 10 by 7
```

## Inheritance

Functions without NatSpec will automatically inherit the documentation of their base function. Exceptions to this are:

- When the parameter names are different.
- When there is more than one base function.
- When there is an explicit `@inheritdoc` tag which specifies which contract should be used to inherit.

--8<-- "snippets/under-construction.md"
