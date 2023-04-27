# NPM Package

## Installation

```bash
npm install "@nomicfoundation/slang"
```

## Parser API

```typescript
import { Language, RuleKind, TokenKind, ProductionKind } from "@nomicfoundation/slang";

const language = new Language("0.8.0");
const parseOutput = language.parse(ProductionKind.ContractDefinition, "contract Foo {}");

const parseTree = parseOutput.parseTree();
assert(parseTree.kind == RuleKind.ContractDefinition);

const children = parseTree.children();
assert(children.length == 4);

assert(children[0].kind == TokenKind.ContractKeyword);
assert(children[1].kind == TokenKind.Identifier);
assert(children[2].kind == TokenKind.OpenBrace);
assert(children[3].kind == TokenKind.CloseBrace);
```
