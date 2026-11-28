// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes memory data) public {
        abi.decode(data, (address[]))[0].transfer(1);
    }
}
