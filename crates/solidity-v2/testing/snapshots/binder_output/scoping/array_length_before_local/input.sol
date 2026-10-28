// A local is not visible before its declaration, so the length is the
// constant.
pragma solidity *;

contract C {
    uint256 constant N = 2;

    function f() public pure {
        uint256[N] memory a;
        uint256 N = 1;
    }
}
