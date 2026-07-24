// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./other.sol";

// The cycle spans two files importing each other.

contract A {
    function f() public {
        new B();
    }
}

// ---- path: other.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./main.sol";

contract B {
    function g() public {
        new A();
    }
}
