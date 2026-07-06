// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() public virtual {}
}

contract B is A {
    function f() public override {}
}
