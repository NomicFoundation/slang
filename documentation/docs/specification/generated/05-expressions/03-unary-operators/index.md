<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-emphasis -->
<!-- cSpell:disable -->

# Unary Operators

<div class="admonition summary">
<p class="admonition-title">Grammar</p>

<pre style="white-space: pre-wrap;"><code id="UnarySuffixExpressionProduction"><span style="color: var(--md-code-hl-keyword-color);">UnarySuffixExpression</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/01-base-expressions#ExpressionProduction">Expression</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">(</span><span style="color: var(--md-code-hl-string-color);">"++"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"--"</span><span style="color: var(--md-code-hl-operator-color);">)</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="UnaryPrefixExpressionProduction"><span style="color: var(--md-code-hl-keyword-color);">UnaryPrefixExpression</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-operator-color);">(</span><span style="color: var(--md-code-hl-string-color);">"++"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"--"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">'!'</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">'~'</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">'-'</span><span style="color: var(--md-code-hl-operator-color);">)</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/01-base-expressions#ExpressionProduction">Expression</a></span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

</div>

--8<-- "specification/notes/05-expressions/03-unary-operators/index.md"
