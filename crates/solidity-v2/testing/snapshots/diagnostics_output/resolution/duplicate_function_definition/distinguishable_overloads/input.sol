// SPDX-License-Identifier: MIT
pragma solidity *;

// Nothing here is a duplicate: every same-named pair can be told apart.

struct S {
    uint256 x;
}

library L {
    // A storage pointer is a different kind of argument from a memory copy,
    // so these two stay distinguishable.
    function f(S storage a) internal view returns (uint256) {
        return a.x;
    }

    function f(S memory a) internal pure returns (uint256) {
        return a.x;
    }
}

contract Base {
    function g(uint256 a) public pure virtual returns (uint256) {
        return a;
    }
}

// A function matching an inherited one overrides it rather than duplicating it.
contract Derived is Base {
    function g(uint256 a) public pure override returns (uint256) {
        return a + 1;
    }
}

contract Special {
    uint256 public total;

    // The special functions are unnamed, so they never take part in the check;
    // each of them has its own at-most-one rule.
    constructor() {
        total = 1;
    }

    fallback() external payable {
        total += 1;
    }

    receive() external payable {
        total += 2;
    }
}
