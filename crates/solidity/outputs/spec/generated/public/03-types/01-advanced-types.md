<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 3.1. Advanced Types

```{ .ebnf #TypeName }

```

<pre ebnf-snippet="TypeName" style="display: none;"><a href="#TypeName"><span class="k">TypeName</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#ArrayTypeName"><span class="k">ArrayTypeName</span></a><br /><span class="o">         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#FunctionType"><span class="k">FunctionType</span></a><br /><span class="o">         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#MappingType"><span class="k">MappingType</span></a><br /><span class="o">         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../02-elementary-types#ElementaryType"><span class="k">ElementaryType</span></a><br /><span class="o">         | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><span class="o">;</span></pre>

```{ .ebnf #ArrayTypeName }

```

<pre ebnf-snippet="ArrayTypeName" style="display: none;"><span class="cm">(* Postfix unary operator *)</span><br /><a href="#ArrayTypeName"><span class="k">ArrayTypeName</span></a><span class="o"> = </span><span class="cm">(* operand: *)</span><span class="o"> </span><a href="#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                </span><span class="cm">(* open_bracket: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenBracket"><span class="k">OPEN_BRACKET</span></a><br /><span class="o">                </span><span class="cm">(* index: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">?</span><br /><span class="o">                </span><span class="cm">(* close_bracket: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseBracket"><span class="k">CLOSE_BRACKET</span></a><span class="o">;</span></pre>

```{ .ebnf #FunctionType }

```

<pre ebnf-snippet="FunctionType" style="display: none;"><a href="#FunctionType"><span class="k">FunctionType</span></a><span class="o"> = </span><span class="cm">(* function_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#FunctionKeyword"><span class="k">FUNCTION_KEYWORD</span></a><br /><span class="o">               </span><span class="cm">(* parameters: *)</span><span class="o"> </span><a href="../../02-definitions/08-functions#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><br /><span class="o">               </span><span class="cm">(* attributes: *)</span><span class="o"> </span><a href="#FunctionTypeAttributes"><span class="k">FunctionTypeAttributes</span></a><br /><span class="o">               </span><span class="cm">(* returns: *)</span><span class="o"> </span><a href="../../02-definitions/08-functions#ReturnsDeclaration"><span class="k">ReturnsDeclaration</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #FunctionTypeAttributes }

```

<pre ebnf-snippet="FunctionTypeAttributes" style="display: none;"><a href="#FunctionTypeAttributes"><span class="k">FunctionTypeAttributes</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#FunctionTypeAttribute"><span class="k">FunctionTypeAttribute</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #FunctionTypeAttribute }

```

<pre ebnf-snippet="FunctionTypeAttribute" style="display: none;"><a href="#FunctionTypeAttribute"><span class="k">FunctionTypeAttribute</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#InternalKeyword"><span class="k">INTERNAL_KEYWORD</span></a><br /><span class="o">                      | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ExternalKeyword"><span class="k">EXTERNAL_KEYWORD</span></a><br /><span class="o">                      | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PrivateKeyword"><span class="k">PRIVATE_KEYWORD</span></a><br /><span class="o">                      | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PublicKeyword"><span class="k">PUBLIC_KEYWORD</span></a><br /><span class="o">                      | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ConstantKeyword"><span class="k">CONSTANT_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><span class="o">                      | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PureKeyword"><span class="k">PURE_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.4.16 *)</span><br /><span class="o">                      | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ViewKeyword"><span class="k">VIEW_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.4.16 *)</span><br /><span class="o">                      | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PayableKeyword"><span class="k">PAYABLE_KEYWORD</span></a><span class="o">;</span></pre>

```{ .ebnf #MappingType }

```

<pre ebnf-snippet="MappingType" style="display: none;"><a href="#MappingType"><span class="k">MappingType</span></a><span class="o"> = </span><span class="cm">(* mapping_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#MappingKeyword"><span class="k">MAPPING_KEYWORD</span></a><br /><span class="o">              </span><span class="cm">(* open_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenParen"><span class="k">OPEN_PAREN</span></a><br /><span class="o">              </span><span class="cm">(* key_type: *)</span><span class="o"> </span><a href="#MappingKey"><span class="k">MappingKey</span></a><br /><span class="o">              </span><span class="cm">(* equal_greater_than: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#EqualGreaterThan"><span class="k">EQUAL_GREATER_THAN</span></a><br /><span class="o">              </span><span class="cm">(* value_type: *)</span><span class="o"> </span><a href="#MappingValue"><span class="k">MappingValue</span></a><br /><span class="o">              </span><span class="cm">(* close_paren: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseParen"><span class="k">CLOSE_PAREN</span></a><span class="o">;</span></pre>

```{ .ebnf #MappingKey }

```

<pre ebnf-snippet="MappingKey" style="display: none;"><a href="#MappingKey"><span class="k">MappingKey</span></a><span class="o"> = </span><span class="cm">(* key_type: *)</span><span class="o"> </span><a href="#MappingKeyType"><span class="k">MappingKeyType</span></a><br /><span class="o">             </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">?</span><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.18 *)</span></pre>

```{ .ebnf #MappingKeyType }

```

<pre ebnf-snippet="MappingKeyType" style="display: none;"><a href="#MappingKeyType"><span class="k">MappingKeyType</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../02-elementary-types#ElementaryType"><span class="k">ElementaryType</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><span class="o">;</span></pre>

```{ .ebnf #MappingValue }

```

<pre ebnf-snippet="MappingValue" style="display: none;"><a href="#MappingValue"><span class="k">MappingValue</span></a><span class="o"> = </span><span class="cm">(* type_name: *)</span><span class="o"> </span><a href="#TypeName"><span class="k">TypeName</span></a><br /><span class="o">               </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">?</span><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.18 *)</span></pre>
