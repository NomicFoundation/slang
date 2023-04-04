<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #SingleQuotedAsciiStringLiteral }
«SingleQuotedAsciiStringLiteral» = "\'" {«EscapeSequence» | '\u{20}'…'~' - ("\'" | '\\')} "\'";
```
