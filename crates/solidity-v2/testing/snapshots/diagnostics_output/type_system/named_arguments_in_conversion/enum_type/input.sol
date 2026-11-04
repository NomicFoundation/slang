// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    enum E {
        A
    }

    function g() public pure {
        E({x: 0});
    }
}
