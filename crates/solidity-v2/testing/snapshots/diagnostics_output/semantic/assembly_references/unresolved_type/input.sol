// SPDX-License-Identifier: MIT
pragma solidity *;

// A variable whose type does not resolve is skipped by the assembly checks,
// whether it is a state variable or a local.
contract C {
    Missing x;

    function f() public {
        Missing m;
        assembly {
            let a := x.length
            x.slot := 1
            pop(x)
            let b := m.slot
            pop(m)
        }
    }
}
