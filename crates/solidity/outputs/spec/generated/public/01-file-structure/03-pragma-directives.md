<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 1.3. Pragma Directives

## Syntax

```{ .ebnf #PragmaDirective }

```

<pre ebnf-snippet="PragmaDirective" style="display: none;"><a href="#PragmaDirective"><span class="k">PragmaDirective</span></a><span class="o"> = </span><a href="../08-keywords#PragmaKeyword"><span class="k">PRAGMA_KEYWORD</span></a><br /><span class="o">                  </span><a href="#Pragma"><span class="k">Pragma</span></a><br /><span class="o">                  </span><a href="../09-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #Pragma }

```

<pre ebnf-snippet="Pragma" style="display: none;"><a href="#Pragma"><span class="k">Pragma</span></a><span class="o"> = </span><a href="#ABICoderPragma"><span class="k">ABICoderPragma</span></a><br /><span class="o">       | </span><a href="#ExperimentalPragma"><span class="k">ExperimentalPragma</span></a><br /><span class="o">       | </span><a href="#VersionPragma"><span class="k">VersionPragma</span></a><span class="o">;</span></pre>

```{ .ebnf #ABICoderPragma }

```

<pre ebnf-snippet="ABICoderPragma" style="display: none;"><a href="#ABICoderPragma"><span class="k">ABICoderPragma</span></a><span class="o"> = </span><a href="#AbicoderKeyword"><span class="k">ABICODER_KEYWORD</span></a><br /><span class="o">                 </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">;</span></pre>

```{ .ebnf #ExperimentalPragma }

```

<pre ebnf-snippet="ExperimentalPragma" style="display: none;"><a href="#ExperimentalPragma"><span class="k">ExperimentalPragma</span></a><span class="o"> = </span><a href="#ExperimentalKeyword"><span class="k">EXPERIMENTAL_KEYWORD</span></a><br /><span class="o">                     </span><a href="#ExperimentalFeature"><span class="k">ExperimentalFeature</span></a><span class="o">;</span></pre>

```{ .ebnf #ExperimentalFeature }

```

<pre ebnf-snippet="ExperimentalFeature" style="display: none;"><a href="#ExperimentalFeature"><span class="k">ExperimentalFeature</span></a><span class="o"> = </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                    | </span><a href="../../05-expressions/05-strings#StringLiteral"><span class="k">StringLiteral</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionPragma }

```

<pre ebnf-snippet="VersionPragma" style="display: none;"><a href="#VersionPragma"><span class="k">VersionPragma</span></a><span class="o"> = </span><a href="#SolidityKeyword"><span class="k">SOLIDITY_KEYWORD</span></a><br /><span class="o">                </span><a href="#VersionPragmaExpressions"><span class="k">VersionPragmaExpressions</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionPragmaExpressions }

```

<pre ebnf-snippet="VersionPragmaExpressions" style="display: none;"><a href="#VersionPragmaExpressions"><span class="k">VersionPragmaExpressions</span></a><span class="o"> = </span><a href="#VersionPragmaExpression"><span class="k">VersionPragmaExpression</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #VersionPragmaExpression }

```

<pre ebnf-snippet="VersionPragmaExpression" style="display: none;"><a href="#VersionPragmaExpression"><span class="k">VersionPragmaExpression</span></a><span class="o"> = </span><a href="#VersionPragmaOrExpression"><span class="k">VersionPragmaOrExpression</span></a><br /><span class="o">                        | </span><a href="#VersionPragmaRangeExpression"><span class="k">VersionPragmaRangeExpression</span></a><br /><span class="o">                        | </span><a href="#VersionPragmaPrefixExpression"><span class="k">VersionPragmaPrefixExpression</span></a><br /><span class="o">                        | </span><a href="#VersionPragmaSpecifier"><span class="k">VersionPragmaSpecifier</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionPragmaSpecifier }

```

<pre ebnf-snippet="VersionPragmaSpecifier" style="display: none;"><a href="#VersionPragmaSpecifier"><span class="k">VersionPragmaSpecifier</span></a><span class="o"> = </span><a href="#VersionPragmaValue"><span class="k">VERSION_PRAGMA_VALUE</span></a><span class="o"> </span><span class="o">(</span><a href="../09-punctuation#Period"><span class="k">PERIOD</span></a><span class="o"> </span><a href="#VersionPragmaValue"><span class="k">VERSION_PRAGMA_VALUE</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #VersionPragmaValue }

```

<pre ebnf-snippet="VersionPragmaValue" style="display: none;"><a href="#VersionPragmaValue"><span class="k">VERSION_PRAGMA_VALUE</span></a><span class="o"> = </span><span class="o">(</span><span class="o">(</span><span class="s2">"0"</span><span class="o">…</span><span class="s2">"9"</span><span class="o">)</span><span class="o"> | </span><span class="s2">"x"</span><span class="o"> | </span><span class="s2">"X"</span><span class="o"> | </span><span class="s2">"*"</span><span class="o">)</span><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #AbicoderKeyword }

```

<pre ebnf-snippet="AbicoderKeyword" style="display: none;"><span class="cm">(* Never reserved *)</span><br /><a href="#AbicoderKeyword"><span class="k">ABICODER_KEYWORD</span></a><span class="o"> = </span><span class="s2">"abicoder"</span><span class="o">;</span></pre>

```{ .ebnf #ExperimentalKeyword }

```

<pre ebnf-snippet="ExperimentalKeyword" style="display: none;"><span class="cm">(* Never reserved *)</span><br /><a href="#ExperimentalKeyword"><span class="k">EXPERIMENTAL_KEYWORD</span></a><span class="o"> = </span><span class="s2">"experimental"</span><span class="o">;</span></pre>

```{ .ebnf #SolidityKeyword }

```

<pre ebnf-snippet="SolidityKeyword" style="display: none;"><span class="cm">(* Never reserved *)</span><br /><a href="#SolidityKeyword"><span class="k">SOLIDITY_KEYWORD</span></a><span class="o"> = </span><span class="s2">"solidity"</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/01-file-structure/03-pragma-directives.md"
