<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #VersionPragmaExpression }
VersionPragmaExpression = VersionPragmaBinaryExpression (* VersionPragmaExpression «VersionPragmaOrOperator» VersionPragmaExpression *) (* Binary Operator, Left Associative *)
                        | VersionPragmaBinaryExpression (* VersionPragmaExpression «VersionPragmaRangeOperator» VersionPragmaExpression *) (* Binary Operator, Left Associative *)
                        | VersionPragmaUnaryExpression (* «VersionPragmaUnaryOperator» VersionPragmaExpression *) (* Unary Operator, Prefix *)
                        | VersionPragmaSpecifier;
```

```{ .ebnf .slang-ebnf #VersionPragmaBinaryExpression }
VersionPragmaBinaryExpression = VersionPragmaExpression «VersionPragmaOrOperator» VersionPragmaExpression (* Binary Operator, Left Associative *);
VersionPragmaBinaryExpression = VersionPragmaExpression «VersionPragmaRangeOperator» VersionPragmaExpression (* Binary Operator, Left Associative *);
```

```{ .ebnf .slang-ebnf #VersionPragmaUnaryExpression }
VersionPragmaUnaryExpression = «VersionPragmaUnaryOperator» VersionPragmaExpression (* Unary Operator, Prefix *);
```
