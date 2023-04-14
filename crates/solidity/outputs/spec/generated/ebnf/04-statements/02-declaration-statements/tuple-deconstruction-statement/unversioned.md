<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #TupleDeconstructionStatement }
TupleDeconstructionStatement = «OpenParen» (((TypeName DataLocation? «Identifier») | (DataLocation? «Identifier»))? («Comma» ((TypeName DataLocation? «Identifier») | (DataLocation? «Identifier»))?)*)? «CloseParen» «Equal» Expression «Semicolon»;
```
