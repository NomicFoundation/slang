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

<pre ebnf-snippet="VersionPragma" style="display: none;"><a href="#VersionPragma"><span class="k">VersionPragma</span></a><span class="o"> = </span><a href="#SolidityKeyword"><span class="k">SOLIDITY_KEYWORD</span></a><br /><span class="o">                </span><a href="#VersionExpressionSets"><span class="k">VersionExpressionSets</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionExpressionSets }

```

<pre ebnf-snippet="VersionExpressionSets" style="display: none;"><a href="#VersionExpressionSets"><span class="k">VersionExpressionSets</span></a><span class="o"> = </span><a href="#VersionExpressionSet"><span class="k">VersionExpressionSet</span></a><span class="o"> </span><span class="o">(</span><a href="../09-punctuation#BarBar"><span class="k">BAR_BAR</span></a><span class="o"> </span><a href="#VersionExpressionSet"><span class="k">VersionExpressionSet</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #VersionExpressionSet }

```

<pre ebnf-snippet="VersionExpressionSet" style="display: none;"><a href="#VersionExpressionSet"><span class="k">VersionExpressionSet</span></a><span class="o"> = </span><a href="#VersionExpression"><span class="k">VersionExpression</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #VersionExpression }

```

<pre ebnf-snippet="VersionExpression" style="display: none;"><a href="#VersionExpression"><span class="k">VersionExpression</span></a><span class="o"> = </span><a href="#VersionRange"><span class="k">VersionRange</span></a><br /><span class="o">                  | </span><a href="#VersionComparator"><span class="k">VersionComparator</span></a><br /><span class="o">                  | </span><a href="#VersionSpecifiers"><span class="k">VersionSpecifiers</span></a><br /><span class="o">                  | </span><a href="#SingleQuotedVersionLiteral"><span class="k">SINGLE_QUOTED_VERSION_LITERAL</span></a><br /><span class="o">                  | </span><a href="#DoubleQuotedVersionLiteral"><span class="k">DOUBLE_QUOTED_VERSION_LITERAL</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionSpecifiers }

```

<pre ebnf-snippet="VersionSpecifiers" style="display: none;"><a href="#VersionSpecifiers"><span class="k">VersionSpecifiers</span></a><span class="o"> = </span><a href="#VersionSpecifier"><span class="k">VERSION_SPECIFIER</span></a><span class="o"> </span><span class="o">(</span><a href="../09-punctuation#Period"><span class="k">PERIOD</span></a><span class="o"> </span><a href="#VersionSpecifier"><span class="k">VERSION_SPECIFIER</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #VersionSpecifier }

```

<pre ebnf-snippet="VersionSpecifier" style="display: none;"><a href="#VersionSpecifier"><span class="k">VERSION_SPECIFIER</span></a><span class="o"> = </span><a href="#VersionSpecifierFragment"><span class="k">«VERSION_SPECIFIER_FRAGMENT»</span></a><span class="o">;</span></pre>

```{ .ebnf #SingleQuotedVersionLiteral }

```

<pre ebnf-snippet="SingleQuotedVersionLiteral" style="display: none;"><a href="#SingleQuotedVersionLiteral"><span class="k">SINGLE_QUOTED_VERSION_LITERAL</span></a><span class="o"> = </span><span class="s2">"'"</span><span class="o"> </span><a href="#VersionSpecifierFragment"><span class="k">«VERSION_SPECIFIER_FRAGMENT»</span></a><span class="o"> </span><span class="o">(</span><span class="s2">"."</span><span class="o"> </span><a href="#VersionSpecifierFragment"><span class="k">«VERSION_SPECIFIER_FRAGMENT»</span></a><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">"'"</span><span class="o">;</span></pre>

```{ .ebnf #DoubleQuotedVersionLiteral }

```

<pre ebnf-snippet="DoubleQuotedVersionLiteral" style="display: none;"><a href="#DoubleQuotedVersionLiteral"><span class="k">DOUBLE_QUOTED_VERSION_LITERAL</span></a><span class="o"> = </span><span class="s2">'"'</span><span class="o"> </span><a href="#VersionSpecifierFragment"><span class="k">«VERSION_SPECIFIER_FRAGMENT»</span></a><span class="o"> </span><span class="o">(</span><span class="s2">"."</span><span class="o"> </span><a href="#VersionSpecifierFragment"><span class="k">«VERSION_SPECIFIER_FRAGMENT»</span></a><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">'"'</span><span class="o">;</span></pre>

```{ .ebnf #VersionSpecifierFragment }

```

<pre ebnf-snippet="VersionSpecifierFragment" style="display: none;"><a href="#VersionSpecifierFragment"><span class="k">«VERSION_SPECIFIER_FRAGMENT»</span></a><span class="o"> = </span><span class="o">(</span><span class="o">(</span><span class="s2">"0"</span><span class="o">…</span><span class="s2">"9"</span><span class="o">)</span><span class="o"> | </span><span class="s2">"x"</span><span class="o"> | </span><span class="s2">"X"</span><span class="o"> | </span><span class="s2">"*"</span><span class="o">)</span><span class="o">+</span><span class="o">;</span></pre>

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
