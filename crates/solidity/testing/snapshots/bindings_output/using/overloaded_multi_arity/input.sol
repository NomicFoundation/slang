library Lib {
    function nop(uint256 a, uint256 b) internal {}
    function nop(uint256 a, uint256 b, uint256 c) internal {}
}

contract Test {
    using Lib for uint256;

    function test(uint256 x, uint256 y, uint256 z) internal {
        x.nop(y, z);
    }
}
