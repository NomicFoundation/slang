<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 6.2. Yul Expressions

## Syntax

```{ .ebnf #YulExpression }

```

<pre ebnf-snippet="YulExpression" style="display: none;"><a href="#YulExpression"><span class="k">YulExpression</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#YulFunctionCallExpression"><span class="k">YulFunctionCallExpression</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#YulLiteral"><span class="k">YulLiteral</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#YulBuiltInFunction"><span class="k">YulBuiltInFunction</span></a><br /><span class="o">              | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#YulPath"><span class="k">YulPath</span></a><span class="o">;</span></pre>

```{ .ebnf #YulFunctionCallExpression }

```

<pre ebnf-snippet="YulFunctionCallExpression" style="display: none;"><span class="cm">(* Postfix unary operator *)</span><br /><a href="#YulFunctionCallExpression"><span class="k">YulFunctionCallExpression</span></a><span class="o"> = </span><span class="cm">(* operand: *)</span><span class="o"> </span><a href="#YulExpression"><span class="k">YulExpression</span></a><br /><span class="o">                            </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                            </span><span class="cm">(* arguments: *)</span><span class="o"> </span><a href="#YulArguments"><span class="k">YulArguments</span></a><br /><span class="o">                            </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #YulArguments }

```

<pre ebnf-snippet="YulArguments" style="display: none;"><a href="#YulArguments"><span class="k">YulArguments</span></a><span class="o"> = </span><span class="o">(</span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#YulExpression"><span class="k">YulExpression</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#YulExpression"><span class="k">YulExpression</span></a><span class="o">)</span><span class="o">*</span><span class="o">)</span><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #YulPaths }

```

<pre ebnf-snippet="YulPaths" style="display: none;"><a href="#YulPaths"><span class="k">YulPaths</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#YulPath"><span class="k">YulPath</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#YulPath"><span class="k">YulPath</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #YulPath }

```

<pre ebnf-snippet="YulPath" style="display: none;"><a href="#YulPath"><span class="k">YulPath</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#YulIdentifier"><span class="k">YUL_IDENTIFIER</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#Period"><span class="k">PERIOD</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#YulIdentifier"><span class="k">YUL_IDENTIFIER</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #YulIdentifier }

```

<pre ebnf-snippet="YulIdentifier" style="display: none;"><span class="cm">(* Introduced in 0.5.8 and deprecated in 0.7.0. *)</span><br /><a href="#YulIdentifier"><span class="k">YUL_IDENTIFIER</span></a><span class="o"> = </span><a href="../../05-expressions/06-identifiers#IdentifierStart"><span class="k">«IDENTIFIER_START»</span></a><span class="o"> </span><span class="o">(</span><a href="../../05-expressions/06-identifiers#IdentifierPart"><span class="k">«IDENTIFIER_PART»</span></a><span class="o"> | </span><span class="s2">"."</span><span class="o">)</span><span class="o">*</span><span class="o">;</span><br /><br /><a href="#YulIdentifier"><span class="k">YUL_IDENTIFIER</span></a><span class="o"> = </span><a href="../../05-expressions/06-identifiers#IdentifierStart"><span class="k">«IDENTIFIER_START»</span></a><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPart"><span class="k">«IDENTIFIER_PART»</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #YulBuiltInFunction }

```

<pre ebnf-snippet="YulBuiltInFunction" style="display: none;"><a href="#YulBuiltInFunction"><span class="k">YulBuiltInFunction</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../03-yul-keywords#YulByteKeyword"><span class="k">YUL_BYTE_KEYWORD</span></a><br /><span class="o">                   | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../03-yul-keywords#YulJumpKeyword"><span class="k">YUL_JUMP_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><span class="o">                   | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../03-yul-keywords#YulJumpiKeyword"><span class="k">YUL_JUMPI_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span></pre>

```{ .ebnf #YulLiteral }

```

<pre ebnf-snippet="YulLiteral" style="display: none;"><a href="#YulLiteral"><span class="k">YulLiteral</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../03-yul-keywords#YulTrueKeyword"><span class="k">YUL_TRUE_KEYWORD</span></a><br /><span class="o">           | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../03-yul-keywords#YulFalseKeyword"><span class="k">YUL_FALSE_KEYWORD</span></a><br /><span class="o">           | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#YulDecimalLiteral"><span class="k">YUL_DECIMAL_LITERAL</span></a><br /><span class="o">           | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#YulHexLiteral"><span class="k">YUL_HEX_LITERAL</span></a><br /><span class="o">           | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../05-expressions/05-strings#HexStringLiteral"><span class="k">HexStringLiteral</span></a><br /><span class="o">           | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../05-expressions/05-strings#StringLiteral"><span class="k">StringLiteral</span></a><span class="o">;</span></pre>

```{ .ebnf #YulDecimalLiteral }

```

<pre ebnf-snippet="YulDecimalLiteral" style="display: none;"><a href="#YulDecimalLiteral"><span class="k">YUL_DECIMAL_LITERAL</span></a><span class="o"> = </span><span class="o">(</span><span class="s2">"0"</span><span class="o"> | </span><span class="o">(</span><span class="s2">"1"</span><span class="o">…</span><span class="s2">"9"</span><span class="o"> </span><span class="s2">"0"</span><span class="o">…</span><span class="s2">"9"</span><span class="o">*</span><span class="o">)</span><span class="o">)</span><span class="o"> </span><span class="o">(?!</span><a href="../../05-expressions/06-identifiers#IdentifierStart"><span class="k">«IDENTIFIER_START»</span></a><span class="o">)</span><span class="o">;</span></pre>

```{ .ebnf #YulHexLiteral }

```

<pre ebnf-snippet="YulHexLiteral" style="display: none;"><a href="#YulHexLiteral"><span class="k">YUL_HEX_LITERAL</span></a><span class="o"> = </span><span class="s2">"0x"</span><span class="o"> </span><a href="../../05-expressions/05-strings#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o">+</span><span class="o"> </span><span class="o">(?!</span><a href="../../05-expressions/06-identifiers#IdentifierStart"><span class="k">«IDENTIFIER_START»</span></a><span class="o">)</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/06-yul/02-yul-expressions.md"
