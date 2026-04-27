contract Fallback {
    event Log(string func, uint256 gas);
    uint status;

    fallback(bytes calldata input) external payable returns (bytes memory output) {
        emit Log("fallback", status);
        output = input;
    }

    receive() external payable {
        emit Log("receive", status);
    }
}
