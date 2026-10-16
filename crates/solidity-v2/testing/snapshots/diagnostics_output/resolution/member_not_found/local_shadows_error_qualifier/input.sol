// SPDX-License-Identifier: MIT
pragma solidity *;

// The parameter `I` shadows the interface, and has no member `Err`.
interface I {
    error Err();
}

contract C {
    function f(uint256 I) public pure {
        revert I.Err();
    }
}
