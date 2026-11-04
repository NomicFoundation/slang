// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    enum E {
        A
    }

    function g() public pure {
        E(0, 1);
    }
}
