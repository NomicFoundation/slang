// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() public payable {}

    function missing() internal {
        // Named directly, a public function is called internally.
        g{value: 1}();
    }

    function present() internal {
        this.g{value: 1}();
    }
}
