<!-- This file is @generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #YulExpression }
YulExpression = YulFunctionCallExpression (* YulExpression «YulFunctionCallOperator» *) (* Unary Operator, Postfix *)
              | «YulLiteral»
              | YulIdentifierPath;
```

```{ .ebnf .slang-ebnf #YulFunctionCallExpression }
YulFunctionCallExpression = YulExpression «YulFunctionCallOperator» (* Unary Operator, Postfix *);
```
