// SPDX-License-Identifier: MIT
pragma solidity *;

// Parenthesizing makes the value a tuple expression typed as a string
// literal rather than a number, so it is not a direct number constant.
bytes32 constant K = ("abc");

contract C {
    function f() public pure {
        assembly {
            let x := K
        }
    }
}
