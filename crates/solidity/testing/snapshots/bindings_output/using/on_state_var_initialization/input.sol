library Lib {
    function nop(uint256 x) returns (uint256) { return x; }
}
contract Test {
    using Lib for uint256;
    uint256 private v1 = 1;
    uint256 private v2 = v1.nop();
}
