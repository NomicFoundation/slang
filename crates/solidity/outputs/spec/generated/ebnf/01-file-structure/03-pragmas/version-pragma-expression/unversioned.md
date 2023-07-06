<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #VersionPragmaExpression }
VersionPragmaExpression = VersionPragmaBinaryExpression
                        | VersionPragmaUnaryExpression
                        | VersionPragmaSpecifier;
VersionPragmaBinaryExpression = VersionPragmaExpression «VersionPragmaOrOperator» VersionPragmaExpression;
VersionPragmaBinaryExpression = VersionPragmaExpression «VersionPragmaRangeOperator» VersionPragmaExpression;
VersionPragmaUnaryExpression = «VersionPragmaUnaryOperator» VersionPragmaExpression;
```
