<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 5.5. Strings

## Syntax

```{ .ebnf #StringExpression }

```

<pre ebnf-snippet="StringExpression" style="display: none;"><a href="#StringExpression"><span class="k">StringExpression</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#StringLiteral"><span class="k">StringLiteral</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.5.14 *)</span><br /><span class="o">                 | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#StringLiterals"><span class="k">StringLiterals</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.5.14 *)</span><br /><span class="o">                 | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#HexStringLiteral"><span class="k">HexStringLiteral</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.5.14 *)</span><br /><span class="o">                 | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#HexStringLiterals"><span class="k">HexStringLiterals</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.5.14 *)</span><br /><span class="o">                 | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#UnicodeStringLiterals"><span class="k">UnicodeStringLiterals</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.7.0 *)</span></pre>

```{ .ebnf #StringLiterals }

```

<pre ebnf-snippet="StringLiterals" style="display: none;"><span class="cm">(* Introduced in 0.5.14 *)</span><br /><a href="#StringLiterals"><span class="k">StringLiterals</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#StringLiteral"><span class="k">StringLiteral</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #StringLiteral }

```

<pre ebnf-snippet="StringLiteral" style="display: none;"><a href="#StringLiteral"><span class="k">StringLiteral</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#SingleQuotedStringLiteral"><span class="k">SINGLE_QUOTED_STRING_LITERAL</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#DoubleQuotedStringLiteral"><span class="k">DOUBLE_QUOTED_STRING_LITERAL</span></a><span class="o">;</span></pre>

```{ .ebnf #SingleQuotedStringLiteral }

```

<pre ebnf-snippet="SingleQuotedStringLiteral" style="display: none;"><span class="cm">(* Deprecated in 0.4.25 *)</span><br /><a href="#SingleQuotedStringLiteral"><span class="k">SINGLE_QUOTED_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">"'"</span><span class="o"> </span><span class="o">(</span><a href="#EscapeSequenceArbitrary"><span class="k">«ESCAPE_SEQUENCE_ARBITRARY»</span></a><span class="o"> | </span><span class="o">!</span><span class="o">(</span><span class="s2">"'"</span><span class="o"> | </span><span class="s2">"\\"</span><span class="o"> | </span><span class="s2">"\r"</span><span class="o"> | </span><span class="s2">"\n"</span><span class="o">)</span><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">"'"</span><span class="o">;</span><br /><br /><span class="cm">(* Introduced in 0.4.25 and deprecated in 0.7.0. *)</span><br /><a href="#SingleQuotedStringLiteral"><span class="k">SINGLE_QUOTED_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">"'"</span><span class="o"> </span><span class="o">(</span><a href="#EscapeSequence"><span class="k">«ESCAPE_SEQUENCE»</span></a><span class="o"> | </span><span class="o">!</span><span class="o">(</span><span class="s2">"'"</span><span class="o"> | </span><span class="s2">"\\"</span><span class="o"> | </span><span class="s2">"\r"</span><span class="o"> | </span><span class="s2">"\n"</span><span class="o">)</span><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">"'"</span><span class="o">;</span><br /><br /><a href="#SingleQuotedStringLiteral"><span class="k">SINGLE_QUOTED_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">"'"</span><span class="o"> </span><span class="o">(</span><a href="#EscapeSequence"><span class="k">«ESCAPE_SEQUENCE»</span></a><span class="o"> | </span><span class="s2">" "</span><span class="o">…</span><span class="s2">"&"</span><span class="o"> | </span><span class="s2">"("</span><span class="o">…</span><span class="s2">"["</span><span class="o"> | </span><span class="s2">"]"</span><span class="o">…</span><span class="s2">"~"</span><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">"'"</span><span class="o">;</span></pre>

```{ .ebnf #DoubleQuotedStringLiteral }

```

<pre ebnf-snippet="DoubleQuotedStringLiteral" style="display: none;"><span class="cm">(* Deprecated in 0.4.25 *)</span><br /><a href="#DoubleQuotedStringLiteral"><span class="k">DOUBLE_QUOTED_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">'"'</span><span class="o"> </span><span class="o">(</span><a href="#EscapeSequenceArbitrary"><span class="k">«ESCAPE_SEQUENCE_ARBITRARY»</span></a><span class="o"> | </span><span class="o">!</span><span class="o">(</span><span class="s2">'"'</span><span class="o"> | </span><span class="s2">"\\"</span><span class="o"> | </span><span class="s2">"\r"</span><span class="o"> | </span><span class="s2">"\n"</span><span class="o">)</span><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">'"'</span><span class="o">;</span><br /><br /><span class="cm">(* Introduced in 0.4.25 and deprecated in 0.7.0. *)</span><br /><a href="#DoubleQuotedStringLiteral"><span class="k">DOUBLE_QUOTED_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">'"'</span><span class="o"> </span><span class="o">(</span><a href="#EscapeSequence"><span class="k">«ESCAPE_SEQUENCE»</span></a><span class="o"> | </span><span class="o">!</span><span class="o">(</span><span class="s2">'"'</span><span class="o"> | </span><span class="s2">"\\"</span><span class="o"> | </span><span class="s2">"\r"</span><span class="o"> | </span><span class="s2">"\n"</span><span class="o">)</span><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">'"'</span><span class="o">;</span><br /><br /><a href="#DoubleQuotedStringLiteral"><span class="k">DOUBLE_QUOTED_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">'"'</span><span class="o"> </span><span class="o">(</span><a href="#EscapeSequence"><span class="k">«ESCAPE_SEQUENCE»</span></a><span class="o"> | </span><span class="s2">" "</span><span class="o">…</span><span class="s2">"!"</span><span class="o"> | </span><span class="s2">"#"</span><span class="o">…</span><span class="s2">"["</span><span class="o"> | </span><span class="s2">"]"</span><span class="o">…</span><span class="s2">"~"</span><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">'"'</span><span class="o">;</span></pre>

```{ .ebnf #HexStringLiterals }

```

<pre ebnf-snippet="HexStringLiterals" style="display: none;"><span class="cm">(* Introduced in 0.5.14 *)</span><br /><a href="#HexStringLiterals"><span class="k">HexStringLiterals</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#HexStringLiteral"><span class="k">HexStringLiteral</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #HexStringLiteral }

```

<pre ebnf-snippet="HexStringLiteral" style="display: none;"><a href="#HexStringLiteral"><span class="k">HexStringLiteral</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#SingleQuotedHexStringLiteral"><span class="k">SINGLE_QUOTED_HEX_STRING_LITERAL</span></a><br /><span class="o">                 | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#DoubleQuotedHexStringLiteral"><span class="k">DOUBLE_QUOTED_HEX_STRING_LITERAL</span></a><span class="o">;</span></pre>

```{ .ebnf #SingleQuotedHexStringLiteral }

```

<pre ebnf-snippet="SingleQuotedHexStringLiteral" style="display: none;"><a href="#SingleQuotedHexStringLiteral"><span class="k">SINGLE_QUOTED_HEX_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">"hex'"</span><span class="o"> </span><a href="#HexStringContents"><span class="k">«HEX_STRING_CONTENTS»</span></a><span class="o">?</span><span class="o"> </span><span class="s2">"'"</span><span class="o">;</span></pre>

```{ .ebnf #DoubleQuotedHexStringLiteral }

```

<pre ebnf-snippet="DoubleQuotedHexStringLiteral" style="display: none;"><a href="#DoubleQuotedHexStringLiteral"><span class="k">DOUBLE_QUOTED_HEX_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">'hex"'</span><span class="o"> </span><a href="#HexStringContents"><span class="k">«HEX_STRING_CONTENTS»</span></a><span class="o">?</span><span class="o"> </span><span class="s2">'"'</span><span class="o">;</span></pre>

```{ .ebnf #HexStringContents }

```

<pre ebnf-snippet="HexStringContents" style="display: none;"><a href="#HexStringContents"><span class="k">«HEX_STRING_CONTENTS»</span></a><span class="o"> = </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o"> </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o"> </span><span class="o">(</span><span class="s2">"_"</span><span class="o">?</span><span class="o"> </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o"> </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #HexCharacter }

```

<pre ebnf-snippet="HexCharacter" style="display: none;"><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o"> = </span><span class="s2">"0"</span><span class="o">…</span><span class="s2">"9"</span><span class="o"> | </span><span class="s2">"a"</span><span class="o">…</span><span class="s2">"f"</span><span class="o"> | </span><span class="s2">"A"</span><span class="o">…</span><span class="s2">"F"</span><span class="o">;</span></pre>

```{ .ebnf #UnicodeStringLiterals }

```

<pre ebnf-snippet="UnicodeStringLiterals" style="display: none;"><span class="cm">(* Introduced in 0.7.0 *)</span><br /><a href="#UnicodeStringLiterals"><span class="k">UnicodeStringLiterals</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#UnicodeStringLiteral"><span class="k">UnicodeStringLiteral</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #UnicodeStringLiteral }

```

<pre ebnf-snippet="UnicodeStringLiteral" style="display: none;"><span class="cm">(* Introduced in 0.7.0 *)</span><br /><a href="#UnicodeStringLiteral"><span class="k">UnicodeStringLiteral</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#SingleQuotedUnicodeStringLiteral"><span class="k">SINGLE_QUOTED_UNICODE_STRING_LITERAL</span></a><br /><span class="o">                     | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#DoubleQuotedUnicodeStringLiteral"><span class="k">DOUBLE_QUOTED_UNICODE_STRING_LITERAL</span></a><span class="o">;</span></pre>

```{ .ebnf #SingleQuotedUnicodeStringLiteral }

```

<pre ebnf-snippet="SingleQuotedUnicodeStringLiteral" style="display: none;"><span class="cm">(* Introduced in 0.7.0 *)</span><br /><a href="#SingleQuotedUnicodeStringLiteral"><span class="k">SINGLE_QUOTED_UNICODE_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">"unicode'"</span><span class="o"> </span><span class="o">(</span><a href="#EscapeSequence"><span class="k">«ESCAPE_SEQUENCE»</span></a><span class="o"> | </span><span class="o">!</span><span class="o">(</span><span class="s2">"'"</span><span class="o"> | </span><span class="s2">"\\"</span><span class="o"> | </span><span class="s2">"\r"</span><span class="o"> | </span><span class="s2">"\n"</span><span class="o">)</span><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">"'"</span><span class="o">;</span></pre>

```{ .ebnf #DoubleQuotedUnicodeStringLiteral }

```

<pre ebnf-snippet="DoubleQuotedUnicodeStringLiteral" style="display: none;"><span class="cm">(* Introduced in 0.7.0 *)</span><br /><a href="#DoubleQuotedUnicodeStringLiteral"><span class="k">DOUBLE_QUOTED_UNICODE_STRING_LITERAL</span></a><span class="o"> = </span><span class="s2">'unicode"'</span><span class="o"> </span><span class="o">(</span><a href="#EscapeSequence"><span class="k">«ESCAPE_SEQUENCE»</span></a><span class="o"> | </span><span class="o">!</span><span class="o">(</span><span class="s2">'"'</span><span class="o"> | </span><span class="s2">"\\"</span><span class="o"> | </span><span class="s2">"\r"</span><span class="o"> | </span><span class="s2">"\n"</span><span class="o">)</span><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">'"'</span><span class="o">;</span></pre>

```{ .ebnf #EscapeSequence }

```

<pre ebnf-snippet="EscapeSequence" style="display: none;"><a href="#EscapeSequence"><span class="k">«ESCAPE_SEQUENCE»</span></a><span class="o"> = </span><span class="s2">"\\"</span><span class="o"> </span><span class="o">(</span><a href="#AsciiEscape"><span class="k">«ASCII_ESCAPE»</span></a><span class="o"> | </span><a href="#HexByteEscape"><span class="k">«HEX_BYTE_ESCAPE»</span></a><span class="o"> | </span><a href="#UnicodeEscape"><span class="k">«UNICODE_ESCAPE»</span></a><span class="o">)</span><span class="o">;</span></pre>

```{ .ebnf #EscapeSequenceArbitrary }

```

<pre ebnf-snippet="EscapeSequenceArbitrary" style="display: none;"><span class="cm">(* Deprecated in 0.4.25 *)</span><br /><a href="#EscapeSequenceArbitrary"><span class="k">«ESCAPE_SEQUENCE_ARBITRARY»</span></a><span class="o"> = </span><span class="s2">"\\"</span><span class="o"> </span><span class="o">(</span><span class="o">!</span><span class="o">(</span><span class="s2">"x"</span><span class="o"> | </span><span class="s2">"u"</span><span class="o">)</span><span class="o"> | </span><a href="#HexByteEscape"><span class="k">«HEX_BYTE_ESCAPE»</span></a><span class="o"> | </span><a href="#UnicodeEscape"><span class="k">«UNICODE_ESCAPE»</span></a><span class="o">)</span><span class="o">;</span></pre>

```{ .ebnf #AsciiEscape }

```

<pre ebnf-snippet="AsciiEscape" style="display: none;"><a href="#AsciiEscape"><span class="k">«ASCII_ESCAPE»</span></a><span class="o"> = </span><span class="s2">"n"</span><span class="o"> | </span><span class="s2">"r"</span><span class="o"> | </span><span class="s2">"t"</span><span class="o"> | </span><span class="s2">"'"</span><span class="o"> | </span><span class="s2">'"'</span><span class="o"> | </span><span class="s2">"\\"</span><span class="o"> | </span><span class="s2">"\r\n"</span><span class="o"> | </span><span class="s2">"\r"</span><span class="o"> | </span><span class="s2">"\n"</span><span class="o">;</span></pre>

```{ .ebnf #HexByteEscape }

```

<pre ebnf-snippet="HexByteEscape" style="display: none;"><a href="#HexByteEscape"><span class="k">«HEX_BYTE_ESCAPE»</span></a><span class="o"> = </span><span class="s2">"x"</span><span class="o"> </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o"> </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o">;</span></pre>

```{ .ebnf #UnicodeEscape }

```

<pre ebnf-snippet="UnicodeEscape" style="display: none;"><a href="#UnicodeEscape"><span class="k">«UNICODE_ESCAPE»</span></a><span class="o"> = </span><span class="s2">"u"</span><span class="o"> </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o"> </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o"> </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o"> </span><a href="#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/05-expressions/05-strings.md"
