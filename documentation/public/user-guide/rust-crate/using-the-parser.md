# Using the Parser

Using the API directly provides us with a more fine-grained control over the parsing process. It allows us to parse not just the input as a top-level source unit, but also individual rules like contracts, various definitions, and even expressions.

Let's start with this simple source file, that contains a single contract:

```solidity
--8<-- "documentation/public/user-guide/inputs/using-the-parser.sol"
```

We begin by creating a `Language` object with a specified version. This is an entry point for our parser API.
Then we can use it to parse the source file, specifying the top-level rule to parse:

```rust
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_parser.rs:imports"
```

```rust
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_parser.rs:parse-input"
```

If the file has errors, we can get them from the `ParseOutput` type, and print them out:

```rust
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_parser.rs:print-errors"
```

Otherwise, we can check if input is valid using this helpful utility:

```rust
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_parser.rs:assert-is-valid"
```

Now, let's try to inspect the resulting CST, and iterate on its children:

```rust
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_parser.rs:inspect-tree"
```

Additionally, we can convert the CST node back into the input string:

```rust
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/using_the_parser.rs:unparse-node"
```
