// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    function bg() public virtual {}
    function bh() public virtual {}
}

contract C is B {
    function missing() internal pure returns (bytes4) {
        // An array element no longer names the declaration.
        return [bg, bh][1].selector;
    }

    function present() internal pure returns (bytes4) {
        return bh.selector;
    }
}
