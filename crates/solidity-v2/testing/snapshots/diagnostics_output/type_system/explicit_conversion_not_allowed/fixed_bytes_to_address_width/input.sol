// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes32 b) public pure returns (address, address) {
        return (address(b), address(bytes20(b)));
    }
}
