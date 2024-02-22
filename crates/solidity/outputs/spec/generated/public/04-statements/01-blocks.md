<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 4.1. Blocks

## Syntax

```{ .ebnf #Block }

```

<pre ebnf-snippet="Block" style="display: none;"><a href="#Block"><span class="k">Block</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">        </span><a href="#Statements"><span class="k">Statements</span></a><span class="o">?</span><br /><span class="o">        </span><a href="../../01-file-structure/09-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #Statements }

```

<pre ebnf-snippet="Statements" style="display: none;"><a href="#Statements"><span class="k">Statements</span></a><span class="o"> = </span><a href="#Statement"><span class="k">Statement</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #Statement }

```

<pre ebnf-snippet="Statement" style="display: none;"><a href="#Statement"><span class="k">Statement</span></a><span class="o"> = </span><a href="#ExpressionStatement"><span class="k">ExpressionStatement</span></a><br /><span class="o">          | </span><a href="../02-declaration-statements#VariableDeclarationStatement"><span class="k">VariableDeclarationStatement</span></a><br /><span class="o">          | </span><a href="../02-declaration-statements#TupleDeconstructionStatement"><span class="k">TupleDeconstructionStatement</span></a><br /><span class="o">          | </span><a href="../03-control-statements#IfStatement"><span class="k">IfStatement</span></a><br /><span class="o">          | </span><a href="../03-control-statements#ForStatement"><span class="k">ForStatement</span></a><br /><span class="o">          | </span><a href="../03-control-statements#WhileStatement"><span class="k">WhileStatement</span></a><br /><span class="o">          | </span><a href="../03-control-statements#DoWhileStatement"><span class="k">DoWhileStatement</span></a><br /><span class="o">          | </span><a href="../03-control-statements#ContinueStatement"><span class="k">ContinueStatement</span></a><br /><span class="o">          | </span><a href="../03-control-statements#BreakStatement"><span class="k">BreakStatement</span></a><br /><span class="o">          | </span><a href="../03-control-statements#DeleteStatement"><span class="k">DeleteStatement</span></a><br /><span class="o">          | </span><a href="../03-control-statements#ReturnStatement"><span class="k">ReturnStatement</span></a><br /><span class="o">          | </span><a href="../04-error-handling#ThrowStatement"><span class="k">ThrowStatement</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><span class="o">          | </span><a href="../03-control-statements#EmitStatement"><span class="k">EmitStatement</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.4.21 *)</span><br /><span class="o">          | </span><a href="../04-error-handling#TryStatement"><span class="k">TryStatement</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">          | </span><a href="../04-error-handling#RevertStatement"><span class="k">RevertStatement</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.8.4 *)</span><br /><span class="o">          | </span><a href="#AssemblyStatement"><span class="k">AssemblyStatement</span></a><br /><span class="o">          | </span><a href="#Block"><span class="k">Block</span></a><br /><span class="o">          | </span><a href="#UncheckedBlock"><span class="k">UncheckedBlock</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.0 *)</span></pre>

```{ .ebnf #UncheckedBlock }

```

<pre ebnf-snippet="UncheckedBlock" style="display: none;"><span class="cm">(* Introduced in 0.8.0 *)</span><br /><a href="#UncheckedBlock"><span class="k">UncheckedBlock</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#UncheckedKeyword"><span class="k">UNCHECKED_KEYWORD</span></a><br /><span class="o">                 </span><a href="#Block"><span class="k">Block</span></a><span class="o">;</span></pre>

```{ .ebnf #ExpressionStatement }

```

<pre ebnf-snippet="ExpressionStatement" style="display: none;"><a href="#ExpressionStatement"><span class="k">ExpressionStatement</span></a><span class="o"> = </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><br /><span class="o">                      </span><a href="../../01-file-structure/09-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #AssemblyStatement }

```

<pre ebnf-snippet="AssemblyStatement" style="display: none;"><a href="#AssemblyStatement"><span class="k">AssemblyStatement</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#AssemblyKeyword"><span class="k">ASSEMBLY_KEYWORD</span></a><br /><span class="o">                    </span><a href="../../05-expressions/05-strings#StringLiteral"><span class="k">StringLiteral</span></a><span class="o">?</span><br /><span class="o">                    </span><a href="#AssemblyFlagsDeclaration"><span class="k">AssemblyFlagsDeclaration</span></a><span class="o">?</span><br /><span class="o">                    </span><a href="../../06-yul/01-yul-statements#YulBlock"><span class="k">YulBlock</span></a><span class="o">;</span></pre>

```{ .ebnf #AssemblyFlagsDeclaration }

```

<pre ebnf-snippet="AssemblyFlagsDeclaration" style="display: none;"><a href="#AssemblyFlagsDeclaration"><span class="k">AssemblyFlagsDeclaration</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                           </span><a href="#AssemblyFlags"><span class="k">AssemblyFlags</span></a><br /><span class="o">                           </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #AssemblyFlags }

```

<pre ebnf-snippet="AssemblyFlags" style="display: none;"><a href="#AssemblyFlags"><span class="k">AssemblyFlags</span></a><span class="o"> = </span><a href="../../05-expressions/05-strings#StringLiteral"><span class="k">StringLiteral</span></a><span class="o"> </span><span class="o">(</span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="../../05-expressions/05-strings#StringLiteral"><span class="k">StringLiteral</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/04-statements/01-blocks.md"
