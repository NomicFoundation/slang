<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.12. Errors

## Syntax

```{ .ebnf #ErrorDefinition }

```

<pre ebnf-snippet="ErrorDefinition" style="display: none;"><span class="cm">(* Introduced in 0.8.4 *)</span><br /><a href="#ErrorDefinition"><span class="k">ErrorDefinition</span></a><span class="o"> = </span><span class="cm">(* error_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/08-keywords#ErrorKeyword"><span class="k">ERROR_KEYWORD</span></a><br /><span class="o">                  </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                  </span><span class="cm">(* members: *)</span><span class="o"> </span><a href="#ErrorParametersDeclaration"><span class="k">ErrorParametersDeclaration</span></a><br /><span class="o">                  </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #ErrorParametersDeclaration }

```

<pre ebnf-snippet="ErrorParametersDeclaration" style="display: none;"><span class="cm">(* Introduced in 0.8.4 *)</span><br /><a href="#ErrorParametersDeclaration"><span class="k">ErrorParametersDeclaration</span></a><span class="o"> = </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                             </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="#ErrorParameters"><span class="k">ErrorParameters</span></a><br /><span class="o">                             </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #ErrorParameters }

```

<pre ebnf-snippet="ErrorParameters" style="display: none;"><span class="cm">(* Introduced in 0.8.4 *)</span><br /><a href="#ErrorParameters"><span class="k">ErrorParameters</span></a><span class="o"> = </span><span class="o">(</span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#ErrorParameter"><span class="k">ErrorParameter</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#ErrorParameter"><span class="k">ErrorParameter</span></a><span class="o">)</span><span class="o">*</span><span class="o">)</span><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #ErrorParameter }

```

<pre ebnf-snippet="ErrorParameter" style="display: none;"><span class="cm">(* Introduced in 0.8.4 *)</span><br /><a href="#ErrorParameter"><span class="k">ErrorParameter</span></a><span class="o"> = </span><span class="cm">(* type_name: *)</span><span class="o"> </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                 </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">?</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/02-definitions/12-errors.md"
