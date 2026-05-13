interface IERC20Upgradable {
    function allowance(address owner) external returns (uint256);
}
library Math {
    function nop(uint256 x) public {}
}
library Test {
    using Math for uint256;

    function test(IERC20Upgradable token) internal {
        token.allowance(msg.sender).nop();
    }
}
