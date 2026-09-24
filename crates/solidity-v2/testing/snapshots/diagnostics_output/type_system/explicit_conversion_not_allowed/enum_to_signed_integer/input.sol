// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    enum E { A, B }

    function f(E e) public pure returns (int256, uint256) {
        return (int256(e), uint256(e));
    }
}
