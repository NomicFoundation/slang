<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 4.2. Declaration Statements

```{ .ebnf #TupleDeconstructionStatement }

```

<pre ebnf-snippet="TupleDeconstructionStatement" style="display: none;"><a href="#TupleDeconstructionStatement"><span class="k">TupleDeconstructionStatement</span></a><span class="o"> = </span><span class="cm">(* target: *)</span><span class="o"> </span><a href="#TupleDeconstructionTarget"><span class="k">TupleDeconstructionTarget</span></a><br /><span class="o">                               </span><span class="cm">(* equal: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Equal"><span class="k">EQUAL</span></a><br /><span class="o">                               </span><span class="cm">(* expression: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><br /><span class="o">                               </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #TupleDeconstructionTarget }

```

<pre ebnf-snippet="TupleDeconstructionTarget" style="display: none;"><a href="#TupleDeconstructionTarget"><span class="k">TupleDeconstructionTarget</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#VarTupleDeconstructionTarget"><span class="k">VarTupleDeconstructionTarget</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><span class="o">                          | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#TypedTupleDeconstructionTarget"><span class="k">TypedTupleDeconstructionTarget</span></a><span class="o">;</span></pre>

```{ .ebnf #VarTupleDeconstructionTarget }

```

<pre ebnf-snippet="VarTupleDeconstructionTarget" style="display: none;"><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><a href="#VarTupleDeconstructionTarget"><span class="k">VarTupleDeconstructionTarget</span></a><span class="o"> = </span><span class="cm">(* var_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#VarKeyword"><span class="k">VAR_KEYWORD</span></a><br /><span class="o">                               </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                               </span><span class="cm">(* elements: *)</span><span class="o"> </span><a href="#UntypedTupleDeconstructionElements"><span class="k">UntypedTupleDeconstructionElements</span></a><br /><span class="o">                               </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #UntypedTupleDeconstructionElements }

```

<pre ebnf-snippet="UntypedTupleDeconstructionElements" style="display: none;"><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><a href="#UntypedTupleDeconstructionElements"><span class="k">UntypedTupleDeconstructionElements</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#UntypedTupleDeconstructionElement"><span class="k">UntypedTupleDeconstructionElement</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#UntypedTupleDeconstructionElement"><span class="k">UntypedTupleDeconstructionElement</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #UntypedTupleDeconstructionElement }

```

<pre ebnf-snippet="UntypedTupleDeconstructionElement" style="display: none;"><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><a href="#UntypedTupleDeconstructionElement"><span class="k">UntypedTupleDeconstructionElement</span></a><span class="o"> = </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #TypedTupleDeconstructionTarget }

```

<pre ebnf-snippet="TypedTupleDeconstructionTarget" style="display: none;"><a href="#TypedTupleDeconstructionTarget"><span class="k">TypedTupleDeconstructionTarget</span></a><span class="o"> = </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                                 </span><span class="cm">(* elements: *)</span><span class="o"> </span><a href="#TypedTupleDeconstructionElements"><span class="k">TypedTupleDeconstructionElements</span></a><br /><span class="o">                                 </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #TypedTupleDeconstructionElements }

```

<pre ebnf-snippet="TypedTupleDeconstructionElements" style="display: none;"><a href="#TypedTupleDeconstructionElements"><span class="k">TypedTupleDeconstructionElements</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#TypedTupleDeconstructionElement"><span class="k">TypedTupleDeconstructionElement</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#TypedTupleDeconstructionElement"><span class="k">TypedTupleDeconstructionElement</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #TypedTupleDeconstructionElement }

```

<pre ebnf-snippet="TypedTupleDeconstructionElement" style="display: none;"><a href="#TypedTupleDeconstructionElement"><span class="k">TypedTupleDeconstructionElement</span></a><span class="o"> = </span><span class="cm">(* member: *)</span><span class="o"> </span><a href="#TypedTupleDeconstructionMember"><span class="k">TypedTupleDeconstructionMember</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #TypedTupleDeconstructionMember }

```

<pre ebnf-snippet="TypedTupleDeconstructionMember" style="display: none;"><a href="#TypedTupleDeconstructionMember"><span class="k">TypedTupleDeconstructionMember</span></a><span class="o"> = </span><span class="cm">(* type_name: *)</span><span class="o"> </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                                 </span><span class="cm">(* storage_location: *)</span><span class="o"> </span><a href="#StorageLocation"><span class="k">StorageLocation</span></a><span class="o">?</span><br /><span class="o">                                 </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">;</span></pre>

```{ .ebnf #VariableDeclarationStatement }

```

<pre ebnf-snippet="VariableDeclarationStatement" style="display: none;"><a href="#VariableDeclarationStatement"><span class="k">VariableDeclarationStatement</span></a><span class="o"> = </span><span class="cm">(* variable_type: *)</span><span class="o"> </span><a href="#VariableDeclarationType"><span class="k">VariableDeclarationType</span></a><br /><span class="o">                               </span><span class="cm">(* storage_location: *)</span><span class="o"> </span><a href="#StorageLocation"><span class="k">StorageLocation</span></a><span class="o">?</span><br /><span class="o">                               </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                               </span><span class="cm">(* value: *)</span><span class="o"> </span><a href="#VariableDeclarationValue"><span class="k">VariableDeclarationValue</span></a><span class="o">?</span><br /><span class="o">                               </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #VariableDeclarationType }

```

<pre ebnf-snippet="VariableDeclarationType" style="display: none;"><a href="#VariableDeclarationType"><span class="k">VariableDeclarationType</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                        | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#VarKeyword"><span class="k">VAR_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span></pre>

```{ .ebnf #VariableDeclarationValue }

```

<pre ebnf-snippet="VariableDeclarationValue" style="display: none;"><a href="#VariableDeclarationValue"><span class="k">VariableDeclarationValue</span></a><span class="o"> = </span><span class="cm">(* equal: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Equal"><span class="k">EQUAL</span></a><br /><span class="o">                           </span><span class="cm">(* expression: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">;</span></pre>

```{ .ebnf #StorageLocation }

```

<pre ebnf-snippet="StorageLocation" style="display: none;"><a href="#StorageLocation"><span class="k">StorageLocation</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#MemoryKeyword"><span class="k">MEMORY_KEYWORD</span></a><br /><span class="o">                | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#StorageKeyword"><span class="k">STORAGE_KEYWORD</span></a><br /><span class="o">                | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#CallDataKeyword"><span class="k">CALL_DATA_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.5.0 *)</span></pre>
