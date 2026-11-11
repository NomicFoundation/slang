pragma solidity *;

contract Test {
    mapping(address owner => mapping(uint256 index => uint256 amount)) public balances;

    function test() public view returns (uint256) {
        return this.balances({index: 1, owner: address(0)});
    }
}
