<!-- This file is generated via `cargo build`. Please don't edit by hand. -->

<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-emphasis -->
<!-- cSpell:disable -->

# Errors

<div class="admonition summary">
<p class="admonition-title">Grammar</p>

<pre style="white-space: pre-wrap;"><code id="ErrorDefinitionProduction"><span style="color: var(--md-code-hl-keyword-color);">ErrorDefinition</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">"error"</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#IdentifierProduction">«Identifier»</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">'('</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/12-errors#ErrorParameterProduction">ErrorParameter</a></span><span style="color: var(--md-code-hl-operator-color);"> { </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/12-errors#ErrorParameterProduction">ErrorParameter</a></span><span style="color: var(--md-code-hl-string-color);">','</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/12-errors#ErrorParameterProduction">ErrorParameter</a></span><span style="color: var(--md-code-hl-operator-color);"> }</span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">')'</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">';'</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code id="ErrorParameterProduction"><span style="color: var(--md-code-hl-keyword-color);">ErrorParameter</span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../03-types/01-advanced-types#TypeNameProduction">TypeName</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[</span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/07-identifiers#IdentifierProduction">«Identifier»</a></span><span style="color: var(--md-code-hl-operator-color);">]</span><span style="color: var(--md-code-hl-operator-color);">;</span><br/></code></pre>

</div>

--8<-- "specification/notes/02-definitions/12-errors/index.md"
