// SPDX-License-Identifier: MIT
pragma solidity *;

// A free (file-level) function must have an implementation body.
function f1() returns (uint);

// A free function with an implementation body is allowed.
function f2() returns (uint) {
    return 0;
}

abstract contract C {
    // A function without a body inside a contract is allowed (it may be
    // overridden), so this is not a free-function-without-body error.
    function f3() internal virtual;
}
