<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #VersionPragmaExpression }
VersionPragmaExpression = VersionPragmaAlternatives | VersionPragmaRange | VersionPragmaComparator | VersionPragmaSpecifier;
VersionPragmaAlternatives = VersionPragmaExpression «BarBar» VersionPragmaExpression;
VersionPragmaRange = VersionPragmaExpression «Minus» VersionPragmaExpression;
VersionPragmaComparator = («Caret» | «Tilde» | «Equal» | «LessThan» | «GreaterThan» | «LessThanEqual» | «GreaterThanEqual») VersionPragmaExpression;
```
