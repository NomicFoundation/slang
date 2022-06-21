# Arrays

Arrays can have a compile-time fixed size, or they can have a dynamic size.

The type of an array of fixed size `k` and element type `T` is written
as `T[k]`, and an array of dynamic size as `T[]`.

For example, an array of 5 dynamic arrays of `uint` is written as
`uint[][5]`. The notation is reversed compared to some other languages.
In Solidity, `X[3]` is always an array containing three elements of type
`X`, even if `X` is itself an array. This is not the case in other
languages such as C.

Indices are zero-based, and access is in the opposite direction of the
declaration.
For example, if you have a variable `uint[][5] memory x`, you access the
seventh `uint` in the third dynamic array using `x[2][6]`, and to access
the third dynamic array, use `x[2]`. Again, if you have an array
`T[5] a` for a type `T` that can also be an array, then `a[2]` always
has type `T`.

## Fixed-Size Byte Arrays

The value types `bytes1`, `bytes2`, `bytes3`, …, `bytes32` hold a sequence of bytes from one to up to 32.

!!! danger "Breaking Change\s"

    Keyword `byte` was an alias for `bytes1`. It was deprecated in `v0.8.0`:
    --8<-- "reference/04-advanced-types/03-arrays/tests/byte-keyword/generated/combined"

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
