// SPDX-License-Identifier: MIT

pragma solidity ^0.6.12;

import "@openzeppelin/contracts/math/Math.sol";
import "@openzeppelin/contracts/math/SafeMath.sol";


library VirtualVote {
    using SafeMath for uint256;

    uint256 private constant _VOTE_DECAY_PERIOD = 1 days;

    struct Data {
        uint104 oldResult;
        uint104 result;
        uint48 time;
    }

    function current(VirtualVote.Data memory self) internal view returns(uint256) {
        uint256 timePassed = Math.min(_VOTE_DECAY_PERIOD, block.timestamp.sub(self.time));
        uint256 timeRemain = _VOTE_DECAY_PERIOD.sub(timePassed);
        return uint256(self.oldResult).mul(timeRemain).add(
            uint256(self.result).mul(timePassed)
        ).div(_VOTE_DECAY_PERIOD);
    }
}
