/*
 * Smooth scrolling for anchor links
 */

window.addEventListener("load", grammarNavigation);
window.addEventListener("hashchange", grammarNavigation);

function grammarNavigation() {
  fixGrammarLinks();

  if (!location.hash) {
    return;
  }

  const codeElement = document.getElementById(location.hash.replace("#", ""));
  if (!codeElement) {
    return;
  }

  // TODO(OmarTawfik):
  // Broken by https://github.com/NomicFoundation/slang/pull/332
  // Will fix in https://github.com/NomicFoundation/slang/issues/76
  //
  // highlightSection(codeElement);

  scrollToElement(codeElement);
}

function highlightSection(codeElement) {
  const sections = document.getElementsByTagName("code");
  const styles = getComputedStyle(document.documentElement);

  for (let i = 0; i < sections.length; i++) {
    sections.item(i).style.background = styles.getPropertyValue("--md-code-bg-color");
  }

  codeElement.style.background = styles.getPropertyValue("--md-code-hl-color");
}

function scrollToElement(codeElement) {
  const fromPosition = document.documentElement.scrollTop;
  const toPosition = codeElement.getBoundingClientRect().top;
  const windowCenter = window.innerHeight / 2;

  window.scrollTo({
    top: fromPosition + toPosition - windowCenter,
  });
}

function fixGrammarLinks() {
  if (document.title !== "Grammar - Slang") {
    // TODO(OmarTawfik): hack to fix navigation on grammar page.
    // To be fixed in: https://github.com/NomicFoundation/slang/issues/343
    return;
  }

  const links = document.getElementsByClassName("slang-global-ebnf-link");

  for (let i = 0; i < links.length; i++) {
    const link = links.item(i).getAttributeNode("href");
    link.value = link.value.replace(/^.*#/, "./#");
  }
}
