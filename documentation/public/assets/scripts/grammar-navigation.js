window.addEventListener("load", prepareGrammar);
window.addEventListener("hashchange", prepareGrammar);

function prepareGrammar() {
  const blocks = collectBlocks();
  createLinks(blocks);
  highlightSelected(blocks);
}

function collectBlocks() {
  const blocks = /** @type Map<string, Element> */ (new Map());
  for (const block of document.querySelectorAll("div .slang-ebnf")) {
    const id = block.getAttribute("id");
    if (!id) {
      continue;
    }

    blocks.set(id, block);
  }

  return blocks;
}

function createLinks(/** @type Map<string, Element> */ blocks) {
  for (const [_, block] of blocks) {
    const spans = block.querySelectorAll("span[class='k']");
    for (const span of spans) {
      const id = span.textContent?.trim();
      if (id && blocks.has(id) && !span.hasAttribute("linked")) {
        span.setAttribute("linked", "");

        const anchor = document.createElement("a");
        anchor.setAttribute("href", `#${id}`);
        anchor.appendChild(span.cloneNode(true));

        span.replaceWith(anchor);
      }
    }
  }
}

function highlightSelected(/** @type Map<string, Element> */ blocks) {
  function setBackground(/** @type Element */ block, /** @type string */ color) {
    const code = block.querySelector("code");
    if (!code) {
      return;
    }

    code.style.background = color;
  }

  for (const [_, block] of blocks) {
    setBackground(block, "var(--md-code-bg-color)");
  }

  const selected = blocks.get(location.hash.replace("#", ""));
  if (selected) {
    setBackground(selected, "var(--md-code-hl-color)");
  }
}
