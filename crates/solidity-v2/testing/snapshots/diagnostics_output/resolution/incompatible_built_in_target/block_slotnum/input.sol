// SPDX-License-Identifier: MIT
pragma solidity *;

// `block.slotnum` needs both 0.8.37 and an Amsterdam target.
contract Foo {
    function f() public view returns (uint64) {
        return block.slotnum;
    }
}
