// SPDX-License-Identifier: MIT
pragma solidity *;

contract Foo {
    event MyEvent();

    function f() public pure returns (bytes32) {
        return MyEvent.selector;
    }
}
