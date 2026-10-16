// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(bytes memory data) public pure {
        abi.decode(data, (X));
    }
}
