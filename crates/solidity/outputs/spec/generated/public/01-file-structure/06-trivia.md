<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 1.6. Trivia

## Syntax

```{ .ebnf #Whitespace }

```

<pre ebnf-snippet="Whitespace" style="display: none;"><a href="#Whitespace"><span class="k">WHITESPACE</span></a><span class="o"> = </span><span class="o">(</span><span class="s2">" "</span><span class="o"> | </span><span class="s2">"\t"</span><span class="o">)</span><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #EndOfLine }

```

<pre ebnf-snippet="EndOfLine" style="display: none;"><a href="#EndOfLine"><span class="k">END_OF_LINE</span></a><span class="o"> = </span><span class="s2">"\r"</span><span class="o">?</span><span class="o"> </span><span class="s2">"\n"</span><span class="o">;</span></pre>

```{ .ebnf #MultilineComment }

```

<pre ebnf-snippet="MultilineComment" style="display: none;"><a href="#MultilineComment"><span class="k">MULTILINE_COMMENT</span></a><span class="o"> = </span><span class="s2">"/"</span><span class="o"> </span><span class="s2">"*"</span><span class="o"> </span><span class="o">(</span><span class="o">!</span><span class="s2">"*"</span><span class="o"> | </span><span class="s2">"*"</span><span class="o">)</span><span class="o">*</span><span class="o"> </span><span class="s2">"*"</span><span class="o"> </span><span class="s2">"/"</span><span class="o">;</span></pre>

```{ .ebnf #SingleLineComment }

```

<pre ebnf-snippet="SingleLineComment" style="display: none;"><a href="#SingleLineComment"><span class="k">SINGLE_LINE_COMMENT</span></a><span class="o"> = </span><span class="s2">"//"</span><span class="o"> </span><span class="o">(</span><span class="o">!</span><span class="o">(</span><span class="s2">"\r"</span><span class="o"> </span><span class="s2">"\n"</span><span class="o">)</span><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/01-file-structure/06-trivia.md"
