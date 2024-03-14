<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.9. Modifiers

## Syntax

```{ .ebnf #ModifierDefinition }

```

<pre ebnf-snippet="ModifierDefinition" style="display: none;"><a href="#ModifierDefinition"><span class="k">ModifierDefinition</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#ModifierKeyword"><span class="k">MODIFIER_KEYWORD</span></a><br /><span class="o">                     </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                     </span><a href="../08-functions#ParametersDeclaration"><span class="k">ParametersDeclaration</span></a><span class="o">?</span><br /><span class="o">                     </span><a href="#ModifierAttributes"><span class="k">ModifierAttributes</span></a><br /><span class="o">                     </span><a href="../08-functions#FunctionBody"><span class="k">FunctionBody</span></a><span class="o">;</span></pre>

```{ .ebnf #ModifierAttributes }

```

<pre ebnf-snippet="ModifierAttributes" style="display: none;"><a href="#ModifierAttributes"><span class="k">ModifierAttributes</span></a><span class="o"> = </span><a href="#ModifierAttribute"><span class="k">ModifierAttribute</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #ModifierAttribute }

```

<pre ebnf-snippet="ModifierAttribute" style="display: none;"><a href="#ModifierAttribute"><span class="k">ModifierAttribute</span></a><span class="o"> = </span><a href="../08-functions#OverrideSpecifier"><span class="k">OverrideSpecifier</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">                  | </span><a href="../../01-file-structure/08-keywords#VirtualKeyword"><span class="k">VIRTUAL_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span></pre>

```{ .ebnf #ModifierInvocation }

```

<pre ebnf-snippet="ModifierInvocation" style="display: none;"><a href="#ModifierInvocation"><span class="k">ModifierInvocation</span></a><span class="o"> = </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><br /><span class="o">                     </span><a href="../../05-expressions/02-function-calls#ArgumentsDeclaration"><span class="k">ArgumentsDeclaration</span></a><span class="o">?</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/02-definitions/09-modifiers.md"
