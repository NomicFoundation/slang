// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    uint256 public x;

    function f() internal pure virtual {}
}

contract B is A {
    function missing() internal view returns (uint256) {
        // A public state variable getter is not reachable through `super`.
        return super.x();
    }

    function present() internal pure {
        super.f();
    }
}
