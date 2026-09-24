// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint160 x) public pure returns (address payable, address payable) {
        return (payable(x), payable(address(x)));
    }
}
