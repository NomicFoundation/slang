// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function internal_function() internal pure {}

    function external_function() external pure {}

    function missing() internal pure returns (bytes4) {
        // `selector` is a member of external function pointers only.
        return internal_function.selector;
    }

    function present() internal view returns (bytes4) {
        return this.external_function.selector;
    }
}
