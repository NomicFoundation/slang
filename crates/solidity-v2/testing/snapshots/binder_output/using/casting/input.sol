interface IERC20 {
}
library SafeERC20 {
    function safeApprove(IERC20 token) internal {}
}
contract Test {
    using SafeERC20 for IERC20;
    function test(address token) public {
        IERC20(token).safeApprove();
    }
}
