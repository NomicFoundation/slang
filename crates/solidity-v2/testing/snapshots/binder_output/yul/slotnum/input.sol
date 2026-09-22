pragma solidity *;

contract Test {
    function test() public view returns (uint64 ret) {
        assembly {
            ret := slotnum()
        }
    }
}
