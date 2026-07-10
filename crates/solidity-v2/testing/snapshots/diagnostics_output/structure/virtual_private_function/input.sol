// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A function cannot be both `private` and `virtual`.
    function f1() private virtual {}

    // A `private` function that is not `virtual` is allowed.
    function f2() private {}

    // A `virtual` function with another visibility is allowed.
    function f3() internal virtual {}
}
