// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public view virtual {}
}

contract B is A {
    // Overrides `A.f` without the `override` specifier.
    function f() public view {}
}
