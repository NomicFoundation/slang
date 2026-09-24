// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    fallback() external payable {}
}

contract P is B {}

contract C {
    function f(address a) public pure returns (P) {
        return P(a);
    }
}
