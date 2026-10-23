pragma solidity *;

abstract contract A {
    function f() public virtual returns (uint256);
}

contract B is A {
    function f() public virtual override returns (uint256) {
        return 1;
    }
}

abstract contract C is A {
    function f() public virtual override returns (uint256);
}

contract D is B, C {
    function f() public override(B, C) returns (uint256) {
        // `C.f` is unimplemented, so `super` skips it and reaches `B.f`.
        return super.f();
    }
}
