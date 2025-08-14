<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 1.2. Pragma Directives

```{ .ebnf #PragmaDirective }

```

<pre ebnf-snippet="PragmaDirective" style="display: none;"><a href="#PragmaDirective"><span class="k">PragmaDirective</span></a><span class="o"> = </span><span class="cm">(* pragma_keyword: *)</span><span class="o"> </span><a href="../06-keywords#PragmaKeyword"><span class="k">PRAGMA_KEYWORD</span></a><br /><span class="o">                  </span><span class="cm">(* pragma: *)</span><span class="o"> </span><a href="#Pragma"><span class="k">Pragma</span></a><br /><span class="o">                  </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #Pragma }

```

<pre ebnf-snippet="Pragma" style="display: none;"><a href="#Pragma"><span class="k">Pragma</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#VersionPragma"><span class="k">VersionPragma</span></a><br /><span class="o">       | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#AbicoderPragma"><span class="k">AbicoderPragma</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.7.5 *)</span><br /><span class="o">       | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#ExperimentalPragma"><span class="k">ExperimentalPragma</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.4.16 *)</span></pre>

```{ .ebnf #AbicoderPragma }

```

<pre ebnf-snippet="AbicoderPragma" style="display: none;"><span class="cm">(* Introduced in 0.7.5 *)</span><br /><a href="#AbicoderPragma"><span class="k">AbicoderPragma</span></a><span class="o"> = </span><span class="cm">(* abicoder_keyword: *)</span><span class="o"> </span><a href="../06-keywords#AbicoderKeyword"><span class="k">ABICODER_KEYWORD</span></a><br /><span class="o">                 </span><span class="cm">(* version: *)</span><span class="o"> </span><a href="#AbicoderVersion"><span class="k">AbicoderVersion</span></a><span class="o">;</span></pre>

```{ .ebnf #ExperimentalPragma }

```

<pre ebnf-snippet="ExperimentalPragma" style="display: none;"><span class="cm">(* Introduced in 0.4.16 *)</span><br /><a href="#ExperimentalPragma"><span class="k">ExperimentalPragma</span></a><span class="o"> = </span><span class="cm">(* experimental_keyword: *)</span><span class="o"> </span><a href="../06-keywords#ExperimentalKeyword"><span class="k">EXPERIMENTAL_KEYWORD</span></a><br /><span class="o">                     </span><span class="cm">(* feature: *)</span><span class="o"> </span><a href="#ExperimentalFeature"><span class="k">ExperimentalFeature</span></a><span class="o">;</span></pre>

```{ .ebnf #AbicoderVersion }

```

<pre ebnf-snippet="AbicoderVersion" style="display: none;"><span class="cm">(* Introduced in 0.7.5 *)</span><br /><a href="#AbicoderVersion"><span class="k">AbicoderVersion</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../06-keywords#AbicoderV1Keyword"><span class="k">ABICODER_V1_KEYWORD</span></a><br /><span class="o">                | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../06-keywords#AbicoderV2Keyword"><span class="k">ABICODER_V2_KEYWORD</span></a><span class="o">;</span></pre>

```{ .ebnf #ExperimentalFeature }

```

<pre ebnf-snippet="ExperimentalFeature" style="display: none;"><span class="cm">(* Introduced in 0.4.16 *)</span><br /><a href="#ExperimentalFeature"><span class="k">ExperimentalFeature</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../06-keywords#ABIEncoderV2Keyword"><span class="k">ABI_ENCODER_V2_KEYWORD</span></a><br /><span class="o">                    | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../06-keywords#SMTCheckerKeyword"><span class="k">SMT_CHECKER_KEYWORD</span></a><br /><span class="o">                    | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../05-expressions/05-strings#StringLiteral"><span class="k">StringLiteral</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionPragma }

```

<pre ebnf-snippet="VersionPragma" style="display: none;"><a href="#VersionPragma"><span class="k">VersionPragma</span></a><span class="o"> = </span><span class="cm">(* solidity_keyword: *)</span><span class="o"> </span><a href="../06-keywords#SolidityKeyword"><span class="k">SOLIDITY_KEYWORD</span></a><br /><span class="o">                </span><span class="cm">(* sets: *)</span><span class="o"> </span><a href="#VersionExpressionSets"><span class="k">VersionExpressionSets</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionExpressionSets }

```

<pre ebnf-snippet="VersionExpressionSets" style="display: none;"><a href="#VersionExpressionSets"><span class="k">VersionExpressionSets</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#VersionExpressionSet"><span class="k">VersionExpressionSet</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../07-punctuation#BarBar"><span class="k">BAR_BAR</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#VersionExpressionSet"><span class="k">VersionExpressionSet</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #VersionExpressionSet }

```

<pre ebnf-snippet="VersionExpressionSet" style="display: none;"><a href="#VersionExpressionSet"><span class="k">VersionExpressionSet</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#VersionExpression"><span class="k">VersionExpression</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #VersionExpression }

```

<pre ebnf-snippet="VersionExpression" style="display: none;"><a href="#VersionExpression"><span class="k">VersionExpression</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#VersionRange"><span class="k">VersionRange</span></a><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#VersionTerm"><span class="k">VersionTerm</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionRange }

```

<pre ebnf-snippet="VersionRange" style="display: none;"><a href="#VersionRange"><span class="k">VersionRange</span></a><span class="o"> = </span><span class="cm">(* start: *)</span><span class="o"> </span><a href="#VersionLiteral"><span class="k">VersionLiteral</span></a><br /><span class="o">               </span><span class="cm">(* minus: *)</span><span class="o"> </span><a href="../07-punctuation#Minus"><span class="k">MINUS</span></a><br /><span class="o">               </span><span class="cm">(* end: *)</span><span class="o"> </span><a href="#VersionLiteral"><span class="k">VersionLiteral</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionTerm }

```

<pre ebnf-snippet="VersionTerm" style="display: none;"><a href="#VersionTerm"><span class="k">VersionTerm</span></a><span class="o"> = </span><span class="cm">(* operator: *)</span><span class="o"> </span><a href="#VersionOperator"><span class="k">VersionOperator</span></a><span class="o">?</span><br /><span class="o">              </span><span class="cm">(* literal: *)</span><span class="o"> </span><a href="#VersionLiteral"><span class="k">VersionLiteral</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionOperator }

```

<pre ebnf-snippet="VersionOperator" style="display: none;"><a href="#VersionOperator"><span class="k">VersionOperator</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Caret"><span class="k">CARET</span></a><br /><span class="o">                | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Tilde"><span class="k">TILDE</span></a><br /><span class="o">                | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#Equal"><span class="k">EQUAL</span></a><br /><span class="o">                | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#LessThan"><span class="k">LESS_THAN</span></a><br /><span class="o">                | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#GreaterThan"><span class="k">GREATER_THAN</span></a><br /><span class="o">                | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#LessThanEqual"><span class="k">LESS_THAN_EQUAL</span></a><br /><span class="o">                | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-punctuation#GreaterThanEqual"><span class="k">GREATER_THAN_EQUAL</span></a><span class="o">;</span></pre>

```{ .ebnf #VersionLiteral }

```

<pre ebnf-snippet="VersionLiteral" style="display: none;"><a href="#VersionLiteral"><span class="k">VersionLiteral</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#SimpleVersionLiteral"><span class="k">SimpleVersionLiteral</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#SingleQuotedVersionLiteral"><span class="k">SINGLE_QUOTED_VERSION_LITERAL</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#DoubleQuotedVersionLiteral"><span class="k">DOUBLE_QUOTED_VERSION_LITERAL</span></a><span class="o">;</span></pre>

```{ .ebnf #SimpleVersionLiteral }

```

<pre ebnf-snippet="SimpleVersionLiteral" style="display: none;"><a href="#SimpleVersionLiteral"><span class="k">SimpleVersionLiteral</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#VersionSpecifier"><span class="k">VERSION_SPECIFIER</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../07-punctuation#Period"><span class="k">PERIOD</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#VersionSpecifier"><span class="k">VERSION_SPECIFIER</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

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

<pre ebnf-snippet="VersionSpecifierFragment" style="display: none;"><a href="#VersionSpecifierFragment"><span class="k">«VERSION_SPECIFIER_FRAGMENT»</span></a><span class="o"> = </span><span class="o">(</span><span class="s2">"0"</span><span class="o">…</span><span class="s2">"9"</span><span class="o"> | </span><span class="s2">"x"</span><span class="o"> | </span><span class="s2">"X"</span><span class="o"> | </span><span class="s2">"*"</span><span class="o">)</span><span class="o">+</span><span class="o">;</span></pre>
