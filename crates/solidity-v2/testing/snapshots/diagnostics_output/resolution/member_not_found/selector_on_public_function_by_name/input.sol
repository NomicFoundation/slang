// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public {}

    function missing() internal pure returns (bytes4) {
        // Named directly, a public function is called internally.
        return g.selector;
    }

    function present() internal view returns (bytes4) {
        return this.g.selector;
    }
}
