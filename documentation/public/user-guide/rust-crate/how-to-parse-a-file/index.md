# How to parse a Solidity file

In this guide, we'll walk you through the process of parsing a Solidity file using Slang. There are two ways to do it: using our CLI app or directly using the API. See [Installation](../#installation) on how to install Slang.

A file must be parsed according to a specific Solidity [version](../../../solidity-specification/supported-versions/). The version has to be explicitly specified and is not inferred from the source. To selectively parse parts of the source code using different versions, e.g. when the contract across multiple files has been flattened, you need to do that manually.

## Using the CLI

After installing our CLI, you should now have `slang_solidity` in your $PATH. By default, `cargo install`s binaries at `$HOME/.cargo/bin`, so make sure you have that in your $PATH in order to execute the CLI from the command-line.

Usage:

```bash
slang_solidity parse [--json] --version <VERSION> <FILE_PATH>
```

All parse errors are printed in a human-readable format; the command will succeed if there are no parse errors, and fail otherwise.

If `--json` is specified, a JSON representation of a Concrete Syntax Tree (CST) is printed to the standard output stream; nothing is printed otherwise.

The resulting parse tree has two node kinds: rules and tokens (see [Concepts](../../concepts.md)).

Example:

```json
// A Rule node
"Rule": {
  // Name of the rule kind
  "kind": "SourceUnit",
  // Length of the rule in Unicode code points, depending on the encoding used
  "text_len": {
    "utf8": 24,
    "utf16": 24,
    "char": 24 // de facto utf32
  },
  "children": [/* Rule or Token nodes */]
}
// A Token node
"Token": {
  // Name of the token kind
  "kind": "PragmaKeyword",
  // Literal value, taken from the source code
  "text": "pragma"
}
```

Because the resulting structure is well-defined and recursive, we can use the popular `jq` tool to quickly analyze the resulting output.

### Example 1: Reconstruct a Solidity file

In this example, we'll use the command-line interface (CLI) to parse a Solidity file and then reconstruct it using the Token `text` fields.

Since the the parse tree is simply a structured view of the underlying source code and our parser is complete (contains trivia), we can simply concatenate the text values of the token to reconstruct the source code.

We'll start with a bare-bones file:

```solidity title="file.sol"
pragma solidity ^0.8.0;
```

#### Step 1: Parse the Solidity file

First, use the `slang_solidity parse` command to parse the Solidity file:

```bash
VERSION=0.8.11
slang_solidity parse --json --version "$VERSION" file.sol > output.json
```

This command parses the Solidity file and outputs the resulting CST in JSON format.

#### Step 2: Reconstruct the source code

Next, let's inspect the tokens in the JSON output:

```bash
JQ_QUERY='recurse | select(.Token?) | .Token'
cat output.json | jq "$JQ_QUERY"
```

This gives us a flat list of the Token nodes:

```json
{
  "kind": "PragmaKeyword",
  "text": "pragma"
}
{
  "kind": "Whitespace",
  "text": " "
}
{
  "kind": "SolidityKeyword",
  "text": "solidity"
}
{
  "kind": "Whitespace",
  "text": " "
}
{
  "kind": "Caret",
  "text": "^"
}
{
  "kind": "VersionPragmaValue",
  "text": "0"
}
{
  "kind": "Period",
  "text": "."
}
{
  "kind": "VersionPragmaValue",
  "text": "8"
}
{
  "kind": "Period",
  "text": "."
}
{
  "kind": "VersionPragmaValue",
  "text": "0"
}
{
  "kind": "Semicolon",
  "text": ";"
}
{
  "kind": "EndOfLine",
  "text": "\n"
}
```

Now, we can adapt the query to select the `text` fields of the nodes and concatenate them:

```bash
$ JQ_QUERY='[recurse | select(.Token?) | .Token.text] | join("")'
$ cat output.json | jq "$JQ_QUERY"
"pragma solidity ^0.8.0;\n"
```

And that gives us back the reconstructed source code! 🎉

### Example 2: List the top-level contracts and their names

Let's try a more practical example. We'll get all the contracts defined in a file and print their names.

To do that, we need to:

1. select Rule nodes of `kind` equal to `ContractDefinition`
2. iterate over their children
3. select Token nodes of `kind` equal to `Identifier`
4. print their `text` fields.

Given this file:

```solidity
// file: title="file.sol"
contract Foo {}
contract Bar {}
contract Baz {}
```

