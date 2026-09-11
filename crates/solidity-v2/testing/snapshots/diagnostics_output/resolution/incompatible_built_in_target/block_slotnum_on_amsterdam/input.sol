// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `block_slotnum`, but with Amsterdam pinned as the EVM version.
contract Foo {
    function f() public view returns (uint64) {
        return block.slotnum;
    }
}
