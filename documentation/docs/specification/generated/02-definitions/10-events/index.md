<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-emphasis -->
<!-- cSpell:disable -->

# Events

<div class="admonition summary">
<p class="admonition-title">Grammar</p>

<pre style="white-space: pre-wrap;"><code><span style="color: var(--md-code-hl-keyword-color);"><span id="EventDefinitionProduction">EventDefinition</span></span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-string-color);">"event"</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-identifiers#IdentifierProduction">«Identifier»</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">'('</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[ </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/10-events#EventParameterProduction">EventParameter</a></span><span style="color: var(--md-code-hl-operator-color);"> { </span><span style="color: var(--md-code-hl-string-color);">','</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/10-events#EventParameterProduction">EventParameter</a></span><span style="color: var(--md-code-hl-operator-color);"> }</span><span style="color: var(--md-code-hl-operator-color);"> ]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">')'</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[ </span><span style="color: var(--md-code-hl-string-color);">"anonymous"</span><span style="color: var(--md-code-hl-operator-color);"> ]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">';'</span><span style="color: var(--md-code-hl-operator-color);"> ;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code><span style="color: var(--md-code-hl-keyword-color);"><span id="EventParameterProduction">EventParameter</span></span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../03-types/01-advanced-types#TypeNameProduction">TypeName</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[ </span><span style="color: var(--md-code-hl-string-color);">"indexed"</span><span style="color: var(--md-code-hl-operator-color);"> ]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[ </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-identifiers#IdentifierProduction">«Identifier»</a></span><span style="color: var(--md-code-hl-operator-color);"> ]</span><span style="color: var(--md-code-hl-operator-color);"> ;</span><br/></code></pre>

</div>

--8<-- "specification/notes/02-definitions/10-events/index.md"
