library Decimal {
    struct D256 {
        uint256 value;
    }
    function from(uint256 x) internal returns (D256) {}
    function div(D256 memory v) internal returns (D256) {}
}
contract Test {
    using Decimal for Decimal.D256;
    function test() public {
        Decimal.from(1).div();
    }
}
