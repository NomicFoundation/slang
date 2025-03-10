// SPDX-License-Identifier: MIT

pragma solidity ^0.6.2;

import "../../interfaces/FlamIncome.sol";
import "./StrategyGenericVault.sol";

contract StrategyFlamIncome is StrategyGenericVault {
    using SafeERC20 for IERC20;
    using Address for address;
    using SafeMath for uint256;

    /**
     * @param _vault The address of the vault
     * @param _converter The address of the converter
     * @param _controller The address of the controller
     * @param _vaultManager The address of the vaultManager
     * @param _weth The address of WETH
     * @param _router The address of the router for swapping tokens
     */
    constructor(
        address _vault,
        address _converter,
        address _controller,
        address _vaultManager,
        address _weth,
        address _router
    )
        public
        StrategyGenericVault(
            string(abi.encodePacked("FlamIncome: ", ERC20(IVault(_vault).token()).symbol())),
            _vault,
            _converter,
            _controller,
            _vaultManager,
            _weth,
            _router
        )
    {}

    function getPricePerFullShare() public view override returns(uint256) {
        return priceE18();
    }

    function priceE18() public view returns (uint256) {
        return IVault(vault).priceE18();
    }
}
