// SPDX-License-Identifier: MIT

pragma solidity ^0.6.2;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/math/SafeMath.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

import "../../interfaces/dYdXSoloMargin.sol";
import "../IConverter.sol";
import "./BaseStrategy.sol";

contract StrategydYdXSoloMargin is BaseStrategy {
    using SafeERC20 for IERC20;
    using Address for address;
    using SafeMath for uint256;

    address public dYdX;
    uint256 marketId;
    IConverter public converter;

    /**
       * @param _dYdX The address of the dYdX Solo Margin contract
       * @param _marketId The dYdX Solo Margin Market ID: https://docs.dydx.exchange/#solo-markets
       * @param _converter The address of the converter
       * @param _controller The address of the controller
       * @param _vaultManager The address of the vaultManager
       * @param _weth The address of WETH
       * @param _router The address of the router for swapping tokens
       */
    constructor(
        address _dYdX,
        uint256 _marketId,
        address _converter,
        address _controller,
        address _vaultManager,
        address _weth,
        address _router
    )
    public
    BaseStrategy(
        string(abi.encodePacked("dYdX SoloMargin: ", ERC20(ISoloMargin(_dYdX).getMarketTokenAddress(_marketId)).symbol())),
        _controller,
        _vaultManager,
        ISoloMargin(_dYdX).getMarketTokenAddress(_marketId),
        _weth,
        _router
    )
    {
        require(_dYdX != address(0), "!_dYdX");
        require(_converter != address(0), "!_converter");
        dYdX = _dYdX;
        marketId = _marketId;
        converter = IConverter(_converter);
        IERC20(ISoloMargin(_dYdX).getMarketTokenAddress(_marketId)).safeApprove(_dYdX, type(uint256).max);
    }

    function _deposit() internal override {
        uint256 _amount = balanceOfWant();
        if (_amount > 0) {
            Account.Info[] memory accounts = new Account.Info[](1);
            accounts[0] = Account.Info({
                owner: address(this),
                number: 0 // Should be MARGIN
            });

            Actions.ActionArgs[] memory actions = new Actions.ActionArgs[](1);
            actions[0] = Actions.ActionArgs({
                actionType: Actions.ActionType.Deposit,
                accountId: 0,
                amount: Types.AssetAmount({
                    sign: true,
                    denomination: Types.AssetDenomination.Wei,
                    ref: Types.AssetReference.Delta,
                    value: _amount
                }),
                primaryMarketId: marketId,
                secondaryMarketId: 0,
                otherAddress: address(this),
                otherAccountId: 0,
                data: ""
            });

            ISoloMargin(dYdX).operate(accounts, actions);
        }
    }

    function _harvest() internal override {
        // Harvest is not necessary in this strategy
        return;
    }

    function _withdraw(uint256 _amount) internal override {
        Account.Info[] memory accounts = new Account.Info[](1);
        accounts[0] = Account.Info({
            owner: address(this),
            number: 0 // Should be MARGIN
        });

        Actions.ActionArgs[] memory actions = new Actions.ActionArgs[](1);
        actions[0] = Actions.ActionArgs({
            actionType: Actions.ActionType.Withdraw,
            accountId: 0,
            amount: Types.AssetAmount({
                sign: false,
                denomination: Types.AssetDenomination.Wei,
                ref: Types.AssetReference.Delta,
                value: _amount
            }),
            primaryMarketId: marketId,
            secondaryMarketId: 0,
            otherAddress: address(this),
            otherAccountId: 0,
            data: ""
        });

        ISoloMargin(dYdX).operate(accounts, actions);

        _amount = balanceOfWant();
        if (_amount > 0) {
            _convert(want, _vaultWant(), _amount);
        }
    }

    function _withdrawAll() internal override {
        uint256 amount = balanceOfPool();
        if (amount > 0) {
            _withdraw(amount);
        }
    }

    function balanceOfPool() public view override returns (uint256) {
        Account.Info memory account = Account.Info({
            owner: address(this),
            number: 0 // Should be MARGIN
        });
        Types.Wei memory balance = ISoloMargin(dYdX).getAccountWei(account, marketId);
        if (balance.sign) {
            return balance.value;
        }
        return 0;
    }

    function _convert(address _from, address _to, uint256 _amount) internal {
        require(converter.convert_rate(_from, _to, _amount) > 0, "!convert_rate");
        IERC20(_from).safeTransfer(address(converter), _amount);
        converter.convert(_from, _to, _amount);
    }
}
