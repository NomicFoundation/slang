// SPDX-License-Identifier: MIT
pragma solidity *;

contract Base {
    constructor(uint256) {}

    function g() public pure returns (uint256) {
        return 1;
    }
}

// A qualified call to a base function is valid in a base-constructor argument,
// even though the names there resolve in the enclosing file scope.
contract Derived is Base(Base.g()) {}
