contract Test {
    function test() public pure returns (uint) {
        TopLevel tl = TopLevel.wrap(20);
        return TopLevel.unwrap(tl);
    }
}

type TopLevel is uint256;
