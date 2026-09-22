// SPDX-License-Identifier: MIT
pragma solidity *;

// The Yul `slotnum()` built-in needs both 0.8.37 and an Amsterdam target.
contract Foo {
    function f() public view returns (uint64 ret) {
        assembly {
            ret := slotnum()
        }
    }
}
