// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    struct Dynamic {
        uint256[] values;
    }

    struct Static {
        uint256 value;
    }

    function f(Dynamic[] calldata a, Static[] calldata b) public pure {
        // A struct holding a dynamically-sized array is dynamically encoded, so
        // an array of them cannot be sliced.
        a[1:];

        // An array of statically encoded structs is fine.
        b[1:];
    }
}
