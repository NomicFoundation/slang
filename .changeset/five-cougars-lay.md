---
"changelog": minor
---

Added error recovery i.e. a CST is _always_ produced, even if there are errors. The erroneous/skipped text is in the CST as a `TokenKind::SKIPPED` token.
