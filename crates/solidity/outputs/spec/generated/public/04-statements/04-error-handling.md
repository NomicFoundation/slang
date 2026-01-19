<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 4.4. Error Handling

```{ .ebnf #TryStatement }

```

<pre ebnf-snippet="TryStatement" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#TryStatement"><span class="k">TryStatement</span></a><span class="o"> = </span><span class="cm">(* try_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#TryKeyword"><span class="k">TRY_KEYWORD</span></a><br /><span class="o">               </span><span class="cm">(* expression: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><br /><span class="o">               </span><span class="cm">(* returns: *)</span><span class="o"> </span><a href="../../02-definitions/08-functions#ReturnsDeclaration"><span class="k">ReturnsDeclaration</span></a><span class="o">?</span><br /><span class="o">               </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="../01-blocks#Block"><span class="k">Block</span></a><br /><span class="o">               </span><span class="cm">(* catch_clauses: *)</span><span class="o"> </span><a href="#CatchClauses"><span class="k">CatchClauses</span></a><span class="o">;</span></pre>

```{ .ebnf #CatchClauses }

```

<pre ebnf-snippet="CatchClauses" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#CatchClauses"><span class="k">CatchClauses</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#CatchClause"><span class="k">CatchClause</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #CatchClause }

```

<pre ebnf-snippet="CatchClause" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#CatchClause"><span class="k">CatchClause</span></a><span class="o"> = </span><span class="cm">(* catch_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#CatchKeyword"><span class="k">CATCH_KEYWORD</span></a><br /><span class="o">              </span><span class="cm">(* error: *)</span><span class="o"> </span><a href="#CatchClauseError"><span class="k">CatchClauseError</span></a><span class="o">?</span><br /><span class="o">              </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="../01-blocks#Block"><span class="k">Block</span></a><span class="o">;</span></pre>

```{ .ebnf #CatchClauseError }

```

<pre ebnf-snippet="CatchClauseError" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#CatchClauseError"><span class="k">CatchClauseError</span></a><span class="o"> = </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">?</span><br /><span class="o">                   </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="../../02-definitions/08-functions#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><span class="o">;</span></pre>

```{ .ebnf #RevertStatement }

```

<pre ebnf-snippet="RevertStatement" style="display: none;"><span class="cm">(* Introduced in 0.8.4 *)</span><br /><a href="#RevertStatement"><span class="k">RevertStatement</span></a><span class="o"> = </span><span class="cm">(* revert_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#RevertKeyword"><span class="k">REVERT_KEYWORD</span></a><br /><span class="o">                  </span><span class="cm">(* error: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><br /><span class="o">                  </span><span class="cm">(* arguments: *)</span><span class="o"> </span><a href="../../05-expressions/02-function-calls#ArgumentsDeclaration"><span class="k">ArgumentsDeclaration</span></a><br /><span class="o">                  </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #ThrowStatement }

```

<pre ebnf-snippet="ThrowStatement" style="display: none;"><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><a href="#ThrowStatement"><span class="k">ThrowStatement</span></a><span class="o"> = </span><span class="cm">(* throw_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ThrowKeyword"><span class="k">THROW_KEYWORD</span></a><br /><span class="o">                 </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>
