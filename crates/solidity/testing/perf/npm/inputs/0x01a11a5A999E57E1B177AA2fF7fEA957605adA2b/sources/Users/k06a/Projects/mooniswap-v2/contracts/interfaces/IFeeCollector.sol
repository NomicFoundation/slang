// SPDX-License-Identifier: MIT

pragma solidity ^0.6.0;


interface IFeeCollector {
    function updateReward(address receiver, uint256 amount) external;
    function updateRewards(address[] calldata receivers, uint256[] calldata amounts) external;
}
