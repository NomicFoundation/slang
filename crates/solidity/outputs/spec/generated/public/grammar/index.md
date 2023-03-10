<!-- This file is generated automatically by infrastructure scripts. Please don't edit by hand. -->

# Grammar

## 1. File Structure

### 1.2. Source Unit

--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/02-source-unit/source-unit.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/02-source-unit/directive.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/02-source-unit/definition.html"

### 1.3. Pragmas

--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/03-pragmas/pragma-directive.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/03-pragmas/version-pragma.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/03-pragmas/version-pragma-specifier.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/03-pragmas/version-pragma-operator.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/03-pragmas/version-pragma-value.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/03-pragmas/abi-coder-pragma.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/03-pragmas/experimental-pragma.html"

### 1.4. Imports

--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/04-imports/import-directive.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/04-imports/simple-import-directive.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/04-imports/star-import-directive.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/04-imports/selecting-import-directive.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/04-imports/selected-import.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/04-imports/import-path.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/04-imports/using-directive.html"

### 1.5. Trivia

--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/05-trivia/leading-trivia.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/05-trivia/trailing-trivia.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/05-trivia/end-of-file-trivia.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/05-trivia/whitespace.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/05-trivia/end-of-line.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/05-trivia/multiline-comment.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/05-trivia/single-line-comment.html"

### 1.7. Keywords

--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/abicoder-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/abstract-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/address-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/anonymous-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/as-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/assembly-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/bool-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/break-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/calldata-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/case-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/catch-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/constant-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/constructor-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/continue-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/contract-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/days-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/default-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/delete-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/do-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/else-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/emit-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/enum-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/error-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/ether-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/event-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/experimental-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/external-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/fallback-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/false-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/finney-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/for-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/from-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/function-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/global-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/gwei-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/hours-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/if-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/immutable-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/import-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/indexed-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/interface-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/internal-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/is-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/leave-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/let-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/library-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/mapping-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/memory-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/minutes-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/modifier-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/new-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/override-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/payable-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/pragma-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/private-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/public-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/pure-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/receive-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/return-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/returns-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/revert-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/seconds-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/solidity-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/storage-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/string-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/struct-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/switch-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/szabo-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/true-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/try-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/type-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/unchecked-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/using-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/view-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/virtual-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/weeks-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/wei-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/while-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/07-keywords/years-keyword.html"

### 1.8 Punctuation

--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/open-paren.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/close-paren.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/open-bracket.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/close-bracket.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/open-brace.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/close-brace.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/comma.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/period.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/question-mark.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/semicolon.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/colon.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/colon-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/equal-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/equal-greater-than.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/star.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/star-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/star-star.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/bar.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/bar-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/bar-bar.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/ampersand.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/ampersand-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/ampersand-ampersand.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/less-than.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/less-than-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/less-than-less-than.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/less-than-less-than-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/greater-than.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/greater-than-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/greater-than-greater-than.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/greater-than-greater-than-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/greater-than-greater-than-greater-than.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/greater-than-greater-than-greater-than-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/plus.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/plus-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/plus-plus.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/minus.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/minus-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/minus-minus.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/minus-greater-than.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/slash.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/slash-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/percent.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/percent-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/bang.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/bang-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/caret.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/caret-equal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/01-file-structure/08-punctuation/tilde.html"

## 2. Definitions

### 2.1. Contracts

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/01-contracts/contract-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/01-contracts/inheritance-specifier-list.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/01-contracts/inheritance-specifier.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/01-contracts/contract-body-element.html"

### 2.2. Interfaces

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/02-interfaces/interface-definition.html"

### 2.3. Libraries

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/03-libraries/library-definition.html"

### 2.4. Structs

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/04-structs/struct-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/04-structs/struct-member.html"

### 2.5. Enums

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/05-enums/enum-definition.html"

### 2.6. Constants

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/06-constants/constant-definition.html"

### 2.7. State Variables

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/07-state-variables/state-variable-declaration.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/07-state-variables/state-variable-attribute.html"

### 2.8. Functions

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/function-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/function-attribute.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/override-specifier.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/parameter-list.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/parameter-declaration.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/argument-list.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/positional-argument-list.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/named-argument-list.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/named-argument.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/constructor-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/constructor-attribute.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/unnamed-function-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/fallback-function-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/fallback-function-attribute.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/receive-function-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/08-functions/receive-function-attribute.html"

