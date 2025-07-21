<!-- This file is generated automatically by infrastructure scripts (crates/codegen/spec/src/lib.rs). Please don't edit by hand. -->

# 2.5. Enums

```{ .ebnf #EnumDefinition }

```

<pre ebnf-snippet="EnumDefinition" style="display: none;"><a href="#EnumDefinition"><span class="k">EnumDefinition</span></a><span class="o"> = </span><span class="cm">(* enum_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#EnumKeyword"><span class="k">ENUM_KEYWORD</span></a><br /><span class="o">                 </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                 </span><span class="cm">(* open_brace: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                 </span><span class="cm">(* members: *)</span><span class="o"> </span><a href="#EnumMembers"><span class="k">EnumMembers</span></a><br /><span class="o">                 </span><span class="cm">(* close_brace: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #EnumMembers }

```

<pre ebnf-snippet="EnumMembers" style="display: none;"><a href="#EnumMembers"><span class="k">EnumMembers</span></a><span class="o"> = </span><span class="o">(</span><span class="cm">(* item: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">)</span><span class="o">*</span><span class="o">)</span><span class="o">?</span><span class="o">;</span></pre>
