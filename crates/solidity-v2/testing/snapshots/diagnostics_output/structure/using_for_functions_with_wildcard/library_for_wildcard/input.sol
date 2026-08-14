// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    function f(uint256 value) internal pure returns (uint256) {
        return value;
    }
}

contract C {
    // Attaching a whole library to `*` is allowed inside a contract.
    using L for *;
}
