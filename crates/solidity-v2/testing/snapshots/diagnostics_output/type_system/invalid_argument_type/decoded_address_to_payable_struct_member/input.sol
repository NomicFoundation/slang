// SPDX-License-Identifier: MIT
pragma solidity *;

struct Order {
    address payable maker;
}

function f(bytes memory data) pure returns (Order memory) {
    return Order({maker: abi.decode(data, (address))});
}
