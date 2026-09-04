// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The function type greedily takes the attributes, so everything after the
    // repeated visibility belongs to the state variable. A mutability is not
    // valid there.
    function() internal public payable x;
}
