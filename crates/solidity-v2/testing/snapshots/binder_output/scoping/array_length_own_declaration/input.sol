// A local is not visible in its own type, so the length is the constant.
pragma solidity *;

contract C {
    uint256 constant N = 2;

    function f() public pure {
        uint256[N] memory N;
    }
}
