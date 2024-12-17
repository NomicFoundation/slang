// SPDX-License-Identifier: MIT

pragma solidity 0.6.12;

import "../../interfaces/Idle.sol";
import "../IConverter.sol";
import "./BaseStrategy.sol";

contract StrategyIdle is BaseStrategy {
    address public immutable idleYieldToken;
    address public immutable IDLE;
    address public immutable COMP;
    IConverter public converter;

    constructor(
        string memory _name,
        address _underlying,
        address _idleYieldToken,
        address _IDLE,
        address _COMP,
        address _converter,
        address _controller,
        address _vaultManager,
        address _weth,
        address _router
    )
        public
        BaseStrategy(_name, _controller, _vaultManager, _underlying, _weth, _router)
    {
        require(_idleYieldToken != address(0), "!_idleYieldToken");
        require(_IDLE != address(0), "!_IDLE");
        require(_COMP != address(0), "!_COMP");
        require(_converter != address(0), "!_converter");
        idleYieldToken = _idleYieldToken;
        IDLE = _IDLE;
        COMP = _COMP;
        converter = IConverter(_converter);
        IERC20(_IDLE).safeApprove(address(_router), type(uint256).max);
        IERC20(_COMP).safeApprove(address(_router), type(uint256).max);
        IERC20(_underlying).safeApprove(address(_converter), type(uint256).max);
        IERC20(_underlying).safeApprove(_idleYieldToken, type(uint256).max);
    }

    function balanceOfPool() public view override returns (uint256) {
        uint256 balance = balanceOfYieldToken();
        return balance
            .mul(pricePerToken())
            .div(1e18);
    }

    function pricePerToken() public view returns (uint256) {
        return IIdleTokenV3_1(idleYieldToken).tokenPrice();
    }

    function balanceOfYieldToken() public view returns (uint256) {
        return IERC20(idleYieldToken).balanceOf(address(this));
    }

    function _deposit() internal override {
        uint256 balance = balanceOfWant();
        if (balance > 0) {
            IIdleTokenV3_1(idleYieldToken).mintIdleToken(balance, true, address(0));
        }
    }

    function _harvest() internal override {
        IIdleTokenV3_1(idleYieldToken).redeemIdleToken(0);
        uint256 remainingWeth = _payHarvestFees(IDLE);

        _liquidateAsset(COMP, want);

        if (remainingWeth > 0) {
            _swapTokens(weth, want, remainingWeth);
        }

        _deposit();
    }

    function _withdraw(uint256 _amount) internal override {
        _amount = _amount.mul(1e18).div(IIdleTokenV3_1(idleYieldToken).tokenPrice());
        IIdleTokenV3_1(idleYieldToken).redeemIdleToken(_amount);

        _liquidateAsset(COMP, want);
        _liquidateAsset(IDLE, want);

        _amount = balanceOfWant();
        if (_amount > 0) {
            _convert(want, _vaultWant(), _amount);
        }
    }

    function _withdrawAll() internal override {
        uint256 balance = balanceOfYieldToken();
        IIdleTokenV3_1(idleYieldToken).redeemIdleToken(balance);

        _liquidateAsset(COMP, want);
        _liquidateAsset(IDLE, want);

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

    function _liquidateAsset(address asset, address to) internal {
        uint256 assetBalance = IERC20(asset).balanceOf(address(this));
        if (assetBalance > 0) {
            _swapTokens(asset, to, assetBalance);
        }
    }
}
