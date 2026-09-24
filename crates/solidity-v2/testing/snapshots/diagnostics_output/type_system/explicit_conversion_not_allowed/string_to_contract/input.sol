// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {}

contract C {
    function f(string memory s) public pure returns (A, A) {
        return (A(s), A(address(0)));
    }
}
