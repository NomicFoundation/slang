// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier m(uint256[] calldata x) {
        _;
    }

    function f(uint256[] memory a) public m(a) {}
}
