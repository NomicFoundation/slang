// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // An empty label doesn't name any dialect.
    function emptyDialect() public pure {
        assembly "" {}
    }
}
