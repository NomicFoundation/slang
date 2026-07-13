contract Test {
    using Utils for uint;

    function test() public {
        type(uint).min.nop();
        type(uint).max.nop();
    }
}

library Utils {
    function nop(uint x) internal {
        return x;
    }
}
