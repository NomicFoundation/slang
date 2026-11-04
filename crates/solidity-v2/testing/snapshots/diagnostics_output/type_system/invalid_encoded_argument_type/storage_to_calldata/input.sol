// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    function f(string calldata) external;
}

contract C {
    string s;

    function main() external view {
        abi.encodeCall(I.f, (s));
    }
}
