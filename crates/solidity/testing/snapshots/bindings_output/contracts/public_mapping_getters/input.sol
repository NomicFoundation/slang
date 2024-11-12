library Lib {
    function nop(uint256 x) internal {}
}
contract Test {
    using Lib for uint;
    function test(TokenState tokenState) public {
        tokenState.balanceOf(msg.sender).nop();
    }
}
contract TokenState {
    mapping(address => uint) public balanceOf;
}
