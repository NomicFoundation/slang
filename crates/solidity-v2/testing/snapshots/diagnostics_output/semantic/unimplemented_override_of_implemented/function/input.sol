// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function f() external virtual {}
}

abstract contract B is A {
    // An implemented function cannot be overridden by an unimplemented one.
    function f() external virtual override;
}
