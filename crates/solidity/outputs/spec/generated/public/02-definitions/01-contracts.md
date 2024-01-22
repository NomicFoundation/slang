<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.1. Contracts

## Syntax

```{ .ebnf #ContractDefinition }

```

<pre ebnf-snippet="ContractDefinition" style="display: none;"><a href="#ContractDefinition"><span class="k">ContractDefinition</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#AbstractKeyword"><span class="k">ABSTRACT_KEYWORD</span></a><span class="o">?</span><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">                     </span><a href="../../01-file-structure/08-keywords#ContractKeyword"><span class="k">CONTRACT_KEYWORD</span></a><br /><span class="o">                     </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                     </span><a href="#InheritanceSpecifier"><span class="k">InheritanceSpecifier</span></a><span class="o">?</span><br /><span class="o">                     </span><a href="../../01-file-structure/09-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                     </span><a href="#ContractMembers"><span class="k">ContractMembers</span></a><span class="o">?</span><br /><span class="o">                     </span><a href="../../01-file-structure/09-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #InheritanceSpecifier }

```

<pre ebnf-snippet="InheritanceSpecifier" style="display: none;"><a href="#InheritanceSpecifier"><span class="k">InheritanceSpecifier</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#IsKeyword"><span class="k">IS_KEYWORD</span></a><br /><span class="o">                       </span><a href="#InheritanceTypes"><span class="k">InheritanceTypes</span></a><span class="o">;</span></pre>

```{ .ebnf #InheritanceTypes }

```

<pre ebnf-snippet="InheritanceTypes" style="display: none;"><a href="#InheritanceTypes"><span class="k">InheritanceTypes</span></a><span class="o"> = </span><a href="#InheritanceType"><span class="k">InheritanceType</span></a><span class="o"> </span><span class="o">(</span><a href="../../01-file-structure/09-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><a href="#InheritanceType"><span class="k">InheritanceType</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #InheritanceType }

```

<pre ebnf-snippet="InheritanceType" style="display: none;"><a href="#InheritanceType"><span class="k">InheritanceType</span></a><span class="o"> = </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><br /><span class="o">                  </span><a href="../../05-expressions/02-function-calls#ArgumentsDeclaration"><span class="k">ArgumentsDeclaration</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #ContractMembers }

```

<pre ebnf-snippet="ContractMembers" style="display: none;"><a href="#ContractMembers"><span class="k">ContractMembers</span></a><span class="o"> = </span><a href="#ContractMember"><span class="k">ContractMember</span></a><span class="o">+</span><span class="o">;</span></pre>

```{ .ebnf #ContractMember }

```

<pre ebnf-snippet="ContractMember" style="display: none;"><a href="#ContractMember"><span class="k">ContractMember</span></a><span class="o"> = </span><a href="../../01-file-structure/05-using-directives#UsingDirective"><span class="k">UsingDirective</span></a><br /><span class="o">               | </span><a href="../08-functions#FunctionDefinition"><span class="k">FunctionDefinition</span></a><br /><span class="o">               | </span><a href="../08-functions#ConstructorDefinition"><span class="k">ConstructorDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.4.22 *)</span><br /><span class="o">               | </span><a href="../08-functions#ReceiveFunctionDefinition"><span class="k">ReceiveFunctionDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">               | </span><a href="../08-functions#FallbackFunctionDefinition"><span class="k">FallbackFunctionDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">               | </span><a href="../08-functions#UnnamedFunctionDefinition"><span class="k">UnnamedFunctionDefinition</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.6.0 *)</span><br /><span class="o">               | </span><a href="../09-modifiers#ModifierDefinition"><span class="k">ModifierDefinition</span></a><br /><span class="o">               | </span><a href="../04-structs#StructDefinition"><span class="k">StructDefinition</span></a><br /><span class="o">               | </span><a href="../05-enums#EnumDefinition"><span class="k">EnumDefinition</span></a><br /><span class="o">               | </span><a href="../10-events#EventDefinition"><span class="k">EventDefinition</span></a><br /><span class="o">               | </span><a href="../07-state-variables#StateVariableDefinition"><span class="k">StateVariableDefinition</span></a><br /><span class="o">               | </span><a href="../12-errors#ErrorDefinition"><span class="k">ErrorDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.8.4 *)</span><br /><span class="o">               | </span><a href="../11-user-defined-value-types#UserDefinedValueTypeDefinition"><span class="k">UserDefinedValueTypeDefinition</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.8 *)</span></pre>

--8<-- "crates/solidity/inputs/language/docs/02-definitions/01-contracts.md"
