// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `yul_slotnum`, pinning the EVM version to Amsterdam.
contract Foo {
    function f() public view returns (uint64 ret) {
        assembly {
            ret := slotnum()
        }
    }
}
