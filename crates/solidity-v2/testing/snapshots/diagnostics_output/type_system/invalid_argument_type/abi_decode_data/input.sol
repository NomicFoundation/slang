pragma solidity *;

function test(string memory data) pure {
    abi.decode(data, (uint256));
}
