// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        blockhash({blockNumber: 1});
    }
}
