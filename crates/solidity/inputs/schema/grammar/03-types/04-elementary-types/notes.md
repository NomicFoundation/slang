--8<-- "crates/solidity/inputs/schema/snippets/under-construction.md"

## Fixed-Size Byte Arrays

The value types `bytes1`, `bytes2`, `bytes3`, …, `bytes32` hold a sequence of bytes from one to up to 32.

## Dynamic String and Byte Arrays

The `bytes` type is similar to `bytes1[]`, but it is packed tightly in calldata and memory.

Variables of type `string` are equal to `bytes` but do not allow length or index access.
If you want to access the byte-representation of a string `s`, use `bytes(s)`. Keep in mind that you are
accessing the low-level bytes of the UTF-8 representation, and not the individual characters.

Memory arrays with dynamic length can be created using the `new` keyword:

```solidity
contract MyContract {
    function myFunction(uint length) public pure {
        bytes memory b = new bytes(length);
    }
}
```
