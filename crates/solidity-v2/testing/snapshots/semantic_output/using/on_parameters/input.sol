library Lib {
    function nop(uint256 x) internal {}
}
contract Test {
    using Lib for uint256;
    function test(uint256 x) public returns (uint256 y) {
        x.nop();
        y.nop();
    }
}
