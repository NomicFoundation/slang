// SPDX-License-Identifier: MIT

pragma solidity ^0.6.12;

import "@openzeppelin/contracts/math/SafeMath.sol";
import "./SafeCast.sol";
import "./VirtualVote.sol";
import "./Vote.sol";


library LiquidVoting {
    using SafeMath for uint256;
    using SafeCast for uint256;
    using Vote for Vote.Data;
    using VirtualVote for VirtualVote.Data;

    struct Data {
        VirtualVote.Data data;
        uint256 _weightedSum;
        uint256 _defaultVotes;
        mapping(address => Vote.Data) votes;
    }

    function updateVote(
        LiquidVoting.Data storage self,
        address user,
        Vote.Data memory oldVote,
        Vote.Data memory newVote,
        uint256 balance,
        uint256 totalSupply,
        uint256 defaultVote,
        function(address, uint256, bool, uint256) emitEvent
    ) internal {
        return _update(self, user, oldVote, newVote, balance, balance, totalSupply, defaultVote, emitEvent);
    }

    function updateBalance(
        LiquidVoting.Data storage self,
        address user,
        Vote.Data memory oldVote,
        uint256 oldBalance,
        uint256 newBalance,
        uint256 newTotalSupply,
        uint256 defaultVote,
        function(address, uint256, bool, uint256) emitEvent
    ) internal {
        return _update(self, user, oldVote, newBalance == 0 ? Vote.init() : oldVote, oldBalance, newBalance, newTotalSupply, defaultVote, emitEvent);
    }

    function _update(
        LiquidVoting.Data storage self,
        address user,
        Vote.Data memory oldVote,
        Vote.Data memory newVote,
        uint256 oldBalance,
        uint256 newBalance,
        uint256 newTotalSupply,
        uint256 defaultVote,
        function(address, uint256, bool, uint256) emitEvent
    ) private {
        uint256 oldWeightedSum = self._weightedSum;
        uint256 newWeightedSum = oldWeightedSum;
        uint256 oldDefaultVotes = self._defaultVotes;
        uint256 newDefaultVotes = oldDefaultVotes;

        if (oldVote.isDefault()) {
            newDefaultVotes = newDefaultVotes.sub(oldBalance);
        } else {
            newWeightedSum = newWeightedSum.sub(oldBalance.mul(oldVote.get(defaultVote)));
        }

        if (newVote.isDefault()) {
            newDefaultVotes = newDefaultVotes.add(newBalance);
        } else {
            newWeightedSum = newWeightedSum.add(newBalance.mul(newVote.get(defaultVote)));
        }

        if (newWeightedSum != oldWeightedSum) {
            self._weightedSum = newWeightedSum;
        }

        if (newDefaultVotes != oldDefaultVotes) {
            self._defaultVotes = newDefaultVotes;
        }

        {
            uint256 newResult = newTotalSupply == 0 ? defaultVote : newWeightedSum.add(newDefaultVotes.mul(defaultVote)).div(newTotalSupply);
            VirtualVote.Data memory data = self.data;

            if (newResult != data.result) {
                VirtualVote.Data storage sdata = self.data;
                (sdata.oldResult, sdata.result, sdata.time) = (
                    data.current().toUint104(),
                    newResult.toUint104(),
                    block.timestamp.toUint48()
                );
            }
        }

        if (!newVote.eq(oldVote)) {
            self.votes[user] = newVote;
        }

        emitEvent(user, newVote.get(defaultVote), newVote.isDefault(), newBalance);
    }
}
