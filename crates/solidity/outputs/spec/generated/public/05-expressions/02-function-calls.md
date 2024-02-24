<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 5.2. Function Calls

## Syntax

```{ .ebnf #ArgumentsDeclaration }

```

<pre ebnf-snippet="ArgumentsDeclaration" style="display: none;"><a href="#ArgumentsDeclaration"><span class="k">ArgumentsDeclaration</span></a><span class="o"> = </span><a href="#PositionalArgumentsDeclaration"><span class="k">PositionalArgumentsDeclaration</span></a><br /><span class="o">                     | </span><a href="#NamedArgumentsDeclaration"><span class="k">NamedArgumentsDeclaration</span></a><span class="o">;</span></pre>

```{ .ebnf #PositionalArgumentsDeclaration }

```

<pre ebnf-snippet="PositionalArgumentsDeclaration" style="display: none;"><a href="#PositionalArgumentsDeclaration"><span class="k">PositionalArgumentsDeclaration</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                                 </span><a href="#PositionalArguments"><span class="k">PositionalArguments</span></a><br /><span class="o">                                 </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #PositionalArguments }

```

<pre ebnf-snippet="PositionalArguments" style="display: none;"><a href="#PositionalArguments"><span class="k">PositionalArguments</span></a><span class="o"> = </span><span class="o">(</span><a href="../01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o"> </span><span class="o">(</span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="../01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">)</span><span class="o">*</span><span class="o">)</span><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #NamedArgumentsDeclaration }

```

<pre ebnf-snippet="NamedArgumentsDeclaration" style="display: none;"><a href="#NamedArgumentsDeclaration"><span class="k">NamedArgumentsDeclaration</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                            </span><a href="#NamedArgumentGroup"><span class="k">NamedArgumentGroup</span></a><span class="o">?</span><br /><span class="o">                            </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #NamedArgumentGroup }

```

<pre ebnf-snippet="NamedArgumentGroup" style="display: none;"><a href="#NamedArgumentGroup"><span class="k">NamedArgumentGroup</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                     </span><a href="#NamedArguments"><span class="k">NamedArguments</span></a><br /><span class="o">                     </span><a href="../../01-file-structure/09-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #NamedArguments }

```

<pre ebnf-snippet="NamedArguments" style="display: none;"><a href="#NamedArguments"><span class="k">NamedArguments</span></a><span class="o"> = </span><span class="o">(</span><a href="#NamedArgument"><span class="k">NamedArgument</span></a><span class="o"> </span><span class="o">(</span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="#NamedArgument"><span class="k">NamedArgument</span></a><span class="o">)</span><span class="o">*</span><span class="o">)</span><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #NamedArgument }

```

<pre ebnf-snippet="NamedArgument" style="display: none;"><a href="#NamedArgument"><span class="k">NamedArgument</span></a><span class="o"> = </span><a href="../06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                </span><a href="../../01-file-structure/09-punctuation#Colon"><span class="k">COLON</span></a><br /><span class="o">                </span><a href="../01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/05-expressions/02-function-calls.md"
