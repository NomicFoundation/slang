<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.10. Events

## Syntax

```{ .ebnf #EventDefinition }

```

<pre ebnf-snippet="EventDefinition" style="display: none;"><a href="#EventDefinition"><span class="k">EventDefinition</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#EventKeyword"><span class="k">EVENT_KEYWORD</span></a><br /><span class="o">                  </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                  </span><a href="#EventParametersDeclaration"><span class="k">EventParametersDeclaration</span></a><br /><span class="o">                  </span><a href="../../01-file-structure/08-keywords#AnonymousKeyword"><span class="k">ANONYMOUS_KEYWORD</span></a><span class="o">?</span><br /><span class="o">                  </span><a href="../../01-file-structure/09-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #EventParametersDeclaration }

```

<pre ebnf-snippet="EventParametersDeclaration" style="display: none;"><a href="#EventParametersDeclaration"><span class="k">EventParametersDeclaration</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                             </span><a href="#EventParameters"><span class="k">EventParameters</span></a><br /><span class="o">                             </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #EventParameters }

```

<pre ebnf-snippet="EventParameters" style="display: none;"><a href="#EventParameters"><span class="k">EventParameters</span></a><span class="o"> = </span><span class="o">(</span><a href="#EventParameter"><span class="k">EventParameter</span></a><span class="o"> </span><span class="o">(</span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="#EventParameter"><span class="k">EventParameter</span></a><span class="o">)</span><span class="o">*</span><span class="o">)</span><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #EventParameter }

```

<pre ebnf-snippet="EventParameter" style="display: none;"><a href="#EventParameter"><span class="k">EventParameter</span></a><span class="o"> = </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                 </span><a href="#EventIndexedAttribute"><span class="k">EventIndexedAttribute</span></a><span class="o">?</span><br /><span class="o">                 </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #EventIndexedAttribute }

```

<pre ebnf-snippet="EventIndexedAttribute" style="display: none;"><a href="#EventIndexedAttribute"><span class="k">EventIndexedAttribute</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#IndexedKeyword"><span class="k">INDEXED_KEYWORD</span></a><br /><span class="o">                        </span><a href="#RepeatedIndexedKeyword"><span class="k">RepeatedIndexedKeyword</span></a><span class="o">?</span><span class="o">;</span><span class="o"> </span><span class="cm">(* Deprecated in 0.8.18 *)</span></pre>

```{ .ebnf #RepeatedIndexedKeyword }

```

<pre ebnf-snippet="RepeatedIndexedKeyword" style="display: none;"><span class="cm">(* Deprecated in 0.8.18 *)</span><br /><a href="#RepeatedIndexedKeyword"><span class="k">RepeatedIndexedKeyword</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#IndexedKeyword"><span class="k">INDEXED_KEYWORD</span></a><span class="o">+</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/02-definitions/10-events.md"
