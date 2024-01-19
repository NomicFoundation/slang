## Array Literals

An array literal is a comma-separated list of one or more expressions,
enclosed in square brackets (`[...]`). For example `[1, a, f(3)]`.
It is always a statically-sized memory array whose length is the number
of expressions.

```solidity
contract MyContract {
    function someFunction() public pure {
        otherFunction([uint(1), 2, 3]);
    }
}
```

## Array Slices

Array slices are a view on a contiguous portion of an array. They are
written as `x[start:end]`, where `start` and `end` are expressions
resulting in a uint256 type (or implicitly convertible to it). The first
element of the slice is `x[start]` and the last element is `x[end - 1]`.

Both `start` and `end` are optional: `start` defaults to `0` and `end`
defaults to the length of the array.

--8<-- "crates/solidity/inputs/language/snippets/under-construction.md"
