// A function local is not visible to a later state variable, so the length
// is the constant.
pragma solidity *;

contract C {
    uint256 constant N = 2;

    function f() public pure {
        uint256 N = 1;
    }

    uint256[N] a;
}
