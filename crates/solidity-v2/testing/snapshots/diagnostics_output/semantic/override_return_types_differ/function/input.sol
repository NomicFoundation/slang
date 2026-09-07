// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function f() internal virtual returns (uint256);
}

abstract contract B is A {
    // Same parameters, different return type.
    function f() internal virtual override returns (uint8);
}
