// A local variable is not visible in its own type name.
pragma solidity *;

contract Test {
    function test() public pure {
        uint[N] memory N;
    }
}
