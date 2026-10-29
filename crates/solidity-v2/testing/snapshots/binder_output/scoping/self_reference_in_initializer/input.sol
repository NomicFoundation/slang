// A local variable is not visible in its own initializer.
pragma solidity *;

contract Test {
    function test() public pure {
        uint x = x + 1;
    }
}
