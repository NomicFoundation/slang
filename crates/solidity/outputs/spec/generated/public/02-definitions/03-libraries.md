<!-- This file is generated automatically by infrastructure scripts (crates/codegen/spec/src/lib.rs). Please don't edit by hand. -->

# 2.3. Libraries

```{ .ebnf #LibraryDefinition }

```

<pre ebnf-snippet="LibraryDefinition" style="display: none;"><a href="#LibraryDefinition"><span class="k">LibraryDefinition</span></a><span class="o"> = </span><span class="cm">(* library_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#LibraryKeyword"><span class="k">LIBRARY_KEYWORD</span></a><br /><span class="o">                    </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                    </span><span class="cm">(* open_brace: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                    </span><span class="cm">(* members: *)</span><span class="o"> </span><a href="#LibraryMembers"><span class="k">LibraryMembers</span></a><br /><span class="o">                    </span><span class="cm">(* close_brace: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #LibraryMembers }

```

<pre ebnf-snippet="LibraryMembers" style="display: none;"><a href="#LibraryMembers"><span class="k">LibraryMembers</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="../01-contracts#ContractMember"><span class="k">ContractMember</span></a><span class="o">*</span><span class="o">;</span></pre>
