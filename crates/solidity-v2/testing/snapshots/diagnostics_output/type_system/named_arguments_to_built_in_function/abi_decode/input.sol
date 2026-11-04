pragma solidity *;

function test(bytes memory input) pure {
    abi.decode({data: input, types: (uint256)});
}
