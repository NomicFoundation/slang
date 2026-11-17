// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    error Err(uint256 a);

    function g() public {
        revert Err(true);
    }
}
