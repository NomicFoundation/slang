// SPDX-License-Identifier: MIT

pragma solidity 0.6.12;

interface IHarvester {
    function addStrategy(address, address, uint256) external;
    function removeStrategy(address, address, uint256) external;
}

