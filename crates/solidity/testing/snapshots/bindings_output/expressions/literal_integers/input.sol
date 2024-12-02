library Lib {
    function nop(uint256 x) internal {}
}
contract Test {
    using Lib for uint256;
    function test() public {
        (50 * 10**uint(4)).nop();
    }
}
