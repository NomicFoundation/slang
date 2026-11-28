// SPDX-License-Identifier: MIT
pragma solidity *;

function pay(address payable[] memory) pure {}

function f(bytes memory data) pure {
    pay(abi.decode(data, (address[])));
}
