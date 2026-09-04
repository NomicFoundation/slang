// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint x;

    function f() public {
        assembly {
            let t := x.length
        }
    }
}
