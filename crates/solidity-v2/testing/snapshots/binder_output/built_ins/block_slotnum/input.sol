pragma solidity *;

contract Test {
    function testBlock() public view returns (uint64) {
        return block.slotnum;
    }
}
