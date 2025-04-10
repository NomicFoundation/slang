---
"@nomicfoundation/slang": minor
---

support Solidity `0.8.29` and [Custom Storage Layouts](https://docs.soliditylang.org/en/v0.8.29/contracts.html#custom-storage-layout):

- `ContractDefinition` nodes will no longer have an optional `InheritanceSpecifier` child directly, but will hold a list of `ContractSpecifier` children
- `ContractSpecifier` nodes have either `InheritanceSpecifier` or `StorageLayoutSpecifier` children
