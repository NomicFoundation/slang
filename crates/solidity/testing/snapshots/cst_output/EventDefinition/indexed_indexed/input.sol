// This replicates a solc's bug, where we could specify indexed multiple times.
// This was fixed in 0.8.18, see <https://github.com/ethereum/solidity/pull/13816>.
event Updated(uint256 indexed indexed id);
