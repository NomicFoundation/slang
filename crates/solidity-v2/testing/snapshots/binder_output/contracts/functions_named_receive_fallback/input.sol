pragma solidity *;

contract Base {
    function receive(uint256 x) public pure virtual returns (uint256) {
        return x;
    }

    function fallback() public payable virtual {}
}

contract Derived is Base {
    receive() external payable {}

    fallback() external {}

    function receive(uint256 y) public pure override returns (uint256) {
        return y + 1;
    }

    function fallback() public payable override {}
}
