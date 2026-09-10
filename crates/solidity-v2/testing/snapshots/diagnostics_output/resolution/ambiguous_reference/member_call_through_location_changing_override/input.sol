// SPDX-License-Identifier: MIT
pragma solidity *;

// `this.f` reaches the contract's external interface, where `B.f` takes `A.f`'s
// slot although the parameter location differs, so the call is not ambiguous.
// Before 0.8.14 the program is valid. From then on the changed location is
// reported on its own.
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
        return this.f(a);
    }
}
