<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 5.3. Primary Expressions

## Syntax

```{ .ebnf #TypeExpression }

```

<pre ebnf-snippet="TypeExpression" style="display: none;"><span class="cm">(* Introduced in 0.5.3 *)</span><br /><a href="#TypeExpression"><span class="k">TypeExpression</span></a><span class="o"> = </span><span class="cm">(* type_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/08-keywords#TypeKeyword"><span class="k">TYPE_KEYWORD</span></a><br /><span class="o">                 </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                 </span><span class="cm">(* type_name: *)</span><span class="o"> </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                 </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #NewExpression }

```

<pre ebnf-snippet="NewExpression" style="display: none;"><a href="#NewExpression"><span class="k">NewExpression</span></a><span class="o"> = </span><span class="cm">(* new_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/08-keywords#NewKeyword"><span class="k">NEW_KEYWORD</span></a><br /><span class="o">                </span><span class="cm">(* type_name: *)</span><span class="o"> </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><span class="o">;</span></pre>

```{ .ebnf #TupleExpression }

```

<pre ebnf-snippet="TupleExpression" style="display: none;"><a href="#TupleExpression"><span class="k">TupleExpression</span></a><span class="o"> = </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                  </span><span class="cm">(* items: *)</span><span class="o"> </span><a href="#TupleValues"><span class="k">TupleValues</span></a><br /><span class="o">                  </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #TupleValues }

```

<pre ebnf-snippet="TupleValues" style="display: none;"><a href="#TupleValues"><span class="k">TupleValues</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#TupleValue"><span class="k">TupleValue</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#TupleValue"><span class="k">TupleValue</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #TupleValue }

```

<pre ebnf-snippet="TupleValue" style="display: none;"><a href="#TupleValue"><span class="k">TupleValue</span></a><span class="o"> = </span><span class="cm">(* expression: *)</span><span class="o"> </span><a href="../01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #ArrayExpression }

```

<pre ebnf-snippet="ArrayExpression" style="display: none;"><a href="#ArrayExpression"><span class="k">ArrayExpression</span></a><span class="o"> = </span><span class="cm">(* open_bracket: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#OpenBracket"><span class="k">OPEN_BRACKET</span></a><br /><span class="o">                  </span><span class="cm">(* items: *)</span><span class="o"> </span><a href="#ArrayValues"><span class="k">ArrayValues</span></a><br /><span class="o">                  </span><span class="cm">(* close_bracket: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#CloseBracket"><span class="k">CLOSE_BRACKET</span></a><span class="o">;</span></pre>

```{ .ebnf #ArrayValues }

```

<pre ebnf-snippet="ArrayValues" style="display: none;"><a href="#ArrayValues"><span class="k">ArrayValues</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="../01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="../01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/05-expressions/03-primary-expressions.md"
