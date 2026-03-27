contract Test {
    using Lib for uint256;
    function test() public {
        uint x;
        x.nop();
    }
}
library Lib {
    function nop(uint256 x) external {}
}
