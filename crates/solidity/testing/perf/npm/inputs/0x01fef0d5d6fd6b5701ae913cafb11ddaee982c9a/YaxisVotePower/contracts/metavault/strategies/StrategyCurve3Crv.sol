// SPDX-License-Identifier: MIT

pragma solidity 0.6.12;

import "../../interfaces/Gauge.sol";
import "../../interfaces/Balancer.sol";

import "./BaseStrategy.sol";

contract StrategyCurve3Crv is BaseStrategy {
    // used for Crv -> weth -> [dai/usdc/usdt] -> 3crv route
    address public immutable crv;

    // for add_liquidity via curve.fi to get back 3CRV (use getMostPremium() for the best stable coin used in the route)
    address public immutable dai;
    address public immutable usdc;
    address public immutable usdt;

    Mintr public immutable crvMintr;
    IStableSwap3Pool public immutable stableSwap3Pool;
    Gauge public immutable gauge; // 3Crv Gauge

    constructor(
        string memory _name,
        address _want,
        address _crv,
        address _weth,
        address _dai,
        address _usdc,
        address _usdt,
        Gauge _gauge,
        Mintr _crvMintr,
        IStableSwap3Pool _stableSwap3Pool,
        address _controller,
        address _vaultManager,
        address _router
    )
        public
        BaseStrategy(_name, _controller, _vaultManager, _want, _weth, _router)
    {
        require(_crv != address(0), "!_crv");
        require(_dai != address(0), "!_dai");
        require(_usdc != address(0), "!_usdc");
        require(_usdt != address(0), "!_usdt");
        require(address(_gauge) != address(0), "!_gauge");
        require(address(_crvMintr) != address(0), "!_crvMintr");
        require(address(_stableSwap3Pool) != address(0), "!_stableSwap3Pool");
        crv = _crv;
        dai = _dai;
        usdc = _usdc;
        usdt = _usdt;
        stableSwap3Pool = _stableSwap3Pool;
        gauge = _gauge;
        crvMintr = _crvMintr;
        IERC20(_want).safeApprove(address(_gauge), type(uint256).max);
        IERC20(_crv).safeApprove(address(_router), type(uint256).max);
        IERC20(_dai).safeApprove(address(_stableSwap3Pool), type(uint256).max);
        IERC20(_usdc).safeApprove(address(_stableSwap3Pool), type(uint256).max);
        IERC20(_usdt).safeApprove(address(_stableSwap3Pool), type(uint256).max);
        IERC20(_want).safeApprove(address(_stableSwap3Pool), type(uint256).max);
    }

    function _deposit() internal override {
        uint256 _wantBal = balanceOfWant();
        if (_wantBal > 0) {
            // deposit [want] to Gauge
            gauge.deposit(_wantBal);
        }
    }

    function _claimReward() internal {
        crvMintr.mint(address(gauge));
    }

    function _addLiquidity() internal {
        uint256[3] memory amounts;
        amounts[0] = IERC20(dai).balanceOf(address(this));
        amounts[1] = IERC20(usdc).balanceOf(address(this));
        amounts[2] = IERC20(usdt).balanceOf(address(this));
        stableSwap3Pool.add_liquidity(amounts, 1);
    }

    function getMostPremium() public view returns (address, uint256) {
        uint256[] memory balances = new uint256[](3);
        balances[0] = stableSwap3Pool.balances(0); // DAI
        balances[1] = stableSwap3Pool.balances(1).mul(10**12); // USDC
        balances[2] = stableSwap3Pool.balances(2).mul(10**12); // USDT

        if (balances[0] < balances[1] && balances[0] < balances[2]) { // DAI
            return (dai, 0);
        }

        if (balances[1] < balances[0] && balances[1] < balances[2]) { // USDC
            return (usdc, 1);
        }

        if (balances[2] < balances[0] && balances[2] < balances[1]) { // USDT
            return (usdt, 2);
        }

        return (dai, 0); // If they're somehow equal, we just want DAI
    }

    function _harvest() internal override {
        _claimReward();
        uint256 _remainingWeth = _payHarvestFees(crv);

        if (_remainingWeth > 0) {
            (address _stableCoin,) = getMostPremium(); // stablecoin we want to convert to
            _swapTokens(weth, _stableCoin, _remainingWeth);
            _addLiquidity();

            if (balanceOfWant() > 0) {
                _deposit();
            }
        }
    }

    function _withdrawAll() internal override {
        uint256 _bal = gauge.balanceOf(address(this));
        _withdraw(_bal);
    }

    function _withdraw(uint256 _amount) internal override {
        gauge.withdraw(_amount);
    }

    function balanceOfPool() public view override returns (uint) {
        return gauge.balanceOf(address(this));
    }
}
