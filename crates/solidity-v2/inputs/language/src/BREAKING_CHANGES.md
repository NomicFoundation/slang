# Breaking Changes

> The following changes have been done to the language grammar/manifest, to adopt the new v2 lexer/parser system:

## Lexical Contexts

- Made `Topic::lexical_context` required, instead of creating a "default" context behind the scenes.

## Terminals

- Removed `Scanner::TrailingContext` as the new lexer has no backtracking, and tries to scan the longest match by default. Terminals now have a priority (to resolve ambiguities), defined by their kind (`Trivia` < `Tokens` < `Keywords`), then by their order of declaration in the grammar. Later definitions like `SingleLineNatSpecComment` (`///`) have a higher priority than earlier definitions like `SingleLineComment` (`//`).
