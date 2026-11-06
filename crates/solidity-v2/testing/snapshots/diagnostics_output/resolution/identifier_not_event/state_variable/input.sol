// SPDX-License-Identifier: MIT
pragma solidity *;

// An `emit` names a state variable.
contract C {
    uint256 x;

    function f() public {
        emit x();
    }
}
