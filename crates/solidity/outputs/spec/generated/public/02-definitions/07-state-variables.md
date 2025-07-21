<!-- This file is generated automatically by infrastructure scripts (crates/codegen/spec/src/lib.rs). Please don't edit by hand. -->

# 2.7. State Variables

```{ .ebnf #StateVariableDefinition }

```

<pre ebnf-snippet="StateVariableDefinition" style="display: none;"><a href="#StateVariableDefinition"><span class="k">StateVariableDefinition</span></a><span class="o"> = </span><span class="cm">(* type_name: *)</span><span class="o"> </span><a href="../../03-types/01-advanced-types#TypeName"><span class="k">TypeName</span></a><br /><span class="o">                          </span><span class="cm">(* attributes: *)</span><span class="o"> </span><a href="#StateVariableAttributes"><span class="k">StateVariableAttributes</span></a><br /><span class="o">                          </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                          </span><span class="cm">(* value: *)</span><span class="o"> </span><a href="#StateVariableDefinitionValue"><span class="k">StateVariableDefinitionValue</span></a><span class="o">?</span><br /><span class="o">                          </span><span class="cm">(* semicolon: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Semicolon"><span class="k">SEMICOLON</span></a><span class="o">;</span></pre>

```{ .ebnf #StateVariableDefinitionValue }

```

<pre ebnf-snippet="StateVariableDefinitionValue" style="display: none;"><a href="#StateVariableDefinitionValue"><span class="k">StateVariableDefinitionValue</span></a><span class="o"> = </span><span class="cm">(* equal: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Equal"><span class="k">EQUAL</span></a><br /><span class="o">                               </span><span class="cm">(* value: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">;</span></pre>

```{ .ebnf #StateVariableAttributes }

```

<pre ebnf-snippet="StateVariableAttributes" style="display: none;"><a href="#StateVariableAttributes"><span class="k">StateVariableAttributes</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#StateVariableAttribute"><span class="k">StateVariableAttribute</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #StateVariableAttribute }

```

<pre ebnf-snippet="StateVariableAttribute" style="display: none;"><a href="#StateVariableAttribute"><span class="k">StateVariableAttribute</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../08-functions#OverrideSpecifier"><span class="k">OverrideSpecifier</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">                       | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ConstantKeyword"><span class="k">CONSTANT_KEYWORD</span></a><br /><span class="o">                       | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#InternalKeyword"><span class="k">INTERNAL_KEYWORD</span></a><br /><span class="o">                       | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PrivateKeyword"><span class="k">PRIVATE_KEYWORD</span></a><br /><span class="o">                       | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PublicKeyword"><span class="k">PUBLIC_KEYWORD</span></a><br /><span class="o">                       | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ImmutableKeyword"><span class="k">IMMUTABLE_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.5 *)</span><br /><span class="o">                       | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#TransientKeyword"><span class="k">TRANSIENT_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.27 *)</span></pre>
