// SPDX-License-Identifier: MIT
pragma solidity *;

// `B.f` names the function through the contract type. The public `B.f` takes
// `memory` where the external `I.f` takes `calldata`, which an override of an
// external function may do. The call is internal, so the memory argument fits
// `B.f` alone and nothing is ambiguous.
interface I {
    function f(uint256[] calldata a) external returns (uint256);
}

contract B is I {
    function f(uint256[] memory a) public override returns (uint256) {
        return a.length;
    }
}

contract C is B {
    function g(uint256[] memory a) public returns (uint256) {
        return B.f(a);
    }
}
