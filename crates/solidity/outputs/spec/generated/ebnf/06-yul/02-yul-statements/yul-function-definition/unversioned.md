<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #YulFunctionDefinition }
YulFunctionDefinition = FUNCTION_KEYWORD YUL_IDENTIFIER OPEN_PAREN Arguments? CLOSE_PAREN (MINUS_GREATER_THAN Results)? YulBlock;
Arguments = YUL_IDENTIFIER (COMMA YUL_IDENTIFIER)*;
Results = YUL_IDENTIFIER (COMMA YUL_IDENTIFIER)*;
```
