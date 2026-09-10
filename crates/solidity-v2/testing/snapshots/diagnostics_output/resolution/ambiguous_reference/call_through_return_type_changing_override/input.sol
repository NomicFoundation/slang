// SPDX-License-Identifier: MIT
pragma solidity *;

// `B.f` hides `A.f` for a bare `f` even though the return types differ, so the
// call is not ambiguous. The return type mismatch is reported by the override
// check instead.
contract A {
    function f() public virtual returns (uint256) {
        return 1;
    }
}

contract B is A {
    function f() public override returns (bool) {
        return true;
    }

    function g() public returns (bool) {
        return f();
    }
}
