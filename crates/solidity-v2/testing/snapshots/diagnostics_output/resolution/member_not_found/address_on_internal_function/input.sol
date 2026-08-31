// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function internal_function() internal pure {}

    function external_function() external pure {}

    function missing() internal pure returns (address) {
        // `address` is a member of external function pointers only.
        return internal_function.address;
    }

    function present() internal view returns (address) {
        return this.external_function.address;
    }
}
