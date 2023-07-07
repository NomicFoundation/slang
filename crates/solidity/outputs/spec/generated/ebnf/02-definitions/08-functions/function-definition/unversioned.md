<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #FunctionDefinition }
FunctionDefinition = FUNCTION_KEYWORD (IDENTIFIER | FALLBACK_KEYWORD | RECEIVE_KEYWORD) ParameterList FunctionAttribute* (RETURNS_KEYWORD ParameterList)? (SEMICOLON | Block);
```
