// SPDX-License-Identifier: MIT
pragma solidity *;

contract Base {
    function initialize(string memory a) external {}
}

contract Derived is Base {
    function initialize(string memory a, string memory b) external {}
}

contract C {
    // Only `Derived`'s own `initialize` is a member of its type name.
    function selector() external pure returns (bytes4) {
        return Derived.initialize.selector;
    }
}
