window.addEventListener("load", () => {
  const productionDivs = collectProductionDivs();
  createLinksToKeywords(productionDivs);
  highlightSelectedKeyword(productionDivs);
});

window.addEventListener("hashchange", () => {
  const productionDivs = collectProductionDivs();
  highlightSelectedKeyword(productionDivs);
});

function collectProductionDivs() {
  const productionDivs = /** @type Map<string, Element> */ (new Map());
  for (const productionDiv of document.querySelectorAll("div .slang-ebnf")) {
    const id = productionDiv.getAttribute("id");
    if (!id) {
      continue;
    }

    productionDivs.set(id, productionDiv);
  }

  return productionDivs;
}

function createLinksToKeywords(/** @type Map<string, Element> */ productionDivs) {
  for (const [_, productionDiv] of productionDivs) {
    const spans = productionDiv.querySelectorAll("span[class='k']");
    for (const span of spans) {
      const spanContents = span.textContent || "";
      if (spanContents.trim().length == 0) {
        continue; // empty string
      }

      // Keyword spans can have multiple keywords (and spaces). For example:
      //        " IDENTIFIER ParameterList"
      // We need to split them up and replace each one individually with an anchor.
      // Splitting the above by word boundary "\b" produces the following:
      //        [" ", "IDENTIFIER", " ", "ParameterList"]

      span.replaceWith(
        ...spanContents.split(/\b/).map((token) => {
          const newSpan = /** @type Element */ (span.cloneNode(true));
          newSpan.textContent = token;

          if (productionDivs.has(token)) {
            // token is a keyword: wrap it with a new anchor link around it:
            const anchor = document.createElement("a");
            anchor.setAttribute("href", `#${token}`);
            anchor.appendChild(newSpan);
            return anchor;
          } else {
            // token is whitespace: return as-is:
            return newSpan;
          }
        }),
      );
    }
  }
}

function highlightSelectedKeyword(/** @type Map<string, Element> */ productionDivs) {
  // First, reset all code blocks to their default background color:

  for (const [_, productionDiv] of productionDivs) {
    setBackground(productionDiv, "var(--md-code-bg-color)");
  }

  // Then, if one was selected (location.hash), highlight it:

  const keyword = location.hash.replace("#", "");
  const selectedProductionDiv = productionDivs.get(keyword);
  if (selectedProductionDiv) {
    setBackground(selectedProductionDiv, "var(--md-code-hl-color)");
  }
}

function setBackground(/** @type Element */ productionDiv, /** @type string */ color) {
  const codeBlock = productionDiv.querySelector("code");

  if (codeBlock) {
    codeBlock.style.background = color;
  }
}
