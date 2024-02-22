<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 1.4. Import Directives

## Syntax

```{ .ebnf #ImportDirective }

```

<pre ebnf-snippet="ImportDirective" style="display: none;"><a href="#ImportDirective"><span class="k">ImportDirective</span></a><span class="o"> = </span><a href="../08-keywords#ImportKeyword"><span class="k">IMPORT_KEYWORD</span></a><br /><span class="o">                  </span><a href="#ImportClause"><span class="k">ImportClause</span></a><br /><span class="o">                  </span><a href="../09-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #ImportClause }

```

<pre ebnf-snippet="ImportClause" style="display: none;"><a href="#ImportClause"><span class="k">ImportClause</span></a><span class="o"> = </span><a href="#PathImport"><span class="k">PathImport</span></a><br /><span class="o">             | </span><a href="#NamedImport"><span class="k">NamedImport</span></a><br /><span class="o">             | </span><a href="#ImportDeconstruction"><span class="k">ImportDeconstruction</span></a><span class="o">;</span></pre>

```{ .ebnf #PathImport }

```

<pre ebnf-snippet="PathImport" style="display: none;"><a href="#PathImport"><span class="k">PathImport</span></a><span class="o"> = </span><a href="../../05-expressions/05-strings#StringLiteral"><span class="k">StringLiteral</span></a><br /><span class="o">             </span><a href="#ImportAlias"><span class="k">ImportAlias</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #NamedImport }

```

<pre ebnf-snippet="NamedImport" style="display: none;"><a href="#NamedImport"><span class="k">NamedImport</span></a><span class="o"> = </span><a href="../09-punctuation#Asterisk"><span class="k">ASTERISK</span></a><br /><span class="o">              </span><a href="#ImportAlias"><span class="k">ImportAlias</span></a><br /><span class="o">              </span><a href="../08-keywords#FromKeyword"><span class="k">FROM_KEYWORD</span></a><br /><span class="o">              </span><a href="../../05-expressions/05-strings#StringLiteral"><span class="k">StringLiteral</span></a><span class="o">;</span></pre>

```{ .ebnf #ImportDeconstruction }

```

<pre ebnf-snippet="ImportDeconstruction" style="display: none;"><a href="#ImportDeconstruction"><span class="k">ImportDeconstruction</span></a><span class="o"> = </span><a href="../09-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                       </span><a href="#ImportDeconstructionSymbols"><span class="k">ImportDeconstructionSymbols</span></a><br /><span class="o">                       </span><a href="../09-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><br /><span class="o">                       </span><a href="../08-keywords#FromKeyword"><span class="k">FROM_KEYWORD</span></a><br /><span class="o">                       </span><a href="../../05-expressions/05-strings#StringLiteral"><span class="k">StringLiteral</span></a><span class="o">;</span></pre>

```{ .ebnf #ImportDeconstructionSymbols }

```

<pre ebnf-snippet="ImportDeconstructionSymbols" style="display: none;"><a href="#ImportDeconstructionSymbols"><span class="k">ImportDeconstructionSymbols</span></a><span class="o"> = </span><a href="#ImportDeconstructionSymbol"><span class="k">ImportDeconstructionSymbol</span></a><span class="o"> </span><span class="o">(</span><a href="../09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="#ImportDeconstructionSymbol"><span class="k">ImportDeconstructionSymbol</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #ImportDeconstructionSymbol }

```

<pre ebnf-snippet="ImportDeconstructionSymbol" style="display: none;"><a href="#ImportDeconstructionSymbol"><span class="k">ImportDeconstructionSymbol</span></a><span class="o"> = </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                             </span><a href="#ImportAlias"><span class="k">ImportAlias</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #ImportAlias }

```

<pre ebnf-snippet="ImportAlias" style="display: none;"><a href="#ImportAlias"><span class="k">ImportAlias</span></a><span class="o"> = </span><a href="../08-keywords#AsKeyword"><span class="k">AS_KEYWORD</span></a><br /><span class="o">              </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/01-file-structure/04-import-directives.md"
