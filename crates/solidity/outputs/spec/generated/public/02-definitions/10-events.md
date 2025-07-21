<!-- This file is generated automatically by infrastructure scripts (crates/codegen/spec/src/lib.rs:29:22). Please don't edit by hand. -->

# 2.10. Events

```{ .ebnf #EventDefinition }

```

<pre ebnf-snippet="EventDefinition" style="display: none;"><a href="#EventDefinition"><span class="k">EventDefinition</span></a><span class="o"> = </span><span class="cm">(* event_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#EventKeyword"><span class="k">EVENT_KEYWORD</span></a><br /><span class="o">                  </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                  </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="#EventParametersDeclaration"><span class="k">EventParametersDeclaration</span></a><br /><span class="o">                  </span><span class="cm">(* anonymous_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#AnonymousKeyword"><span class="k">ANONYMOUS_KEYWORD</span></a><span class="o">?</span><br /><span class="o">                  </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #EventParametersDeclaration }

```

<pre ebnf-snippet="EventParametersDeclaration" style="display: none;"><a href="#EventParametersDeclaration"><span class="k">EventParametersDeclaration</span></a><span class="o"> = </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                             </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="#EventParameters"><span class="k">EventParameters</span></a><br /><span class="o">                             </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #EventParameters }

```

<pre ebnf-snippet="EventParameters" style="display: none;"><a href="#EventParameters"><span class="k">EventParameters</span></a><span class="o"> = </span><span class="o">(</span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#EventParameter"><span class="k">EventParameter</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#EventParameter"><span class="k">EventParameter</span></a><span class="o">)</span><span class="o">*</span><span class="o">)</span><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #EventParameter }

```

<pre ebnf-snippet="EventParameter" style="display: none;"><a href="#EventParameter"><span class="k">EventParameter</span></a><span class="o"> = </span><span class="cm">(* type_name: *)</span><span class="o"> </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                 </span><span class="cm">(* indexed_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#IndexedKeyword"><span class="k">INDEXED_KEYWORD</span></a><span class="o">?</span><br /><span class="o">                 </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">?</span><span class="o">;</span></pre>
