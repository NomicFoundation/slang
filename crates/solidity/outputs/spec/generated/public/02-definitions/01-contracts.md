<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 2.1. Contracts

```{ .ebnf #ContractDefinition }

```

<pre ebnf-snippet="ContractDefinition" style="display: none;"><a href="#ContractDefinition"><span class="k">ContractDefinition</span></a><span class="o"> = </span><span class="cm">(* abstract_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#AbstractKeyword"><span class="k">ABSTRACT_KEYWORD</span></a><span class="o">?</span><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">                     </span><span class="cm">(* contract_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ContractKeyword"><span class="k">CONTRACT_KEYWORD</span></a><br /><span class="o">                     </span><span class="cm">(* name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">                     </span><span class="cm">(* specifiers: *)</span><span class="o"> </span><a href="#ContractSpecifiers"><span class="k">ContractSpecifiers</span></a><br /><span class="o">                     </span><span class="cm">(* open_brace: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#OpenBrace"><span class="k">OPEN_BRACE</span></a><br /><span class="o">                     </span><span class="cm">(* members: *)</span><span class="o"> </span><a href="#ContractMembers"><span class="k">ContractMembers</span></a><br /><span class="o">                     </span><span class="cm">(* close_brace: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#CloseBrace"><span class="k">CLOSE_BRACE</span></a><span class="o">;</span></pre>

```{ .ebnf #ContractSpecifiers }

```

<pre ebnf-snippet="ContractSpecifiers" style="display: none;"><a href="#ContractSpecifiers"><span class="k">ContractSpecifiers</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#ContractSpecifier"><span class="k">ContractSpecifier</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #ContractSpecifier }

```

<pre ebnf-snippet="ContractSpecifier" style="display: none;"><a href="#ContractSpecifier"><span class="k">ContractSpecifier</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#InheritanceSpecifier"><span class="k">InheritanceSpecifier</span></a><br /><span class="o">                  | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#StorageLayoutSpecifier"><span class="k">StorageLayoutSpecifier</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.8.29 *)</span></pre>

```{ .ebnf #InheritanceSpecifier }

```

<pre ebnf-snippet="InheritanceSpecifier" style="display: none;"><a href="#InheritanceSpecifier"><span class="k">InheritanceSpecifier</span></a><span class="o"> = </span><span class="cm">(* is_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#IsKeyword"><span class="k">IS_KEYWORD</span></a><br /><span class="o">                       </span><span class="cm">(* types: *)</span><span class="o"> </span><a href="#InheritanceTypes"><span class="k">InheritanceTypes</span></a><span class="o">;</span></pre>

```{ .ebnf #InheritanceTypes }

```

<pre ebnf-snippet="InheritanceTypes" style="display: none;"><a href="#InheritanceTypes"><span class="k">InheritanceTypes</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#InheritanceType"><span class="k">InheritanceType</span></a><span class="o"> </span><span class="o">(</span><span class="cm">(* separator: *)</span><span class="o"> </span><a href="../../01-file-structure/07-punctuation#Comma"><span class="k">COMMA</span></a><span class="o"> </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#InheritanceType"><span class="k">InheritanceType</span></a><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #InheritanceType }

```

<pre ebnf-snippet="InheritanceType" style="display: none;"><a href="#InheritanceType"><span class="k">InheritanceType</span></a><span class="o"> = </span><span class="cm">(* type_name: *)</span><span class="o"> </span><a href="../../05-expressions/06-identifiers#IdentifierPath"><span class="k">IdentifierPath</span></a><br /><span class="o">                  </span><span class="cm">(* arguments: *)</span><span class="o"> </span><a href="../../05-expressions/02-function-calls#ArgumentsDeclaration"><span class="k">ArgumentsDeclaration</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #StorageLayoutSpecifier }

```

<pre ebnf-snippet="StorageLayoutSpecifier" style="display: none;"><span class="cm">(* Introduced in 0.8.29 *)</span><br /><a href="#StorageLayoutSpecifier"><span class="k">StorageLayoutSpecifier</span></a><span class="o"> = </span><span class="cm">(* layout_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#LayoutKeyword"><span class="k">LAYOUT_KEYWORD</span></a><br /><span class="o">                         </span><span class="cm">(* at_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#AtKeyword"><span class="k">AT_KEYWORD</span></a><br /><span class="o">                         </span><span class="cm">(* expression: *)</span><span class="o"> </span><a href="../../05-expressions/01-base-expressions#Expression"><span class="k">Expression</span></a><span class="o">;</span></pre>

```{ .ebnf #ContractMembers }

```

<pre ebnf-snippet="ContractMembers" style="display: none;"><a href="#ContractMembers"><span class="k">ContractMembers</span></a><span class="o"> = </span><span class="cm">(* item: *)</span><span class="o"> </span><a href="#ContractMember"><span class="k">ContractMember</span></a><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #ContractMember }

```

<pre ebnf-snippet="ContractMember" style="display: none;"><a href="#ContractMember"><span class="k">ContractMember</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/04-using-directives#UsingDirective"><span class="k">UsingDirective</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../08-functions#FunctionDefinition"><span class="k">FunctionDefinition</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../08-functions#ConstructorDefinition"><span class="k">ConstructorDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.4.22 *)</span><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../08-functions#ReceiveFunctionDefinition"><span class="k">ReceiveFunctionDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../08-functions#FallbackFunctionDefinition"><span class="k">FallbackFunctionDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.0 *)</span><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../08-functions#UnnamedFunctionDefinition"><span class="k">UnnamedFunctionDefinition</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.6.0 *)</span><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../09-modifiers#ModifierDefinition"><span class="k">ModifierDefinition</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../04-structs#StructDefinition"><span class="k">StructDefinition</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../05-enums#EnumDefinition"><span class="k">EnumDefinition</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../10-events#EventDefinition"><span class="k">EventDefinition</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../12-errors#ErrorDefinition"><span class="k">ErrorDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.8.4 *)</span><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../11-user-defined-value-types#UserDefinedValueTypeDefinition"><span class="k">UserDefinedValueTypeDefinition</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.8.8 *)</span><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../07-state-variables#StateVariableDefinition"><span class="k">StateVariableDefinition</span></a><span class="o">;</span></pre>
