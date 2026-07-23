// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier someModifier() {
        _;
    }
}

// A free (file-level) function cannot have modifier invocations.
function fun() C.someModifier {}

// A free function without modifier invocations is allowed.
function ok() {}
