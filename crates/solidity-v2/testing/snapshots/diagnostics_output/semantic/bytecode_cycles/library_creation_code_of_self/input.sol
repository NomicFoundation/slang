// SPDX-License-Identifier: MIT
pragma solidity *;

// The library's public function reaches an access to the library's own
// creation code through an internal function of another library.

library L1 {
    function foo() public pure returns (bytes memory) {
        return L2.foo();
    }
}

library L2 {
    function foo() internal pure returns (bytes memory) {
        return type(L1).creationCode;
    }
}
