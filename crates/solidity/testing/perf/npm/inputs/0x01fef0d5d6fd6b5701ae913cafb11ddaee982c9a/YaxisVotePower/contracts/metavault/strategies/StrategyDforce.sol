// SPDX-License-Identifier: MIT

pragma solidity 0.6.12;

import "../../interfaces/DForce.sol";
import "../IConverter.sol";
import "./BaseStrategy.sol";

contract StrategyDforce is BaseStrategy {
    address public immutable dToken;
    address public immutable pool;
    address public immutable DF;
    IConverter public converter;

    constructor(
        string memory _name,
        address _underlying,
        address _dToken,
        address _pool,
        address _DF,
        address _converter,
        address _controller,
        address _vaultManager,
        address _weth,
        address _router
    )
        public
        BaseStrategy(_name, _controller, _vaultManager, _underlying, _weth, _router)
    {
        require(_dToken != address(0), "!_dToken");
        require(_pool != address(0), "!_pool");
        require(_DF != address(0), "!_DF");
        require(_converter != address(0), "!_converter");
        dToken = _dToken;
        pool = _pool;
        DF = _DF;
        converter = IConverter(_converter);
        IERC20(_DF).safeApprove(address(_router), type(uint256).max);
        IERC20(_underlying).safeApprove(address(_converter), type(uint256).max);
        IERC20(_underlying).safeApprove(_dToken, type(uint256).max);
        IERC20(_dToken).safeApprove(_pool, type(uint256).max);
    }

    function balanceOfPool() public view override returns (uint256) {
        return (dRewards(pool).balanceOf(address(this)))
            .mul(dERC20(dToken).getExchangeRate())
            .div(1e18)
            .add(balanceOfdToken());
    }

    function balanceOfdToken() public view returns (uint256) {
        return dERC20(dToken).getTokenBalance(address(this));
    }

    function _deposit() internal override {
        uint256 _amount = balanceOfWant();
        if (_amount > 0) {
            dERC20(dToken).mint(address(this), _amount);
        }
        uint256 _dToken = IERC20(dToken).balanceOf(address(this));
        if (_dToken > 0) {
            dRewards(pool).stake(_dToken);
        }
    }

    function _harvest() internal override {
        dRewards(pool).getReward();
        uint256 _remainingWeth = _payHarvestFees(DF);

        if (_remainingWeth > 0) {
            _swapTokens(weth, want, _remainingWeth);

            if (balanceOfWant() > 0) {
                _deposit();
            }
        }
    }

    function _withdraw(uint256 _amount) internal override {
        _amount = _amount.mul(1e18).div(dERC20(dToken).getExchangeRate());
        uint256 _before = IERC20(dToken).balanceOf(address(this));
        dRewards(pool).withdraw(_amount);
        uint256 _after = IERC20(dToken).balanceOf(address(this));
        _amount = _after.sub(_before);
        dERC20(dToken).redeem(address(this), _amount);
        _amount = balanceOfWant();
        if (_amount > 0) {
            _convert(want, _vaultWant(), _amount);
        }
    }

    function _withdrawAll() internal override {
        dRewards(pool).exit();
        uint256 _amount = IERC20(dToken).balanceOf(address(this));
        if (_amount > 0) {
            dERC20(dToken).redeem(address(this), _amount);
            _amount = balanceOfWant();
            _convert(want, _vaultWant(), _amount);
        }
    }

    function _convert(address _from, address _to, uint256 _amount) internal {
        require(converter.convert_rate(_from, _to, _amount) > 0, "!convert_rate");
        IERC20(_from).safeTransfer(address(converter), _amount);
        converter.convert(_from, _to, _amount);
    }
}
