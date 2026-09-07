// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public virtual {}
}

contract B is A {
    // Loosening non-payable to `payable` is not allowed.
    function f() public payable override {}
}
