// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // Invalid: the conflict is inside a mapping nested in a function-type
    // parameter, which is only reached once type names are traversed.
    function(mapping(address owner => address owner) storage) internal m;
}
