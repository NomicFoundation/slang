// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    constructor() payable {}
}

contract C {
    function missing() public returns (A) {
        return (new A).value(2)();              // should be rejected
    }

    function present() public returns (A) {
        return new A{value: 2}();              // should be accepted
    }
}
