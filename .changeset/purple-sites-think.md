---
"@nomicfoundation/slang": patch
---

Fixes to binding rules:

- Update `TupleDeconstructionStatement` so that their definiens is the `TypedTupleMember`/`UntypedTupleMember` for each variable declared.
- Update `YulVariableDeclarationStatement` so that their definiens is the `YulIdentifier` for each variable declared.
