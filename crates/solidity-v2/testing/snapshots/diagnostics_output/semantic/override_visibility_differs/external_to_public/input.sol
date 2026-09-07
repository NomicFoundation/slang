// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() external pure virtual {}
}

contract B is A {
    // Widening an `external` function to `public` is the one allowed change.
    function f() public pure override {}
}
