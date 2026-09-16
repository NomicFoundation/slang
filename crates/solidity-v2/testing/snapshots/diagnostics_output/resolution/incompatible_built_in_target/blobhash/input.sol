// SPDX-License-Identifier: MIT
pragma solidity *;

// `blobhash` requires both 0.8.24 and a Cancun target. 0.8.24 still defaults to
// Shanghai, so that version reports the target incompatibility on its own,
// while earlier versions report the version gate as well.
contract Foo {
    function f() public view returns (bytes32) {
        return blobhash(0);
    }
}
