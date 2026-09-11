// SPDX-License-Identifier: MIT
pragma solidity *;

// `B.f` may not drop `A.f`'s body, which is reported on `B.f`. The slot keeps
// `A.f`'s implementation, so `C` is concrete and is not reported.
contract A {
    function f() public virtual {}
}

abstract contract B is A {
    function f() public virtual override;
}

contract C is B {}
