// SPDX-License-Identifier: MIT

pragma solidity 0.6.12;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/math/SafeMath.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";

import "../IStableSwap3Pool.sol";
import "../ISwap.sol";
import "../IVaultManager.sol";
import "../IStrategy.sol";
import "../IController.sol";

/**
 * @title BaseStrategy
 * @notice The BaseStrategy is an abstract contract which all
 * yAxis strategies should inherit functionality from. It gives
 * specific security properties which make it hard to write an
 * insecure strategy.
 * @notice All state-changing functions implemented in the strategy
 * should be internal, since any public or externally-facing functions
 * are already handled in the BaseStrategy.
 * @notice The following functions must be implemented by a strategy:
 * - function _deposit() internal virtual;
 * - function _harvest() internal virtual;
 * - function _withdraw(uint256 _amount) internal virtual;
 * - function _withdrawAll() internal virtual;
 * - function balanceOfPool() public view override virtual returns (uint256);
 */
abstract contract BaseStrategy is IStrategy {
    using SafeERC20 for IERC20;
    using Address for address;
    using SafeMath for uint256;

    uint256 public constant ONE_HUNDRED_PERCENT = 10000;

    address public immutable override want;
    address public immutable weth;
    address public immutable controller;
    IVaultManager public immutable vaultManager;

    string public override name;
    ISwap public router;

    event ApproveForSpender(address token, address spender, uint256 amount);
    event SetController(address controller);
    event SetRouter(address router);
    event Skim();
    event Withdraw(address vault, uint256 amount);

    /**
     * @param _controller The address of the controller
     * @param _vaultManager The address of the vaultManager
     * @param _want The desired token of the strategy
     * @param _weth The address of WETH
     * @param _router The address of the router for swapping tokens
     */
    constructor(
        string memory _name,
        address _controller,
        address _vaultManager,
        address _want,
        address _weth,
        address _router
    ) public {
        require(_controller != address(0), "!_controller");
        require(_vaultManager != address(0), "!_vaultManager");
        require(_want != address(0), "!_want");
        require(_weth != address(0), "!_weth");
        require(_router != address(0), "!_router");
        name = _name;
        want = _want;
        controller = _controller;
        vaultManager = IVaultManager(_vaultManager);
        weth = _weth;
        router = ISwap(_router);
        IERC20(_weth).safeApprove(address(_router), type(uint256).max);
    }

    /**
     * GOVERNANCE-ONLY FUNCTIONS
     */

    /**
     * @notice Approves a token address to be spent by an address
     * @param _token The address of the token
     * @param _spender The address of the spender
     * @param _amount The amount to spend
     */
    function approveForSpender(IERC20 _token, address _spender, uint256 _amount) external {
        require(msg.sender == vaultManager.governance(), "!governance");
        _token.safeApprove(_spender, _amount);
        emit ApproveForSpender(address(_token), _spender, _amount);
    }

    /**
     * @notice Sets the address of the ISwap-compatible router
     * @param _router The address of the router
     */
    function setRouter(address _router) external {
        require(msg.sender == vaultManager.governance(), "!governance");
        router = ISwap(_router);
        IERC20(weth).safeApprove(address(_router), 0);
        IERC20(weth).safeApprove(address(_router), type(uint256).max);
        emit SetRouter(_router);
    }

    /**
     * AUTHORIZED-ONLY FUNCTIONS
     */

    /**
     * @notice Deposits funds to the strategy's pool
     */
    function deposit() external override onlyAuthorized {
        _deposit();
    }

    /**
     * @notice Harvest funds in the strategy's pool
     */
    function harvest() external override onlyAuthorized {
        _harvest();
    }

    /**
     * @notice Sends stuck want tokens in the strategy to the controller
     */
    function skim() external override onlyAuthorized {
        IERC20(want).safeTransfer(controller, balanceOfWant());
        emit Skim();
    }

    /**
     * @notice Sends stuck tokens in the strategy to the controller
     * @param _asset The address of the token to withdraw
     */
    function withdraw(address _asset) external override onlyAuthorized {
        require(want != _asset, "want");

        IERC20 _assetToken = IERC20(_asset);
        uint256 _balance = _assetToken.balanceOf(address(this));
        _assetToken.safeTransfer(controller, _balance);
    }

    /**
     * @notice Initiated from a vault, withdraws funds from the pool
     * @param _amount The amount of the want token to withdraw
     */
    function withdraw(uint256 _amount) external override onlyAuthorized {
        uint256 _balance = balanceOfWant();
        if (_balance < _amount) {
            _amount = _withdrawSome(_amount.sub(_balance));
            _amount = _amount.add(_balance);
        }

        address _token = _vaultWant();
        address _vault = IController(controller).vaults(_token);
        require(_vault != address(0), "!vault"); // additional protection so we don't burn the funds
        IERC20(_token).safeTransfer(_vault, _amount);
        emit Withdraw(_vault, _amount);
    }

    /**
     * @notice Withdraws all funds from the strategy
     */
    function withdrawAll() external override onlyAuthorized {
        _withdrawAll();

        address _token = _vaultWant();
        uint256 _balance = IERC20(_token).balanceOf(address(this));

        address _vault = IController(controller).vaults(_token);
        require(_vault != address(0), "!vault"); // additional protection so we don't burn the funds
        IERC20(_token).safeTransfer(_vault, _balance);
        emit Withdraw(_vault, _balance);
    }

    /**
     * EXTERNAL VIEW FUNCTIONS
     */

    /**
     * @notice Returns the strategy's balance of the want token plus the balance of pool
     */
    function balanceOf() external override view returns (uint256) {
        return balanceOfWant().add(balanceOfPool());
    }

    /**
     * PUBLIC VIEW FUNCTIONS
     */

    /**
     * @notice Returns the balance of the pool
     * @dev Must be implemented by the strategy
     */
    function balanceOfPool() public view override virtual returns (uint256);

    /**
     * @notice Returns the balance of the want token on the strategy
     */
    function balanceOfWant() public view override returns (uint256) {
        return IERC20(want).balanceOf(address(this));
    }

    /**
     * INTERNAL FUNCTIONS
     */

    function _deposit() internal virtual;

    function _harvest() internal virtual;

    function _payHarvestFees(
        address _poolToken
    ) internal returns (uint256 _wethBal) {
        uint256 _amount = IERC20(_poolToken).balanceOf(address(this));
        _swapTokens(_poolToken, weth, _amount);
        _wethBal = IERC20(weth).balanceOf(address(this));

        if (_wethBal > 0) {
            // get all the necessary variables in a single call
            (
                address yax,
                address stakingPool,
                uint256 stakingPoolShareFee,
                address treasury,
                uint256 treasuryFee,
                address insurance,
                uint256 insurancePoolFee
            ) = vaultManager.getHarvestFeeInfo();

            uint256 _fee;

            // pay the staking pool with YAX
            if (stakingPoolShareFee > 0 && stakingPool != address(0)) {
                _fee = _wethBal.mul(stakingPoolShareFee).div(ONE_HUNDRED_PERCENT);
                _swapTokens(weth, yax, _fee);
                IERC20(yax).safeTransfer(stakingPool, IERC20(yax).balanceOf(address(this)));
            }

            // pay the treasury with YAX
            if (treasuryFee > 0 && treasury != address(0)) {
                _fee = _wethBal.mul(treasuryFee).div(ONE_HUNDRED_PERCENT);
                _swapTokens(weth, yax, _fee);
                IERC20(yax).safeTransfer(treasury, IERC20(yax).balanceOf(address(this)));
            }

            // pay the insurance pool with YAX
            if (insurancePoolFee > 0 && insurance != address(0)) {
                _fee = _wethBal.mul(insurancePoolFee).div(ONE_HUNDRED_PERCENT);
                _swapTokens(weth, yax, _fee);
                IERC20(yax).safeTransfer(insurance, IERC20(yax).balanceOf(address(this)));
            }

            // return the remaining WETH balance
            _wethBal = IERC20(weth).balanceOf(address(this));
        }
    }

    function _swapTokens(address _input, address _output, uint256 _amount) internal {
        address[] memory path = new address[](2);
        path[0] = _input;
        path[1] = _output;
        router.swapExactTokensForTokens(
            _amount,
            1,
            path,
            address(this),
            // solhint-disable-next-line not-rely-on-time
            block.timestamp.add(1800)
        );
    }

    function _vaultWant() internal returns (address) {
        return IController(controller).strategyTokens(address(this));
    }

    function _withdraw(uint256 _amount) internal virtual;

    function _withdrawAll() internal virtual;

    function _withdrawSome(uint256 _amount) internal returns (uint256) {
        address _token = _vaultWant();
        uint256 _before = IERC20(_token).balanceOf(address(this));
        _withdraw(_amount);
        uint256 _after = IERC20(_token).balanceOf(address(this));
        _amount = _after.sub(_before);

        return _amount;
    }

    /**
     * MODIFIERS
     */

    modifier onlyAuthorized() {
        require(msg.sender == controller
             || msg.sender == vaultManager.strategist()
             || msg.sender == vaultManager.governance(),
             "!authorized"
        );
        _;
    }
}
