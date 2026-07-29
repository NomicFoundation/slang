// SPDX-License-Identifier: MIT
pragma solidity *;

contract Foo {
    function f() public view returns (uint256 result) {
        assembly {
            result := difficulty()
        }
    }
}
