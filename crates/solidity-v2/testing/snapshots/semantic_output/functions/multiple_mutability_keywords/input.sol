contract Test {
    function foo() public pure view returns (uint) {
        return 1;
    }

    function withCallback(function () external pure view returns (uint) cb) public {}
}