We can construct the `jq` query as follows:

```bash
$ VERSION="0.8.0"
$ JQ_QUERY='.Rule | .. | select(.kind? == "ContractDefinition")' # Pick Rule nodes of kind "ContractDefinition"
$ JQ_QUERY+=' | .children[]'                                     # Iterate over their children
$ JQ_QUERY+=' | select(.Token?.kind == "Identifier")'            # Select Token nodes of kind "Identifier"
$ JQ_QUERY+=' | .Token.text'                                     # Print their text fields
$ slang_solidity parse --json --version "$VERSION" file.sol | jq "$JQ_QUERY"
"Foo"
"Bar"
"Baz"
```

## Using the Rust library

The Rust package, which is a regular crate published to crates.io, can be added to a project as a dependency using the following command:

```bash
cargo add slang_solidity
```

Using the API directly provides us with a more fine-grained control over the parsing process. It allows us to parse not just the input as a top-level source unit, but also individual rules like contracts, various definitions, and even expressions.

We begin by creating a `Language` struct with a specified version. This is an entry point for our parser API.

```rust
// We use `anyhow` for a convenient and simplistic error handling/propagation
use anyhow::Result;
use semver::Version;
use slang_solidity::cst::Node;
use slang_solidity::kinds::{RuleKind, TokenKind};
use slang_solidity::language::Language;

fn main() -> Result<()> {
  let language = Language::new(Version::parse("0.8.0")?)?;
  // Or, using the version directly:
  let language = Language::new(Version::new(0, 8, 0))?;

  // ...
}
```

Next, we need to read the source code. The `parse` function accepts a `&str` slice, so we need to load an owned `String` directly from the file or slice another existing `String` buffer.

Let's use the convenient [`std::fs::read_to_string`](https://doc.rust-lang.org/stable/std/fs/fn.read_to_string.html) helper to make our lives a bit easier.

```rust
// ...cont.
let source_code = std::fs::read_to_string("file.sol")?;

let parse_tree/*: ParseOutput*/ = language.parse(RuleKind::SourceUnit, &source_code);
```

The `ParseOutput` type that results from this process provides the following helpful functions:

-   `fn errors()/is_valid()` that return structured parse errors, if any,
-   `fn tree()` that gives us back a CST (partial if there were parse errors),
-   `fn create_tree_cursor()` that creates a `Cursor` type used to conveniently walk the parse tree.

### Example 1: Reconstruct the Solidity file

Let's try the same example, but this time we'll use the API directly.

We'll start with this file:

```solidity title="file.sol"
pragma solidity ^0.8.0;
```

#### Step 1: Parse the Solidity file

We can re-use the snippets above:

```rust
// ...cont.
let source_code = std::fs::read_to_string("file.sol")?;

let parse_tree/*: ParseOutput*/ = language.parse(RuleKind::SourceUnit, &source_code);
```

#### Step 2: Reconstruct the source code

The API already includes a convenience `Node::unparse` that does exactly that:

```rust
let output/*: String*/ = parse_tree.node().unparse();
assert_eq!(output, "pragma solidity ^0.8.0\n");
```

However, for the sake of practice, let's do this ourselves by walking the tree ourselves.

The `Cursor` type implements an `Iterator` trait by yielding the tree nodes in a depth-first search (DFS) fashion.

To only visit the `Token` nodes, we can use the [`Iterator::filter_map`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter_map) iterator adapter that filters and maps the tree token nodes using the helper [`Node::into_token`](https://docs.rs/slang_solidity/latest/slang_solidity/cst/enum.Node.html#method.into_token) conversion function.

```rust
let cursor = parse_tree.create_tree_cursor();
let only_tokens_iter = cursor.filter_map(|node| node.into_token());

let mut source = vec![];
for token_node in only_tokens_iter {
  source.push_str(&token_node.text);
}

assert_eq!(output, "pragma solidity ^0.8.0\n");
```

### Example 2: List the top-level contracts and their names

In addition to the `Iterator` implementation, the `Cursor` type also provides procedural-style functions that allow you to navigate the source parse tree in a step-by-step manner.

Given the following simple file:

```{ .solidity}
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.sol"
```

We will list the top-level contracts and their names. To do that, we need to visit `ContractDefinition` rule nodes and then their `Identifier` children:

```{ .rust }
--8<-- "crates/solidity/outputs/cargo/tests/src/doc_examples/cursor_api.rs:example-list-contract-names"
```
