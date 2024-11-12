library Lib {
    function nop(uint256 x) internal {}
}
contract Test {
    using Lib for uint;
    function test(TokenState tokenState) public {
        tokenState.values(1).nop();
    }
}
contract TokenState {
    uint[] public values;
}
