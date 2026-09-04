// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The function type greedily takes the attributes, so everything after the
    // repeated visibility belongs to the state variable. 'external' is not a
    // valid visibility there.
    function() internal public external x;
}
