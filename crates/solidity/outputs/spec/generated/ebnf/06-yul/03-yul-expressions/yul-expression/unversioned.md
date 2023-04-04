<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #YulExpression }
YulExpression = YulFunctionCallExpression | YulLiteral | YulIdentifierPath;
YulFunctionCallExpression = YulExpression («OpenParen» [YulExpression {«Comma» YulExpression}] «CloseParen»);
```
