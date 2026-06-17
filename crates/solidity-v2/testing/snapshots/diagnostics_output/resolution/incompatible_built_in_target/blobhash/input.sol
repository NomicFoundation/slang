// SPDX-License-Identifier: MIT
pragma solidity *;

contract Foo {
    function f() public view returns (bytes32) {
        return blobhash(0);
    }
}
