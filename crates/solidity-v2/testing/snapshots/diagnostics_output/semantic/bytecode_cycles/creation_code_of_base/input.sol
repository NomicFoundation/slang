// SPDX-License-Identifier: MIT
pragma solidity *;

// Accessing a base's creation code embeds the base's bytecode in the derived
// contract, but not the other way around, so there is no cycle.

contract Base {
    function f() public pure returns (uint256) {
        return 1;
    }
}

contract Test is Base {
    function creationBase() public pure returns (bytes memory) {
        return type(Base).creationCode;
    }
}
