<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #YulExpression }
YulExpression = YulFunctionCallExpression | YulLiteral | YulIdentifierPath;
YulFunctionCallExpression = YulExpression OPEN_PAREN (YulExpression (COMMA YulExpression)*)? CLOSE_PAREN;
```
