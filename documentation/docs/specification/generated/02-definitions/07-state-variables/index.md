<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-emphasis -->
<!-- cSpell:disable -->

# State Variables

<div class="admonition summary">
<p class="admonition-title">Grammar</p>

<pre style="white-space: pre-wrap;"><code id="StateVariableDeclarationProduction"><span style="color: var(--md-code-hl-keyword-color);">StateVariableDeclaration</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../03-types/01-advanced-types#TypeNameProduction">TypeName</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">{</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/07-state-variables#StateVariableAttributeProduction">StateVariableAttribute</a></span><span style="color: var(--md-code-hl-operator-color);">}</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#IdentifierProduction">«Identifier»</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-string-color);">'='</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/01-base-expressions#ExpressionProduction">Expression</a></span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">';'</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="StateVariableAttributeProduction"><span style="color: var(--md-code-hl-keyword-color);">StateVariableAttribute</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/08-functions#OverrideSpecifierProduction">OverrideSpecifier</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"constant"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"immutable"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"internal"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"private"</span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-string-color);">"public"</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

</div>

--8<-- "specification/notes/02-definitions/07-state-variables/index.md"
