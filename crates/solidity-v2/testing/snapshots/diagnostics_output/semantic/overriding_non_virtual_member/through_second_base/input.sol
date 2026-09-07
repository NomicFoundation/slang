// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function f() external view {}
}

contract B {
    function f() public virtual {}
}

// `C.f` overrides both `A.f` and `B.f`. Against `B.f` everything is fine.
// Against `A.f` it overrides a non-virtual function and loosens `view`, so
// both errors must be reported even though `B` is checked first.
contract C is B, A {
    function f() public override(A, B) {}
}
