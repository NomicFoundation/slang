// SPDX-License-Identifier: MIT
pragma solidity *;

// A free (file-level) function cannot be payable.
function pay() payable {}

// A non-payable free function is allowed.
function noPay() {}

contract C {
    // A payable function in a contract is allowed.
    function payInContract() public payable {}
}
