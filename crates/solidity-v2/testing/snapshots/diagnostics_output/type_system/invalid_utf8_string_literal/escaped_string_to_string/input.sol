// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure returns (string memory, string memory, bytes memory) {
        // A `\x` escape can produce bytes that are not valid UTF-8.
        return (string("\xff"), string("\xc3\xa9"), bytes("\xff"));
    }
}
