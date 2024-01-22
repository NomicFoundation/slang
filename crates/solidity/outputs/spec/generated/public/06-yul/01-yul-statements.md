<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 6.1. Yul Statements

## Syntax

```{ .ebnf #YulBlock }

```

<pre ebnf-snippet="YulBlock" style="display: none;"><a href="#YulBlock"><span class="k">YulBlock</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">           </span><a href="#YulStatements"><span class="k">YulStatements</span></a><span class="o">?</span><br /><span class="o">           </span><a href="../../01-file-structure/09-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #YulStatements }

```

<pre ebnf-snippet="YulStatements" style="display: none;"><a href="#YulStatements"><span class="k">YulStatements</span></a><span class="o"> = </span><a href="#YulStatement"><span class="k">YulStatement</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #YulStatement }

```

<pre ebnf-snippet="YulStatement" style="display: none;"><a href="#YulStatement"><span class="k">YulStatement</span></a><span class="o"> = </span><a href="#YulBlock"><span class="k">YulBlock</span></a><br /><span class="o">             | </span><a href="#YulFunctionDefinition"><span class="k">YulFunctionDefinition</span></a><br /><span class="o">             | </span><a href="#YulVariableDeclarationStatement"><span class="k">YulVariableDeclarationStatement</span></a><br /><span class="o">             | </span><a href="#YulAssignmentStatement"><span class="k">YulAssignmentStatement</span></a><br /><span class="o">             | </span><a href="#YulIfStatement"><span class="k">YulIfStatement</span></a><br /><span class="o">             | </span><a href="#YulForStatement"><span class="k">YulForStatement</span></a><br /><span class="o">             | </span><a href="#YulSwitchStatement"><span class="k">YulSwitchStatement</span></a><br /><span class="o">             | </span><a href="#YulLeaveStatement"><span class="k">YulLeaveStatement</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">             | </span><a href="#YulBreakStatement"><span class="k">YulBreakStatement</span></a><br /><span class="o">             | </span><a href="#YulContinueStatement"><span class="k">YulContinueStatement</span></a><br /><span class="o">             | </span><a href="../02-yul-expressions#YulExpression"><span class="k">YulExpression</span></a><span class="o">;</span></pre>

```{ .ebnf #YulFunctionDefinition }

```

<pre ebnf-snippet="YulFunctionDefinition" style="display: none;"><a href="#YulFunctionDefinition"><span class="k">YulFunctionDefinition</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulFunctionKeyword"><span class="k">YUL_FUNCTION_KEYWORD</span></a><br /><span class="o">                        </span><a href="../02-yul-expressions#YulIdentifier"><span class="k">YUL_IDENTIFIER</span></a><br /><span class="o">                        </span><a href="#YulParametersDeclaration"><span class="k">YulParametersDeclaration</span></a><br /><span class="o">                        </span><a href="#YulReturnsDeclaration"><span class="k">YulReturnsDeclaration</span></a><span class="o">?</span><br /><span class="o">                        </span><a href="#YulBlock"><span class="k">YulBlock</span></a><span class="o">;</span></pre>

```{ .ebnf #YulParametersDeclaration }

```

<pre ebnf-snippet="YulParametersDeclaration" style="display: none;"><a href="#YulParametersDeclaration"><span class="k">YulParametersDeclaration</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                           </span><a href="#YulParameters"><span class="k">YulParameters</span></a><span class="o">?</span><br /><span class="o">                           </span><a href="../../01-file-structure/09-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #YulParameters }

```

<pre ebnf-snippet="YulParameters" style="display: none;"><a href="#YulParameters"><span class="k">YulParameters</span></a><span class="o"> = </span><a href="../02-yul-expressions#YulIdentifier"><span class="k">YUL_IDENTIFIER</span></a><span class="o"> </span><span class="o">(</span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="../02-yul-expressions#YulIdentifier"><span class="k">YUL_IDENTIFIER</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #YulReturnsDeclaration }

```

<pre ebnf-snippet="YulReturnsDeclaration" style="display: none;"><a href="#YulReturnsDeclaration"><span class="k">YulReturnsDeclaration</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#MinusGreaterThan"><span class="k">MINUS_GREATER_THAN</span></a><br /><span class="o">                        </span><a href="#YulReturnVariables"><span class="k">YulReturnVariables</span></a><span class="o">;</span></pre>

```{ .ebnf #YulReturnVariables }

```

