contract Test {
    mapping(address owner => mapping(uint256 index => uint256)) private _ownedTokens;

    function test(address from) public {
        mapping(uint256 index => uint256) storage _ownedTokensByOwner = _ownedTokens[from];
    }
}
