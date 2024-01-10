# Using the Parser

Using the API directly provides us with a more fine-grained control over the parsing process. It allows us to parse not just the input as a top-level source unit, but also individual rules like contracts, various definitions, and even expressions.

## Parsing Source Files

Let's start with this simple source file, that contains a single contract:

```solidity title="input.sol"
--8<-- "documentation/public/user-guide/inputs/using-the-parser.sol"
```

We begin by creating a `Language` object with a specified version. This is an entry point for our parser API.
Then we can use it to parse the source file, specifying the top-level rule to parse:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-parser.ts:imports"

--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-parser.ts:parse-input"
```

## Checking for Syntax Errors

If the file has errors, we can get them from the `ParseOutput` type, and print them out:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-parser.ts:print-errors"
```

Otherwise, we can check if input is valid using this helpful utility:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-parser.ts:assert-is-valid"
```

## Inspecting the Parse Tree

Now, let's try to inspect the resulting CST, and iterate on its children:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-parser.ts:inspect-tree"
```

Additionally, we can convert the CST node back into the input string:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/using-the-parser.ts:unparse-node"
```
