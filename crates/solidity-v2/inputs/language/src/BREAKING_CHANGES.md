# Breaking Changes

> The following changes have been done to the language grammar/manifest, to adopt the new v2 lexer/parser system:

## Lexical Contexts

- Group `Section` entries under a new top-level `Language::contexts` field, where each `LexicalContext` defines the `identifier_token` for its keywords (replacing `Keyword::identifier`).

## Terminals

- Removed `Scanner::TrailingContext` as the new lexer has no backtracking, and tries to scan the longest match by default. Terminals now have a priority (to resolve ambiguities), defined by their kind (`Trivia` < `Tokens` < `Keywords`), then by their order of declaration in the grammar. Later definitions like `SingleLineNatSpecComment` (`///`) have a higher priority than earlier definitions like `SingleLineComment` (`//`).
- Promoted `TokenDefinition::enabled` to `TokenItem::enabled`, as the new lexer doesn't version individual definitions (all of them are matched in all versions). We can decide on whether to keep or remove the additional `TokenDefinition` structure after we consider how to handle post-lexing processing (e.g. `DecimalLiteral` and `YulIdentifier` version breaks).
- Promoted `KeywordDefinition::enabled` to `KeywordItem::enabled`, as the new lexer doesn't version individual definitions (all of them are matched in all versions). We can decide on whether to keep or remove the additional `KeywordDefinition` structure after we consider how to handle variations in `reserved` status between them (mainly the four `*fixed*` keywords).

## Validation

The old grammar was slightly more restrictive in some corner cases, which is now relaxed to unblock performance improvements.
The changes shouldn't affect correctness for valid inputs, but may allow some invalid inputs to be parsed successfully.
We should consider adding validation for these at a later stage if needed:

- `HexLiteral`:
    - Uppercase `0X` was disabled in `0.5.0`.
- `SingleQuotedStringLiteral` and `DoubleQuotedStringLiteral`:
    - Escaping arbitrary escape sequences was disabled in `0.4.25`, instead of valid `AsciiEscape` values.
    - Unicode characters were disabled in `0.7.0`.
- `SingleQuotedUnicodeStringLiteral` and `DoubleQuotedUnicodeStringLiteral`:
    - They were only enabled after `0.7.0`.
- `HexLiteral` and `YulHexLiteral` and `DecimalLiteral` and `YulDecimalLiteral`:
    - It was illegal for them to be followed by `IdentifierStart`. Now we will produce two separate tokens rather than rejecting it.

## Grammar

The following changes modify the language definition to support the new parser and resolve grammar ambiguities.
In some cases we also try to simplify the model.

### AddressKeyword

- Made the `address` keyword reserved in all versions, handling the few cases where it can be used as an identifier separately.
- `IdentifierPathElement` handles the cases where `address` can be used as an `Identifier`, either in an `IdentifierPath` or a `MemberAccessExpression`.

### IdentifierPathElement

New enum added to allow the reserved `address` keyword in identifier paths and member access expressions
(from Solidity 0.6.0):

- Variants: `Identifier` | `AddressKeyword` (enabled from 0.6.0)
- Used in `MemberAccessExpression` and in `IdentifierPath`

### IdentifierPath

Changed from a `Separated` list of `Identifier`, to a list of `IdentifierPathElement`, to capture the reserved
`AddressKeyword` as part of the path.

- **Before**: `Separated(name = IdentifierPath, reference = Identifier, separator = Period)`
- **After**: `Separated(name = IdentifierPath, reference = IdentifierPathElement, separator = Period)`

### VariableDeclarationStatement (Consolidated)

Major restructuring to consolidate `TupleDeconstructionStatement` and `VariableDeclarationStatement` into a single unified type with three variants.

- **Before (V1)**: Two separate statement types:

    - `VariableDeclarationStatement`: `variable_type`, `storage_location?`, `name`, `value?`, `semicolon`
    - `TupleDeconstructionStatement`: `var_keyword?`, `open_paren`, `elements`, `close_paren`, `equal`, `expression`, `semicolon`

- **After (V2)**: Single unified statement with three variants:
    - `VariableDeclarationStatement`: Contains `target: VariableDeclarationOption`, `semicolon`
    - `VariableDeclarationOption`: Enum with three variants:
        1. `SingleTypedDeclaration`: For `int x = ...` syntax (with optional value)
        2. `MultiTypedDeclaration`: For `(bool a, , int b) = ...` syntax (value required)
        3. `UntypedDeclaration`: For `var a = ...` or `var (a, , b) = ...` syntax (till 0.5.0, value required)

This makes certain cases that were allowed before disallowed in V2, in particular having untyped declarations (like `(a, bool b) = ...`)
or having typed together with `var` (like `var (a, bool b) = ...`).
The cases where using empty tuples are still ambiguous, `(,,,) = ...` can still be a `VariableDeclarationStatement` (`MultiTypedDeclaration`) or an `AssignmentExpression` with a `TupleExpression` on the lhs.

Removed types from V1:

- `TupleDeconstructionStatement`
- `TupleDeconstructionElements`, `TupleDeconstructionElement`
- `TupleMember`, `TypedTupleMember`, `UntypedTupleMember`
- `VariableDeclarationType`

Added types in V2:

- `VariableDeclaration`
- `VariableDeclarationOption`
- `SingleTypedDeclaration`, `MultiTypedDeclaration`, `UntypedDeclaration`
- `MultiTypedDeclarationElements`, `MultiTypedDeclarationElement`
- `UntypedDeclarationNames`, `UntypedTupleDeclaration`
- `UntypedTupleDeclarationElements`, `UntypedTupleDeclarationElement`

### NamedArgumentsDeclaration

- Changed `arguments` field from `Optional(NamedArgumentGroup)` to `Required(NamedArgumentGroup)`.
- This avoids ambiguity with empty argument lists `()` which could be either positional or named.
