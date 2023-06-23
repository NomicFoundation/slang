<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #TupleDeconstructionStatement }
TupleDeconstructionStatement = OPEN_PAREN (((TypeName DataLocation? IDENTIFIER) | (DataLocation? IDENTIFIER))? (COMMA ((TypeName DataLocation? IDENTIFIER) | (DataLocation? IDENTIFIER))?)*)? CLOSE_PAREN EQUAL Expression SEMICOLON;
```