### 2.9. Modifiers

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/09-modifiers/modifier-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/09-modifiers/modifier-attribute.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/09-modifiers/modifier-invocation.html"

### 2.10 Events

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/10-events/event-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/10-events/event-parameter.html"

### 2.11. User Defined Value Types

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/11-user-defined-value-types/user-defined-value-type-definition.html"

### 2.12. Errors

--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/12-errors/error-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/02-definitions/12-errors/error-parameter.html"

## 3. Types

### 3.1. Advanced Types

--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/01-advanced-types/type-name.html"

### 3.2. Function Types

--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/02-function-types/function-type.html"

### 3.3. Mapping Types

--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/03-mapping-types/mapping-type.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/03-mapping-types/mapping-key-type.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/03-mapping-types/mapping-value-type.html"

### 3.4. Elementary Types

--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/04-elementary-types/elementary-type.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/04-elementary-types/address-type.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/04-elementary-types/payable-type.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/04-elementary-types/fixed-bytes-type.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/04-elementary-types/signed-integer-type.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/04-elementary-types/unsigned-integer-type.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/04-elementary-types/signed-fixed-type.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/03-types/04-elementary-types/unsigned-fixed-type.html"

## 4. Statements

### 4.1. Blocks

--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/01-blocks/block.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/01-blocks/unchecked-block.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/01-blocks/statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/01-blocks/simple-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/01-blocks/expression-statement.html"

### 4.2. Declaration Statements

--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/02-declaration-statements/tuple-deconstruction-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/02-declaration-statements/variable-declaration-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/02-declaration-statements/data-location.html"

### 4.3. Control Statements

--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/03-control-statements/if-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/03-control-statements/for-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/03-control-statements/while-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/03-control-statements/do-while-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/03-control-statements/continue-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/03-control-statements/break-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/03-control-statements/return-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/03-control-statements/emit-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/03-control-statements/delete-statement.html"

### 4.4. Error Handling

--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/04-error-handling/try-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/04-error-handling/catch-clause.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/04-statements/04-error-handling/revert-statement.html"

## 5. Expressions

### 5.1. Base Expressions

--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/01-base-expressions/expression.html"

### 5.2. Primary Expressions

--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/02-primary-expressions/primary-expression.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/02-primary-expressions/type-expression.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/02-primary-expressions/new-expression.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/02-primary-expressions/tuple-expression.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/02-primary-expressions/array-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/02-primary-expressions/boolean-literal.html"

### 5.3. Numbers

--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/03-numbers/numeric-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/03-numbers/hex-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/03-numbers/decimal-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/03-numbers/decimal-number.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/03-numbers/decimal-exponent.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/03-numbers/number-unit.html"

### 5.4. Strings

--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/string-expression.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/hex-string-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/possibly-separated-pairs-of-hex-digits.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/hex-character.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/ascii-string-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/single-quoted-ascii-string-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/double-quoted-ascii-string-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/unicode-string-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/single-quoted-unicode-string-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/double-quoted-unicode-string-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/escape-sequence.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/ascii-escape.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/hex-byte-escape.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/04-strings/unicode-escape.html"

### 5.5. Identifiers

--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/05-identifiers/identifier-path.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/05-identifiers/identifier.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/05-identifiers/keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/05-identifiers/reserved-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/05-identifiers/raw-identifier.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/05-identifiers/identifier-start.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/05-expressions/05-identifiers/identifier-part.html"

## 6. Yul

### 6.1. Assembly Block

--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/01-assembly-block/assembly-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/01-assembly-block/evmasm.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/01-assembly-block/assembly-flags.html"

### 6.2. Yul Statements

--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-block.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-variable-declaration.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-function-definition.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-assignment-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-if-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-leave-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-break-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-continue-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-for-statement.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/02-yul-statements/yul-switch-statement.html"

### 6.3. Yul Expressions

--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/03-yul-expressions/yul-expression.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/03-yul-expressions/yul-function-call.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/03-yul-expressions/yul-identifier-path.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/03-yul-expressions/yul-identifier.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/03-yul-expressions/yul-keyword.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/03-yul-expressions/yul-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/03-yul-expressions/yul-hex-literal.html"
--8<-- "crates/solidity/outputs/spec/generated/ebnf/06-yul/03-yul-expressions/yul-decimal-literal.html"
