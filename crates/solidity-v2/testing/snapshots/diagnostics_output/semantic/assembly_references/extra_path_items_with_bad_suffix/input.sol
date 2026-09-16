// SPDX-License-Identifier: MIT
pragma solidity *;

// A path can be too long and carry an unsupported suffix at the same time,
// and both are reported.
contract C {
    uint x;

    function f() public {
        assembly {
            let t := x.length.foo
        }
    }
}
