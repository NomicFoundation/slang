<!-- This file is generated automatically by infrastructure scripts (crates/codegen/spec/src/lib.rs). Please don't edit by hand. -->

# 3.2. Elementary Types

```{ .ebnf #ElementaryType }

```

<pre ebnf-snippet="ElementaryType" style="display: none;"><a href="#ElementaryType"><span class="k">ElementaryType</span></a><span class="o"> = </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#BoolKeyword"><span class="k">BOOL_KEYWORD</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#ByteKeyword"><span class="k">BYTE_KEYWORD</span></a><span class="o"> </span><span class="cm">(* Deprecated in 0.8.0 *)</span><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#StringKeyword"><span class="k">STRING_KEYWORD</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="#AddressType"><span class="k">AddressType</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#BytesKeyword"><span class="k">BYTES_KEYWORD</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#IntKeyword"><span class="k">INT_KEYWORD</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#UintKeyword"><span class="k">UINT_KEYWORD</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#FixedKeyword"><span class="k">FIXED_KEYWORD</span></a><br /><span class="o">               | </span><span class="cm">(* variant: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#UfixedKeyword"><span class="k">UFIXED_KEYWORD</span></a><span class="o">;</span></pre>

```{ .ebnf #AddressType }

```

<pre ebnf-snippet="AddressType" style="display: none;"><a href="#AddressType"><span class="k">AddressType</span></a><span class="o"> = </span><span class="cm">(* address_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#AddressKeyword"><span class="k">ADDRESS_KEYWORD</span></a><br /><span class="o">              </span><span class="cm">(* payable_keyword: *)</span><span class="o"> </span><a href="../../01-file-structure/06-keywords#PayableKeyword"><span class="k">PAYABLE_KEYWORD</span></a><span class="o">?</span><span class="o">;</span><span class="o"> </span><span class="cm">(* Introduced in 0.5.0 *)</span></pre>
