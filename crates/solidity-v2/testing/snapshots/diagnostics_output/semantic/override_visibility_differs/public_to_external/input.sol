// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public virtual {}
}

contract B is A {
    // Narrowing a `public` function to `external` is not allowed.
    function f() external override {}
}
