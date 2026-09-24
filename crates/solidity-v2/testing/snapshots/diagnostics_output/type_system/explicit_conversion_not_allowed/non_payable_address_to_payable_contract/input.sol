// SPDX-License-Identifier: MIT
pragma solidity *;

contract P {
    receive() external payable {}
}

contract C {
    function f(address a) public pure returns (P, P) {
        return (P(a), P(payable(a)));
    }
}
