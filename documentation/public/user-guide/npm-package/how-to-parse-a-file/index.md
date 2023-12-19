# How to parse a Solidity file

In this guide, we'll walk you through the process of parsing a Solidity file using Slang. See [Installation](../#installation) on how to install Slang.

A file must be parsed according to a specific Solidity [version](../../../solidity-specification/supported-versions/). The version has to be explicitly specified and is not inferred from the source. To selectively parse parts of the source code using different versions, e.g. when the contract across multiple files has been flattened, you need to do that manually.

## Using the NPM package

Start by adding the Slang package as a dependency to your project:

```bash
npm install "@nomicfoundation/slang"
```

Using the API directly provides us with a more fine-grained control over the parsing process; we can parse individual rules like contracts, various definitions or even expressions.

We start by creating a `Language` struct with a given version. This is an entry point for our parser API.

```ts
import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/kinds";
import { Cursor } from "@nomicfoundation/slang/cursor";

const source = "int256 constant z = 1 + 2;";
const language = new Language("0.8.11");

const parseOutput = language.parse(RuleKind.SourceUnit, source);
const cursor: Cursor = parseOutput.createTreeCursor();
```

The resulting `ParseOutput` class exposes these helpful functions:

-   `errors()/isValid()` that return structured parse errors, if any,
-   `tree()` that gives us back a CST (partial if there were parse errors),
-   `fn createTreeCursor()` that creates a `Cursor` type used to conveniently walk the parse tree.

### Example 1: Reconstruct the Solidity file

Let's try the same example, only now using the API directly.

We'll start with this file:

```solidity
// file: file.sol
pragma solidity ^0.8.0;
```

#### Step 1: Parse the Solidity file

Let's naively (ignore the errors) read the file and parse it:

```ts
import { fs } from "node:fs";
const data = fs.readFileSync("file.sol", "utf8");

let parseTree = language.parse(RuleKind.SourceUnit, data);
```

#### Step 2: Reconstruct the source code

The `Cursor` visits the tree nodes in a depth-first search (DFS) fashion. Since our CST is complete (includes trivia such as whitespace), it's enough to visit the `Token` nodes and concatenate their text to reconstruct the original source code.

Let's do that:

```ts
import { TokenNode } from "@nomicfoundation/slang/cst";

let output = "";
while (cursor.goToNext()) {
    let node = cursor.node();
    if (node instanceof TokenNode) {
        output += node.text;
    }
}

// Jest-style assertion for clarity
expect(output).toEqual("pragma solidity ^0.8.0\n");
```

### Example 2: List the top-level contracts and their names

The `Cursor` type provides procedural-style functions that allow you to navigate the source in a step-by-step manner. In addition to `goToNext`, we can go to the parent, first child, next sibling, etc., as well as nodes with a given kind.

To list the top-level contracts and their names, we need to visit the `ContractDefinition` rule nodes and then their `Identifier` children.

Let's do that:

```{ .ts }
--8<-- "crates/solidity/outputs/npm/tests/src/doc-examples/list-contract-names.ts"
```
