<!-- This file is generated automatically by infrastructure scripts (crates/codegen/spec/src/lib.rs:29:22). Please don't edit by hand. -->

# 4.3. Control Statements

```{ .ebnf #IfStatement }

```

<pre ebnf-snippet="IfStatement" style="display: none;"><a href="#IfStatement"><span class="k">IfStatement</span></a><span class="o"> = </span><span class="cm">(* if_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#IfKeyword"><span class="k">IF_KEYWORD</span></a><br /><span class="o">              </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">              </span><span class="cm">(* condition: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><br /><span class="o">              </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><br /><span class="o">              </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="../01-blocks#Statement"><span class="k">Statement</span></a><br /><span class="o">              </span><span class="cm">(* else_branch: *)</span><span class="o"> </span><a href="#ElseBranch"><span class="k">ElseBranch</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #ElseBranch }

```

<pre ebnf-snippet="ElseBranch" style="display: none;"><a href="#ElseBranch"><span class="k">ElseBranch</span></a><span class="o"> = </span><span class="cm">(* else_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ElseKeyword"><span class="k">ELSE_KEYWORD</span></a><br /><span class="o">             </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="../01-blocks#Statement"><span class="k">Statement</span></a><span class="o">;</span></pre>

```{ .ebnf #ForStatement }

```

<pre ebnf-snippet="ForStatement" style="display: none;"><a href="#ForStatement"><span class="k">ForStatement</span></a><span class="o"> = </span><span class="cm">(* for_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ForKeyword"><span class="k">FOR_KEYWORD</span></a><br /><span class="o">               </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">               </span><span class="cm">(* initialization: *)</span><span class="o"> </span><a href="#ForStatementInitialization"><span class="k">ForStatementInitialization</span></a><br /><span class="o">               </span><span class="cm">(* condition: *)</span><span class="o"> </span><a href="#ForStatementCondition"><span class="k">ForStatementCondition</span></a><br /><span class="o">               </span><span class="cm">(* iterator: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">?</span><br /><span class="o">               </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><br /><span class="o">               </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="../01-blocks#Statement"><span class="k">Statement</span></a><span class="o">;</span></pre>

```{ .ebnf #ForStatementInitialization }

```

<pre ebnf-snippet="ForStatementInitialization" style="display: none;"><a href="#ForStatementInitialization"><span class="k">ForStatementInitialization</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../02-declaration-statements#TupleDeconstructionStatement"><span class="k">TupleDeconstructionStatement</span></a><br /><span class="o">                           | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../02-declaration-statements#VariableDeclarationStatement"><span class="k">VariableDeclarationStatement</span></a><br /><span class="o">                           | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../01-blocks#ExpressionStatement"><span class="k">ExpressionStatement</span></a><br /><span class="o">                           | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #ForStatementCondition }

```

<pre ebnf-snippet="ForStatementCondition" style="display: none;"><a href="#ForStatementCondition"><span class="k">ForStatementCondition</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../01-blocks#ExpressionStatement"><span class="k">ExpressionStatement</span></a><br /><span class="o">                      | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #WhileStatement }

```

<pre ebnf-snippet="WhileStatement" style="display: none;"><a href="#WhileStatement"><span class="k">WhileStatement</span></a><span class="o"> = </span><span class="cm">(* while_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#WhileKeyword"><span class="k">WHILE_KEYWORD</span></a><br /><span class="o">                 </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                 </span><span class="cm">(* condition: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><br /><span class="o">                 </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><br /><span class="o">                 </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="../01-blocks#Statement"><span class="k">Statement</span></a><span class="o">;</span></pre>

```{ .ebnf #DoWhileStatement }

```

<pre ebnf-snippet="DoWhileStatement" style="display: none;"><a href="#DoWhileStatement"><span class="k">DoWhileStatement</span></a><span class="o"> = </span><span class="cm">(* do_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#DoKeyword"><span class="k">DO_KEYWORD</span></a><br /><span class="o">                   </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="../01-blocks#Statement"><span class="k">Statement</span></a><br /><span class="o">                   </span><span class="cm">(* while_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#WhileKeyword"><span class="k">WHILE_KEYWORD</span></a><br /><span class="o">                   </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                   </span><span class="cm">(* condition: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><br /><span class="o">                   </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><br /><span class="o">                   </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #ContinueStatement }

```

<pre ebnf-snippet="ContinueStatement" style="display: none;"><a href="#ContinueStatement"><span class="k">ContinueStatement</span></a><span class="o"> = </span><span class="cm">(* continue_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ContinueKeyword"><span class="k">CONTINUE_KEYWORD</span></a><br /><span class="o">                    </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #BreakStatement }

```

<pre ebnf-snippet="BreakStatement" style="display: none;"><a href="#BreakStatement"><span class="k">BreakStatement</span></a><span class="o"> = </span><span class="cm">(* break_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#BreakKeyword"><span class="k">BREAK_KEYWORD</span></a><br /><span class="o">                 </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #ReturnStatement }

```

<pre ebnf-snippet="ReturnStatement" style="display: none;"><a href="#ReturnStatement"><span class="k">ReturnStatement</span></a><span class="o"> = </span><span class="cm">(* return_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ReturnKeyword"><span class="k">RETURN_KEYWORD</span></a><br /><span class="o">                  </span><span class="cm">(* expression: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">?</span><br /><span class="o">                  </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #EmitStatement }

```

<pre ebnf-snippet="EmitStatement" style="display: none;"><span class="cm">(* Introduced in 0.4.21 *)</span><br /><a href="#EmitStatement"><span class="k">EmitStatement</span></a><span class="o"> = </span><span class="cm">(* emit_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#EmitKeyword"><span class="k">EMIT_KEYWORD</span></a><br /><span class="o">                </span><span class="cm">(* event: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><br /><span class="o">                </span><span class="cm">(* arguments: *)</span><span class="o"> </span><a href="../../05-expressions/02-function-calls#ArgumentsDeclaration"><span class="k">ArgumentsDeclaration</span></a><br /><span class="o">                </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>
