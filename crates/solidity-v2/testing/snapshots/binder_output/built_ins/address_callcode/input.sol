// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function missing(address target, address payable payableTarget) public returns (bool, bool) {
        (bool success, ) = target.callcode("");                     // should be rejected
        (bool payableSuccess, ) = payableTarget.callcode("");       // should be rejected
        return (success, payableSuccess);
    }

    function present(address target) public returns (bool) {
        (bool success, ) = target.delegatecall("");                 // should be accepted
        return success;
    }
}
