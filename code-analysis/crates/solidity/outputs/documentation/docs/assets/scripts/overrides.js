/*
 * Smooth scrolling for anchor links
 */

window.addEventListener("load", grammarNavigation);
window.addEventListener("hashchange", grammarNavigation);

function grammarNavigation() {
  if (!location.hash) {
    return;
  }

  const codeElement = document.getElementById(location.hash.replace("#", ""));
  if (!codeElement || codeElement.tagName != "CODE") {
    return;
  }

  highlightSection(codeElement);
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
  const currentPosition = document.documentElement.scrollTop;
  const destinationPosition = codeElement.getBoundingClientRect().top;
  const preMargin = parseFloat(getComputedStyle(document.querySelector("pre")).marginTop.replace("px", ""));
  const headerHeight = document.getElementsByTagName("header").item(0).getBoundingClientRect().height;

  window.scrollTo({
    top: currentPosition + destinationPosition - preMargin - headerHeight,
  });
}
