<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 1.2. Source Unit

## Syntax

```{ .ebnf #SourceUnit }

```

<pre ebnf-snippet="SourceUnit" style="display: none;"><a href="#SourceUnit"><span class="k">SourceUnit</span></a><span class="o"> = </span><a href="#SourceUnitMembers"><span class="k">SourceUnitMembers</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #SourceUnitMembers }

```

<pre ebnf-snippet="SourceUnitMembers" style="display: none;"><a href="#SourceUnitMembers"><span class="k">SourceUnitMembers</span></a><span class="o"> = </span><a href="#SourceUnitMember"><span class="k">SourceUnitMember</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #SourceUnitMember }

```

<pre ebnf-snippet="SourceUnitMember" style="display: none;"><a href="#SourceUnitMember"><span class="k">SourceUnitMember</span></a><span class="o"> = </span><a href="../03-pragma-directives#PragmaDirective"><span class="k">PragmaDirective</span></a><br /><span class="o">                 | </span><a href="../04-import-directives#ImportDirective"><span class="k">ImportDirective</span></a><br /><span class="o">                 | </span><a href="../../02-definitions/01-contracts#ContractDefinition"><span class="k">ContractDefinition</span></a><br /><span class="o">                 | </span><a href="../../02-definitions/02-interfaces#InterfaceDefinition"><span class="k">InterfaceDefinition</span></a><br /><span class="o">                 | </span><a href="../../02-definitions/03-libraries#LibraryDefinition"><span class="k">LibraryDefinition</span></a><br /><span class="o">                 | </span><a href="../../02-definitions/04-structs#StructDefinition"><span class="k">StructDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">                 | </span><a href="../../02-definitions/05-enums#EnumDefinition"><span class="k">EnumDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">                 | </span><a href="../../02-definitions/08-functions#FunctionDefinition"><span class="k">FunctionDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.7.1 *)</span><br /><span class="o">                 | </span><a href="../../02-definitions/06-constants#ConstantDefinition"><span class="k">ConstantDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.7.4 *)</span><br /><span class="o">                 | </span><a href="../../02-definitions/12-errors#ErrorDefinition"><span class="k">ErrorDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.8.4 *)</span><br /><span class="o">                 | </span><a href="../../02-definitions/11-user-defined-value-types#UserDefinedValueTypeDefinition"><span class="k">UserDefinedValueTypeDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.8.8 *)</span><br /><span class="o">                 | </span><a href="../05-using-directives#UsingDirective"><span class="k">UsingDirective</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.8.13 *)</span><br /><span class="o">                 | </span><a href="../../02-definitions/10-events#EventDefinition"><span class="k">EventDefinition</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.22 *)</span></pre>

--8<-- "crates/solidity/inputs/language/docs/01-file-structure/02-source-unit.md"
