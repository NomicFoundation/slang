// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint8 a) internal pure returns (uint) {
        return a;
    }

    function f(uint256 a) internal pure returns (uint) {
        return 2 * a;
    }

    function g(uint256 a) internal pure returns (uint) {
        return a;
    }

    function ambiguous() internal pure returns (uint) {
        // Parentheses pass the typing of what they wrap straight through, but
        // an overload set is not a typing a value position accepts: it is
        // reported and sunk where the parentheses are. So the enclosing call
        // has nothing left to narrow down, even though `300` selects a unique
        // overload of the unwrapped `f(300)`.
        return (f)(300);
    }

    function unambiguous() internal pure returns (uint) {
        // A single declaration passes through untouched.
        return (g)(300);
    }
}
