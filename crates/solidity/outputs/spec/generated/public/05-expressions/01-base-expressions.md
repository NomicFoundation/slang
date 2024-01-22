<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 5.1. Base Expressions

## Syntax

```{ .ebnf #Expression }

```

<pre ebnf-snippet="Expression" style="display: none;"><a href="#Expression"><span class="k">Expression</span></a><span class="o"> = </span><a href="#AssignmentExpression"><span class="k">AssignmentExpression</span></a><br /><span class="o">           | </span><a href="#ConditionalExpression"><span class="k">ConditionalExpression</span></a><br /><span class="o">           | </span><a href="#OrExpression"><span class="k">OrExpression</span></a><br /><span class="o">           | </span><a href="#AndExpression"><span class="k">AndExpression</span></a><br /><span class="o">           | </span><a href="#EqualityExpression"><span class="k">EqualityExpression</span></a><br /><span class="o">           | </span><a href="#ComparisonExpression"><span class="k">ComparisonExpression</span></a><br /><span class="o">           | </span><a href="#BitwiseOrExpression"><span class="k">BitwiseOrExpression</span></a><br /><span class="o">           | </span><a href="#BitwiseXorExpression"><span class="k">BitwiseXorExpression</span></a><br /><span class="o">           | </span><a href="#BitwiseAndExpression"><span class="k">BitwiseAndExpression</span></a><br /><span class="o">           | </span><a href="#ShiftExpression"><span class="k">ShiftExpression</span></a><br /><span class="o">           | </span><a href="#AdditiveExpression"><span class="k">AdditiveExpression</span></a><br /><span class="o">           | </span><a href="#MultiplicativeExpression"><span class="k">MultiplicativeExpression</span></a><br /><span class="o">           | </span><a href="#ExponentiationExpression"><span class="k">ExponentiationExpression</span></a><br /><span class="o">           | </span><a href="#PostfixExpression"><span class="k">PostfixExpression</span></a><br /><span class="o">           | </span><a href="#PrefixExpression"><span class="k">PrefixExpression</span></a><br /><span class="o">           | </span><a href="#FunctionCallExpression"><span class="k">FunctionCallExpression</span></a><br /><span class="o">           | </span><a href="#MemberAccessExpression"><span class="k">MemberAccessExpression</span></a><br /><span class="o">           | </span><a href="#IndexAccessExpression"><span class="k">IndexAccessExpression</span></a><br /><span class="o">           | </span><a href="../03-primary-expressions#NewExpression"><span class="k">NewExpression</span></a><br /><span class="o">           | </span><a href="../03-primary-expressions#TupleExpression"><span class="k">TupleExpression</span></a><br /><span class="o">           | </span><a href="../03-primary-expressions#TypeExpression"><span class="k">TypeExpression</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.5.3 *)</span><br /><span class="o">           | </span><a href="../03-primary-expressions#ArrayExpression"><span class="k">ArrayExpression</span></a><br /><span class="o">           | </span><a href="../04-numbers#HexNumberExpression"><span class="k">HexNumberExpression</span></a><br /><span class="o">           | </span><a href="../04-numbers#DecimalNumberExpression"><span class="k">DecimalNumberExpression</span></a><br /><span class="o">           | </span><a href="../05-strings#StringExpression"><span class="k">StringExpression</span></a><br /><span class="o">           | </span><a href="../../03-types/02-elementary-types#ElementaryType"><span class="k">ElementaryType</span></a><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#TrueKeyword"><span class="k">TRUE_KEYWORD</span></a><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#FalseKeyword"><span class="k">FALSE_KEYWORD</span></a><br /><span class="o">           | </span><a href="../06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><span class="o">;</span></pre>

```{ .ebnf #MemberAccess }

```

<pre ebnf-snippet="MemberAccess" style="display: none;"><a href="#MemberAccess"><span class="k">MemberAccess</span></a><span class="o"> = </span><a href="../06-identifiers#Identifier"><span class="k">IDENTIFIER</span></a><br /><span class="o">             | </span><a href="../../01-file-structure/08-keywords#AddressKeyword"><span class="k">ADDRESS_KEYWORD</span></a><span class="o">;</span></pre>

```{ .ebnf #IndexAccessEnd }

```

<pre ebnf-snippet="IndexAccessEnd" style="display: none;"><a href="#IndexAccessEnd"><span class="k">IndexAccessEnd</span></a><span class="o"> = </span><a href="../../01-file-structure/09-punctuation#Colon"><span class="k">COLON</span></a><br /><span class="o">                 </span><a href="#Expression"><span class="k">Expression</span></a><span class="o">?</span><span class="o">;</span></pre>

--8<-- "crates/solidity/inputs/language/docs/05-expressions/01-base-expressions.md"
