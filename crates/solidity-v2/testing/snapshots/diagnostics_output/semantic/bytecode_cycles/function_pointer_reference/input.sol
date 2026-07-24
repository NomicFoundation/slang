// SPDX-License-Identifier: MIT
pragma solidity *;

// `f` is only referenced as a value, never called. It could still run
// through the function pointer, so its body is reachable.

function f() {
    new D();
}

contract D {
    function() internal ptr = f;
}
