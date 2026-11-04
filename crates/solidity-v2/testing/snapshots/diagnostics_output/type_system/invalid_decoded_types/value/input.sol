pragma solidity *;

function test(bytes memory data) pure {
    abi.decode(data, 1);
}
