// --- path: a.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

struct S {
    uint256 x;
}

using {f} for S global;

function gen() pure returns (S memory) {}

function f(S memory _x) pure returns (uint256) {
    return _x.x;
}

function f1(S memory _x) pure returns (uint256) {
    return _x.x + 1;
}

// --- path: b.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import {gen as g, f1 as f, S} from "./a.sol";

contract C {
    // Here `f` is `a.sol`'s `f1`, so two different functions end up attached
    // to `S` under the name `f`: this one and the global one.
    using {f} for S;

    function ambiguous() public pure returns (uint256) {
        return g().f();
    }
}
