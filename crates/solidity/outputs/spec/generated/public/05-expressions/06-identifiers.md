<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 5.6. Identifiers

## Syntax

```{ .ebnf #IdentifierPath }

```

<pre ebnf-snippet="IdentifierPath" style="display: none;"><a href="#IdentifierPath"><span class="k">IdentifierPath</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#Identifier"><span class="k">IDENTIFIER</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#Period"><span class="k">PERIOD</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #Identifier }

```

<pre ebnf-snippet="Identifier" style="display: none;"><a href="#Identifier"><span class="k">IDENTIFIER</span></a><span class="o"> = </span><a href="#IdentifierStart"><span class="k">«IDENTIFIER_START»</span></a><span class="o"> </span><a href="#IdentifierPart"><span class="k">«IDENTIFIER_PART»</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #IdentifierStart }

```

<pre ebnf-snippet="IdentifierStart" style="display: none;"><a href="#IdentifierStart"><span class="k">«IDENTIFIER_START»</span></a><span class="o"> = </span><span class="s2">"_"</span><span class="o"> | </span><span class="s2">"$"</span><span class="o"> | </span><span class="s2">"a"</span><span class="o">…</span><span class="s2">"z"</span><span class="o"> | </span><span class="s2">"A"</span><span class="o">…</span><span class="s2">"Z"</span><span class="o">;</span></pre>

```{ .ebnf #IdentifierPart }

```

<pre ebnf-snippet="IdentifierPart" style="display: none;"><a href="#IdentifierPart"><span class="k">«IDENTIFIER_PART»</span></a><span class="o"> = </span><a href="#IdentifierStart"><span class="k">«IDENTIFIER_START»</span></a><span class="o"> | </span><span class="s2">"0"</span><span class="o">…</span><span class="s2">"9"</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/05-expressions/06-identifiers.md"
