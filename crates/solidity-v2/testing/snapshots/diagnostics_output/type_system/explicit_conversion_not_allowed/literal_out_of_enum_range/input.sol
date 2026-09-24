// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    enum E { A, B }

    function f() public pure returns (E, E) {
        return (E(2), E(1));
    }
}
