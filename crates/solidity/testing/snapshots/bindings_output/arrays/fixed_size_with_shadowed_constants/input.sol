uint constant X = 1;
uint constant Y = X + 3;

contract Test {
    uint constant X = 2; // shadows the global constant

    using Lib for uint[4];
    using Lib for uint[5];

    function test() internal {
        uint[Y] memory x;
        x.nop(); // should bind to the first definition of nop() only
    }
}

library Lib {
    function nop(uint[4] memory self) internal {}
    function nop(uint[5] memory self) internal {}
}
