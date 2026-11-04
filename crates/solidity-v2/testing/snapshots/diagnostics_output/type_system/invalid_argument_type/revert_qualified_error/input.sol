// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    error Err(uint256 a);
}

contract C {
    function g() public {
        revert L.Err(true);
    }
}
