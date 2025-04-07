// SPDX-License-Identifier: MIT

pragma solidity 0.6.12;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";

import "./IVaultManager.sol";

/**
 * @title yAxisMetaVaultManager
 * @notice This contract serves as the central point for governance-voted
 * variables. Fees and permissioned addresses are stored and referenced in
 * this contract only.
 */
contract yAxisMetaVaultManager is IVaultManager { // solhint-disable-line contract-name-camelcase
    using SafeERC20 for IERC20;

    address public override governance;
    address public override harvester;
    address public override insurancePool;
    address public override stakingPool;
    address public override strategist;
    address public override treasury;
    address public override yax;

    /**
     *  The following fees are all mutable.
     *  They are updated by governance (community vote).
     */
    uint256 public override insuranceFee;
    uint256 public override insurancePoolFee;
    uint256 public override stakingPoolShareFee;
    uint256 public override treasuryBalance;
    uint256 public override treasuryFee;
    uint256 public override withdrawalProtectionFee;

    mapping(address => bool) public override vaults;
    mapping(address => bool) public override controllers;

    /**
     * @param _yax The address of the YAX token
     */
    constructor(address _yax) public {
        yax = _yax;
        governance = msg.sender;
        strategist = msg.sender;
        harvester = msg.sender;
        stakingPoolShareFee = 2000;
        treasuryBalance = 20000e18;
        treasuryFee = 500;
        withdrawalProtectionFee = 10;
    }

    /**
     * GOVERNANCE-ONLY FUNCTIONS
     */

    /**
     * @notice Allows governance to pull tokens out of this contract
     * (it should never hold tokens)
     * @param _token The address of the token
     * @param _amount The amount to withdraw
     * @param _to The address to send to
     */
    function governanceRecoverUnsupported(
        IERC20 _token,
        uint256 _amount,
        address _to
    ) external {
        require(msg.sender == governance, "!governance");
        _token.safeTransfer(_to, _amount);
    }

    /**
     * @notice Sets the governance address
     * @param _governance The address of the governance
     */
    function setGovernance(address _governance) external {
        require(msg.sender == governance, "!governance");
        governance = _governance;
    }

    /**
     * @notice Sets the insurance fee
     * @dev Throws if setting fee over 1%
     * @param _insuranceFee The value for the insurance fee
     */
    function setInsuranceFee(uint256 _insuranceFee) public {
        require(msg.sender == governance, "!governance");
        require(_insuranceFee <= 100, "_insuranceFee over 1%");
        insuranceFee = _insuranceFee;
    }

    /**
     * @notice Sets the insurance pool address
     * @param _insurancePool The address of the insurance pool
     */
    function setInsurancePool(address _insurancePool) public {
        require(msg.sender == governance, "!governance");
        insurancePool = _insurancePool;
    }

    /**
     * @notice Sets the insurance pool fee
     * @dev Throws if setting fee over 20%
     * @param _insurancePoolFee The value for the insurance pool fee
     */
    function setInsurancePoolFee(uint256 _insurancePoolFee) public {
        require(msg.sender == governance, "!governance");
        require(_insurancePoolFee <= 2000, "_insurancePoolFee over 20%");
        insurancePoolFee = _insurancePoolFee;
    }

    /**
     * @notice Sets the staking pool address
     * @param _stakingPool The address of the staking pool
     */
    function setStakingPool(address _stakingPool) public {
        require(msg.sender == governance, "!governance");
        stakingPool = _stakingPool;
    }

    /**
     * @notice Sets the staking pool share fee
     * @dev Throws if setting fee over 50%
     * @param _stakingPoolShareFee The value for the staking pool fee
     */
    function setStakingPoolShareFee(uint256 _stakingPoolShareFee) public {
        require(msg.sender == governance, "!governance");
        require(_stakingPoolShareFee <= 5000, "_stakingPoolShareFee over 50%");
        stakingPoolShareFee = _stakingPoolShareFee;
    }

    /**
     * @notice Sets the strategist address
     * @param _strategist The address of the strategist
     */
    function setStrategist(address _strategist) external {
        require(msg.sender == governance, "!governance");
        strategist = _strategist;
    }

    /**
     * @notice Sets the treasury address
     * @param _treasury The address of the treasury
     */
    function setTreasury(address _treasury) public {
        require(msg.sender == governance, "!governance");
        treasury = _treasury;
    }

    /**
     * @notice Sets the maximum treasury balance
     * @dev Strategies will read this value to determine whether or not
     * to give the treasury the treasuryFee
     * @param _treasuryBalance The maximum balance of the treasury
     */
    function setTreasuryBalance(uint256 _treasuryBalance) public {
        require(msg.sender == governance, "!governance");
        treasuryBalance = _treasuryBalance;
    }

    /**
     * @notice Sets the treasury fee
     * @dev Throws if setting fee over 20%
     * @param _treasuryFee The value for the treasury fee
     */
    function setTreasuryFee(uint256 _treasuryFee) public {
        require(msg.sender == governance, "!governance");
        require(_treasuryFee <= 2000, "_treasuryFee over 20%");
        treasuryFee = _treasuryFee;
    }

    /**
     * @notice Sets the withdrawal protection fee
     * @dev Throws if setting fee over 1%
     * @param _withdrawalProtectionFee The value for the withdrawal protection fee
     */
    function setWithdrawalProtectionFee(uint256 _withdrawalProtectionFee) public {
        require(msg.sender == governance, "!governance");
        require(_withdrawalProtectionFee <= 100, "_withdrawalProtectionFee over 1%");
        withdrawalProtectionFee = _withdrawalProtectionFee;
    }

    /**
     * @notice Sets the YAX address
     * @param _yax The address of the YAX token
     */
    function setYax(address _yax) external {
        require(msg.sender == governance, "!governance");
        yax = _yax;
    }

    /**
     * (GOVERNANCE|STRATEGIST)-ONLY FUNCTIONS
     */

    /**
     * @notice Sets the status for a controller
     * @param _controller The address of the controller
     * @param _status The status of the controller
     */
    function setControllerStatus(address _controller, bool _status) external {
        require(msg.sender == strategist || msg.sender == governance, "!strategist");
        controllers[_controller] = _status;
    }

    /**
     * @notice Sets the harvester address
     * @param _harvester The address of the harvester
     */
    function setHarvester(address _harvester) external {
        require(msg.sender == strategist || msg.sender == governance, "!strategist");
        harvester = _harvester;
    }

    /**
     * @notice Sets the status for a vault
     * @param _vault The address of the vault
     * @param _status The status of the vault
     */
    function setVaultStatus(address _vault, bool _status) external {
        require(msg.sender == strategist || msg.sender == governance, "!strategist");
        vaults[_vault] = _status;
    }

    /**
     * EXTERNAL VIEW FUNCTIONS
     */

    /**
     * @notice Returns a tuple of:
     *     YAX token,
     *     Staking pool address,
     *     Staking pool share fee,
     *     Treasury address,
     *     Checks the balance of the treasury and returns the treasury fee
     *         if below the treasuryBalance, or 0 if above
     */
    function getHarvestFeeInfo()
        external
        view
        override
        returns (address, address, uint256, address, uint256, address, uint256)
    {
        return (
            yax,
            stakingPool,
            stakingPoolShareFee,
            treasury,
            IERC20(yax).balanceOf(treasury) >= treasuryBalance ? 0 : treasuryFee,
            insurancePool,
            insurancePoolFee
        );
    }
}
