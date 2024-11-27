contract A {
    function total() public returns (uint256) {}
}
contract B is A {
    using Lib for uint256;
    function total() public returns (uint256) {
        return super.total().nop();
    }
}
library Lib {
    function nop(uint256 x) internal returns (uint256) {}
}
