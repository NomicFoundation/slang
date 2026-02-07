contract Base {
    uint constant BASE_SIZE = 4;
}

contract Test is Base {
    uint constant TEST_SIZE = 4;

    using Lib for uint[4];

    function test() internal {
        uint[4] memory a;
        uint[TEST_SIZE] memory b;
        uint[BASE_SIZE] memory c;
        uint[(BASE_SIZE + TEST_SIZE) / 2] memory d;
        a.nop();
        b.nop();
        c.nop();
        d.nop();
    }
}

library Lib {
    function nop(uint[4] memory self) internal {}
}
