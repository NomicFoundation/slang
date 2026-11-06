// SPDX-License-Identifier: MIT
pragma solidity *;

// A `revert` names a state variable.
contract C {
    uint256 x;

    function f() public {
        revert x();
    }
}
