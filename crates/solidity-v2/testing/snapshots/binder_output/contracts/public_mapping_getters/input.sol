contract Test {
    function test(TokenState tokenState) public {
        tokenState.owners(1).balance;
    }
}
contract TokenState {
    mapping(uint => address) public owners;
}
