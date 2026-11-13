// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    function bg() public virtual {}
}

contract C is B {
    function missing(bool b) internal pure returns (bytes4) {
        // The conditional no longer names the declaration.
        return (b ? bg : bg).selector;
    }

    function present() internal pure returns (bytes4) {
        return bg.selector;
    }
}
