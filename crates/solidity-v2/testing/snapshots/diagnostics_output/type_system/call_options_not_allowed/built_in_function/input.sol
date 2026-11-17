// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function missing() internal pure {
        keccak256{gas: 1000}("");
    }

    function present() internal {
        address(this).call{gas: 1000}("");
        address(this).delegatecall{gas: 1000}("");
        address(this).staticcall{gas: 1000}("");
    }
}
