// SPDX-License-Identifier: MIT
pragma solidity *;

contract P {
    receive() external payable {}
}

contract C {
    function f(bytes memory data) public pure returns (P) {
        return P(abi.decode(data, (address)));
    }
}
