// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes memory data) public {
        abi.decode(data, (address)).transfer(1);
    }
}
