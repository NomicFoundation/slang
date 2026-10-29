// SPDX-License-Identifier: MIT
pragma solidity *;

contract D {
    constructor() payable {}
}

contract C {
    function missing() internal {
        // Only a contract creation takes call options.
        new uint256[]{value: 1}(3);
    }

    function present() internal {
        new D{value: 1}();
    }
}
