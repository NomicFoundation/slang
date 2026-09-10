// SPDX-License-Identifier: MIT
pragma solidity *;

// A changed parameter data location plays no part in deciding that `B.f`
// implements `A.f`, so `B` is concrete. From 0.8.14 the changed location is
// reported on its own.
abstract contract A {
    function f(uint256[] calldata a) internal pure virtual returns (uint256);
}

contract B is A {
    function f(uint256[] memory a) internal pure override returns (uint256) {
        return a.length;
    }
}
