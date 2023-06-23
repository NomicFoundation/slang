<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #VersionPragmaExpression }
«VersionPragmaExpression» = VersionPragmaAlternatives
                          | VersionPragmaRange
                          | VersionPragmaComparator
                          | VersionPragmaSpecifier;
VersionPragmaAlternatives = «VersionPragmaExpression» BAR_BAR «VersionPragmaExpression»;
VersionPragmaRange = «VersionPragmaExpression» MINUS «VersionPragmaExpression»;
VersionPragmaComparator = (CARET | TILDE | EQUAL | LESS_THAN | GREATER_THAN | LESS_THAN_EQUAL | GREATER_THAN_EQUAL) «VersionPragmaExpression»;
```
