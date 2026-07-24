// SPDX-License-Identifier: MIT
pragma solidity *;

// Accessing your own creation code is a cycle. Accessing a base's creation
// code is not, since inheritance embeds no bytecode.

contract Base {
    function f() public pure returns (uint256) {
        return 1;
    }
}

contract Test1 is Base {
    function creation() public pure returns (bytes memory) {
        return type(Test1).creationCode;
    }
}

contract Test2 is Base {
    function creationBase() public pure returns (bytes memory) {
        return type(Base).creationCode;
    }
}
