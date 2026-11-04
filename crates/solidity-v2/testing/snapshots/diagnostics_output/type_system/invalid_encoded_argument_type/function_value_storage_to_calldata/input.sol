// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    string s;

    function main(function(string calldata) external f) external view {
        abi.encodeCall(f, (s));
    }
}
