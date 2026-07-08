// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function a() public virtual;
}

// `B` implements the inherited function, so it need not be `abstract`.
contract B is A {
    function a() public override {}
}
