// SPDX-License-Identifier: MIT

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "../interfaces/IMooniswapFactoryGovernance.sol";
import "../libraries/LiquidVoting.sol";
import "../libraries/MooniswapConstants.sol";
import "../libraries/SafeCast.sol";


abstract contract MooniswapGovernance is ERC20, Ownable, ReentrancyGuard {
    using Vote for Vote.Data;
    using LiquidVoting for LiquidVoting.Data;
    using VirtualVote for VirtualVote.Data;
    using SafeCast for uint256;

    event FeeVoteUpdate(address indexed user, uint256 fee, bool isDefault, uint256 amount);
    event SlippageFeeVoteUpdate(address indexed user, uint256 slippageFee, bool isDefault, uint256 amount);
    event DecayPeriodVoteUpdate(address indexed user, uint256 decayPeriod, bool isDefault, uint256 amount);

    IMooniswapFactoryGovernance public mooniswapFactoryGovernance;
    LiquidVoting.Data private _fee;
    LiquidVoting.Data private _slippageFee;
    LiquidVoting.Data private _decayPeriod;

    constructor(IMooniswapFactoryGovernance _mooniswapFactoryGovernance) internal {
        mooniswapFactoryGovernance = _mooniswapFactoryGovernance;
        _fee.data.result = _mooniswapFactoryGovernance.defaultFee().toUint104();
        _slippageFee.data.result = _mooniswapFactoryGovernance.defaultSlippageFee().toUint104();
        _decayPeriod.data.result = _mooniswapFactoryGovernance.defaultDecayPeriod().toUint104();
    }

    function setMooniswapFactoryGovernance(IMooniswapFactoryGovernance newMooniswapFactoryGovernance) external onlyOwner {
        mooniswapFactoryGovernance = newMooniswapFactoryGovernance;
        this.discardFeeVote();
        this.discardSlippageFeeVote();
        this.discardDecayPeriodVote();
    }

    function fee() public view returns(uint256) {
        return _fee.data.current();
    }

    function slippageFee() public view returns(uint256) {
        return _slippageFee.data.current();
    }

    function decayPeriod() public view returns(uint256) {
        return _decayPeriod.data.current();
    }

    function virtualFee() external view returns(uint104, uint104, uint48) {
        return (_fee.data.oldResult, _fee.data.result, _fee.data.time);
    }

    function virtualSlippageFee() external view returns(uint104, uint104, uint48) {
        return (_slippageFee.data.oldResult, _slippageFee.data.result, _slippageFee.data.time);
    }

    function virtualDecayPeriod() external view returns(uint104, uint104, uint48) {
        return (_decayPeriod.data.oldResult, _decayPeriod.data.result, _decayPeriod.data.time);
    }

    function feeVotes(address user) external view returns(uint256) {
        return _fee.votes[user].get(mooniswapFactoryGovernance.defaultFee);
    }

    function slippageFeeVotes(address user) external view returns(uint256) {
        return _slippageFee.votes[user].get(mooniswapFactoryGovernance.defaultSlippageFee);
    }

    function decayPeriodVotes(address user) external view returns(uint256) {
        return _decayPeriod.votes[user].get(mooniswapFactoryGovernance.defaultDecayPeriod);
    }

    function feeVote(uint256 vote) external {
        require(vote <= MooniswapConstants._MAX_FEE, "Fee vote is too high");

        _fee.updateVote(msg.sender, _fee.votes[msg.sender], Vote.init(vote), balanceOf(msg.sender), totalSupply(), mooniswapFactoryGovernance.defaultFee(), _emitFeeVoteUpdate);
    }

    function slippageFeeVote(uint256 vote) external {
        require(vote <= MooniswapConstants._MAX_SLIPPAGE_FEE, "Slippage fee vote is too high");

        _slippageFee.updateVote(msg.sender, _slippageFee.votes[msg.sender], Vote.init(vote), balanceOf(msg.sender), totalSupply(), mooniswapFactoryGovernance.defaultSlippageFee(), _emitSlippageFeeVoteUpdate);
    }

    function decayPeriodVote(uint256 vote) external {
        require(vote <= MooniswapConstants._MAX_DECAY_PERIOD, "Decay period vote is too high");
        require(vote >= MooniswapConstants._MIN_DECAY_PERIOD, "Decay period vote is too low");

        _decayPeriod.updateVote(msg.sender, _decayPeriod.votes[msg.sender], Vote.init(vote), balanceOf(msg.sender), totalSupply(), mooniswapFactoryGovernance.defaultDecayPeriod(), _emitDecayPeriodVoteUpdate);
    }

    function discardFeeVote() external {
        _fee.updateVote(msg.sender, _fee.votes[msg.sender], Vote.init(), balanceOf(msg.sender), totalSupply(), mooniswapFactoryGovernance.defaultFee(), _emitFeeVoteUpdate);
    }

    function discardSlippageFeeVote() external {
        _slippageFee.updateVote(msg.sender, _slippageFee.votes[msg.sender], Vote.init(), balanceOf(msg.sender), totalSupply(), mooniswapFactoryGovernance.defaultSlippageFee(), _emitSlippageFeeVoteUpdate);
    }

    function discardDecayPeriodVote() external {
        _decayPeriod.updateVote(msg.sender, _decayPeriod.votes[msg.sender], Vote.init(), balanceOf(msg.sender), totalSupply(), mooniswapFactoryGovernance.defaultDecayPeriod(), _emitDecayPeriodVoteUpdate);
    }

    function _emitFeeVoteUpdate(address account, uint256 newFee, bool isDefault, uint256 newBalance) private {
        emit FeeVoteUpdate(account, newFee, isDefault, newBalance);
    }

    function _emitSlippageFeeVoteUpdate(address account, uint256 newSlippageFee, bool isDefault, uint256 newBalance) private {
        emit SlippageFeeVoteUpdate(account, newSlippageFee, isDefault, newBalance);
    }

    function _emitDecayPeriodVoteUpdate(address account, uint256 newDecayPeriod, bool isDefault, uint256 newBalance) private {
        emit DecayPeriodVoteUpdate(account, newDecayPeriod, isDefault, newBalance);
    }

    function _beforeTokenTransfer(address from, address to, uint256 amount) internal override {
        if (from == to) {
            // ignore transfers to self
            return;
        }

        IMooniswapFactoryGovernance _mooniswapFactoryGovernance = mooniswapFactoryGovernance;
        bool updateFrom = !(from == address(0) || _mooniswapFactoryGovernance.isFeeCollector(from));
        bool updateTo = !(to == address(0) || _mooniswapFactoryGovernance.isFeeCollector(to));

        if (!updateFrom && !updateTo) {
            // mint to feeReceiver or burn from feeReceiver
            return;
        }

        uint256 balanceFrom = (from != address(0)) ? balanceOf(from) : 0;
        uint256 balanceTo = (to != address(0)) ? balanceOf(to) : 0;
        uint256 newTotalSupply = totalSupply()
            .add(from == address(0) ? amount : 0)
            .sub(to == address(0) ? amount : 0);

        ParamsHelper memory params = ParamsHelper({
            from: from,
            to: to,
            updateFrom: updateFrom,
            updateTo: updateTo,
            amount: amount,
            balanceFrom: balanceFrom,
            balanceTo: balanceTo,
            newTotalSupply: newTotalSupply
        });

        (uint256 defaultFee, uint256 defaultSlippageFee, uint256 defaultDecayPeriod) = _mooniswapFactoryGovernance.defaults();

        _updateOnTransfer(params, defaultFee, _emitFeeVoteUpdate, _fee);
        _updateOnTransfer(params, defaultSlippageFee, _emitSlippageFeeVoteUpdate, _slippageFee);
        _updateOnTransfer(params, defaultDecayPeriod, _emitDecayPeriodVoteUpdate, _decayPeriod);
    }

    struct ParamsHelper {
        address from;
        address to;
        bool updateFrom;
        bool updateTo;
        uint256 amount;
        uint256 balanceFrom;
        uint256 balanceTo;
        uint256 newTotalSupply;
    }

    function _updateOnTransfer(
        ParamsHelper memory params,
        uint256 defaultValue,
        function(address, uint256, bool, uint256) internal emitEvent,
        LiquidVoting.Data storage votingData
    ) private {
        Vote.Data memory voteFrom = votingData.votes[params.from];
        Vote.Data memory voteTo = votingData.votes[params.to];

        if (voteFrom.isDefault() && voteTo.isDefault() && params.updateFrom && params.updateTo) {
            emitEvent(params.from, voteFrom.get(defaultValue), true, params.balanceFrom.sub(params.amount));
            emitEvent(params.to, voteTo.get(defaultValue), true, params.balanceTo.add(params.amount));
            return;
        }

        if (params.updateFrom) {
            votingData.updateBalance(params.from, voteFrom, params.balanceFrom, params.balanceFrom.sub(params.amount), params.newTotalSupply, defaultValue, emitEvent);
        }

        if (params.updateTo) {
            votingData.updateBalance(params.to, voteTo, params.balanceTo, params.balanceTo.add(params.amount), params.newTotalSupply, defaultValue, emitEvent);
        }
    }
}
