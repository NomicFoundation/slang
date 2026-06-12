contract Test {
    function test() public pure returns (uint) {
        uint x = 1;
        uint y = x;
        uint x = 2;
        return x + y;
    }
}
