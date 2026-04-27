library Lib {
    function nop(uint x) internal {}
}
contract Test {
    using Lib for uint;
    function test(uint256[] memory data) public {
        data.length.nop();
    }
}
