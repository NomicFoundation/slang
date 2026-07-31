// SPDX-License-Identifier: MIT
pragma solidity *;

// In Derived's bytecode, `f()` runs the empty override, so Derived does not
// depend on itself. Only Base embeds Derived's bytecode, which is no cycle.

contract Base {
    uint256 counter;

    function f() internal virtual {
        new Derived();
    }

    function g() public {
        f();
    }
}

contract Derived is Base {
    function f() internal override {
        counter = 1;
    }
}
