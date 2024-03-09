<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# 5.4. Numbers

## Syntax

```{ .ebnf #HexNumberExpression }

```

<pre ebnf-snippet="HexNumberExpression" style="display: none;"><a href="#HexNumberExpression"><span class="k">HexNumberExpression</span></a><span class="o"> = </span><a href="#HexLiteral"><span class="k">HEX_LITERAL</span></a><br /><span class="o">                      </span><a href="#NumberUnit"><span class="k">NumberUnit</span></a><span class="o">?</span><span class="o">;</span><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span></pre>

```{ .ebnf #DecimalNumberExpression }

```

<pre ebnf-snippet="DecimalNumberExpression" style="display: none;"><a href="#DecimalNumberExpression"><span class="k">DecimalNumberExpression</span></a><span class="o"> = </span><a href="#DecimalLiteral"><span class="k">DECIMAL_LITERAL</span></a><br /><span class="o">                          </span><a href="#NumberUnit"><span class="k">NumberUnit</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #HexLiteral }

```

<pre ebnf-snippet="HexLiteral" style="display: none;"><a href="#HexLiteral"><span class="k">HEX_LITERAL</span></a><span class="o"> = </span><span class="s2">"0x"</span><span class="o"> </span><a href="../05-strings#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o">+</span><span class="o"> </span><span class="o">(</span><span class="s2">"_"</span><span class="o"> </span><a href="../05-strings#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o">+</span><span class="o">)</span><span class="o">*</span><span class="o">;</span><br /><br /><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><a href="#HexLiteral"><span class="k">HEX_LITERAL</span></a><span class="o"> = </span><span class="s2">"0X"</span><span class="o"> </span><a href="../05-strings#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o">+</span><span class="o"> </span><span class="o">(</span><span class="s2">"_"</span><span class="o"> </span><a href="../05-strings#HexCharacter"><span class="k">«HEX_CHARACTER»</span></a><span class="o">+</span><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #DecimalLiteral }

```

<pre ebnf-snippet="DecimalLiteral" style="display: none;"><a href="#DecimalLiteral"><span class="k">DECIMAL_LITERAL</span></a><span class="o"> = </span><a href="#DecimalDigits"><span class="k">«DECIMAL_DIGITS»</span></a><span class="o"> </span><a href="#DecimalExponent"><span class="k">«DECIMAL_EXPONENT»</span></a><span class="o">?</span><span class="o">;</span><br /><br /><span class="cm">(* Deprecated in 0.5.0 *)</span><br /><a href="#DecimalLiteral"><span class="k">DECIMAL_LITERAL</span></a><span class="o"> = </span><a href="#DecimalDigits"><span class="k">«DECIMAL_DIGITS»</span></a><span class="o"> </span><span class="s2">"."</span><span class="o"> </span><a href="#DecimalExponent"><span class="k">«DECIMAL_EXPONENT»</span></a><span class="o">?</span><span class="o">;</span><br /><br /><span class="cm">(* Introduced in 0.5.0 *)</span><br /><a href="#DecimalLiteral"><span class="k">DECIMAL_LITERAL</span></a><span class="o"> = </span><a href="#DecimalDigits"><span class="k">«DECIMAL_DIGITS»</span></a><span class="o"> </span><span class="o">(</span><span class="s2">"."</span><span class="o"> </span><a href="#DecimalDigits"><span class="k">«DECIMAL_DIGITS»</span></a><span class="o">)</span><span class="o">?</span><span class="o"> </span><a href="#DecimalExponent"><span class="k">«DECIMAL_EXPONENT»</span></a><span class="o">?</span><span class="o">;</span><br /><br /><a href="#DecimalLiteral"><span class="k">DECIMAL_LITERAL</span></a><span class="o"> = </span><span class="s2">"."</span><span class="o"> </span><a href="#DecimalDigits"><span class="k">«DECIMAL_DIGITS»</span></a><span class="o"> </span><a href="#DecimalExponent"><span class="k">«DECIMAL_EXPONENT»</span></a><span class="o">?</span><span class="o">;</span><br /><br /><a href="#DecimalLiteral"><span class="k">DECIMAL_LITERAL</span></a><span class="o"> = </span><a href="#DecimalDigits"><span class="k">«DECIMAL_DIGITS»</span></a><span class="o"> </span><span class="s2">"."</span><span class="o"> </span><a href="#DecimalDigits"><span class="k">«DECIMAL_DIGITS»</span></a><span class="o"> </span><a href="#DecimalExponent"><span class="k">«DECIMAL_EXPONENT»</span></a><span class="o">?</span><span class="o">;</span></pre>

```{ .ebnf #DecimalDigits }

```

<pre ebnf-snippet="DecimalDigits" style="display: none;"><a href="#DecimalDigits"><span class="k">«DECIMAL_DIGITS»</span></a><span class="o"> = </span><span class="o">(</span><span class="s2">"0"</span><span class="o">…</span><span class="s2">"9"</span><span class="o">)</span><span class="o">+</span><span class="o"> </span><span class="o">(</span><span class="s2">"_"</span><span class="o"> </span><span class="o">(</span><span class="s2">"0"</span><span class="o">…</span><span class="s2">"9"</span><span class="o">)</span><span class="o">+</span><span class="o">)</span><span class="o">*</span><span class="o">;</span></pre>

```{ .ebnf #DecimalExponent }

```

<pre ebnf-snippet="DecimalExponent" style="display: none;"><a href="#DecimalExponent"><span class="k">«DECIMAL_EXPONENT»</span></a><span class="o"> = </span><span class="o">(</span><span class="s2">"e"</span><span class="o"> | </span><span class="s2">"E"</span><span class="o">)</span><span class="o"> </span><span class="s2">"-"</span><span class="o">?</span><span class="o"> </span><a href="#DecimalDigits"><span class="k">«DECIMAL_DIGITS»</span></a><span class="o">;</span></pre>

```{ .ebnf #NumberUnit }

```

<pre ebnf-snippet="NumberUnit" style="display: none;"><a href="#NumberUnit"><span class="k">NumberUnit</span></a><span class="o"> = </span><a href="../../01-file-structure/08-keywords#WeiKeyword"><span class="k">WEI_KEYWORD</span></a><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#GweiKeyword"><span class="k">GWEI_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Introduced in 0.6.11 *)</span><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#SzaboKeyword"><span class="k">SZABO_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.7.0 *)</span><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#FinneyKeyword"><span class="k">FINNEY_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.7.0 *)</span><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#EtherKeyword"><span class="k">ETHER_KEYWORD</span></a><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#SecondsKeyword"><span class="k">SECONDS_KEYWORD</span></a><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#MinutesKeyword"><span class="k">MINUTES_KEYWORD</span></a><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#HoursKeyword"><span class="k">HOURS_KEYWORD</span></a><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#DaysKeyword"><span class="k">DAYS_KEYWORD</span></a><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#WeeksKeyword"><span class="k">WEEKS_KEYWORD</span></a><br /><span class="o">           | </span><a href="../../01-file-structure/08-keywords#YearsKeyword"><span class="k">YEARS_KEYWORD</span></a><span class="o">;</span><span class="o"> </span><span class="cm">(* Deprecated in 0.5.0 *)</span></pre>

--8<-- "crates/solidity/inputs/language/docs/05-expressions/04-numbers.md"
