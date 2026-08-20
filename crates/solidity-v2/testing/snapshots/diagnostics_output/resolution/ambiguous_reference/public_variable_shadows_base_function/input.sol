// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    function x(uint256) public pure returns (uint256) {
        return 1;
    }
}

contract B is A {
    uint256 public x;

    // Negative control: `x` matches both the state variable and the base
    // function, but only one of the candidates is a variable, so the reference
    // denotes it and is not ambiguous.
    function f() internal view returns (uint256) {
        return x;
    }
}
