// SPDX-License-Identifier: MIT
pragma solidity *;

library D {
    function f(bytes calldata) internal pure {}
}

contract C {
    using D for bytes;

    function g(bytes memory x) public pure {
        // An internal function takes its first argument as is, and memory
        // doesn't convert to calldata.
        x.f();
    }
}
