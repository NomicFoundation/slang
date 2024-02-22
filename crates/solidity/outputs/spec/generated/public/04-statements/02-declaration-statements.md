<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 4.2. Declaration Statements

## Syntax

```{ .ebnf #TupleDeconstructionStatement }

```

<pre ebnf-snippet="TupleDeconstructionStatement" style="display: none;"><a href="#TupleDeconstructionStatement"><span class="k">TupleDeconstructionStatement</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#VarKeyword"><span class="k">VAR_KEYWORD</span></a><span class="o">?</span><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><span class="o">                               </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                               </span><a href="#TupleDeconstructionElements"><span class="k">TupleDeconstructionElements</span></a><br /><span class="o">                               </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><br /><span class="o">                               </span><a href="../../01-file-structure/09-punctuation#Equal"><span class="k">EQUAL</span></a><br /><span class="o">                               </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><br /><span class="o">                               </span><a href="../../01-file-structure/09-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #TupleDeconstructionElements }

```

<pre ebnf-snippet="TupleDeconstructionElements" style="display: none;"><a href="#TupleDeconstructionElements"><span class="k">TupleDeconstructionElements</span></a><span class="o"> = </span><a href="#TupleDeconstructionElement"><span class="k">TupleDeconstructionElement</span></a><span class="o"> </span><span class="o">(</span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="#TupleDeconstructionElement"><span class="k">TupleDeconstructionElement</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #TupleDeconstructionElement }

```

<pre ebnf-snippet="TupleDeconstructionElement" style="display: none;"><a href="#TupleDeconstructionElement"><span class="k">TupleDeconstructionElement</span></a><span class="o"> = </span><a href="#TupleMember"><span class="k">TupleMember</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #TupleMember }

```

<pre ebnf-snippet="TupleMember" style="display: none;"><a href="#TupleMember"><span class="k">TupleMember</span></a><span class="o"> = </span><a href="#TypedTupleMember"><span class="k">TypedTupleMember</span></a><br /><span class="o">            | </span><a href="#UntypedTupleMember"><span class="k">UntypedTupleMember</span></a><span class="o">;</span></pre>

```{ .ebnf #TypedTupleMember }

```

<pre ebnf-snippet="TypedTupleMember" style="display: none;"><a href="#TypedTupleMember"><span class="k">TypedTupleMember</span></a><span class="o"> = </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                   </span><a href="#StorageLocation"><span class="k">StorageLocation</span></a><span class="o">?</span><br /><span class="o">                   </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">;</span></pre>

```{ .ebnf #UntypedTupleMember }

```

<pre ebnf-snippet="UntypedTupleMember" style="display: none;"><a href="#UntypedTupleMember"><span class="k">UntypedTupleMember</span></a><span class="o"> = </span><a href="#StorageLocation"><span class="k">StorageLocation</span></a><span class="o">?</span><br /><span class="o">                     </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">;</span></pre>

```{ .ebnf #VariableDeclarationStatement }

```

<pre ebnf-snippet="VariableDeclarationStatement" style="display: none;"><a href="#VariableDeclarationStatement"><span class="k">VariableDeclarationStatement</span></a><span class="o"> = </span><a href="#VariableDeclarationType"><span class="k">VariableDeclarationType</span></a><br /><span class="o">                               </span><a href="#StorageLocation"><span class="k">StorageLocation</span></a><span class="o">?</span><br /><span class="o">                               </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                               </span><a href="#VariableDeclarationValue"><span class="k">VariableDeclarationValue</span></a><span class="o">?</span><br /><span class="o">                               </span><a href="../../01-file-structure/09-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #VariableDeclarationType }

```

<pre ebnf-snippet="VariableDeclarationType" style="display: none;"><a href="#VariableDeclarationType"><span class="k">VariableDeclarationType</span></a><span class="o"> = </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                        | </span><a href="../../01-file-structure/08-keywords#VarKeyword"><span class="k">VAR_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span></pre>

```{ .ebnf #VariableDeclarationValue }

```

<pre ebnf-snippet="VariableDeclarationValue" style="display: none;"><a href="#VariableDeclarationValue"><span class="k">VariableDeclarationValue</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#Equal"><span class="k">EQUAL</span></a><br /><span class="o">                           </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">;</span></pre>

```{ .ebnf #StorageLocation }

```

<pre ebnf-snippet="StorageLocation" style="display: none;"><a href="#StorageLocation"><span class="k">StorageLocation</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#MemoryKeyword"><span class="k">MEMORY_KEYWORD</span></a><br /><span class="o">                | </span><a href="../../01-file-structure/08-keywords#StorageKeyword"><span class="k">STORAGE_KEYWORD</span></a><br /><span class="o">                | </span><a href="../../01-file-structure/08-keywords#CallDataKeyword"><span class="k">CALL_DATA_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.5.0 *)</span></pre>

--8<-- "crates/solidity/inputs/language/docs/04-statements/02-declaration-statements.md"
