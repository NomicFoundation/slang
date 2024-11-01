contract BasicToken {
    function balanceOf() public returns (uint256);
}
contract Token is BasicToken {
}
library Lib {
    function min256(uint256 a) internal {}
}
contract Test {
    using Lib for uint256;
    Token token;
    function test() public {
        token.balanceOf().min256();
    }
}
