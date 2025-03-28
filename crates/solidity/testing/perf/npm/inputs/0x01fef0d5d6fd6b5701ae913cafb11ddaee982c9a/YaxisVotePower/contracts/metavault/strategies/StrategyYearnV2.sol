// SPDX-License-Identifier: MIT

pragma solidity 0.6.12;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

import "../../interfaces/YearnV2.sol";
import "../IConverter.sol";
import "./BaseStrategy.sol";

contract StrategyYearnV2 is BaseStrategy {
    address public immutable yvToken;
    IConverter public converter;

    constructor(
        string memory _name,
        address _yvToken,
        address _underlying,
        address _converter,
        address _controller,
        address _vaultManager,
        address _weth,
        address _router
    )
        public
        BaseStrategy(
            _name,
            _controller,
            _vaultManager,
            _underlying,
            _weth,
            _router
        )
    {
        require(_yvToken != address(0), "!_yvToken");
        require(_converter != address(0), "!_converter");
        yvToken = _yvToken;
        converter = IConverter(_converter);
        IERC20(_underlying).safeApprove(_converter, type(uint256).max);
        IERC20(_underlying).safeApprove(_yvToken, type(uint256).max);
    }

    function balanceOfPool() public view override returns (uint256) {
        if (ERC20(yvToken).totalSupply() == 0) {
            return 0;
        }
        uint256 balance = IERC20(yvToken).balanceOf(address(this));
        return balance
            .mul(IYearnV2Vault(yvToken).pricePerShare())
            .div(1e18);
    }

    function _deposit() internal override {
        IYearnV2Vault(yvToken).deposit();
    }

    function _harvest() internal override {
        // TODO: add a way to harvest the interest earned amount
        return;
    }

    function _withdraw(uint256 _amount) internal override {
        IYearnV2Vault vaultToken = IYearnV2Vault(yvToken);
        _amount = _amount.mul(1e18).div(vaultToken.pricePerShare());
        vaultToken.withdraw(_amount);
        _amount = balanceOfWant();
        if (_amount > 0) {
            _convert(want, _vaultWant(), _amount);
        }
    }

    function _withdrawAll() internal override {
        uint256 balance = IERC20(yvToken).balanceOf(address(this));
        IYearnV2Vault(yvToken).withdraw();

        balance = balanceOfWant();
        if (balance > 0) {
            _convert(want, _vaultWant(), balance);
        }
    }

    function _convert(address _from, address _to, uint256 _amount) internal {
        require(converter.convert_rate(_from, _to, _amount) > 0, "!convert_rate");
        IERC20(_from).safeTransfer(address(converter), _amount);
        converter.convert(_from, _to, _amount);
    }
}
