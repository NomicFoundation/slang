library Lib {
    function to160(uint256) internal returns (uint160) {}
}

contract Test {
    using Lib for uint256;

    function test() internal {
        uint256 ratio;
        ((ratio >> 32) + (ratio % (1 << 32) == 0 ? 0 : 1)).to160();
    }
}
