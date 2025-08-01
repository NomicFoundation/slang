<!-- This file is generated automatically by infrastructure scripts (crates/codegen/spec/src/lib.rs:29:22). Please don't edit by hand. -->

# 2.11. User Defined Value Types

```{ .ebnf #UserDefinedValueTypeDefinition }

```

<pre ebnf-snippet="UserDefinedValueTypeDefinition" style="display: none;"><span class="cm">(* Introduced in 0.8.8 *)</span><br /><a href="#UserDefinedValueTypeDefinition"><span class="k">UserDefinedValueTypeDefinition</span></a><span class="o"> = </span><span class="cm">(* type_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#TypeKeyword"><span class="k">TYPE_KEYWORD</span></a><br /><span class="o">                                 </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                                 </span><span class="cm">(* is_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#IsKeyword"><span class="k">IS_KEYWORD</span></a><br /><span class="o">                                 </span><span class="cm">(* value_type: *)</span><span class="o"> </span><a href="../../03-types/02-elementary-types#ElementaryType"><span class="k">ElementaryType</span></a><br /><span class="o">                                 </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>
