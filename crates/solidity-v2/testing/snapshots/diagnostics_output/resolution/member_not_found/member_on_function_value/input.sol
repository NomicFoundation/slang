// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() internal pure returns (uint8) {
        return 1;
    }

    function missing() internal pure {
        // A function value has no named members of its own.
        f.f();
    }
}
