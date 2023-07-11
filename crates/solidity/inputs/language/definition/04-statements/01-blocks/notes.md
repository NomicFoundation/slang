--8<-- "crates/solidity/inputs/language/snippets/under-construction.md"

## Unchecked Blocks

By default, all arithmetic operations are checked for underflow or overflow, which mean that if the result of an
operation falls outside the value range of the type, the call is reverted through a failing assertion.

In `v0.8.0`, this can be disabled using the `unchecked` block, resulting in wrapping arithmetic.

```solidity
unchecked {
  i++;
}
```
