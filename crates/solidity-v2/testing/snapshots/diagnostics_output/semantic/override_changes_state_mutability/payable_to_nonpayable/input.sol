// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public payable virtual {}
}

contract B is A {
    // A `payable` function cannot change its mutability at all, not even to
    // a stricter one.
    function f() public override {}
}
