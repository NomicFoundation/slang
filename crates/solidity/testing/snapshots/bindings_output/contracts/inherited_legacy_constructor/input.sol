contract token {
    mapping(address => uint256) public balanceOf;

    function token(uint256, string, string) public {}
}

contract Test is token {
    function merge(address target) {
        token old = token(address(0x7F2176cEB16dcb648dc924eff617c3dC2BEfd30d));
        old.balanceOf(target);
    }
}
