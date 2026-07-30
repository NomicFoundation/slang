// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Invalid: named return parameter in a function type.
    function() external returns (uint256 x) a;

    // Invalid: named return parameter in a function type that is nested inside
    // another function type's parameter list.
    function(function() external returns (uint256 inner)) external b;

    // Invalid: each named return parameter is flagged.
    function() external returns (uint256 p, uint256 q) c;

    // Valid: unnamed return parameters.
    function(uint256) external returns (uint256) d;
}
