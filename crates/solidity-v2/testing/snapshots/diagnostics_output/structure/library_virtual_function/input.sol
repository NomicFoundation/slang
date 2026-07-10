// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    // A function in a library cannot be marked `virtual`.
    function f1() internal virtual {}

    // A non-virtual library function is allowed.
    function f2() internal {}
}

contract C {
    // A `virtual` function is allowed outside a library.
    function f3() internal virtual {}
}
