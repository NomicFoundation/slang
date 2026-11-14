// SPDX-License-Identifier: MIT
pragma solidity *;

interface I {
    function f(uint256) external;
}

contract C {
    function g() public pure returns (bytes memory) {
        return abi.encodeCall(I.f, (1));
    }
}
