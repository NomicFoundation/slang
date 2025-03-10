// SPDX-License-Identifier: MIT

pragma solidity ^0.6.0;


interface IGovernanceModule {
    function notifyStakeChanged(address account, uint256 newBalance) external;
    function notifyStakesChanged(address[] calldata accounts, uint256[] calldata newBalances) external;
}
