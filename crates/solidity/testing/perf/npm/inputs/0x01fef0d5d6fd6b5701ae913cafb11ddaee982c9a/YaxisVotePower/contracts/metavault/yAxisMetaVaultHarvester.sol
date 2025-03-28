// SPDX-License-Identifier: MIT

pragma solidity 0.6.12;

import "@openzeppelin/contracts/math/SafeMath.sol";

import "./IController.sol";
import "./IHarvester.sol";
import "./IVaultManager.sol";

/**
 * @title yAxisMetaVaultHarvester
 * @notice This contract is to be used as a central point to call
 * harvest on all strategies for any given token. It has its own
 * permissions for harvesters (set by the strategist or governance).
 */
contract yAxisMetaVaultHarvester is IHarvester { // solhint-disable-line contract-name-camelcase
    using SafeMath for uint256;

    IVaultManager public vaultManager;
    IController public controller;

    struct Strategy {
        uint256 timeout;
        uint256 lastCalled;
        address[] addresses;
    }

    mapping(address => Strategy) public strategies;
    mapping(address => bool) public isHarvester;

    /**
     * @notice Logged when a controller is set
     */
    event ControllerSet(address indexed controller);

    /**
     * @notice Logged when harvest is called for a strategy
     */
    event Harvest(
        address indexed controller,
        address indexed strategy
    );

    /**
     * @notice Logged when a harvester is set
     */
    event HarvesterSet(address indexed harvester, bool status);

    /**
     * @notice Logged when a strategy is added for a token
     */
    event StrategyAdded(address indexed token, address indexed strategy, uint256 timeout);

    /**
     * @notice Logged when a strategy is removed for a token
     */
    event StrategyRemoved(address indexed token, address indexed strategy, uint256 timeout);

    /**
     * @notice Logged when a vault manger is set
     */
    event VaultManagerSet(address indexed vaultManager);

    /**
     * @param _vaultManager The address of the yAxisMetaVaultManager contract
     * @param _controller The address of the controller
     */
    constructor(address _vaultManager, address _controller) public {
        vaultManager = IVaultManager(_vaultManager);
        controller = IController(_controller);
    }

    /**
     * (GOVERNANCE|STRATEGIST)-ONLY FUNCTIONS
     */

    /**
     * @notice Adds a strategy to the rotation for a given token and sets a timeout
     * @param _token The address of the token
     * @param _strategy The address of the strategy
     * @param _timeout The timeout between harvests
     */
    function addStrategy(
        address _token,
        address _strategy,
        uint256 _timeout
    ) external override onlyStrategist {
        strategies[_token].addresses.push(_strategy);
        strategies[_token].timeout = _timeout;
        emit StrategyAdded(_token, _strategy, _timeout);
    }

    /**
     * @notice Removes a strategy from the rotation for a given token and sets a timeout
     * @param _token The address of the token
     * @param _strategy The address of the strategy
     * @param _timeout The timeout between harvests
     */
    function removeStrategy(
        address _token,
        address _strategy,
        uint256 _timeout
    ) external override onlyStrategist {
        uint256 tail = strategies[_token].addresses.length;
        uint256 index;
        bool found;
        for (uint i; i < tail; i++) {
            if (strategies[_token].addresses[i] == _strategy) {
                index = i;
                found = true;
                break;
            }
        }

        if (found) {
            strategies[_token].addresses[index] = strategies[_token].addresses[tail.sub(1)];
            strategies[_token].addresses.pop();
            strategies[_token].timeout = _timeout;
            emit StrategyRemoved(_token, _strategy, _timeout);
        }
    }

    /**
     * @notice Sets the address of the controller
     * @param _controller The address of the controller
     */
    function setController(IController _controller) external onlyStrategist {
        controller = _controller;
        emit ControllerSet(address(_controller));
    }

    /**
     * @notice Sets the status of a harvester address to be able to call harvest functions
     * @param _harvester The address of the harvester
     * @param _status The status to allow the harvester to harvest
     */
    function setHarvester(address _harvester, bool _status) public onlyStrategist {
        isHarvester[_harvester] = _status;
        emit HarvesterSet(_harvester, _status);
    }

    /**
     * @notice Sets the address of the vault manager contract
     * @param _vaultManager The address of the vault manager
     */
    function setVaultManager(address _vaultManager) external onlyStrategist {
        vaultManager = IVaultManager(_vaultManager);
        emit VaultManagerSet(_vaultManager);
    }

    /**
     * (GOVERNANCE|STRATEGIST|HARVESTER)-ONLY FUNCTIONS
     */

    /**
     * @notice Harvests a given strategy on the provided controller
     * @dev This function ignores the timeout
     * @param _controller The address of the controller
     * @param _strategy The address of the strategy
     */
    function harvest(
        IController _controller,
        address _strategy
    ) public onlyHarvester {
        _controller.harvestStrategy(_strategy);
        emit Harvest(address(_controller), _strategy);
    }

    /**
     * @notice Harvests the next available strategy for a given token and
     * rotates the strategies
     * @param _token The address of the token
     */
    function harvestNextStrategy(address _token) external {
        require(canHarvest(_token), "!canHarvest");
        address strategy = strategies[_token].addresses[0];
        harvest(controller, strategy);
        uint256 k = strategies[_token].addresses.length;
        if (k > 1) {
            address[] memory _strategies = new address[](k);
            for (uint i; i < k-1; i++) {
                _strategies[i] = strategies[_token].addresses[i+1];
            }
            _strategies[k-1] = strategy;
            strategies[_token].addresses = _strategies;
        }
        // solhint-disable-next-line not-rely-on-time
        strategies[_token].lastCalled = block.timestamp;
    }

    /**
     * EXTERNAL VIEW FUNCTIONS
     */

    /**
     * @notice Returns the addresses of the strategies for a given token
     * @param _token The address of the token
     */
    function strategyAddresses(address _token) external view returns (address[] memory) {
        return strategies[_token].addresses;
    }

    /**
     * PUBLIC VIEW FUNCTIONS
     */

    /**
     * @notice Returns the availability of a token's strategy to be harvested
     * @param _token The address of the token
     */
    function canHarvest(address _token) public view returns (bool) {
        Strategy storage strategy = strategies[_token];
        if (strategy.addresses.length == 0 ||
            // solhint-disable-next-line not-rely-on-time
            strategy.lastCalled > block.timestamp.sub(strategy.timeout)) {
            return false;
        }
        return true;
    }

    /**
     * MODIFIERS
     */

    modifier onlyHarvester() {
        require(isHarvester[msg.sender], "!harvester");
        _;
    }

    modifier onlyStrategist() {
        require(vaultManager.controllers(msg.sender)
             || msg.sender == vaultManager.strategist()
             || msg.sender == vaultManager.governance(),
             "!strategist"
        );
        _;
    }
}
