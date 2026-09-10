// SPDX-License-Identifier: MIT
pragma solidity *;

// A bare `f` sees `A.f` and `B.f` as two overloads, and a calldata argument
// fits both, so the call is ambiguous. From 0.8.14 the changed location is
// reported as well.
contract A {
    function f(uint256[] memory a) public virtual returns (uint256) {
        return a.length;
    }
}

contract B is A {
    function f(uint256[] calldata a) public override returns (uint256) {
        return a.length + 1;
    }

    function g(uint256[] calldata a) external returns (uint256) {
        return f(a);
    }
}
