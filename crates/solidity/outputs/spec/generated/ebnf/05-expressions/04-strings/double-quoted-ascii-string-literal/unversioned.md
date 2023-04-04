<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

```{ .ebnf .slang-ebnf #DoubleQuotedAsciiStringLiteral }
«DoubleQuotedAsciiStringLiteral» = '"' {«EscapeSequence» | '\u{20}'…'~' - ('"' | '\\')} '"';
```
