pragma solidity *;

function test(bytes memory data) pure {
    abi.decode(data, (uint256, (bool, bool)));
}
