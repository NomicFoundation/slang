<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.2. Interfaces

## Syntax

```{ .ebnf #InterfaceDefinition }

```

<pre ebnf-snippet="InterfaceDefinition" style="display: none;"><a href="#InterfaceDefinition"><span class="k">InterfaceDefinition</span></a><span class="o"> = </span><span class="cm">(* interface_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/08-keywords#InterfaceKeyword"><span class="k">INTERFACE_KEYWORD</span></a><br /><span class="o">                      </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                      </span><span class="cm">(* inheritence: *)</span><span class="o"> </span><a href="../01-contracts#InheritanceSpecifier"><span class="k">InheritanceSpecifier</span></a><span class="o">?</span><br /><span class="o">                      </span><span class="cm">(* open_brace: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                      </span><span class="cm">(* members: *)</span><span class="o"> </span><a href="#InterfaceMembers"><span class="k">InterfaceMembers</span></a><br /><span class="o">                      </span><span class="cm">(* close_brace: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #InterfaceMembers }

```

<pre ebnf-snippet="InterfaceMembers" style="display: none;"><a href="#InterfaceMembers"><span class="k">InterfaceMembers</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="../01-contracts#ContractMember"><span class="k">ContractMember</span></a><span class="o">*</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/02-definitions/02-interfaces.md"
