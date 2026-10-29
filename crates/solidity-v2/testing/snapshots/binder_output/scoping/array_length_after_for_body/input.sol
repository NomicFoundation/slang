// A local declared in a loop body is not visible after the loop, so the
// length is the constant.
pragma solidity *;

contract C {
    uint256 constant N = 2;

    function f() public pure {
        for (uint256 i = 0; i < 1; i++) {
            uint256 N = i;
        }
        uint256[N] memory a;
    }
}
