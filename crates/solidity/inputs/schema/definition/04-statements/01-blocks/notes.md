--8<-- "crates/solidity/inputs/schema/snippets/under-construction.md"

## Blocks

By default, all arithmetic operations are checked for underflow or overflow, which mean that if the result of an
operation falls outside the value range of the type, the call is reverted through a failing assertion. This can be
disabled using the `unchecked` block, resulting in wrapping arithmetic.
