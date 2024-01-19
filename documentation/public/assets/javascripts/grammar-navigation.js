window.addEventListener("load", () => {
  populateCodeBlocks();
  highlightSelectedCodeBlock();
});

window.addEventListener("hashchange", () => {
  highlightSelectedCodeBlock();
});

// Markdown code blocks don't support links between identifiers.
// So in order to do that, we generate the links into a hidden element,
// then after page load, we inject this snippet's contents into the code block.
function populateCodeBlocks() {
  for (const snippetsPre of document.querySelectorAll("pre[ebnf-snippet]")) {
    const id = snippetsPre.getAttribute("ebnf-snippet");

    if (id) {
      const codeBlock = document.getElementById(id)?.querySelector("code");

      if (codeBlock) {
        codeBlock.replaceChildren(...snippetsPre.childNodes);
        snippetsPre.remove();
      }
    }
  }
}

// Make sure the currently selected code block is highlighted.
function highlightSelectedCodeBlock() {
  // First, reset all code blocks to their default background color:
  for (const codeBlock of document.querySelectorAll("code")) {
    codeBlock.style.background = "var(--md-code-bg-color)";
  }

  // Second, if one was selected (via location.hash), highlight it:
  const id = location.hash.replace("#", "");
  const codeBlock = document.getElementById(id)?.querySelector("code");

  if (codeBlock) {
    codeBlock.style.background = "var(--md-code-hl-color)";
  }
}
