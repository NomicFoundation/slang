// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    function f() external pure returns (uint256);
}

contract B is I {
    // Returning an extra value is a different return type.
    function f() public pure returns (uint256, uint256) {}
}
