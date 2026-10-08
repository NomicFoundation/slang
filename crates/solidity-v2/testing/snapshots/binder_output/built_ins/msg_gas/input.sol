// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function removed() internal view returns (uint256) {
        return msg.gas;                 // should be rejected
    }

    function replacement() internal view returns (uint256) {
        return gasleft();               // should be accepted
    }
}