<pre ebnf-snippet="YulReturnVariables" style="display: none;"><a href="#YulReturnVariables"><span class="k">YulReturnVariables</span></a><span class="o"> = </span><a href="../02-yul-expressions#YulIdentifier"><span class="k">YUL_IDENTIFIER</span></a><span class="o"> </span><span class="o">(</span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="../02-yul-expressions#YulIdentifier"><span class="k">YUL_IDENTIFIER</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #YulVariableDeclarationStatement }

```

<pre ebnf-snippet="YulVariableDeclarationStatement" style="display: none;"><a href="#YulVariableDeclarationStatement"><span class="k">YulVariableDeclarationStatement</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulLetKeyword"><span class="k">YUL_LET_KEYWORD</span></a><br /><span class="o">                                  </span><a href="../02-yul-expressions#YulIdentifierPaths"><span class="k">YulIdentifierPaths</span></a><br /><span class="o">                                  </span><a href="#YulVariableDeclarationValue"><span class="k">YulVariableDeclarationValue</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #YulVariableDeclarationValue }

```

<pre ebnf-snippet="YulVariableDeclarationValue" style="display: none;"><a href="#YulVariableDeclarationValue"><span class="k">YulVariableDeclarationValue</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#ColonEqual"><span class="k">COLON_EQUAL</span></a><br /><span class="o">                              </span><a href="../02-yul-expressions#YulExpression"><span class="k">YulExpression</span></a><span class="o">;</span></pre>

```{ .ebnf #YulAssignmentStatement }

```

<pre ebnf-snippet="YulAssignmentStatement" style="display: none;"><a href="#YulAssignmentStatement"><span class="k">YulAssignmentStatement</span></a><span class="o"> = </span><a href="../02-yul-expressions#YulIdentifierPaths"><span class="k">YulIdentifierPaths</span></a><br /><span class="o">                         </span><a href="../../01-file-structure/09-punctuation#ColonEqual"><span class="k">COLON_EQUAL</span></a><br /><span class="o">                         </span><a href="../02-yul-expressions#YulExpression"><span class="k">YulExpression</span></a><span class="o">;</span></pre>

```{ .ebnf #YulIfStatement }

```

<pre ebnf-snippet="YulIfStatement" style="display: none;"><a href="#YulIfStatement"><span class="k">YulIfStatement</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulIfKeyword"><span class="k">YUL_IF_KEYWORD</span></a><br /><span class="o">                 </span><a href="../02-yul-expressions#YulExpression"><span class="k">YulExpression</span></a><br /><span class="o">                 </span><a href="#YulBlock"><span class="k">YulBlock</span></a><span class="o">;</span></pre>

```{ .ebnf #YulLeaveStatement }

```

<pre ebnf-snippet="YulLeaveStatement" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#YulLeaveStatement"><span class="k">YulLeaveStatement</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulLeaveKeyword"><span class="k">YUL_LEAVE_KEYWORD</span></a><span class="o">;</span></pre>

```{ .ebnf #YulBreakStatement }

```

<pre ebnf-snippet="YulBreakStatement" style="display: none;"><a href="#YulBreakStatement"><span class="k">YulBreakStatement</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulBreakKeyword"><span class="k">YUL_BREAK_KEYWORD</span></a><span class="o">;</span></pre>

```{ .ebnf #YulContinueStatement }

```

<pre ebnf-snippet="YulContinueStatement" style="display: none;"><a href="#YulContinueStatement"><span class="k">YulContinueStatement</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulContinueKeyword"><span class="k">YUL_CONTINUE_KEYWORD</span></a><span class="o">;</span></pre>

```{ .ebnf #YulForStatement }

```

<pre ebnf-snippet="YulForStatement" style="display: none;"><a href="#YulForStatement"><span class="k">YulForStatement</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulForKeyword"><span class="k">YUL_FOR_KEYWORD</span></a><br /><span class="o">                  </span><a href="#YulBlock"><span class="k">YulBlock</span></a><br /><span class="o">                  </span><a href="../02-yul-expressions#YulExpression"><span class="k">YulExpression</span></a><br /><span class="o">                  </span><a href="#YulBlock"><span class="k">YulBlock</span></a><br /><span class="o">                  </span><a href="#YulBlock"><span class="k">YulBlock</span></a><span class="o">;</span></pre>

```{ .ebnf #YulSwitchStatement }

```

<pre ebnf-snippet="YulSwitchStatement" style="display: none;"><a href="#YulSwitchStatement"><span class="k">YulSwitchStatement</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulSwitchKeyword"><span class="k">YUL_SWITCH_KEYWORD</span></a><br /><span class="o">                     </span><a href="../02-yul-expressions#YulExpression"><span class="k">YulExpression</span></a><br /><span class="o">                     </span><a href="#YulSwitchCases"><span class="k">YulSwitchCases</span></a><span class="o">;</span></pre>

```{ .ebnf #YulSwitchCases }

```

<pre ebnf-snippet="YulSwitchCases" style="display: none;"><a href="#YulSwitchCases"><span class="k">YulSwitchCases</span></a><span class="o"> = </span><a href="#YulSwitchCase"><span class="k">YulSwitchCase</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #YulSwitchCase }

```

<pre ebnf-snippet="YulSwitchCase" style="display: none;"><a href="#YulSwitchCase"><span class="k">YulSwitchCase</span></a><span class="o"> = </span><a href="#YulDefaultCase"><span class="k">YulDefaultCase</span></a><br /><span class="o">              | </span><a href="#YulValueCase"><span class="k">YulValueCase</span></a><span class="o">;</span></pre>

```{ .ebnf #YulDefaultCase }

```

<pre ebnf-snippet="YulDefaultCase" style="display: none;"><a href="#YulDefaultCase"><span class="k">YulDefaultCase</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulDefaultKeyword"><span class="k">YUL_DEFAULT_KEYWORD</span></a><br /><span class="o">                 </span><a href="#YulBlock"><span class="k">YulBlock</span></a><span class="o">;</span></pre>

```{ .ebnf #YulValueCase }

```

<pre ebnf-snippet="YulValueCase" style="display: none;"><a href="#YulValueCase"><span class="k">YulValueCase</span></a><span class="o"> = </span><a href="../03-yul-keywords#YulCaseKeyword"><span class="k">YUL_CASE_KEYWORD</span></a><br /><span class="o">               </span><a href="../02-yul-expressions#YulLiteral"><span class="k">YulLiteral</span></a><br /><span class="o">               </span><a href="#YulBlock"><span class="k">YulBlock</span></a><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/06-yul/01-yul-statements.md"
