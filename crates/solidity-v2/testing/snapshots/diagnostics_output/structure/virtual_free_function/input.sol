// SPDX-License-Identifier: MIT
pragma solidity *;

// A free (file-level) function cannot be marked `virtual`.
function f1() virtual {}

// A free function that is not `virtual` is allowed.
function f2() {}

contract C {
    // A `virtual` function inside a contract is allowed.
    function f3() internal virtual {}
}
