// SPDX-License-Identifier: MIT
pragma solidity 0.6.12;

import "@openzeppelin/contracts/math/SafeMath.sol";

import "./interfaces/IMasterChef.sol";
import "./interfaces/IUniswapV2Pair.sol";
import "./interfaces/IVoteProxy.sol";
import "./interfaces/IYaxisBar.sol";

contract YaxisVotePower is IVoteProxy {
    using SafeMath for uint256;

    uint256 public constant PID = 6;
    // solhint-disable-next-line const-name-snakecase
    uint8 public constant override decimals = uint8(18);

    // ETH/YAX token
    IUniswapV2Pair public immutable yaxEthUniswapV2Pair;

    // YAX token
    IERC20 public immutable yax;

    // YaxisChef contract
    IMasterChef public immutable chef;

    // sYAX token
    IYaxisBar public immutable yaxisBar;

    constructor(
        address _yax,
        address _yaxisChef,
        address _yaxisBar,
        address _yaxEthUniswapV2Pair
    )
        public
    {
        yax = IERC20(_yax);
        chef = IMasterChef(_yaxisChef);
        yaxisBar = IYaxisBar(_yaxisBar);
        yaxEthUniswapV2Pair = IUniswapV2Pair(_yaxEthUniswapV2Pair);
    }

    function totalSupply()
        external
        view
        override
        returns (uint256 _supply)
    {
        (uint256 _yaxReserves,,) = yaxEthUniswapV2Pair.getReserves();
        _supply = yaxEthUniswapV2Pair.totalSupply();
        _supply = _supply == 0
            ? 1e18
            : _supply;
        uint256 _lpStakingYax = _yaxReserves
            .mul(yaxEthUniswapV2Pair.balanceOf(address(chef)))
            .div(_supply);
        _supply = sqrt(
            yax.totalSupply()
                .add(_lpStakingYax)
                .add(yaxisBar.availableBalance())
        );
    }

    function balanceOf(
        address _voter
    )
        external
        view
        override
        returns (uint256 _balance)
    {
        (uint256 _stakeAmount,,) = chef.userInfo(PID, _voter);
        (uint256 _yaxReserves,,) = yaxEthUniswapV2Pair.getReserves();
        uint256 _supply = yaxEthUniswapV2Pair.totalSupply();
        _supply = _supply == 0
            ? 1e18
            : _supply;
        uint256 _lpStakingYax = _yaxReserves
            .mul(_stakeAmount)
            .div(_supply)
            .add(chef.pendingYaxis(PID, _voter));
        _supply = yaxisBar.totalSupply();
        _supply = _supply == 0
            ? 1e18
            : _supply;
        uint256 _syaxAmount = yaxisBar.balanceOf(_voter)
            .mul(yaxisBar.availableBalance())
            .div(_supply);
        _balance = sqrt(
            yax.balanceOf(_voter)
                .add(_lpStakingYax)
                .add(_syaxAmount)
        );
    }

    function sqrt(
        uint256 x
    )
        private
        pure
        returns (uint256 y)
    {
        uint256 z = (x + 1) / 2;
        y = x;
        while (z < y) {
            y = z;
            z = (x / z + z) / 2;
        }
        y = y * (10 ** 9);
    }
}
