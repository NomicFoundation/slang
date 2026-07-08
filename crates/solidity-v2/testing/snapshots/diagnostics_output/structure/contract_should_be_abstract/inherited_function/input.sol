// SPDX-License-Identifier: MIT
pragma solidity *;

abstract contract A {
    function a() public virtual;
}

// `B` inherits an unimplemented function from `A` without implementing it.
contract B is A {}
