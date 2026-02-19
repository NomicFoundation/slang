<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.8. Functions

```{ .ebnf #FunctionDefinition }

```

<pre ebnf-snippet="FunctionDefinition" style="display: none;"><a href="#FunctionDefinition"><span class="k">FunctionDefinition</span></a><span class="o"> = </span><span class="cm">(* function_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#FunctionKeyword"><span class="k">FUNCTION_KEYWORD</span></a><br /><span class="o">                     </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="#FunctionName"><span class="k">FunctionName</span></a><br /><span class="o">                     </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><br /><span class="o">                     </span><span class="cm">(* attributes: *)</span><span class="o"> </span><a href="#FunctionAttributes"><span class="k">FunctionAttributes</span></a><br /><span class="o">                     </span><span class="cm">(* returns: *)</span><span class="o"> </span><a href="#ReturnsDeclaration"><span class="k">ReturnsDeclaration</span></a><span class="o">?</span><br /><span class="o">                     </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="#FunctionBody"><span class="k">FunctionBody</span></a><span class="o">;</span></pre>

```{ .ebnf #FunctionName }

```

<pre ebnf-snippet="FunctionName" style="display: none;"><a href="#FunctionName"><span class="k">FunctionName</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">             | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#FallbackKeyword"><span class="k">FALLBACK_KEYWORD</span></a><br /><span class="o">             | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ReceiveKeyword"><span class="k">RECEIVE_KEYWORD</span></a><span class="o">;</span></pre>

```{ .ebnf #ParametersDeclaration }

```

<pre ebnf-snippet="ParametersDeclaration" style="display: none;"><a href="#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><span class="o"> = </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                        </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="#Parameters"><span class="k">Parameters</span></a><br /><span class="o">                        </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #Parameters }

```

<pre ebnf-snippet="Parameters" style="display: none;"><a href="#Parameters"><span class="k">Parameters</span></a><span class="o"> = </span><span class="o">(</span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#Parameter"><span class="k">Parameter</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#Parameter"><span class="k">Parameter</span></a><span class="o">)</span><span class="o">*</span><span class="o">)</span><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #Parameter }

```

<pre ebnf-snippet="Parameter" style="display: none;"><a href="#Parameter"><span class="k">Parameter</span></a><span class="o"> = </span><span class="cm">(* type_name: *)</span><span class="o"> </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">            </span><span class="cm">(* storage_location: *)</span><span class="o"> </span><a href="../../04-statements/02-declaration-statements#StorageLocation"><span class="k">StorageLocation</span></a><span class="o">?</span><br /><span class="o">            </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #FunctionAttributes }

```

<pre ebnf-snippet="FunctionAttributes" style="display: none;"><a href="#FunctionAttributes"><span class="k">FunctionAttributes</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#FunctionAttribute"><span class="k">FunctionAttribute</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #FunctionAttribute }

```

<pre ebnf-snippet="FunctionAttribute" style="display: none;"><a href="#FunctionAttribute"><span class="k">FunctionAttribute</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../09-modifiers#ModifierInvocation"><span class="k">ModifierInvocation</span></a><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#OverrideSpecifier"><span class="k">OverrideSpecifier</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ConstantKeyword"><span class="k">CONSTANT_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ExternalKeyword"><span class="k">EXTERNAL_KEYWORD</span></a><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#InternalKeyword"><span class="k">INTERNAL_KEYWORD</span></a><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PayableKeyword"><span class="k">PAYABLE_KEYWORD</span></a><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PrivateKeyword"><span class="k">PRIVATE_KEYWORD</span></a><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PublicKeyword"><span class="k">PUBLIC_KEYWORD</span></a><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PureKeyword"><span class="k">PURE_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.4.16 *)</span><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ViewKeyword"><span class="k">VIEW_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.4.16 *)</span><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#VirtualKeyword"><span class="k">VIRTUAL_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span></pre>

```{ .ebnf #OverrideSpecifier }

```

<pre ebnf-snippet="OverrideSpecifier" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#OverrideSpecifier"><span class="k">OverrideSpecifier</span></a><span class="o"> = </span><span class="cm">(* override_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#OverrideKeyword"><span class="k">OVERRIDE_KEYWORD</span></a><br /><span class="o">                    </span><span class="cm">(* overridden: *)</span><span class="o"> </span><a href="#OverridePathsDeclaration"><span class="k">OverridePathsDeclaration</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #OverridePathsDeclaration }

```

<pre ebnf-snippet="OverridePathsDeclaration" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#OverridePathsDeclaration"><span class="k">OverridePathsDeclaration</span></a><span class="o"> = </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">                           </span><span class="cm">(* paths: *)</span><span class="o"> </span><a href="#OverridePaths"><span class="k">OverridePaths</span></a><br /><span class="o">                           </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #OverridePaths }

```

<pre ebnf-snippet="OverridePaths" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#OverridePaths"><span class="k">OverridePaths</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #ReturnsDeclaration }

```

<pre ebnf-snippet="ReturnsDeclaration" style="display: none;"><a href="#ReturnsDeclaration"><span class="k">ReturnsDeclaration</span></a><span class="o"> = </span><span class="cm">(* returns_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ReturnsKeyword"><span class="k">RETURNS_KEYWORD</span></a><br /><span class="o">                     </span><span class="cm">(* variables: *)</span><span class="o"> </span><a href="#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><span class="o">;</span></pre>

```{ .ebnf #FunctionBody }

```

<pre ebnf-snippet="FunctionBody" style="display: none;"><a href="#FunctionBody"><span class="k">FunctionBody</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../04-statements/01-blocks#Block"><span class="k">Block</span></a><br /><span class="o">             | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #ConstructorDefinition }

```

<pre ebnf-snippet="ConstructorDefinition" style="display: none;"><span class="cm">(* Introduced in 0.4.22 *)</span><br /><a href="#ConstructorDefinition"><span class="k">ConstructorDefinition</span></a><span class="o"> = </span><span class="cm">(* constructor_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ConstructorKeyword"><span class="k">CONSTRUCTOR_KEYWORD</span></a><br /><span class="o">                        </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><br /><span class="o">                        </span><span class="cm">(* attributes: *)</span><span class="o"> </span><a href="#ConstructorAttributes"><span class="k">ConstructorAttributes</span></a><br /><span class="o">                        </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="../../04-statements/01-blocks#Block"><span class="k">Block</span></a><span class="o">;</span></pre>

```{ .ebnf #ConstructorAttributes }

```

<pre ebnf-snippet="ConstructorAttributes" style="display: none;"><span class="cm">(* Introduced in 0.4.22 *)</span><br /><a href="#ConstructorAttributes"><span class="k">ConstructorAttributes</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#ConstructorAttribute"><span class="k">ConstructorAttribute</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #ConstructorAttribute }

```

<pre ebnf-snippet="ConstructorAttribute" style="display: none;"><span class="cm">(* Introduced in 0.4.22 *)</span><br /><a href="#ConstructorAttribute"><span class="k">ConstructorAttribute</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../09-modifiers#ModifierInvocation"><span class="k">ModifierInvocation</span></a><br /><span class="o">                     | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#InternalKeyword"><span class="k">INTERNAL_KEYWORD</span></a><br /><span class="o">                     | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#OverrideKeyword"><span class="k">OVERRIDE_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 and deprecated in 0.6.7. *)</span><br /><span class="o">                     | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PayableKeyword"><span class="k">PAYABLE_KEYWORD</span></a><br /><span class="o">                     | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PublicKeyword"><span class="k">PUBLIC_KEYWORD</span></a><br /><span class="o">                     | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#VirtualKeyword"><span class="k">VIRTUAL_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 and deprecated in 0.6.7. *)</span></pre>

```{ .ebnf #UnnamedFunctionDefinition }

```

<pre ebnf-snippet="UnnamedFunctionDefinition" style="display: none;"><span class="cm">(* Deprecated in 0.6.0 *)</span><br /><a href="#UnnamedFunctionDefinition"><span class="k">UnnamedFunctionDefinition</span></a><span class="o"> = </span><span class="cm">(* function_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#FunctionKeyword"><span class="k">FUNCTION_KEYWORD</span></a><br /><span class="o">                            </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><br /><span class="o">                            </span><span class="cm">(* attributes: *)</span><span class="o"> </span><a href="#UnnamedFunctionAttributes"><span class="k">UnnamedFunctionAttributes</span></a><br /><span class="o">                            </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="#FunctionBody"><span class="k">FunctionBody</span></a><span class="o">;</span></pre>

```{ .ebnf #UnnamedFunctionAttributes }

```

<pre ebnf-snippet="UnnamedFunctionAttributes" style="display: none;"><span class="cm">(* Deprecated in 0.6.0 *)</span><br /><a href="#UnnamedFunctionAttributes"><span class="k">UnnamedFunctionAttributes</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#UnnamedFunctionAttribute"><span class="k">UnnamedFunctionAttribute</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #UnnamedFunctionAttribute }

```

<pre ebnf-snippet="UnnamedFunctionAttribute" style="display: none;"><span class="cm">(* Deprecated in 0.6.0 *)</span><br /><a href="#UnnamedFunctionAttribute"><span class="k">UnnamedFunctionAttribute</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../09-modifiers#ModifierInvocation"><span class="k">ModifierInvocation</span></a><br /><span class="o">                         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ExternalKeyword"><span class="k">EXTERNAL_KEYWORD</span></a><br /><span class="o">                         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#InternalKeyword"><span class="k">INTERNAL_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><span class="o">                         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PayableKeyword"><span class="k">PAYABLE_KEYWORD</span></a><br /><span class="o">                         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PrivateKeyword"><span class="k">PRIVATE_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><span class="o">                         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PublicKeyword"><span class="k">PUBLIC_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span></pre>

```{ .ebnf #FallbackFunctionDefinition }

```

<pre ebnf-snippet="FallbackFunctionDefinition" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#FallbackFunctionDefinition"><span class="k">FallbackFunctionDefinition</span></a><span class="o"> = </span><span class="cm">(* fallback_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#FallbackKeyword"><span class="k">FALLBACK_KEYWORD</span></a><br /><span class="o">                             </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><br /><span class="o">                             </span><span class="cm">(* attributes: *)</span><span class="o"> </span><a href="#FallbackFunctionAttributes"><span class="k">FallbackFunctionAttributes</span></a><br /><span class="o">                             </span><span class="cm">(* returns: *)</span><span class="o"> </span><a href="#ReturnsDeclaration"><span class="k">ReturnsDeclaration</span></a><span class="o">?</span><br /><span class="o">                             </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="#FunctionBody"><span class="k">FunctionBody</span></a><span class="o">;</span></pre>

```{ .ebnf #FallbackFunctionAttributes }

```

<pre ebnf-snippet="FallbackFunctionAttributes" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#FallbackFunctionAttributes"><span class="k">FallbackFunctionAttributes</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#FallbackFunctionAttribute"><span class="k">FallbackFunctionAttribute</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #FallbackFunctionAttribute }

```

<pre ebnf-snippet="FallbackFunctionAttribute" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#FallbackFunctionAttribute"><span class="k">FallbackFunctionAttribute</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../09-modifiers#ModifierInvocation"><span class="k">ModifierInvocation</span></a><br /><span class="o">                          | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#OverrideSpecifier"><span class="k">OverrideSpecifier</span></a><br /><span class="o">                          | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ExternalKeyword"><span class="k">EXTERNAL_KEYWORD</span></a><br /><span class="o">                          | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PayableKeyword"><span class="k">PAYABLE_KEYWORD</span></a><br /><span class="o">                          | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PureKeyword"><span class="k">PURE_KEYWORD</span></a><br /><span class="o">                          | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ViewKeyword"><span class="k">VIEW_KEYWORD</span></a><br /><span class="o">                          | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#VirtualKeyword"><span class="k">VIRTUAL_KEYWORD</span></a><span class="o">;</span></pre>

```{ .ebnf #ReceiveFunctionDefinition }

```

<pre ebnf-snippet="ReceiveFunctionDefinition" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#ReceiveFunctionDefinition"><span class="k">ReceiveFunctionDefinition</span></a><span class="o"> = </span><span class="cm">(* receive_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ReceiveKeyword"><span class="k">RECEIVE_KEYWORD</span></a><br /><span class="o">                            </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><br /><span class="o">                            </span><span class="cm">(* attributes: *)</span><span class="o"> </span><a href="#ReceiveFunctionAttributes"><span class="k">ReceiveFunctionAttributes</span></a><br /><span class="o">                            </span><span class="cm">(* body: *)</span><span class="o"> </span><a href="#FunctionBody"><span class="k">FunctionBody</span></a><span class="o">;</span></pre>

```{ .ebnf #ReceiveFunctionAttributes }

```

<pre ebnf-snippet="ReceiveFunctionAttributes" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#ReceiveFunctionAttributes"><span class="k">ReceiveFunctionAttributes</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#ReceiveFunctionAttribute"><span class="k">ReceiveFunctionAttribute</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #ReceiveFunctionAttribute }

```

<pre ebnf-snippet="ReceiveFunctionAttribute" style="display: none;"><span class="cm">(* Introduced in 0.6.0 *)</span><br /><a href="#ReceiveFunctionAttribute"><span class="k">ReceiveFunctionAttribute</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../09-modifiers#ModifierInvocation"><span class="k">ModifierInvocation</span></a><br /><span class="o">                         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#OverrideSpecifier"><span class="k">OverrideSpecifier</span></a><br /><span class="o">                         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ExternalKeyword"><span class="k">EXTERNAL_KEYWORD</span></a><br /><span class="o">                         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PayableKeyword"><span class="k">PAYABLE_KEYWORD</span></a><br /><span class="o">                         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#VirtualKeyword"><span class="k">VIRTUAL_KEYWORD</span></a><span class="o">;</span></pre>
