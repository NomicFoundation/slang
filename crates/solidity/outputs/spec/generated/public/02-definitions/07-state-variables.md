<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.7. State Variables

## Syntax

```{ .ebnf #StateVariableDefinition }

```

<pre ebnf-snippet="StateVariableDefinition" style="display: none;"><a href="#StateVariableDefinition"><span class="k">StateVariableDefinition</span></a><span class="o"> = </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                          </span><a href="#StateVariableAttributes"><span class="k">StateVariableAttributes</span></a><br /><span class="o">                          </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                          </span><a href="#StateVariableDefinitionValue"><span class="k">StateVariableDefinitionValue</span></a><span class="o">?</span><br /><span class="o">                          </span><a href="../../01-file-structure/09-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #StateVariableDefinitionValue }

```

<pre ebnf-snippet="StateVariableDefinitionValue" style="display: none;"><a href="#StateVariableDefinitionValue"><span class="k">StateVariableDefinitionValue</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#Equal"><span class="k">EQUAL</span></a><br /><span class="o">                               </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">;</span></pre>

```{ .ebnf #StateVariableAttributes }

```

<pre ebnf-snippet="StateVariableAttributes" style="display: none;"><a href="#StateVariableAttributes"><span class="k">StateVariableAttributes</span></a><span class="o"> = </span><a href="#StateVariableAttribute"><span class="k">StateVariableAttribute</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #StateVariableAttribute }

```

<pre ebnf-snippet="StateVariableAttribute" style="display: none;"><a href="#StateVariableAttribute"><span class="k">StateVariableAttribute</span></a><span class="o"> = </span><a href="../08-functions#OverrideSpecifier"><span class="k">OverrideSpecifier</span></a><br /><span class="o">                       | </span><a href="../../01-file-structure/08-keywords#ConstantKeyword"><span class="k">CONSTANT_KEYWORD</span></a><br /><span class="o">                       | </span><a href="../../01-file-structure/08-keywords#InternalKeyword"><span class="k">INTERNAL_KEYWORD</span></a><br /><span class="o">                       | </span><a href="../../01-file-structure/08-keywords#PrivateKeyword"><span class="k">PRIVATE_KEYWORD</span></a><br /><span class="o">                       | </span><a href="../../01-file-structure/08-keywords#PublicKeyword"><span class="k">PUBLIC_KEYWORD</span></a><br /><span class="o">                       | </span><a href="../../01-file-structure/08-keywords#ImmutableKeyword"><span class="k">IMMUTABLE_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.6.5 *)</span></pre>

--8<-- "crates/solidity/inputs/language/docs/02-definitions/07-state-variables.md"
