<!-- This file is generated via `cargo build`. Please don't edit by hand. -->

<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-emphasis -->
<!-- cSpell:disable -->

# Assembly Block

<div class="admonition summary">
<p class="admonition-title">Grammar</p>

<pre style="white-space: pre-wrap;"><code id="AssemblyStatementProduction"><span style="color: var(--md-code-hl-keyword-color);">AssemblyStatement</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">"assembly"</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-string-color);">'"evmasm"'</span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../06-yul/01-assembly-block#AssemblyFlagsProduction">AssemblyFlags</a></span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../06-yul/02-yul-statements#YulBlockProduction">YulBlock</a></span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="AssemblyFlagsProduction"><span style="color: var(--md-code-hl-keyword-color);">AssemblyFlags</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">'('</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/06-strings#DoubleQuotedAsciiStringLiteralProduction">«DoubleQuotedAsciiStringLiteral»</a></span><span style="color: var(--md-code-hl-operator-color);"> { </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/06-strings#DoubleQuotedAsciiStringLiteralProduction">«DoubleQuotedAsciiStringLiteral»</a></span><span style="color: var(--md-code-hl-string-color);">','</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/06-strings#DoubleQuotedAsciiStringLiteralProduction">«DoubleQuotedAsciiStringLiteral»</a></span><span style="color: var(--md-code-hl-operator-color);"> }</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">')'</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

</div>

--8<-- "specification/notes/06-yul/01-assembly-block/index.md"
