---
"@nomicfoundation/slang": patch
---

- Fix binding rules for `TupleDeconstructionStatement` so that their definiens is the `TypedTupleMember`/`UntypedTupleMember` for each variable declared.
- Fix binding rules for `YulVariableDeclarationStatement` so that their definiens is the `YulIdentifier` for each variable declared.
