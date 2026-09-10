// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public view virtual {}
}

contract B is A {
    // Loosening `view` to non-payable is not allowed.
    function f() public override {}
}
