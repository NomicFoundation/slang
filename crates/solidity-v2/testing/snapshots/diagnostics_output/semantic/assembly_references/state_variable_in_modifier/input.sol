// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint x;

    modifier m() {
        assembly {
            x := 2
        }
        _;
    }

    function f() public m {}
}
