// SPDX-License-Identifier: MIT
pragma solidity *;

contract Base {
    function publicFn(uint256 x) public pure returns (uint256) {
        return x;
    }
}

contract Derived is Base {
    function encode() public pure returns (bytes memory) {
        // Expected regular external function type, or external view on public function.
        return abi.encodeCall(Base.publicFn, (1));
    }
}
