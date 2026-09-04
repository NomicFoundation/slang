// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Every invalid attribute extracted from the function type gets its own
    // diagnostic: two mutabilities and an `external` visibility.
    function() internal public payable pure external x;
}
