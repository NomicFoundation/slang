<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.4. Structs

## Syntax

```{ .ebnf #StructDefinition }

```

<pre ebnf-snippet="StructDefinition" style="display: none;"><a href="#StructDefinition"><span class="k">StructDefinition</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#StructKeyword"><span class="k">STRUCT_KEYWORD</span></a><br /><span class="o">                   </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                   </span><a href="../../01-file-structure/09-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                   </span><a href="#StructMembers"><span class="k">StructMembers</span></a><br /><span class="o">                   </span><a href="../../01-file-structure/09-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #StructMembers }

```

<pre ebnf-snippet="StructMembers" style="display: none;"><a href="#StructMembers"><span class="k">StructMembers</span></a><span class="o"> = </span><a href="#StructMember"><span class="k">StructMember</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #StructMember }

```

<pre ebnf-snippet="StructMember" style="display: none;"><a href="#StructMember"><span class="k">StructMember</span></a><span class="o"> = </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">               </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">               </span><a href="../../01-file-structure/09-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/02-definitions/04-structs.md"
