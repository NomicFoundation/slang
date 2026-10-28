// A local declared in a block is not visible after it, so the length is the
// constant.
pragma solidity *;

contract C {
    uint256 constant N = 2;

    function f() public pure {
        {
            uint256 N = 1;
        }
        uint256[N] memory a;
    }
}
