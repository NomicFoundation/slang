// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The dialect label is case sensitive.
    function dialectIsCaseSensitive() public pure {
        assembly "EVMASM" {}
    }
}
