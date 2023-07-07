<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #ErrorDefinition }
ErrorDefinition = ERROR_KEYWORD IDENTIFIER OPEN_PAREN (ErrorParameter (COMMA ErrorParameter)*)? CLOSE_PAREN SEMICOLON;
```
