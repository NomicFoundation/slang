// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {
        payable(msg.sender).transfer(true);
    }
}
