// SPDX-License-Identifier: MIT

pragma solidity 0.6.12;

import "@openzeppelin/contracts/math/SafeMath.sol";

import "./IStableSwap3PoolOracle.sol";
import "../interfaces/Chainlink.sol";

contract StableSwap3PoolOracle is IStableSwap3PoolOracle {
    using SafeMath for uint256;

    uint256 public constant MAX_ROUND_TIME = 1 hours;
    uint256 public constant MAX_STALE_ANSWER = 24 hours;
    uint256 public constant ETH_USD_MUL = 1e10; // ETH-USD feed is to 8 decimals

    address public immutable ethUsd;
    address[3] public feeds;

    constructor(
        address _feedETHUSD,
        address _feedDAIETH,
        address _feedUSDCETH,
        address _feedUSDTETH
    )
        public
    {
        ethUsd = _feedETHUSD;
        feeds[0] = _feedDAIETH;
        feeds[1] = _feedUSDCETH;
        feeds[2] = _feedUSDTETH;
    }

    /**
     * @notice Retrieves the current price of ETH/USD as provided by Chainlink
     * @dev Reverts if the answer from Chainlink is not safe
     */
    function getEthereumPrice() external view override returns (uint256 _price) {
        _price = getSafeAnswer(ethUsd);
        require(_price > 0, "!getEthereumPrice");
        _price = _price.mul(ETH_USD_MUL);

    }

    /**
     * @notice Retrieves the minimum price of the 3pool tokens as provided by Chainlink
     * @dev Reverts if none of the Chainlink nodes are safe
     */
    function getPrices() external view override returns (uint256 _minPrice, uint256 _maxPrice) {
        for (uint8 i = 0; i < 3; i++) {
            // get the safe answer from Chainlink
            uint256 _answer = getSafeAnswer(feeds[i]);

            // store the first iteration regardless (handle that later if 0)
            // otherwise,check that _answer is greater than 0 and only store it if less
            // than the previously observed price
            if (i == 0) {
                _minPrice = _answer;
                _maxPrice = _answer;
            } else if (_answer > 0 && _answer < _minPrice) {
                _minPrice = _answer;
            } else if (_answer > 0 && _answer > _maxPrice) {
                _maxPrice = _answer;
            }
        }

        // if we couldn't get a valid price from any of the Chainlink feeds,
        // revert because nothing is safe
        require(_minPrice > 0 && _maxPrice > 0, "!getPrices");
    }

    /**
     * @notice Get and check the answer provided by Chainlink
     * @param _feed The address of the Chainlink price feed
     */
    function getSafeAnswer(address _feed) public view override returns (uint256) {
        (
            uint80 roundId,
            int256 answer,
            uint256 startedAt,
            uint256 updatedAt,
            uint80 answeredInRound
        ) = AggregatorV3Interface(_feed).latestRoundData();

        // latest round is carried over from previous round
        if (answeredInRound < roundId) {
            return 0;
        }

        // latest answer is stale
        // solhint-disable-next-line not-rely-on-time
        if (updatedAt < block.timestamp.sub(MAX_STALE_ANSWER)) {
            return 0;
        }

        // round has taken too long to collect answers
        if (updatedAt.sub(startedAt) > MAX_ROUND_TIME) {
            return 0;
        }

        // Chainlink already rejects answers outside of a range (like what would cause
        // a negative answer)
        return uint256(answer);
    }
}
