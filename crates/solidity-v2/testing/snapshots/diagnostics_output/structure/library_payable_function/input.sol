// SPDX-License-Identifier: MIT
pragma solidity *;

library L {
    // A library function cannot be payable.
    function pay() public payable {}

    // A non-payable library function is fine.
    function noPay() public {}
}

contract C {
    // A payable function in a contract is allowed.
    function pay() public payable {}
}
