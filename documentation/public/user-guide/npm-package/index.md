# NPM Package

## Installation

Start by adding the Slang package as a dependency to your project:

```bash
npm install "@nomicfoundation/slang"
```

## Parser API

The API is initialized with a language version to create a `Language` object.
Specifying the correct version is important, as it will affect the grammar used to parse inputs.

You can then use it to parse different inputs belonging to that version.
Each `parse()` operation accepts the input source code, and a `ProductionKind` variant.
This allows callers to parse entire source files (`ProductionKind.SourceUnit`), individual contracts (`ProductionKind.ContractDefinition`),
methods (`ProductionKind.FunctionDefinition`), or any other syntax nodes.

```typescript
import { Language } from "@nomicfoundation/slang/language";
import { RuleKind, TokenKind } from "@nomicfoundation/slang/syntax/nodes";
import { ProductionKind } from "@nomicfoundation/slang/syntax/parser";

const language = new Language("0.8.0");
const parseOutput = language.parse(ProductionKind.ContractDefinition, "contract Foo {}");
```

The resulting `ParseOutput` object will contain syntax errors (if any), and the parse tree corresponding to the input source code.
You can then iterate over the resulting children, and assert that they match the expected syntax nodes:

```typescript
const parseTree = parseOutput.parseTree();
assert(parseTree.kind == RuleKind.ContractDefinition);

const children = parseTree.children();
assert(children.length == 4);

assert(children[0].kind == TokenKind.ContractKeyword);
assert(children[1].kind == TokenKind.Identifier);
assert(children[2].kind == TokenKind.OpenBrace);
assert(children[3].kind == TokenKind.CloseBrace);
```
