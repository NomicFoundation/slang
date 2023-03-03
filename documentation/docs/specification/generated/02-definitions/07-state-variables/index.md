<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-space-in-emphasis -->
<!-- cSpell:disable -->

# State Variables

<div class="admonition summary">
<p class="admonition-title">Grammar</p>

<pre style="white-space: pre-wrap;"><code><span style="color: var(--md-code-hl-keyword-color);"><span id="StateVariableDeclarationProduction">StateVariableDeclaration</span></span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../03-types/01-advanced-types#TypeNameProduction">TypeName</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">{ </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/07-state-variables#StateVariableAttributeProduction">StateVariableAttribute</a></span><span style="color: var(--md-code-hl-operator-color);"> }</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/05-identifiers#IdentifierProduction">«Identifier»</a></span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-operator-color);">[ </span><span style="color: var(--md-code-hl-string-color);">'='</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../05-expressions/01-base-expressions#ExpressionProduction">Expression</a></span><span style="color: var(--md-code-hl-operator-color);"> ]</span><span style="color: var(--md-code-hl-operator-color);"> </span><span style="color: var(--md-code-hl-string-color);">';'</span><span style="color: var(--md-code-hl-operator-color);"> ;</span><br/></code></pre>

<pre style="white-space: pre-wrap;"><code><span style="color: var(--md-code-hl-keyword-color);"><span id="StateVariableAttributeProduction">StateVariableAttribute</span></span><span style="color: var(--md-code-hl-operator-color);"> = </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../02-definitions/08-functions#OverrideSpecifierProduction">OverrideSpecifier</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../01-file-structure/07-keywords#ConstantKeywordProduction">«ConstantKeyword»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../01-file-structure/07-keywords#ImmutableKeywordProduction">«ImmutableKeyword»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../01-file-structure/07-keywords#InternalKeywordProduction">«InternalKeyword»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../01-file-structure/07-keywords#PrivateKeywordProduction">«PrivateKeyword»</a></span><span style="color: var(--md-code-hl-operator-color);"> | </span><span style="color: var(--md-code-hl-keyword-color);"><a href="../../01-file-structure/07-keywords#PublicKeywordProduction">«PublicKeyword»</a></span><span style="color: var(--md-code-hl-operator-color);"> ;</span><br/></code></pre>

</div>

--8<-- "specification/notes/02-definitions/07-state-variables/index.md"
