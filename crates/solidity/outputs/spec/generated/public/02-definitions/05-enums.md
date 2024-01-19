<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.5. Enums

## Syntax

```{ .ebnf #EnumDefinition }

```

<pre ebnf-snippet="EnumDefinition" style="display: none;"><a href="#EnumDefinition"><span class="k">EnumDefinition</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#EnumKeyword"><span class="k">ENUM_KEYWORD</span></a><br /><span class="o">                 </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                 </span><a href="../../01-file-structure/09-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                 </span><a href="#EnumMembers"><span class="k">EnumMembers</span></a><span class="o">?</span><br /><span class="o">                 </span><a href="../../01-file-structure/09-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #EnumMembers }

```

<pre ebnf-snippet="EnumMembers" style="display: none;"><a href="#EnumMembers"><span class="k">EnumMembers</span></a><span class="o"> = </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o"> </span><span class="o">(</span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/02-definitions/05-enums.md"
