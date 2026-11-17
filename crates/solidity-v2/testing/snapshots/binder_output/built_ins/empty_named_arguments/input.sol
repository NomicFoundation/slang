pragma solidity *;

library Lib {
    function nop(uint256) internal {}
}

contract Test {
    using Lib for uint256;

    function test() internal view {
        gasleft({}).nop();
    }
}
