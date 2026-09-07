// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() internal virtual {}
}

contract B is A {
    // Widening an `internal` function to `public` is not allowed.
    function f() public override {}
}
